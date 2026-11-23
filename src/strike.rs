//! Alpha and color bitmaps.

use super::internal::*;
use super::{FontRef, GlyphId};

#[cfg(feature = "scale")]
use alloc::vec::Vec;
#[cfg(all(feature = "libm", feature = "scale"))]
use core_maths::CoreFloat;

/// Proxy for rematerializing strike collections.
#[derive(Copy, Clone)]
pub struct BitmapStrikesProxy {
    bitmaps: (u32, u32),
    color_bitmaps: (u32, u32),
    upem: u16,
    is_apple: bool,
}

impl BitmapStrikesProxy {
    /// Creates strike collection proxy from the specified font.
    pub fn from_font<'a>(font: &FontRef<'a>) -> Self {
        let upem = font.head().map(|h| h.units_per_em()).unwrap_or(1);
        let mut bitmaps = (0, 0);
        let eblc = font.table_offset(raw_tag(b"EBLC"));
        if eblc != 0 {
            let ebdt = font.table_offset(raw_tag(b"EBDT"));
            if ebdt != 0 {
                bitmaps = (eblc, ebdt);
            }
        }
        let mut color_bitmaps = (0, 0);
        let sbix = font.table_offset(raw_tag(b"sbix"));
        let mut is_apple = false;
        if sbix != 0 {
            color_bitmaps = (sbix, sbix);
            use super::string::StringId::Family;
            if let Some(name) = font.localized_strings().find_by_id(Family, None) {
                is_apple = name.chars().eq("Apple Color Emoji".chars());
            }
        } else {
            let cblc = font.table_offset(raw_tag(b"CBLC"));
            if cblc != 0 {
                let cbdt = font.table_offset(raw_tag(b"CBDT"));
                if cbdt != 0 {
                    color_bitmaps = (cblc, cbdt);
                }
            }
        }
        Self {
            bitmaps,
            color_bitmaps,
            upem,
            is_apple,
        }
    }

    /// Returns true if the font has alpha bitmap strikes.
    pub fn has_alpha(&self) -> bool {
        self.bitmaps.0 != 0
    }

    /// Returns true if the font has color bitmap strikes.
    pub fn has_color(&self) -> bool {
        self.color_bitmaps.0 != 0
    }

    /// Materializes an alpha strike iterator for the specified font. This
    /// proxy must have been created from the same font.
    pub fn materialize_alpha<'a>(&self, font: &FontRef<'a>) -> BitmapStrikes<'a> {
        self.materialize_impl(font.data, self.bitmaps.0, self.bitmaps.1, self.upem, false)
    }

    /// Materializes a color strike iterator for the specified font. This
    /// proxy must have been created from the same font.
    pub fn materialize_color<'a>(&self, font: &FontRef<'a>) -> BitmapStrikes<'a> {
        self.materialize_impl(
            font.data,
            self.color_bitmaps.0,
            self.color_bitmaps.1,
            self.upem,
            self.is_apple,
        )
    }

    fn materialize_impl<'a>(
        &self,
        data: &'a [u8],
        loc: u32,
        dat: u32,
        upem: u16,
        is_apple: bool,
    ) -> BitmapStrikes<'a> {
        if loc == 0 {
            BitmapStrikes::new(&[], &[], upem, false, false)
        } else if loc == dat {
            let loc = data.get(loc as usize..).unwrap_or(&[]);
            BitmapStrikes::new(loc, loc, upem, true, is_apple)
        } else {
            let loc = data.get(loc as usize..).unwrap_or(&[]);
            let dat = data.get(dat as usize..).unwrap_or(&[]);
            BitmapStrikes::new(loc, dat, upem, false, false)
        }
    }
}

/// Iterator over a collection of bitmap strikes.
#[derive(Copy, Clone)]
pub struct BitmapStrikes<'a> {
    data: Bytes<'a>,
    bitmap_data: &'a [u8],
    is_sbix: bool,
    is_apple: bool,
    upem: u16,
    len: usize,
    pos: usize,
}

impl<'a> BitmapStrikes<'a> {
    fn new(
        data: &'a [u8],
        bitmap_data: &'a [u8],
        upem: u16,
        is_sbix: bool,
        is_apple: bool,
    ) -> Self {
        let data = Bytes::new(data);
        Self {
            data,
            bitmap_data,
            upem,
            is_sbix,
            is_apple,
            len: data.read_or_default::<u32>(4) as usize,
            pos: 0,
        }
    }

