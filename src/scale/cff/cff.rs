//! Adobe compact font format.

use super::hint::{Hinter, HinterState};
use super::internal::*;
use crate::{FontRef, GlyphId};

use super::TRACE;

use core::ops::Range;

pub const CFF_: RawTag = raw_tag(b"CFF ");
pub const CFF2: RawTag = raw_tag(b"CFF2");

/// Compact font format (CFF or CFF2).
#[derive(Copy, Clone)]
pub struct Cff<'a> {
    data: &'a [u8],
    proxy: CffProxy,
}

impl<'a> Cff<'a> {
    /// Returns the glyph with the specified glyph id.
    pub fn get(&self, glyph_id: GlyphId) -> Option<Glyph> {
        Glyph::new(self.data, &self.proxy, glyph_id)
    }
}

/// Proxy for rematerializing a CFF table.
#[derive(Copy, Clone)]
pub struct CffProxy {
    cff: u32,
    gsubrs: Index,
    subrs: Index,
    char_strings: Index,
    fd_array: Index,
    fd_select: u32,
    private: (u32, u32),
    vstore: u32,
    vsindex: u16,
    font_matrix: Option<Transform>,
    is_cff2: bool,
    is_simple: bool,
    units_per_em: u16,
}

impl CffProxy {
    pub fn from_font<'a>(font: &FontRef<'a>) -> Option<Self> {
        let mut offset = font.table_offset(CFF2);
        if offset == 0 {
            offset = font.table_offset(CFF_);
        }
        if offset == 0 {
            return None;
        }
        let units_per_em = font.head().map(|head| head.units_per_em()).unwrap_or(1000);

        Self::parse(font.data, offset, units_per_em)
    }

    pub fn materialize<'a>(&self, font: &FontRef<'a>) -> Cff<'a> {
        let data = font.data.get(self.cff as usize..).unwrap_or(&[]);
        Cff { data, proxy: *self }
    }

    fn parse(data: &[u8], cff: u32, units_per_em: u16) -> Option<Self> {
        let data = data.get(cff as usize..)?;
        let mut c = Stream::new(data);
        let major = c.read::<u8>()?;
        let _minor = c.read::<u8>()?;
        let is_cff2 = if major == 2 {
            true
        } else if major != 1 {
            return None;
        } else {
            false
        };
        let header_size = c.read::<u8>()? as usize;
        let top_dict_range;
        let gsubrs = if is_cff2 {
            if header_size < 5 {
                return None;
            }
            let top_dict_len = c.read::<u16>()? as usize;
            c.set_offset(header_size)?;
            let start = c.offset();
            top_dict_range = start..start + top_dict_len;
            c.skip(top_dict_len);
            Index::new2(data, c.offset() as u32)?
        } else {
            if header_size < 4 {
                return None;
            }
            c.set_offset(header_size)?;
            c.skip(Index::new(data, c.offset() as u32)?.len(data)? as usize)?;
            let top_idx = Index::new(data, c.offset() as u32)?;
            if top_idx.count(data) != 1 {
                return None;
            }
            c.skip(top_idx.len(data)?)?;
            top_dict_range = top_idx.get_range(data, 0)?;
            c.skip(Index::new(data, c.offset() as u32)?.len(data)? as usize)?;
            Index::new(data, c.offset() as u32)?
        };
        let mut loader = Loader {
            char_strings: 0,
            subrs: 0,
            private: 0..0,
            fd_array: 0,
            fd_select: 0,
            font_matrix: None,
            vstore: 0,
            vsindex: 0,
            ok: true,
            units_per_em,
        };
        parse_dict(data, top_dict_range, None, &mut loader)?;
        if !loader.ok {
            return None;
        }
        let mut is_simple = !is_cff2;
        let mut fd_array = Index::empty();
        let mut private = (0u32, 0u32);
        let mut vstore = 0;
        if is_cff2 {
            if loader.vstore != 0 {
                vstore = loader.vstore as u32 + 2;
            }
            fd_array = Index::new2(data, loader.fd_array as u32)?;
            if fd_array.count(data) == 0 {
                is_simple = true;
            } else if fd_array.count(data) == 1 {
                is_simple = true;
                let font_dict_range = fd_array.get_range(data, 0)?;
                parse_dict(data, font_dict_range, None, &mut loader)?;
                private = (loader.private.start as u32, loader.private.end as u32);
                if private.0 != 0 {
                    let blend = if vstore != 0 {
                        Some(BlendData::new(data, vstore, &[]))
                    } else {
                        None
                    };
                    parse_dict(data, loader.private.clone(), blend, &mut loader)?;
                }
            } else if loader.fd_select == 0 {
                return None;
            }
        } else {
            private = (loader.private.start as u32, loader.private.end as u32);
            if loader.fd_array != 0 {
                fd_array = Index::new(data, loader.fd_array as u32)?;
                is_simple = false;
            }
        }
        if private.0 != 0 {
            parse_dict(data, loader.private.clone(), None, &mut loader)?;
        }
        if !loader.ok || loader.char_strings == 0 {
            return None;
        }
        let char_strings = if is_cff2 {
            Index::new2(data, loader.char_strings as u32)?
        } else {
            Index::new(data, loader.char_strings as u32)?
        };
        let subrs = if loader.subrs != 0 {
            if is_cff2 {
                Index::new2(data, loader.subrs as u32)?
            } else {
                Index::new(data, loader.subrs as u32)?
            }
        } else {
            Index::empty()
        };
        Some(Self {
            cff,
            gsubrs,
            subrs,
            char_strings,
            fd_array,
            fd_select: loader.fd_select as u32,
            font_matrix: loader.font_matrix,
            private,
            vstore,
            vsindex: loader.vsindex,
            is_cff2,
            is_simple,
            units_per_em: loader.units_per_em,
        })
    }
}

/// Compact font format glyph outline.
#[derive(Copy, Clone)]
pub struct Glyph<'a> {
    data: &'a [u8],
    proxy: CffProxy,
    fd: u16,
    private: (u32, u32),
    vsindex: u16,
    subrs: Index,
    range: (u32, u32),
}

impl<'a> Glyph<'a> {
    /// Returns a char string for the specified CFF table and glyph id.
    fn new(data: &'a [u8], proxy: &CffProxy, glyph_id: GlyphId) -> Option<Self> {
        let range = proxy.char_strings.get_range(data, glyph_id as u32)?;
        let mut fd = 0u16;
        let mut private = proxy.private;
        let mut vsindex = proxy.vsindex;
        let mut subrs = proxy.subrs;
        if !proxy.is_simple {
            fd = parse_fd_select(data, proxy.fd_select, glyph_id)? as u16;
            let mut loader = Loader {
                char_strings: 0,
                subrs: 0,
                private: 0..0,
                fd_array: 0,
                fd_select: 0,
                font_matrix: None,
                vstore: 0,
                vsindex,
                ok: true,
                units_per_em: proxy.units_per_em,
            };
            let fd_range = proxy.fd_array.get_range(data, fd as u32)?;
            let blend = if proxy.vstore != 0 {
                Some(BlendData::new(data, proxy.vstore, &[]))
            } else {
                None
            };
            parse_dict(data, fd_range, blend, &mut loader)?;
            private = (loader.private.start as u32, loader.private.end as u32);
            parse_dict(data, loader.private.clone(), blend, &mut loader)?;
            vsindex = loader.vsindex;
            if loader.subrs != 0 {
                subrs = if proxy.is_cff2 {
                    Index::new2(data, loader.subrs as u32)?
                } else {
                    Index::new(data, loader.subrs as u32)?
                };
            }
        }
        Some(Self {
            data,
            proxy: *proxy,
            fd,
            private,
            vsindex,
            subrs,
            range: (range.start as u32, range.end as u32),
        })
    }