    /// Returns the bitmap strike at the specified index.
    fn get(&self, index: usize) -> Option<BitmapStrike<'a>> {
        if index >= self.len {
            return None;
        }
        let offset = if self.is_sbix {
            self.data.read::<u32>(8 + index * 4)? as usize
        } else {
            8 + index * 48
        };
        Some(BitmapStrike {
            data: self.data,
            bitmap_data: self.bitmap_data,
            upem: self.upem,
            is_sbix: self.is_sbix,
            is_apple: self.is_apple,
            offset,
        })
    }

    /// Searches for a strike that matches the specified size and glyph
    /// identifier. Returns the strike of the nearest suitable size, preferring
    /// larger strikes if no exact match is available.
    ///
    /// ## Iteration behavior
    /// This function searches the entire strike collection without regard
    /// for the current state of the iterator.
    pub fn find_by_nearest_ppem(&self, ppem: u16, glyph_id: GlyphId) -> Option<BitmapStrike<'a>> {
        let mut best = None;
        let mut best_size = 0;
        for i in 0..self.len {
            let strike = match self.get(i) {
                Some(strike) => strike,
                _ => continue,
            };
            if !strike.contains(glyph_id) {
                continue;
            }
            best = Some(strike);
            let strike_ppem = strike.ppem();
            if strike_ppem > best_size {
                best = Some(strike);
                best_size = strike_ppem;
            }
            if strike_ppem >= ppem {
                return Some(strike);
            }
        }
        best
    }

    /// Searches for a strike that exactly matches the specified size and glyph
    /// identifier.
    ///
    /// ## Iteration behavior
    /// This function searches the entire strike collection without regard
    /// for the current state of the iterator.
    pub fn find_by_exact_ppem(&self, ppem: u16, glyph_id: GlyphId) -> Option<BitmapStrike<'a>> {
        for i in 0..self.len {
            let strike = match self.get(i) {
                Some(strike) => strike,
                _ => continue,
            };
            if !strike.contains(glyph_id) {
                continue;
            }
            if strike.ppem() == ppem {
                return Some(strike);
            }
        }
        None
    }

    /// Searches for a strike with the largest size that contains the specified
    /// glyph.
    ///
    /// ## Iteration behavior
    /// This function searches the entire strike collection without regard
    /// for the current state of the iterator.
    pub fn find_by_largest_ppem(&self, glyph_id: GlyphId) -> Option<BitmapStrike<'a>> {
        let mut largest = None;
        let mut largest_ppem = 0;
        for i in 0..self.len {
            let strike = match self.get(i) {
                Some(strike) => strike,
                _ => continue,
            };
            if !strike.contains(glyph_id) {
                continue;
            }
            let strike_ppem = strike.ppem();
            if largest.is_none() || strike_ppem > largest_ppem {
                largest = Some(strike);
                largest_ppem = strike_ppem;
            }
        }
        largest
    }
}

impl_iter!(BitmapStrikes, BitmapStrike);

/// Collection of bitmaps of a specific size and format.
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BitmapStrike<'a> {
    data: Bytes<'a>,
    bitmap_data: &'a [u8],
    offset: usize,
    upem: u16,
    is_sbix: bool,
    is_apple: bool,
}

impl<'a> BitmapStrike<'a> {
    /// Returns the device pixel density for which the strike was designed.
    pub fn ppi(&self) -> u16 {
        if self.is_sbix {
            self.data.read_or_default::<u16>(self.offset + 2)
        } else {
            72
        }
    }

    /// Returns the bit depth of the strike.
    pub fn bit_depth(&self) -> u8 {
        if self.is_sbix {
            return 32;
        }
        self.data.read_or_default::<u8>(self.offset + 46)
    }

    /// Returns the size of the strike in pixels per em.
    pub fn ppem(&self) -> u16 {
        if self.is_sbix {
            self.data.read_or_default::<u16>(self.offset)
        } else {
            self.data.read_or_default::<u8>(self.offset + 45) as u16
        }
    }

    /// Returns true if the specified glyph is covered by the strike.
    pub fn contains(&self, glyph_id: GlyphId) -> bool {
        get_coverage(&self.data, self.offset, self.is_sbix, glyph_id).unwrap_or(false)
    }

    /// Returns the bitmap for the specified glyph, if available.
    #[cfg(feature = "scale")]
    pub(crate) fn get(&self, glyph_id: GlyphId) -> Option<Bitmap<'a>> {
        let loc = get_location(&self.data, self.offset, self.is_sbix, glyph_id)?;
        loc.get(self.bitmap_data, self.upem, self.is_apple)
    }
}

/// Format of a bitmap.
#[cfg(feature = "scale")]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BitmapFormat {
    /// Alpha mask with the specified number of bits.
    Alpha(u8),
    /// Packed alpha mask with the specified number of bits.
    Packed(u8),
    /// 32-bit RGBA color.
    Color,
    /// PNG encoded.
    Png,
}

#[cfg(feature = "scale")]
impl BitmapFormat {
    /// Returns the number of channels.
    pub fn channels(&self) -> u32 {
        match self {
            Self::Alpha(..) | Self::Packed(..) => 1,
            _ => 4,
        }
    }