    /// Returns the index of the associated font dictionary.
    pub fn subfont_index(&self) -> u16 {
        self.fd
    }

    /// Computes the bounding box of the glyph.
    pub fn _bounds(&self, scale: f32, coords: &[i16]) -> [f32; 4] {
        let mut sink = PathBounds {
            some: false,
            xmin: core::f32::MAX,
            xmax: core::f32::MIN,
            ymin: core::f32::MAX,
            ymax: core::f32::MIN,
        };
        if let Some(xform) = self.proxy.font_matrix {
            let xform = Transform::combine(&xform, &Transform::scale(scale));
            let mut sink = TransformSink {
                xform,
                inner: &mut sink,
            };
            self.parse(coords, &mut sink);
        } else if scale != 1. {
            let mut sink = ScaleSink {
                scale,
                inner: &mut sink,
            };
            self.parse(coords, &mut sink);
        } else {
            self.parse(coords, &mut sink);
        }
        if sink.some {
            [sink.xmin, sink.xmax, sink.ymin, sink.ymax]
        } else {
            [0.; 4]
        }
    }

    /// Evaluates the glyph, directing the result to the specified sink.
    pub fn path(
        &self,
        scale: f32,
        coords: &[i16],
        hinting: Option<&HinterState>,
        sink: &mut impl GlyphSink,
    ) -> bool {
        if let Some(state) = hinting {
            let scale = state.scale();
            if let Some(mut xform) = self.proxy.font_matrix {
                xform.x *= scale;
                xform.y *= scale;
                let mut sink = TransformSink { xform, inner: sink };
                let mut hinter = Hinter::new(state, &mut sink);
                self.parse(coords, &mut hinter)
            } else {
                let mut hinter = Hinter::new(state, sink);
                self.parse(coords, &mut hinter)
            }
        } else if let Some(xform) = self.proxy.font_matrix {
            let xform = Transform::combine(&xform, &Transform::scale(scale));
            let mut sink = TransformSink { xform, inner: sink };
            self.parse(coords, &mut sink)
        } else {
            let mut sink = ScaleSink { scale, inner: sink };
            self.parse(coords, &mut sink)
        }
    }

    /// Attempts to parse the private dictionary associated with the char string,
    /// directing the results to the supplied sink.
    pub(super) fn eval_private_dict(&self, coords: &[i16], sink: &mut impl DictionarySink) -> bool {
        if self.private.0 == 0 {
            return true;
        }
        let blend = if self.proxy.vstore != 0 {
            Some(BlendData::new(self.data, self.proxy.vstore, coords))
        } else {
            None
        };
        let range = self.private.0 as usize..self.private.1 as usize;
        parse_dict(self.data, range, blend, sink).is_some()
    }

    pub(super) fn parse(&self, coords: &[i16], sink: &mut impl GlyphSink) -> bool {
        if self.range.0 == self.range.1 {
            return true;
        }
        let mut state = ParseState {
            open: false,
            have_width: false,
            stem_count: 0,
            vsindex: self.vsindex,
        };
        let blend = if self.proxy.vstore != 0 && !coords.is_empty() {
            Some(BlendData::new(self.data, self.proxy.vstore, coords))
        } else {
            None
        };
        let mut blend_state = BlendState::new();
        let range = self.range.0 as usize..self.range.1 as usize;
        let mut stack = Stack::new();
        self.parse_imp(
            &mut state,
            blend,
            &mut blend_state,
            &mut stack,
            range,
            0,
            0.,
            0.,
            sink,
        )
        .is_some()
    }

    fn parse_imp(
        &self,
        s: &mut ParseState,
        blend: Option<BlendData>,
        blend_state: &mut BlendState,
        stack: &mut Stack,
        range: Range<usize>,
        depth: u32,
        mut x: f32,
        mut y: f32,
        b: &mut impl GlyphSink,
    ) -> Option<(f32, f32)> {
        if depth > 10 {
            return None;
        }
        use opcodes::*;
        let mut c = Stream::with_range(self.data, range)?;
        while c.remaining() != 0 {
            let mut op = c.read::<u8>()? as u16;
            if op == ESCAPE {
                let b1 = c.read::<u8>()?;
                op = b1 as u16 | 12 << 8;
            }
            if TRACE && op < 38 {
                //println!("{}", opcodes::NAMES[op as usize]);
            }
            match op {
                0 | 2 | 9 | 13 | 17 => {
                    return None;
                }
                VSINDEX => {
                    if !self.proxy.is_cff2 {
                        return None;
                    }
                    if stack.is_empty() {
                        return None;
                    }
                    s.vsindex = stack.pop() as u16;
                }
                BLEND => {
                    if !self.proxy.is_cff2 {
                        return None;
                    }
                    let mut total = stack.len();
                    if total < 1 {
                        return None;
                    }
                    total -= 1;
                    let count = stack.pop() as usize;
                    if count > total {
                        return None;
                    }
                    if let Some(ref blend) = blend {
                        total = blend_state.apply(
                            blend,
                            s.vsindex,
                            &mut stack.elements[0..total],
                            count,
                        )?;
                    }
                    stack.top -= total - count;
                }
                SHORTINT => {
                    stack.push(c.read::<i16>()? as f32)?;
                }
                32..=246 => {
                    stack.push(op as f32 - 139.)?;
                }
                247..=250 => {
                    let b1 = c.read::<u8>()? as f32;
                    stack.push((op as f32 - 247.) * 256. + b1 + 108.)?;
                }
                251..=254 => {
                    let b1 = c.read::<u8>()? as f32;
                    stack.push(-((op as f32 - 251.) * 256. + b1) - 108.)?;
                }
                255 => {
                    stack.push(c.read::<i32>()? as f32 / 65536.)?;
                }
                RETURN => {
                    break;
                }
                ENDCHAR => {
                    if !stack.is_empty() && !s.have_width {
                        s.have_width = true;
                        stack.clear();
                    }
                    if s.open {
                        s.open = false;
                        b.close();
                    }
                    break;
                }
                HSTEM | VSTEM | HSTEMHM | VSTEMHM => {
                    let mut i = 0;
                    let len = if stack.is_odd() && !s.have_width {
                        s.have_width = true;
                        i = 1;
                        stack.len() - 1
                    } else {
                        stack.len()
                    };
                    let horz = op == HSTEM || op == HSTEMHM;
                    let mut u = 0.;
                    while i < stack.len() {
                        u += stack.get(i);
                        let w = stack.get(i + 1);
                        let v = u + w;
                        if horz {
                            b.hstem(u, v);
                        } else {
                            b.vstem(u, v);
                        }
                        u = v;
                        i += 2;
                    }
                    s.stem_count += len / 2;
                    stack.clear();
                }
                HINTMASK | CNTRMASK => {
                    let mut i = 0;
                    let len = if stack.is_odd() && !s.have_width {
                        s.have_width = true;
                        i = 1;
                        stack.len() - 1
                    } else {
                        stack.len()
                    };
                    let mut u = 0.;
                    while i < stack.len() {
                        u += stack.get(i);
                        let w = stack.get(i + 1);
                        let v = u + w;
                        b.vstem(u, v);
                        u = v;
                        i += 2;
                    }
                    s.stem_count += len / 2;
                    let count = (s.stem_count + 7) / 8;
                    let mask = c.read_bytes(count)?;
                    if op == HINTMASK {
                        b.hint_mask(mask);
                    } else {
                        b.counter_mask(mask);
                    }
                    stack.clear();
                }
                RMOVETO => {
                    let mut i = 0;
                    if stack.len() == 3 && !s.have_width {
                        s.have_width = true;
                        i = 1;
                    } else if stack.len() < 2 {
                        return None;
                    }
                    if !s.open {
                        s.open = true;
                    } else {
                        b.close();
                    }
                    x += stack.get(i);
                    y += stack.get(i + 1);
                    b.move_to(x, y);
                    stack.clear();
                }
                HMOVETO | VMOVETO => {
                    let mut i = 0;
                    if stack.len() == 2 && !s.have_width {
                        s.have_width = true;
                        i = 1;
                    } else if stack.is_empty() {
                        return None;
                    }
                    if !s.open {
                        s.open = true;
                    } else {
                        b.close();
                    }
                    if op == HMOVETO {
                        x += stack.get(i);
                    } else {
                        y += stack.get(i);
                    }
                    b.move_to(x, y);
                    stack.clear();
                }
                RLINETO => {
                    if stack.is_odd() {
                        return None;
                    }
                    let mut i = 0;
                    while i < stack.len() {
                        x += stack.get(i);
                        y += stack.get(i + 1);
                        b.line_to(x, y);
                        i += 2;
                    }
                    stack.clear();
                }
                HLINETO => {
                    let mut i = 0;
                    while i < stack.len() {
                        x += stack.get(i);
                        i += 1;
                        b.line_to(x, y);
                        if i == stack.len() {
                            break;
                        }
                        y += stack.get(i);
                        i += 1;
                        b.line_to(x, y);
                    }
                    stack.clear();
                }
                VLINETO => {
                    let mut i = 0;
                    while i < stack.len() {
                        y += stack.get(i);
                        i += 1;
                        b.line_to(x, y);
                        if i == stack.len() {
                            break;
                        }
                        x += stack.get(i);
                        i += 1;
                        b.line_to(x, y);
                    }
                    stack.clear();
                }
                RRCURVETO => {
                    if stack.len() % 6 != 0 {
                        return None;
                    }
                    let mut i = 0;
                    while i < stack.len() {
                        let x1 = x + stack.get(i);
                        let y1 = y + stack.get(i + 1);
                        let x2 = x1 + stack.get(i + 2);
                        let y2 = y1 + stack.get(i + 3);
                        x = x2 + stack.get(i + 4);
                        y = y2 + stack.get(i + 5);
                        b.curve_to(x1, y1, x2, y2, x, y);
                        i += 6;
                    }
                    stack.clear();
                }
                RCURVELINE => {
                    if stack.len() < 8 || (stack.len() - 2) % 6 != 0 {
                        return None;
                    }
                    let mut i = 0;
                    while i < stack.len() - 2 {
                        let x1 = x + stack.get(i);
                        let y1 = y + stack.get(i + 1);
                        let x2 = x1 + stack.get(i + 2);
                        let y2 = y1 + stack.get(i + 3);
                        x = x2 + stack.get(i + 4);
                        y = y2 + stack.get(i + 5);
                        b.curve_to(x1, y1, x2, y2, x, y);
                        i += 6;
                    }
                    x += stack.get(i);
                    y += stack.get(i + 1);
                    b.line_to(x, y);
                    stack.clear();
                }
                RLINECURVE => {
                    if stack.len() < 8 || (stack.len() - 6) & 1 != 0 {
                        return None;
                    }
                    let mut i = 0;
                    while i < stack.len() - 6 {
                        x += stack.get(i);
                        y += stack.get(i + 1);
                        b.line_to(x, y);
                        i += 2;
                    }
                    let x1 = x + stack.get(i);
                    let y1 = y + stack.get(i + 1);
                    let x2 = x1 + stack.get(i + 2);
                    let y2 = y1 + stack.get(i + 3);
                    x = x2 + stack.get(i + 4);
                    y = y2 + stack.get(i + 5);
                    b.curve_to(x1, y1, x2, y2, x, y);
                    stack.clear();
                }
                VVCURVETO => {
                    let mut i = 0;
                    if stack.is_odd() {
                        x += stack.get(0);
                        i += 1;
                    }
                    if (stack.len() - i) % 4 != 0 {
                        return None;
                    }
                    while i < stack.len() {
                        let x1 = x;
                        let y1 = y + stack.get(i);
                        let x2 = x1 + stack.get(i + 1);
                        let y2 = y1 + stack.get(i + 2);
                        x = x2;
                        y = y2 + stack.get(i + 3);
                        b.curve_to(x1, y1, x2, y2, x, y);
                        i += 4;
                    }
                    stack.clear();
                }
                HHCURVETO => {
                    let mut i = 0;
                    if stack.is_odd() {
                        y += stack.get(0);
                        i += 1;
                    }
                    if (stack.len() - i) % 4 != 0 {
                        return None;
                    }
                    while i < stack.len() {
                        let x1 = x + stack.get(i);
                        let y1 = y;
                        let x2 = x1 + stack.get(i + 1);
                        let y2 = y1 + stack.get(i + 2);
                        x = x2 + stack.get(i + 3);
                        y = y2;
                        b.curve_to(x1, y1, x2, y2, x, y);
                        i += 4;
                    }
                    stack.clear();
                }
                VHCURVETO => {
                    if stack.len() < 4 {
                        return None;
                    }
                    stack.reverse();
                    while !stack.is_empty() {
                        if stack.len() < 4 {
                            return None;
                        }
                        let x1 = x;
                        let y1 = y + stack.pop();
                        let x2 = x1 + stack.pop();
                        let y2 = y1 + stack.pop();
                        x = x2 + stack.pop();
                        y = y2 + if stack.len() == 1 { stack.pop() } else { 0.0 };
                        b.curve_to(x1, y1, x2, y2, x, y);
                        if stack.is_empty() {
                            break;
                        }
                        if stack.len() < 4 {
                            return None;
                        }
                        let x1 = x + stack.pop();
                        let y1 = y;
                        let x2 = x1 + stack.pop();
                        let y2 = y1 + stack.pop();
                        y = y2 + stack.pop();
                        x = x2 + if stack.len() == 1 { stack.pop() } else { 0.0 };
                        b.curve_to(x1, y1, x2, y2, x, y);
                    }
                    debug_assert!(stack.is_empty());
                }
                HVCURVETO => {
                    if stack.len() < 4 {
                        return None;
                    }
                    stack.reverse();
                    while !stack.is_empty() {
                        if stack.len() < 4 {
                            return None;
                        }
                        let x1 = x + stack.pop();
                        let y1 = y;
                        let x2 = x1 + stack.pop();
                        let y2 = y1 + stack.pop();
                        y = y2 + stack.pop();
                        x = x2 + if stack.len() == 1 { stack.pop() } else { 0.0 };
                        b.curve_to(x1, y1, x2, y2, x, y);
                        if stack.is_empty() {
                            break;
                        }
                        if stack.len() < 4 {
                            return None;
                        }
                        let x1 = x;
                        let y1 = y + stack.pop();
                        let x2 = x1 + stack.pop();
                        let y2 = y1 + stack.pop();
                        x = x2 + stack.pop();
                        y = y2 + if stack.len() == 1 { stack.pop() } else { 0.0 };
                        b.curve_to(x1, y1, x2, y2, x, y);
                    }
                    debug_assert!(stack.is_empty());
                }
                HFLEX => {
                    if stack.len() != 7 {
                        return None;
                    }
                    let dx1 = x + stack.get(0);
                    let dy1 = y;
                    let dx2 = dx1 + stack.get(1);
                    let dy2 = dy1 + stack.get(2);
                    let dx3 = dx2 + stack.get(3);
                    let dy3 = dy2;
                    let dx4 = dx3 + stack.get(4);
                    let dy4 = dy2;
                    let dx5 = dx4 + stack.get(5);
                    let dy5 = y;
                    x = dx5 + stack.get(6);
                    b.curve_to(dx1, dy1, dx2, dy2, dx3, dy3);
                    b.curve_to(dx4, dy4, dx5, dy5, x, y);
                    stack.clear();
                }
                FLEX => {
                    if stack.len() != 13 {
                        return None;
                    }
                    let dx1 = x + stack.get(0);
                    let dy1 = y + stack.get(1);
                    let dx2 = dx1 + stack.get(2);
                    let dy2 = dy1 + stack.get(3);
                    let dx3 = dx2 + stack.get(4);
                    let dy3 = dy2 + stack.get(5);
                    let dx4 = dx3 + stack.get(6);
                    let dy4 = dy3 + stack.get(7);
                    let dx5 = dx4 + stack.get(8);
                    let dy5 = dy4 + stack.get(9);
                    x = dx5 + stack.get(10);
                    y = dy5 + stack.get(11);
                    b.curve_to(dx1, dy1, dx2, dy2, dx3, dy3);
                    b.curve_to(dx4, dy4, dx5, dy5, x, y);
                    stack.clear();
                }
                HFLEX1 => {
                    if stack.len() != 9 {
                        return None;
                    }
                    let dx1 = x + stack.get(0);
                    let dy1 = y + stack.get(1);
                    let dx2 = dx1 + stack.get(2);
                    let dy2 = dy1 + stack.get(3);
                    let dx3 = dx2 + stack.get(4);
                    let dy3 = dy2;
                    let dx4 = dx3 + stack.get(5);
                    let dy4 = dy2;
                    let dx5 = dx4 + stack.get(6);
                    let dy5 = dy4 + stack.get(7);
                    x = dx5 + stack.get(8);
                    b.curve_to(dx1, dy1, dx2, dy2, dx3, dy3);
                    b.curve_to(dx4, dy4, dx5, dy5, x, y);
                    stack.clear();
                }
                FLEX1 => {
                    if stack.len() != 11 {
                        return None;
                    }
                    let dx1 = x + stack.get(0);
                    let dy1 = y + stack.get(1);
                    let dx2 = dx1 + stack.get(2);
                    let dy2 = dy1 + stack.get(3);
                    let dx3 = dx2 + stack.get(4);
                    let dy3 = dy2 + stack.get(5);
                    let dx4 = dx3 + stack.get(6);
                    let dy4 = dy3 + stack.get(7);
                    let dx5 = dx4 + stack.get(8);
                    let dy5 = dy4 + stack.get(9);
                    fn abs(x: f32) -> f32 {
                        if x < 0. {
                            -x
                        } else {
                            x
                        }
                    }
                    if abs(dx5 - x) > abs(dy5 - y) {
                        x = dx5 + stack.get(10);
                    } else {
                        y = dy5 + stack.get(10);
                    }
                    b.curve_to(dx1, dy1, dx2, dy2, dx3, dy3);
                    b.curve_to(dx4, dy4, dx5, dy5, x, y);
                    stack.clear();
                }
                CALLSUBR | CALLGSUBR => {
                    if stack.is_empty() {
                        return None;
                    }
                    let data = self.data;
                    let count = if op == CALLSUBR {
                        self.subrs.count(data)
                    } else {
                        self.proxy.gsubrs.count(data)
                    };
                    let bias = if count < 1240 {
                        107
                    } else if count < 33900 {
                        1131
                    } else {
                        32768
                    };
                    let index = stack.pop() as i32 + bias;
                    let range = if op == CALLSUBR {
                        self.subrs.get_range(data, index as u32)?
                    } else {
                        self.proxy.gsubrs.get_range(data, index as u32)?
                    };
                    let pos =
                        self.parse_imp(s, blend, blend_state, stack, range, depth + 1, x, y, b)?;
                    x = pos.0;
                    y = pos.1;
                }
                _ => {
                    return None;
                }
            }
        }
        Some((x, y))
    }
}