    /// Returns the buffer size for a bitmap of the specified dimensions in this
    /// format.
    pub fn buffer_size(&self, width: u32, height: u32) -> usize {
        // Saturates rather than wraps, so an impossible size is never taken
        // for a small one.
        (width as usize)
            .saturating_mul(height as usize)
            .saturating_mul(self.channels() as usize)
    }
}

/// Bitmap glyph.
#[cfg(feature = "scale")]
#[derive(Copy, Clone)]
pub struct Bitmap<'a> {
    pub format: BitmapFormat,
    pub ppem: u16,
    pub width: u32,
    pub height: u32,
    pub left: i32,
    pub top: i32,
    pub data: &'a [u8],
}

#[cfg(feature = "scale")]
impl<'a> Bitmap<'a> {
    fn new(data: &BitmapData<'a>, upem: u16, is_apple: bool) -> Option<Self> {
        let format = if data.is_png {
            BitmapFormat::Png
        } else if data.bit_depth == 32 {
            // 32-bit pixels are whole bytes, so bit-aligned data is laid out
            // like byte-aligned data.
            BitmapFormat::Color
        } else if data.is_packed {
            BitmapFormat::Packed(data.bit_depth)
        } else {
            BitmapFormat::Alpha(data.bit_depth)
        };
        let (width, height, left, top) = if data.is_png {
            let png = Bytes::new(data.data);
            let width = png.read::<u32>(16)?;
            let height = png.read::<u32>(20)?;
            let (left, mut top) = (data.metrics.x as i32, data.metrics.y as i32);
            if data.is_sbix {
                if top == 0 && is_apple {
                    let s = data.ppem as f32 / upem as f32;
                    top = (-100. * s).round() as i32;
                }
                top += height as i32;
            }
            (width, height, left, top)
        } else {
            (
                data.width as u32,
                data.height as u32,
                data.metrics.x as i32,
                data.metrics.y as i32,
            )
        };
        Some(Bitmap {
            format,
            ppem: data.ppem,
            width,
            height,
            left,
            top,
            data: data.data,
        })
    }

    /// Returns the size of an image buffer necessary for storing the decoded bitmap.
    pub fn decoded_size(&self) -> usize {
        self.format.buffer_size(self.width, self.height)
    }

    /// Returns the width, height and buffer size for a scaled version of the bitmap.
    pub fn scaled_size(&self, size: f32) -> (u32, u32, usize) {
        let mut w = self.width;
        let mut h = self.height;
        if size != 0. {
            let scale = size / self.ppem as f32;
            w = (w as f32 * scale) as u32;
            h = (h as f32 * scale) as u32;
        }
        (w, h, self.format.buffer_size(w, h))
    }

    /// Decodes the bitmap into the specified target buffer.
    pub fn decode(&self, scratch: Option<&mut Vec<u8>>, target: &mut [u8]) -> bool {
        let mut tmp = Vec::new();
        let scratch = if let Some(scratch) = scratch {
            scratch
        } else {
            &mut tmp
        };
        let size = self.decoded_size();
        if target.len() < size {
            return false;
        }
        let dst = &mut target[..size];
        let w = self.width as usize;
        let h = self.height as usize;
        // Apart from PNG, each format needs a fixed number of source bytes.
        // Checking it here keeps every index below in bounds, whatever the
        // dimensions: a bitmap with no pixels (a blank glyph) decodes to
        // nothing.
        let src = self.data_len().and_then(|len| self.data.get(..len));
        match (self.format, src) {
            (BitmapFormat::Png, _) => {
                use super::scale::decode_png;
                scratch.clear();
                return decode_png(self.data, scratch, dst).is_some();
            }
            (_, None) => return false,
            (BitmapFormat::Packed(bits), Some(src)) => match bits {
                1 => {
                    for (x, out) in dst.iter_mut().enumerate() {
                        *out = (src[x >> 3] >> (!x & 7) & 1) * 255;
                    }
                }
                2 => {
                    for (x, out) in dst.iter_mut().enumerate() {
                        *out = (src[x >> 2] >> (!(x * 2) & 6) & 3) * 85;
                    }
                }
                4 => {
                    for (x, out) in dst.iter_mut().enumerate() {
                        *out = (src[x >> 1] >> (!(x * 4) & 4) & 15) * 17;
                    }
                }
                8 => dst.copy_from_slice(src),
                _ => return false,
            },
            (BitmapFormat::Alpha(bits), Some(src)) => match bits {
                1 | 2 | 4 => {
                    // Each row starts on a byte.
                    let stride = (w * bits as usize + 7) / 8;
                    for y in 0..h {
                        let row = &src[y * stride..][..stride];
                        let out = &mut dst[y * w..][..w];
                        for (x, out) in out.iter_mut().enumerate() {
                            *out = match bits {
                                1 => (row[x >> 3] >> (!x & 7) & 1) * 255,
                                2 => (row[x >> 2] >> (!(x * 2) & 6) & 3) * 85,
                                _ => (row[x >> 1] >> (!(x * 4) & 4) & 15) * 17,
                            };
                        }
                    }
                }
                8 => dst.copy_from_slice(src),
                _ => return false,
            },
            (BitmapFormat::Color, Some(src)) => dst.copy_from_slice(src),
        }
        true
    }