#[derive(Default)]
struct PathBounds {
    some: bool,
    xmin: f32,
    xmax: f32,
    ymin: f32,
    ymax: f32,
}

impl PathBounds {
    fn insert(&mut self, x: f32, y: f32) {
        if x < self.xmin {
            self.xmin = x;
        }
        if x > self.xmax {
            self.xmax = x;
        }
        if y < self.ymin {
            self.ymin = y;
        }
        if y > self.ymax {
            self.ymax = y;
        }
    }
}

impl GlyphSink for PathBounds {
    fn move_to(&mut self, x: f32, y: f32) {
        self.some = true;
        self.insert(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.some = true;
        self.insert(x, y);
    }

    fn curve_to(&mut self, cx1: f32, cy1: f32, cx2: f32, cy2: f32, x: f32, y: f32) {
        self.some = true;
        self.insert(cx1, cy1);
        self.insert(cx2, cy2);
        self.insert(x, y);
    }

    fn close(&mut self) {}
}

#[derive(Copy, Clone)]
struct Transform {
    pub xx: f32,
    pub xy: f32,
    pub yx: f32,
    pub yy: f32,
    pub x: f32,
    pub y: f32,
}

impl Transform {
    fn scale(scale: f32) -> Self {
        Self {
            xx: scale,
            xy: 0.,
            yx: 0.,
            yy: scale,
            x: 0.,
            y: 0.,
        }
    }

    #[inline(always)]
    fn transform_point(&self, x: f32, y: f32) -> (f32, f32) {
        (
            (x * self.xx + y * self.yx) + self.x,
            (x * self.xy + y * self.yy) + self.y,
        )
    }

    #[allow(clippy::suspicious_operation_groupings)]
    fn combine(a: &Transform, b: &Transform) -> Self {
        let xx = a.xx * b.xx + a.yx * b.xy;
        let yx = a.xx * b.yx + a.yx * b.yy;
        let xy = a.xy * b.xx + a.yy * b.xy;
        let yy = a.xy * b.yx + a.yy * b.yy;
        let x = a.x * b.xx + a.y * b.xy + b.x;
        let y = a.x * b.yx + a.y * b.yy + b.y;
        Self {
            xx,
            xy,
            yx,
            yy,
            x,
            y,
        }
    }
}

struct TransformSink<'a, S> {
    xform: Transform,
    inner: &'a mut S,
}

impl<'a, S: GlyphSink> GlyphSink for TransformSink<'a, S> {
    fn move_to(&mut self, x: f32, y: f32) {
        let (x, y) = self.xform.transform_point(x, y);
        self.inner.move_to(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let (x, y) = self.xform.transform_point(x, y);
        self.inner.line_to(x, y);
    }

    fn curve_to(&mut self, cx1: f32, cy1: f32, cx2: f32, cy2: f32, x: f32, y: f32) {
        let (cx1, cy1) = self.xform.transform_point(cx1, cy1);
        let (cx2, cy2) = self.xform.transform_point(cx2, cy2);
        let (x, y) = self.xform.transform_point(x, y);
        self.inner.curve_to(cx1, cy1, cx2, cy2, x, y);
    }

    fn close(&mut self) {
        self.inner.close();
    }
}

struct ScaleSink<'a, S> {
    scale: f32,
    inner: &'a mut S,
}

impl<'a, S: GlyphSink> GlyphSink for ScaleSink<'a, S> {
    fn move_to(&mut self, x: f32, y: f32) {
        let s = self.scale;
        self.inner.move_to(x * s, y * s);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let s = self.scale;
        self.inner.line_to(x * s, y * s);
    }

    fn curve_to(&mut self, cx1: f32, cy1: f32, cx2: f32, cy2: f32, x: f32, y: f32) {
        let s = self.scale;
        self.inner
            .curve_to(cx1 * s, cy1 * s, cx2 * s, cy2 * s, x * s, y * s);
    }

    fn close(&mut self) {
        self.inner.close();
    }
}

/// Trait for receiving the path commands of a glyph.
#[allow(unused_variables)]
pub trait GlyphSink {
    fn hstem(&mut self, y: f32, dy: f32) {}
    fn vstem(&mut self, x: f32, dx: f32) {}
    fn hint_mask(&mut self, mask: &[u8]) {}
    fn counter_mask(&mut self, mask: &[u8]) {}
    fn move_to(&mut self, x: f32, y: f32);
    fn line_to(&mut self, x: f32, y: f32);
    fn curve_to(&mut self, cx1: f32, cy1: f32, cx2: f32, cy2: f32, x: f32, y: f32);
    fn close(&mut self);
}

/// A trait that acts as a callback during dictionary parsing.
#[allow(unused_variables)]
pub trait DictionarySink {
    /// Specifies the char string type. Only type 2 is supported.
    fn char_string_type(&mut self, ty: u32) {}

    /// Specifies the offset to the char string index.
    fn char_strings(&mut self, offset: usize) {}

    /// Specifies the offset and length of the private dictionary.
    fn private_dictionary(&mut self, offset: usize, len: usize) {}

    /// Specifies the hinting alignment zones.
    fn blue_values(&mut self, values: &[f32]) {}

    /// Specifies the hinting alignment zones for the font family.
    fn family_blues(&mut self, values: &[f32]) {}

    /// Specifies other hinting alignment zones.
    fn other_blues(&mut self, values: &[f32]) {}

    /// Specifies other hinting alignment zones for the font family.
    fn family_other_blues(&mut self, values: &[f32]) {}

    /// Specifies a value for overshoot suppression.
    fn blue_scale(&mut self, scale: f32) {}

    /// Specifies a value for small overshoot supression.
    fn blue_shift(&mut self, shift: f32) {}

    /// Specifies an alignment zone extension.
    fn blue_fuzz(&mut self, fuzz: f32) {}

    /// Specifies the offset to the local subroutine index.
    fn subroutines(&mut self, offset: usize) {}

    /// Specifies the offset to the font dictionary index.
    fn fd_array(&mut self, offset: usize) {}

    /// Specifies the offset to the font dictionary selector table.
    fn fd_select(&mut self, offset: usize) {}

    fn font_matrix(&mut self, values: &[f32]) {}

    fn language_group(&mut self, group: u32) {}

    /// Specifies the offset to the variation store.
    fn variation_store(&mut self, offset: usize) {}

    /// Specifies the outer index in the variation store.
    fn variation_store_index(&mut self, index: u16) {}
}

struct Loader {
    char_strings: usize,
    subrs: usize,
    private: Range<usize>,
    fd_select: usize,
    fd_array: usize,
    font_matrix: Option<Transform>,
    vstore: usize,
    vsindex: u16,
    ok: bool,
    units_per_em: u16,
}

impl DictionarySink for Loader {
    fn char_string_type(&mut self, ty: u32) {
        if ty != 2 {
            self.ok = false;
        }
    }

    fn char_strings(&mut self, offset: usize) {
        self.char_strings = offset;
    }

    fn private_dictionary(&mut self, offset: usize, len: usize) {
        self.private = offset..offset + len;
    }

    fn subroutines(&mut self, offset: usize) {
        self.subrs = self.private.start + offset;
    }

    fn fd_select(&mut self, offset: usize) {
        self.fd_select = offset;
    }

    fn fd_array(&mut self, offset: usize) {
        self.fd_array = offset;
    }

    fn font_matrix(&mut self, values: &[f32]) {
        if values.len() == 6 {
            self.font_matrix = Some(Transform {
                xx: values[0] * self.units_per_em as f32,
                xy: values[1] * self.units_per_em as f32,
                yx: values[2] * self.units_per_em as f32,
                yy: values[3] * self.units_per_em as f32,
                x: values[4],
                y: values[5],
            });
        }
    }

    fn variation_store(&mut self, offset: usize) {
        self.vstore = offset;
    }