    /// Returns the number of bytes of source data the bitmap needs, or `None`
    /// if that does not fit in memory. PNG data has no fixed size.
    fn data_len(&self) -> Option<usize> {
        let w = self.width as usize;
        let h = self.height as usize;
        match self.format {
            BitmapFormat::Alpha(bits) => {
                let stride = w.checked_mul(bits as usize)?.checked_add(7)? / 8;
                stride.checked_mul(h)
            }
            BitmapFormat::Packed(bits) => {
                let bits = w.checked_mul(h)?.checked_mul(bits as usize)?;
                Some(bits.checked_add(7)? / 8)
            }
            BitmapFormat::Color => w.checked_mul(h)?.checked_mul(4),
            BitmapFormat::Png => None,
        }
    }
}

/// The location of a bitmap in the bitmap data table.
#[cfg(feature = "scale")]
#[derive(Copy, Clone)]
struct Location {
    format: u8,
    flags: u8,
    offset: u32,
    size: u32,
    ppem: u16,
    bit_depth: u8,
    width: u8,
    height: u8,
    metrics: Metrics,
    vertical_metrics: Metrics,
}

#[cfg(feature = "scale")]
impl Location {
    /// Gets a bitmap from this location in the specified data source.
    pub fn get<'a>(&self, data: &'a [u8], upem: u16, is_apple: bool) -> Option<Bitmap<'a>> {
        Bitmap::new(&get_data(data, self)?, upem, is_apple)
    }
}

fn get_coverage(table: &[u8], strike_base: usize, is_sbix: bool, glyph_id: u16) -> Option<bool> {
    if is_sbix {
        return Some(sbix_range(table, strike_base, glyph_id, 0).is_some());
    }
    let b = Bytes::with_offset(table, strike_base)?;
    if glyph_id < b.read(40)? || glyph_id > b.read(42)? {
        return None;
    }
    let count = b.read::<u32>(8)? as usize;
    let array_offset = b.read::<u32>(0)? as usize;
    let b = Bytes::with_offset(table, array_offset)?;
    for i in 0..count {
        let offset = i * 8;
        let first = b.read::<u16>(offset)?;
        if glyph_id < first {
            return None;
        }
        if glyph_id > b.read::<u16>(offset + 2)? {
            continue;
        }
        return Some(true);
    }
    None
}

#[cfg(feature = "scale")]
fn get_location(
    table: &[u8],
    strike_base: usize,
    is_sbix: bool,
    glyph_id: u16,
) -> Option<Location> {
    let d = Bytes::new(table);
    if is_sbix {
        let (start, end) = sbix_range(table, strike_base, glyph_id, 0)?;
        let len = (end - start) as usize;
        let start = start as usize;
        let x = d.read::<i16>(start)?;
        let y = d.read::<i16>(start + 2)?;
        let ppem = d.read::<u16>(strike_base)?;
        return Some(Location {
            ppem,
            bit_depth: 32,
            width: 0,
            height: 0,
            metrics: Metrics {
                x: x as i8,
                y: y as i8,
                advance: 0,
            },
            vertical_metrics: Metrics::default(),
            format: 0xFF,
            flags: 1,
            offset: start as u32 + 8,
            size: len as u32 - 8,
        });
    }
    if glyph_id < d.read(strike_base + 40)? || glyph_id > d.read(strike_base + 42)? {
        return None;
    }
    let count = d.read::<u32>(strike_base + 8)? as usize;
    let ppem = d.read::<u8>(strike_base + 45)? as u16;
    let bit_depth = d.read::<u8>(strike_base + 46)?;
    let flags = d.read::<u8>(strike_base + 47)?;
    let array_offset = d.read::<u32>(strike_base)? as usize;
    for i in 0..count {
        let offset = array_offset + i * 8;
        let first = d.read::<u16>(offset)?;
        if glyph_id < first {
            return None;
        }
        if glyph_id > d.read::<u16>(offset + 2)? {
            continue;
        }
        let offset = array_offset + d.read::<u32>(offset + 4)? as usize;
        let index_format = d.read::<u16>(offset)?;
        let image_format = d.read::<u16>(offset + 2)? as u8;
        let image_offset = d.read::<u32>(offset + 4)?;
        let base = offset + 8;
        let mut loc = Location {
            ppem,
            bit_depth,
            width: 0,
            height: 0,
            metrics: Metrics::default(),
            vertical_metrics: Metrics::default(),
            format: image_format,
            flags,
            offset: 0,
            size: 0,
        };
        match index_format {
            1 | 3 => {
                // An offset per glyph and one past the last. Equal offsets
                // mean the glyph has no bitmap in this strike.
                let index = (glyph_id - first) as usize;
                let (offset1, offset2) = if index_format == 1 {
                    let rec = base + index * 4;
                    (d.read::<u32>(rec)?, d.read::<u32>(rec + 4)?)
                } else {
                    let rec = base + index * 2;
                    (d.read::<u16>(rec)? as u32, d.read::<u16>(rec + 2)? as u32)
                };
                if offset2 <= offset1 {
                    return None;
                }
                loc.offset = image_offset.checked_add(offset1)?;
                loc.size = offset2 - offset1;
                return Some(loc);
            }
            2 => {
                loc.size = d.read::<u32>(base)?;
                loc.offset = image_offset + loc.size * (glyph_id - first) as u32;
                let (w, h) = get_metrics(
                    &d,
                    base + 4,
                    flags,
                    true,
                    &mut loc.metrics,
                    &mut loc.vertical_metrics,
                )?;
                loc.width = w;
                loc.height = h;
                return Some(loc);
            }
            4 => {
                // The number of glyphs, then a (glyph, offset) pair for each,
                // sorted by glyph, and one more pair to end the last image.
                let records = base + 4;
                let mut l = 0;
                let mut h = d.read::<u32>(base)? as usize;
                while l < h {
                    use core::cmp::Ordering::*;
                    let i = l + (h - l) / 2;
                    let rec = records + i * 4;
                    let id = d.read::<u16>(rec)?;
                    match glyph_id.cmp(&id) {
                        Less => h = i,
                        Greater => l = i + 1,
                        Equal => {
                            let offset1 = d.read::<u16>(rec + 2)? as u32;
                            let offset2 = d.read::<u16>(rec + 6)? as u32;
                            if offset2 <= offset1 {
                                return None;
                            }
                            loc.offset = image_offset.checked_add(offset1)?;
                            loc.size = offset2 - offset1;
                            return Some(loc);
                        }
                    }
                }
                return None;
            }
            _ => {
                return None;
            }
        }
    }
    None
}