    fn variation_store_index(&mut self, index: u16) {
        self.vsindex = index;
    }
}

struct IndexMetadata {
    count: u32,
    offsets_offset: u32,
    data_offset: u32,
    offset_size: u32,
}

impl IndexMetadata {
    pub fn unpack(data: &[u8], key: u32) -> Option<Self> {
        let is_cff2 = key & 1 != 0;
        let offset = key >> 1;
        let b = Bytes::with_offset(data, offset as usize)?;
        if is_cff2 {
            let count = b.read::<u32>(0)?;
            if count == 0 {
                Some(Self {
                    count: 0,
                    offsets_offset: 4,
                    data_offset: 0,
                    offset_size: 0,
                })
            } else {
                let offset_size = b.read::<u8>(4)? as u32;
                let offsets_offset = offset + 5;
                let data_offset = 5 + offset_size * (count + 1);
                Some(Self {
                    count,
                    offsets_offset,
                    data_offset,
                    offset_size,
                })
            }
        } else {
            let count = b.read::<u16>(0)? as u32;
            if count == 0 {
                Some(Self {
                    count: 0,
                    offsets_offset: 2,
                    data_offset: 0,
                    offset_size: 0,
                })
            } else {
                let offset_size = b.read::<u8>(2)? as u32;
                let offsets_offset = offset + 3;
                let data_offset = 3 + offset_size * (count + 1);
                Some(Self {
                    count,
                    offsets_offset,
                    data_offset,
                    offset_size,
                })
            }
        }
    }
}

#[derive(Copy, Clone, Default)]
struct Index {
    key: u32,
}

impl Index {
    fn empty() -> Self {
        Self { key: 0 }
    }

    fn new(data: &[u8], offset: u32) -> Option<Self> {
        let key = offset << 1;
        IndexMetadata::unpack(data, key)?;
        Some(Self { key })
    }

    fn new2(data: &[u8], offset: u32) -> Option<Self> {
        let key = (offset << 1) | 1;
        IndexMetadata::unpack(data, key)?;
        Some(Self { key })
    }

    #[inline(always)]
    fn get_offset(&self, data: &[u8], meta: &IndexMetadata, index: u32) -> Option<usize> {
        let offsize = meta.offset_size;
        let base = (meta.offsets_offset + offsize * index) as usize;
        let b = Bytes::new(data);
        match offsize {
            1 => Some(b.read::<u8>(base)? as usize),
            2 => Some(b.read::<u16>(base)? as usize),
            3 => Some(b.read::<U24>(base)?.0 as usize),
            4 => Some(b.read::<u32>(base)? as usize),
            _ => None,
        }
    }

    fn count(&self, data: &[u8]) -> u32 {
        let is_cff2 = self.key & 1 != 0;
        if is_cff2 {
            Bytes::new(data).read_or_default::<u32>((self.key >> 1) as usize)
        } else {
            Bytes::new(data).read_or_default::<u16>((self.key >> 1) as usize) as u32
        }
    }

    fn len(&self, data: &[u8]) -> Option<usize> {
        let meta = IndexMetadata::unpack(data, self.key)?;
        if meta.count == 0 {
            return Some(meta.offsets_offset as usize);
        }
        let last = self.get_offset(data, &meta, meta.count)?;
        Some(meta.data_offset as usize + last - 1)
    }

    fn get_range(&self, data: &[u8], index: u32) -> Option<Range<usize>> {
        let meta = IndexMetadata::unpack(data, self.key)?;
        if index >= meta.count {
            return None;
        }
        let base = ((self.key >> 1) + meta.data_offset) as usize;
        let offset1 = self.get_offset(data, &meta, index)?;
        let offset2 = self.get_offset(data, &meta, index + 1)?;
        let start = base + offset1 - 1;
        let end = base + offset2 - 1;
        if end < start || end > data.len() {
            return None;
        }
        Some(start..end)
    }
}

#[derive(Copy, Clone)]
struct BlendData<'a> {
    store: Bytes<'a>,
    coords: &'a [i16],
}

impl<'a> BlendData<'a> {
    fn new(data: &'a [u8], store: u32, coords: &'a [i16]) -> Self {
        let store = Bytes::with_offset(data, store as usize).unwrap_or_else(|| Bytes::new(&[]));
        Self { store, coords }
    }
}

#[derive(Copy, Clone)]
struct BlendState {
    scalars: [f32; 32],
    region_count: usize,
    index: u16,
    present: bool,
}

impl BlendState {
    fn new() -> Self {
        Self {
            scalars: [0f32; 32],
            region_count: 0,
            index: 0,
            present: false,
        }
    }

    fn apply(
        &mut self,
        data: &BlendData,
        index: u16,
        values: &mut [f32],
        count: usize,
    ) -> Option<usize> {
        let len = values.len();
        if count >= len {
            return None;
        }
        if !self.present || index != self.index {
            let (ok, region_count) = Self::compute_scalars(data, index, &mut self.scalars)?;
            self.region_count = region_count;
            self.index = index;
            self.present = true;
            if !ok {
                for i in 0..region_count {
                    self.scalars[i] = 0.;
                }
            }
        }
        let region_count = self.region_count;
        let region_value_count = region_count * count;
        let total_count = region_value_count + count;
        if total_count > len {
            return None;
        }
        let base = len - total_count;
        let mut region_idx = base + count;
        for i in 0..count {
            let mut delta = 0.;
            for j in 0..region_count {
                delta += self.scalars[j] * values[region_idx + j];
            }
            region_idx += region_count;
            values[base + i] += delta;
        }
        Some(total_count)
    }

    #[allow(clippy::needless_range_loop)]
    fn compute_scalars(data: &BlendData, outer: u16, scalars: &mut [f32]) -> Option<(bool, usize)> {
        let b = &data.store;
        let vary_coords = data.coords;
        let coords_len = vary_coords.len();
        let store = 0;
        if outer >= b.read::<u16>(store + 6)? {
            return None;
        }
        let region_base = store + b.read::<u32>(store + 2)? as usize;
        let axis_count = b.read::<u16>(region_base)? as usize;
        let region_record_size = axis_count * 6;
        let region_count = b.read::<u16>(region_base + 2)? as usize;
        let data_base = store + b.read::<u32>(store + 8 + outer as usize * 4)? as usize;
        let region_index_base = data_base + 6;
        let region_index_count = b.read::<u16>(data_base + 4)? as usize;
        if region_index_count > scalars.len() {
            return Some((false, region_index_count));
        }
        for i in 0..region_index_count {
            let region_index = b.read::<u16>(region_index_base + i * 2)? as usize;
            if region_index >= region_count {
                return None;
            }
            let region_offset = region_base + 4 + region_index * region_record_size;
            let mut scalar = 1.;
            for axis in 0..axis_count {
                fn f2dot14_to_f32(x: i16) -> f32 {
                    ((x as i32) << 2) as f32 / 65536.
                }
                let region_axis_base = region_offset + axis * 6;
                let start = f2dot14_to_f32(b.read::<i16>(region_axis_base)?);
                let peak = f2dot14_to_f32(b.read::<i16>(region_axis_base + 2)?);
                let end = f2dot14_to_f32(b.read::<i16>(region_axis_base + 4)?);
                let coord = if axis >= coords_len {
                    0.
                } else {
                    f2dot14_to_f32(vary_coords[axis])
                };
                if (start > peak || peak > end)
                    || (start < 0. && end > 0. && peak != 0.)
                    || peak == 0.
                {
                    continue;
                } else if coord < start || coord > end {
                    scalar = 0.;
                    break;
                } else if coord == peak {
                    continue;
                } else if coord < peak {
                    scalar *= (coord - start) / (peak - start)
                } else {
                    scalar *= (end - coord) / (end - peak)
                };
            }
            scalars[i] = scalar;
        }
        Some((true, region_index_count))
    }
}