#[cfg(feature = "scale")]
fn get_data<'a>(table: &'a [u8], loc: &Location) -> Option<BitmapData<'a>> {
    let depth = loc.bit_depth as usize;
    let mut bitmap = BitmapData {
        data: &[],
        ppem: loc.ppem,
        bit_depth: loc.bit_depth,
        width: loc.width,
        height: loc.height,
        metrics: loc.metrics,
        vertical_metrics: loc.vertical_metrics,
        is_packed: false,
        is_png: false,
        is_sbix: false,
    };
    let d = &Bytes::new(table);
    let offset = loc.offset as usize;
    let flags = loc.flags;
    let size = loc.size as usize;
    match loc.format {
        0xFF => {
            bitmap.data = d.read_bytes(offset, size)?;
            bitmap.is_png = true;
            bitmap.is_sbix = true;
            Some(bitmap)
        }
        1 => {
            bitmap.read_metrics(d, offset, flags, false)?;
            let w = (bitmap.width as usize * depth + 7) / 8;
            let h = bitmap.height as usize;
            bitmap.data = d.read_bytes(offset + 5, w * h)?;
            Some(bitmap)
        }
        2 => {
            bitmap.read_metrics(d, offset, flags, false)?;
            let w = bitmap.width as usize * depth;
            let h = bitmap.height as usize;
            bitmap.data = d.read_bytes(offset + 5, (w * h + 7) / 8)?;
            bitmap.is_packed = true;
            Some(bitmap)
        }
        5 => {
            bitmap.data = d.read_bytes(offset, size)?;
            bitmap.is_packed = true;
            Some(bitmap)
        }
        6 => {
            bitmap.read_metrics(d, offset, flags, true)?;
            let w = (bitmap.width as usize * depth + 7) / 8;
            let h = bitmap.height as usize;
            bitmap.data = d.read_bytes(offset + 8, w * h)?;
            Some(bitmap)
        }
        7 => {
            bitmap.read_metrics(d, offset, flags, true)?;
            let w = bitmap.width as usize * depth;
            let h = bitmap.height as usize;
            bitmap.data = d.read_bytes(offset + 8, (w * h + 7) / 8)?;
            bitmap.is_packed = true;
            Some(bitmap)
        }
        17 => {
            bitmap.read_metrics(d, offset, flags, false)?;
            let size = d.read::<u32>(offset + 5)? as usize;
            bitmap.data = d.read_bytes(offset + 9, size)?;
            bitmap.is_png = true;
            Some(bitmap)
        }
        18 => {
            bitmap.read_metrics(d, offset, flags, true)?;
            let size = d.read::<u32>(offset + 8)? as usize;
            bitmap.data = d.read_bytes(offset + 12, size)?;
            bitmap.is_png = true;
            Some(bitmap)
        }
        19 => {
            let size = d.read::<u32>(offset)? as usize;
            bitmap.data = d.read_bytes(offset + 4, size)?;
            bitmap.is_png = true;
            Some(bitmap)
        }
        _ => None,
    }
}

fn sbix_range(table: &[u8], strike_base: usize, glyph_id: u16, recurse: i32) -> Option<(u32, u32)> {
    const DUPE: RawTag = raw_tag(b"dupe");
    const PNG: RawTag = raw_tag(b"png ");
    if recurse > 1 {
        return None;
    }
    let b = Bytes::new(table);
    let id = glyph_id as usize;
    let base = strike_base + 4;
    let mut start = b.read::<u32>(base + id * 4)?;
    let mut end = b.read::<u32>(base + (id + 1) * 4)?;
    if end <= start {
        return None;
    }
    start += strike_base as u32;
    end += strike_base as u32;
    let tag = b.read::<u32>(start as usize + 4)?;
    if tag == DUPE {
        let dupe = b.read::<u16>(start as usize + 8)?;
        sbix_range(table, strike_base, dupe, recurse + 1)
    } else if tag == PNG {
        Some((start, end))
    } else {
        None
    }
}

#[cfg(feature = "scale")]
#[derive(Copy, Clone)]
struct BitmapData<'a> {
    pub data: &'a [u8],
    pub ppem: u16,
    pub bit_depth: u8,
    pub width: u8,
    pub height: u8,
    pub metrics: Metrics,
    pub vertical_metrics: Metrics,
    pub is_packed: bool,
    pub is_png: bool,
    pub is_sbix: bool,
}

#[cfg(feature = "scale")]
impl<'a> BitmapData<'a> {
    fn read_metrics(&mut self, d: &Bytes, offset: usize, flags: u8, big: bool) -> Option<()> {
        let (w, h) = get_metrics(
            d,
            offset,
            flags,
            big,
            &mut self.metrics,
            &mut self.vertical_metrics,
        )?;
        self.width = w;
        self.height = h;
        Some(())
    }
}

#[cfg(feature = "scale")]
#[derive(Copy, Clone, Default)]
struct Metrics {
    pub x: i8,
    pub y: i8,
    pub advance: u8,
}

#[cfg(feature = "scale")]
fn get_metrics(
    d: &Bytes,
    offset: usize,
    flags: u8,
    big: bool,
    h: &mut Metrics,
    v: &mut Metrics,
) -> Option<(u8, u8)> {
    let width = d.read::<u8>(offset + 1)?;
    let height = d.read::<u8>(offset)?;
    if big {
        h.x = d.read::<i8>(offset + 2)?;
        h.y = d.read::<i8>(offset + 3)?;
        h.advance = d.read::<u8>(offset + 4)?;
        v.x = d.read::<i8>(offset + 5)?;
        v.y = d.read::<i8>(offset + 6)?;
        v.advance = d.read::<u8>(offset + 7)?;
    } else {
        let m = if flags & 2 != 0 { v } else { h };
        m.x = d.read::<i8>(offset + 2)?;
        m.y = d.read::<i8>(offset + 3)?;
        m.advance = d.read::<u8>(offset + 4)?;
    }
    Some((width, height))
}

#[cfg(all(test, feature = "scale"))]
mod tests {
    use super::{Bitmap, BitmapData, BitmapFormat, BitmapStrikes, Metrics};
    use BitmapFormat::{Alpha, Color, Packed};

    const FORMATS: [BitmapFormat; 9] = [
        Alpha(1),
        Alpha(2),
        Alpha(4),
        Alpha(8),
        Packed(1),
        Packed(2),
        Packed(4),
        Packed(8),
        Color,
    ];