struct ParseState {
    open: bool,
    have_width: bool,
    stem_count: usize,
    vsindex: u16,
}

const MAX_STACK: usize = 513;

#[derive(Copy, Clone)]
struct Stack {
    elements: [f32; MAX_STACK],
    top: usize,
}

impl Stack {
    fn new() -> Self {
        Self {
            elements: [0.; MAX_STACK],
            top: 0,
        }
    }

    fn push(&mut self, value: f32) -> Option<()> {
        if self.top == MAX_STACK {
            return None;
        }
        self.elements[self.top] = value;
        self.top += 1;
        Some(())
    }

    fn pop(&mut self) -> f32 {
        self.top -= 1;
        self.elements[self.top]
    }

    fn len(&self) -> usize {
        self.top
    }

    fn is_empty(&self) -> bool {
        self.top == 0
    }

    fn is_odd(&self) -> bool {
        self.top & 1 == 1
    }

    fn get(&self, index: usize) -> f32 {
        self.elements[index]
    }

    fn reverse(&mut self) {
        self.elements.split_at_mut(self.top).0.reverse();
    }

    fn clear(&mut self) {
        self.top = 0;
    }
}

fn parse_dict<Sink: DictionarySink>(
    data: &[u8],
    range: Range<usize>,
    blend: Option<BlendData>,
    sink: &mut Sink,
) -> Option<()> {
    use opcodes::*;
    if range.is_empty() {
        return Some(());
    }
    let mut s = Stream::with_range(data, range)?;
    let mut operands = [0f32; MAX_STACK];
    let mut blend_state = BlendState::new();
    let mut vsindex = 0u16;
    loop {
        let (mut op, mut n) = parse_dict_entry(&mut s, &mut operands)?;
        if op == DICT_BLEND {
            if n == 0 {
                return None;
            }
            let count = operands[n - 1] as usize;
            if let Some(ref blend) = blend {
                blend_state.apply(blend, vsindex, &mut operands[0..n - 1], count);
                n = count;
            } else {
                n = count;
            }
            let (op2, n2) = parse_dict_entry(&mut s, &mut operands)?;
            if n2 != 0 {
                return None;
            }
            op = op2;
        }
        let ops = &mut operands[0..n];
        match op {
            0xFFFF => break,
            CHAR_STRING_TYPE => {
                if n != 1 {
                    return None;
                }
                sink.char_string_type(ops[0] as u32);
            }
            CHAR_STRINGS => {
                if n != 1 {
                    return None;
                }
                sink.char_strings(ops[0] as usize);
            }
            PRIVATE => {
                if n != 2 {
                    return None;
                }
                sink.private_dictionary(ops[1] as usize, ops[0] as usize);
            }
            FD_ARRAY => {
                if n != 1 {
                    return None;
                }
                sink.fd_array(ops[0] as usize);
            }
            FD_SELECT => {
                if n != 1 {
                    return None;
                }
                sink.fd_select(ops[0] as usize);
            }
            FONT_MATRIX => {
                if n == 6 {
                    sink.font_matrix(ops);
                }
            }
            LANGUAGE_GROUP => {
                sink.language_group(ops[0] as u32);
            }
            BLUE_VALUES => {
                delta_vector(ops);
                sink.blue_values(ops);
            }
            OTHER_BLUES => {
                delta_vector(ops);
                sink.other_blues(ops);
            }
            FAMILY_BLUES => {
                delta_vector(ops);
                sink.family_blues(ops);
            }
            FAMILY_OTHER_BLUES => {
                delta_vector(ops);
                sink.family_other_blues(ops);
            }
            BLUE_SCALE => {
                if n != 1 {
                    return None;
                }
                sink.blue_scale(ops[0]);
            }
            BLUE_SHIFT => {
                if n != 1 {
                    return None;
                }
                sink.blue_shift(ops[0]);
            }
            BLUE_FUZZ => {
                if n != 1 {
                    return None;
                }
                sink.blue_fuzz(ops[0]);
            }
            DICT_VSINDEX => {
                if n != 1 {
                    return None;
                }
                vsindex = ops[0] as u16;
            }
            DICT_BLEND => {}
            VSTORE => {
                if n != 1 {
                    return None;
                }
                sink.variation_store(ops[0] as usize);
            }
            STD_HW => {}
            STD_VW => {}
            STEM_SNAP_H => {}
            STEM_SNAP_V => {}
            SUBRS => {
                if n != 1 {
                    return None;
                }
                sink.subroutines(ops[0] as usize);
            }
            _ => {}
        }
    }
    Some(())
}

fn delta_vector(ops: &mut [f32]) {
    if ops.is_empty() {
        return;
    }
    let mut s = ops[0];
    for v in ops.iter_mut().skip(1) {
        s += *v;
        *v = s;
    }
}

fn parse_dict_entry(s: &mut Stream, operands: &mut [f32]) -> Option<(u16, usize)> {
    let max_operands = operands.len();
    let mut n = 0;
    loop {
        if s.remaining() == 0 {
            return Some((0xFFFF, 0));
        }
        let b0 = s.read::<u8>()?;
        if b0 <= 24 {
            let mut b1 = 0u8;
            if b0 == 12 {
                b1 = s.read()?;
            }
            return Some(((b0 as u16) << 8 | b1 as u16, n));
        }
        match b0 {
            28 | 29 | 32..=254 => {
                if n == max_operands {
                    return None;
                }
                operands[n] = parse_integer(s, b0 as i32)?;
                n += 1;
            }
            30 => {
                if n == max_operands {
                    return None;
                }
                operands[n] = parse_real(s)?;
                n += 1;
            }
            _ => {
                return None;
            }
        }
    }
}

#[inline(always)]
fn parse_integer(s: &mut Stream, b0: i32) -> Option<f32> {
    match b0 {
        28 => Some(s.read::<i16>()? as f32),
        29 => Some(s.read::<i32>()? as f32),
        32..=246 => Some((b0 - 139) as f32),
        247..=250 => Some(((b0 - 247) * 256 + s.read::<u8>()? as i32 + 108) as f32),
        251..=254 => Some((-(b0 - 251) * 256 - s.read::<u8>()? as i32 - 108) as f32),
        _ => None,
    }
}