    fn bitmap(format: BitmapFormat, width: u32, height: u32, data: &[u8]) -> Bitmap<'_> {
        Bitmap {
            format,
            ppem: 12,
            width,
            height,
            left: 0,
            top: 0,
            data,
        }
    }

    /// Bytes of source data for a `width` x `height` bitmap.
    fn data_len(format: BitmapFormat, width: usize, height: usize) -> usize {
        match format {
            Alpha(bits) => (width * bits as usize + 7) / 8 * height,
            Packed(bits) => (width * height * bits as usize + 7) / 8,
            _ => width * height * 4,
        }
    }

    /// Decodes into a buffer `extra` bytes larger than needed, filled with a
    /// marker, and returns the result and the buffer.
    fn decode(bitmap: &Bitmap, extra: usize) -> (bool, Vec<u8>) {
        let mut target = vec![0xAA; bitmap.decoded_size() + extra];
        let ok = bitmap.decode(None, &mut target);
        (ok, target)
    }

    // A glyph can be present in a strike with no pixels, such as the space
    // in many bitmap fonts (#139). It decodes to an empty image.
    #[test]
    fn decode_empty() {
        for format in FORMATS {
            for (w, h) in [(0, 3), (3, 0), (0, 0)] {
                let (ok, target) = decode(&bitmap(format, w, h, &[]), 4);
                assert!(ok, "{format:?} {w}x{h}");
                assert_eq!(target, [0xAA; 4], "{format:?} {w}x{h}");
            }
        }
    }

    #[test]
    fn decode_short_data() {
        for format in FORMATS {
            let data = vec![0xFF; data_len(format, 3, 2) - 1];
            let (ok, target) = decode(&bitmap(format, 3, 2, &data), 0);
            assert!(!ok, "{format:?}");
            assert!(target.iter().all(|&b| b == 0xAA), "{format:?}");
        }
    }

    #[test]
    fn decode_into_larger_target() {
        for format in FORMATS {
            let data = vec![0xFF; data_len(format, 3, 2)];
            let bitmap = bitmap(format, 3, 2, &data);
            let size = bitmap.decoded_size();
            let (ok, target) = decode(&bitmap, 4);
            assert!(ok, "{format:?}");
            assert!(target[..size].iter().all(|&b| b == 0xFF), "{format:?}");
            assert_eq!(target[size..], [0xAA; 4], "{format:?}");
        }
    }

    #[test]
    fn decode_unsupported_depth() {
        for format in [Alpha(3), Alpha(32), Packed(3), Packed(32)] {
            let data = [0xFF; 32];
            let (ok, _) = decode(&bitmap(format, 2, 2, &data), 0);
            assert!(!ok, "{format:?}");
        }
    }

    #[test]
    fn decode_pixels() {
        let cases: [(BitmapFormat, &[u8], &[u8]); 7] = [
            // Rows are padded to a byte.
            (
                Alpha(1),
                &[0b1010_0000, 0b0100_0000],
                &[255, 0, 255, 0, 255, 0],
            ),
            (
                Alpha(4),
                &[0xF0, 0x80, 0x12, 0x30],
                &[255, 0, 136, 17, 34, 51],
            ),
            (Alpha(8), &[1, 2, 3, 4, 5, 6], &[1, 2, 3, 4, 5, 6]),
            // Rows follow each other bit by bit.
            (Packed(1), &[0b1010_1000], &[255, 0, 255, 0, 255, 0]),
            (Packed(4), &[0xF0, 0x81, 0x23], &[255, 0, 136, 17, 34, 51]),
            (Packed(8), &[1, 2, 3, 4, 5, 6], &[1, 2, 3, 4, 5, 6]),
            (Color, &[1; 24], &[1; 24]),
        ];
        for (format, data, pixels) in cases {
            let (ok, target) = decode(&bitmap(format, 3, 2, data), 0);
            assert!(ok, "{format:?}");
            assert_eq!(target, pixels, "{format:?}");
        }
    }

    // Four 2-bit pixels per byte, the first in the high bits.
    #[test]
    fn decode_2_bit_pixels() {
        for format in [Alpha(2), Packed(2)] {
            let (ok, target) = decode(&bitmap(format, 4, 1, &[0b11_10_01_00]), 0);
            assert!(ok, "{format:?}");
            assert_eq!(target, [255, 170, 85, 0], "{format:?}");
        }
    }

    // A 32-bit strike stores whole bytes per pixel, so its bit-aligned image
    // formats hold the same bytes as the byte-aligned ones.
    #[test]
    fn packed_32_bit_is_color() {
        let pixels = [1, 2, 3, 4, 5, 6, 7, 8];
        let data = BitmapData {
            data: &pixels,
            ppem: 12,
            bit_depth: 32,
            width: 2,
            height: 1,
            metrics: Metrics::default(),
            vertical_metrics: Metrics::default(),
            is_packed: true,
            is_png: false,
            is_sbix: false,
        };
        let bitmap = Bitmap::new(&data, 1000, false).unwrap();
        assert_eq!(bitmap.format, Color);
        let (ok, target) = decode(&bitmap, 0);
        assert!(ok);
        assert_eq!(target, pixels);
    }

    /// An index subtable: its glyph range, formats, the bytes after its
    /// header, and the image data it points into.
    struct Subtable {
        first: u16,
        last: u16,
        index_format: u16,
        image_format: u16,
        body: Vec<u8>,
        images: Vec<u8>,
    }

    /// An image in format 1: small metrics, then byte-aligned rows.
    fn image(width: u8, height: u8, rows: &[u8]) -> Vec<u8> {
        let mut image = vec![height, width, 0, height, 6];
        image.extend(rows);
        image
    }

    /// Index format 1 or 3: an offset per glyph from `first`, and one past
    /// the last image. An empty image leaves the glyph out of the strike.
    fn offsets(index_format: u16, first: u16, images: &[&[u8]]) -> Subtable {
        let mut body = Vec::new();
        let mut offset = 0;
        for image in images.iter().chain(&[&[][..]]) {
            match index_format {
                1 => body.extend((offset as u32).to_be_bytes()),
                _ => body.extend((offset as u16).to_be_bytes()),
            }
            offset += image.len();
        }
        Subtable {
            first,
            last: first + images.len() as u16 - 1,
            index_format,
            image_format: 1,
            body,
            images: images.concat(),
        }
    }

    /// Index format 4: (glyph, offset) pairs for the glyphs present, and one
    /// past the last image.
    fn sparse(first: u16, last: u16, glyphs: &[(u16, &[u8])]) -> Subtable {
        let mut body = (glyphs.len() as u32).to_be_bytes().to_vec();
        let mut offset = 0;
        for (glyph_id, image) in glyphs.iter().chain(&[(0, &[][..])]) {
            body.extend(glyph_id.to_be_bytes());
            body.extend((offset as u16).to_be_bytes());
            offset += image.len();
        }
        Subtable {
            first,
            last,
            index_format: 4,
            image_format: 1,
            body,
            images: glyphs.iter().flat_map(|g| g.1).copied().collect(),
        }
    }

    /// Builds EBLC and EBDT tables with one strike, 12 ppem and 1 bit deep.
    fn tables(subtables: &[Subtable]) -> (Vec<u8>, Vec<u8>) {
        let mut ebdt = vec![0, 2, 0, 0];
        let mut records = Vec::new();
        let mut bodies = Vec::new();
        for s in subtables {
            let offset = subtables.len() * 8 + bodies.len();
            records.extend(s.first.to_be_bytes());
            records.extend(s.last.to_be_bytes());
            records.extend((offset as u32).to_be_bytes());
            bodies.extend(s.index_format.to_be_bytes());
            bodies.extend(s.image_format.to_be_bytes());
            bodies.extend((ebdt.len() as u32).to_be_bytes());
            bodies.extend(&s.body);
            bodies.resize(bodies.len().next_multiple_of(4), 0);
            ebdt.extend(&s.images);
        }
        let first = subtables.iter().map(|s| s.first).min().unwrap();
        let last = subtables.iter().map(|s| s.last).max().unwrap();
        let mut eblc = vec![0, 2, 0, 0, 0, 0, 0, 1];
        eblc.extend(56u32.to_be_bytes());
        eblc.extend(((records.len() + bodies.len()) as u32).to_be_bytes());
        eblc.extend((subtables.len() as u32).to_be_bytes());
        eblc.extend([0; 28]);
        eblc.extend(first.to_be_bytes());
        eblc.extend(last.to_be_bytes());
        eblc.extend([12, 12, 1, 1]);
        eblc.extend(records);
        eblc.extend(bodies);
        (eblc, ebdt)
    }

    /// Looks a glyph up in the first strike and returns the size and data of
    /// its bitmap.
    fn lookup(eblc: &[u8], ebdt: &[u8], glyph_id: u16) -> Option<(u32, u32, Vec<u8>)> {
        let strikes = BitmapStrikes::new(eblc, ebdt, 1000, false, false);
        let bitmap: Bitmap = strikes.get(0)?.get(glyph_id)?;
        Some((bitmap.width, bitmap.height, bitmap.data.to_vec()))
    }

    // Equal consecutive offsets mean the glyph has no bitmap in the strike.
    // A blank glyph is still present: it has metrics, and no rows.
    #[test]
    fn missing_and_blank_glyphs() {
        let (blank, ink, ink2) = (image(0, 0, &[]), image(1, 1, &[0x80]), image(2, 1, &[0xC0]));
        let (eblc, ebdt) = tables(&[
            offsets(1, 1, &[&blank, &[], &ink]),
            offsets(3, 4, &[&blank, &[], &ink2]),
        ]);
        let blank = Some((0, 0, vec![]));
        let ink = Some((1, 1, vec![0x80]));
        let ink2 = Some((2, 1, vec![0xC0]));
        let found: Vec<_> = (1..=6).map(|gid| lookup(&eblc, &ebdt, gid)).collect();
        assert_eq!(found, [blank.clone(), None, ink, blank, None, ink2]);
    }

    #[test]
    fn sparse_glyphs() {
        let (blank, ink, ink2) = (image(0, 0, &[]), image(1, 1, &[0x80]), image(2, 1, &[0xC0]));
        let (eblc, ebdt) = tables(&[sparse(10, 20, &[(12, &ink), (14, &blank), (16, &ink2)])]);
        // Includes glyphs in range below, between and above the records.
        let found: Vec<_> = (10..=20).map(|gid| lookup(&eblc, &ebdt, gid)).collect();
        let mut expected = vec![None; 11];
        expected[2] = Some((1, 1, vec![0x80]));
        expected[4] = Some((0, 0, vec![]));
        expected[6] = Some((2, 1, vec![0xC0]));
        assert_eq!(found, expected);
    }
}