fn parse_real(s: &mut Stream) -> Option<f32> {
    const MAX_LEN: usize = 64;
    let mut buf = [0u8; MAX_LEN];
    let mut n = 0;
    let mut done = false;
    let mut missing_exp = true;
    loop {
        let b = s.read::<u8>()?;
        let n0 = (b >> 4) & 0xF;
        let n1 = b & 0xF;
        for j in 0..2 {
            if n + 1 == MAX_LEN {
                return None;
            }
            let nibble = if j == 0 { n0 } else { n1 };
            match nibble {
                0x0..=0x9 => {
                    buf[n] = b'0' + nibble;
                    n += 1;
                    missing_exp = false;
                }
                0xA => {
                    buf[n] = b'.';
                    n += 1;
                }
                0xB => {
                    buf[n] = b'E';
                    n += 1;
                    missing_exp = true;
                }
                0xC => {
                    buf[n] = b'E';
                    buf[n + 1] = b'-';
                    n += 2;
                    missing_exp = true;
                }
                0xE => {
                    buf[n] = b'-';
                    n += 1;
                }
                0xF => {
                    done = true;
                    break;
                }
                _ => {
                    return None;
                }
            }
        }
        if done {
            break;
        }
    }
    if missing_exp && n < MAX_LEN {
        buf[n] = b'0';
        n += 1;
    }
    core::str::from_utf8(&buf[0..n]).map_or(None, |b| b.parse::<f32>().ok())
}

fn parse_fd_select(data: &[u8], offset: u32, glyph_id: u16) -> Option<usize> {
    if offset == 0 {
        return Some(0);
    }
    let d = Bytes::new(data);
    let base = offset as usize;
    let fmt = d.read::<u8>(base)?;
    if fmt == 0 {
        return Some(d.read::<u8>(base + 1 + glyph_id as usize)? as usize);
    } else if fmt == 3 {
        let nranges = d.read::<u16>(base + 1)? as usize;
        let mut l = 0;
        let mut h = nranges;
        while l < h {
            let i = (l + h) / 2;
            let rec = base + 3 + i * 3;
            let first = d.read::<u16>(rec)?;
            if glyph_id < first {
                h = i;
            } else if glyph_id >= d.read::<u16>(rec + 3)? {
                l = i + 1;
            } else {
                return Some(d.read::<u8>(rec + 2)? as usize);
            }
        }
    } else if fmt == 4 {
        let nranges = d.read::<u32>(base + 1)? as usize;
        let mut l = 0;
        let mut h = nranges;
        while l < h {
            let i = (l + h) / 2;
            let rec = base + 5 + i * 6;
            let first = d.read::<u32>(rec)?;
            if (glyph_id as u32) < first {
                h = i;
            } else if glyph_id as u32 > d.read::<u32>(rec + 6)? {
                l = i + 1;
            } else {
                return Some(d.read::<u16>(rec + 4)? as usize);
            }
        }
    }
    None
}

#[allow(dead_code)]
mod opcodes {
    // top
    pub const CHAR_STRING_TYPE: u16 = 12 << 8 | 6;
    pub const CHAR_STRINGS: u16 = 17 << 8;
    pub const PRIVATE: u16 = 18 << 8;
    pub const FD_ARRAY: u16 = 12 << 8 | 36;
    pub const FD_SELECT: u16 = 12 << 8 | 37;
    pub const FONT_MATRIX: u16 = 12 << 8 | 7;

    // private
    pub const BLUE_VALUES: u16 = 6 << 8;
    pub const OTHER_BLUES: u16 = 7 << 8;
    pub const FAMILY_BLUES: u16 = 8 << 8;
    pub const FAMILY_OTHER_BLUES: u16 = 9 << 8;
    pub const BLUE_SCALE: u16 = 12 << 8 | 9;
    pub const BLUE_SHIFT: u16 = 12 << 8 | 10;
    pub const BLUE_FUZZ: u16 = 12 << 8 | 11;
    pub const STD_HW: u16 = 10 << 8;
    pub const STD_VW: u16 = 11 << 8;
    pub const STEM_SNAP_H: u16 = 12 << 8 | 12;
    pub const STEM_SNAP_V: u16 = 12 << 8 | 13;
    pub const LANGUAGE_GROUP: u16 = 12 << 8 | 17;
    pub const SUBRS: u16 = 19 << 8;
    pub const DEFAULT_WIDTH_X: u16 = 20 << 8;
    pub const NOMINAL_WIDTH_X: u16 = 21 << 8;
    pub const DICT_VSINDEX: u16 = 22 << 8;
    pub const DICT_BLEND: u16 = 23 << 8;
    pub const VSTORE: u16 = 24 << 8;

    // char strings
    pub const HSTEM: u16 = 1;
    pub const VSTEM: u16 = 3;
    pub const VMOVETO: u16 = 4;
    pub const RLINETO: u16 = 5;
    pub const HLINETO: u16 = 6;
    pub const VLINETO: u16 = 7;
    pub const RRCURVETO: u16 = 8;
    pub const CALLSUBR: u16 = 10;
    pub const RETURN: u16 = 11;
    pub const ESCAPE: u16 = 12;
    pub const ENDCHAR: u16 = 14;
    pub const VSINDEX: u16 = 15;
    pub const BLEND: u16 = 16;
    pub const HSTEMHM: u16 = 18;
    pub const HINTMASK: u16 = 19;
    pub const CNTRMASK: u16 = 20;
    pub const RMOVETO: u16 = 21;
    pub const HMOVETO: u16 = 22;
    pub const VSTEMHM: u16 = 23;
    pub const RCURVELINE: u16 = 24;
    pub const RLINECURVE: u16 = 25;
    pub const VVCURVETO: u16 = 26;
    pub const HHCURVETO: u16 = 27;
    pub const SHORTINT: u16 = 28;
    pub const CALLGSUBR: u16 = 29;
    pub const VHCURVETO: u16 = 30;
    pub const HVCURVETO: u16 = 31;

    // 2 bytes
    pub const HFLEX: u16 = 34 | 12 << 8;
    pub const FLEX: u16 = 35 | 12 << 8;
    pub const HFLEX1: u16 = 36 | 12 << 8;
    pub const FLEX1: u16 = 37 | 12 << 8;

    #[rustfmt::skip]
    pub const NAMES: [&'static str; 38] = [
        "", "hstem", "", "vstem", "vmoveto", "rlineto", "hlineto", "vlineto", "rrcurveto", "",
        "callsubr", "return", "escape", "", "endchar", "vsindex", "blend", "", "hstemhm", "hintmask",
        "cntrmask", "rmoveto", "hmoveto", "vstemhm", "rcurveline", "rlinecurve", "vvcurveto",
        "hhcurveto", "shortint", "callgsubr", "vhcurveto", "hvcurveto", "", "", "hflex", "flex",
        "hflex1", "flex1",
    ];
}
