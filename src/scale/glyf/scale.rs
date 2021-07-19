use super::cache::{Cache, CacheSlot};
use super::internal::{
    fixed::{div, mul, muldiv, round},
    Stream,
};
use super::proxy::GlyfProxy;
use super::Point;
use crate::{metrics::MetricsProxy, CacheKey, FontRef};

#[derive(Default)]
pub struct Scaler {
    pub unscaled: Vec<Point>,
    pub scaled: Vec<Point>,
    pub original: Vec<Point>,
    pub deltas: Vec<Point>,
    pub tags: Vec<u8>,
    pub contours: Vec<u16>,
    pub cache: Cache,
}

impl Scaler {
    pub fn new(max_entries: usize) -> Self {
        Self {
            cache: Cache::new(max_entries),
            ..Self::default()
        }
    }

    pub fn scale(&mut self, state: &mut ScalerState, glyph_id: u16) -> Option<()> {
        self.scaled.clear();
        self.unscaled.clear();
        self.original.clear();
        self.deltas.clear();
        self.tags.clear();
        self.contours.clear();
        if glyph_id >= state.metrics.glyph_count() {
            return None;
        }
        self.load(state, glyph_id, 0)?;
        let pp0x = state.phantom[0].x;
        if pp0x != 0 {
            for p in &mut self.scaled {
                p.x -= pp0x;
            }
        }
        Some(())
    }
}

/// Loading
impl Scaler {
    fn load(&mut self, state: &mut ScalerState, glyph_id: u16, recurse: u8) -> Option<()> {
        use core::cmp::Ordering::*;
        if recurse > 32 {
            return None;
        }
        let mut s = Stream::new(state.proxy.glyph_data(state.data, glyph_id)?);
        if s.len() == 0 {
            return Some(());
        }
        let point_base = self.scaled.len();
        let contour_base = self.contours.len();
        let initial = s.read::<i16>()?;
        match initial.cmp(&0) {
            Greater => {
                const XSHORT: u8 = 2;
                const YSHORT: u8 = 4;
                const REPEAT: u8 = 8;
                const XSAME: u8 = 16;
                const YSAME: u8 = 32;
                let contour_count = initial as usize;
                let contour_end = contour_base + contour_count;
                let bounds = [s.read_i16()?, s.read_i16()?, s.read_i16()?, s.read_i16()?];
                self.setup(state, bounds, glyph_id, recurse);
                self.contours.resize(contour_end, 0);
                let mut last_contour = 0;
                for i in 0..contour_count {
                    let contour = s.read::<u16>()?;
                    if contour < last_contour {
                        return None;
                    }
                    self.contours[contour_base + i] = contour;
                    last_contour = contour;
                }
                let mut point_count = last_contour as usize + 1;
                self.scaled
                    .resize(point_base + point_count, Point::new(0, 0));
                self.tags.resize(point_base + point_count, 0);
                let ins_len = s.read::<u16>()?;
                let ins = s.read_bytes(ins_len as usize)?;
                let point_end = point_base + point_count;
                let mut i = point_base;
                while i < point_end {
                    let tag = s.read::<u8>()?;
                    if tag & REPEAT != 0 {
                        let count = (s.read::<u8>()? as usize + 1).min(point_end - i);
                        for t in &mut self.tags[i..i + count] {
                            *t = tag;
                        }
                        i += count;
                    } else {
                        self.tags[i] = tag;
                        i += 1;
                    }
                }
                let mut v = 0i32;
                for (&tag, point) in (&self.tags[point_base..])
                    .iter()
                    .zip(&mut self.scaled[point_base..])
                {
                    let mut delta = 0i32;
                    if tag & XSHORT != 0 {
                        delta = s.read::<u8>()? as i32;
                        if tag & XSAME == 0 {
                            delta = -delta;
                        }
                    } else if tag & XSAME == 0 {
                        delta = s.read::<i16>()? as i32;
                    }
                    v += delta;
                    point.x = v;
                }
                v = 0;
                for (tag, point) in (&mut self.tags[point_base..])
                    .iter_mut()
                    .zip(&mut self.scaled[point_base..])
                {
                    let mut delta = 0i32;
                    let t = *tag;
                    if t & YSHORT != 0 {
                        delta = s.read::<u8>()? as i32;
                        if t & YSAME == 0 {
                            delta = -delta;
                        }
                    } else if t & YSAME == 0 {
                        delta = s.read::<i16>()? as i32;
                    }
                    v += delta;
                    point.y = v;
                    *tag &= 1;
                }
                self.push_phantom(state);
                point_count += 4;
                let point_end = point_base + point_count;
                if state.vary {
                    self.unscaled.clear();
                    self.unscaled.resize(point_count, Point::new(0, 0));
                    self.original.clear();
                    self.original.resize(point_count, Point::new(0, 0));
                    if state.proxy.deltas(
                        state.data,
                        state.coords,
                        glyph_id,
                        &self.scaled[point_base..],
                        &mut self.tags[point_base..],
                        &self.contours[contour_base..],
                        &mut self.unscaled[..],
                        &mut self.original[..],
                    ) {
                        for (d, p) in self.original[..point_count]
                            .iter()
                            .zip(self.scaled[point_base..].iter_mut())
                        {
                            p.x += d.x;
                            p.y += d.y;
                        }
                    }
                }
                let hinted = state.hint && !ins.is_empty();
                if hinted {
                    self.unscaled.clear();
                    self.unscaled.extend_from_slice(&self.scaled[point_base..]);
                }
                if state.have_scale {
                    let scale = state.scale;
                    for p in &mut self.scaled[point_base..] {
                        p.x = mul(p.x, scale);
                        p.y = mul(p.y, scale);
                    }
                    self.save_phantom(state, point_base, point_count);
                }
                if hinted {
                    self.original.clear();
                    self.original
                        .extend_from_slice(&self.scaled[point_base..point_end]);
                    for p in &mut self.scaled[point_end - 4..] {
                        p.x = round(p.x);
                        p.y = round(p.y);
                    }
                    self.hint(state, point_base, contour_base, ins, false);
                }
                if point_base != 0 {
                    for c in &mut self.contours[contour_base..contour_end] {
                        *c += point_base as u16;
                    }
                }
                self.scaled.truncate(self.scaled.len() - 4);
                self.tags.truncate(self.tags.len() - 4);
                Some(())
            }
            Less => {
                const ARGS_ARE_WORDS: u16 = 0x001;
                const ARGS_ARE_XY_VALUES: u16 = 0x002;
                const ROUND_XY_TO_GRID: u16 = 0x004;
                const HAVE_SCALE: u16 = 0x008;
                const MORE_COMPONENTS: u16 = 32;
                const HAVE_XY_SCALE: u16 = 64;
                const HAVE_MATRIX: u16 = 128;
                const HAVE_TRANSFORM: u16 = HAVE_SCALE | HAVE_XY_SCALE | HAVE_MATRIX;
                const HAVE_INSTRUCTIONS: u16 = 256;
                const USE_MY_METRICS: u16 = 512;
                const SCALED_COMPONENT_OFFSET: u16 = 0x800;
                const UNSCALED_COMPONENT_OFFSET: u16 = 4096;
                let bounds = [s.read_i16()?, s.read_i16()?, s.read_i16()?, s.read_i16()?];
                self.setup(state, bounds, glyph_id, recurse);
                if state.have_scale {
                    let scale = state.scale;
                    for p in state.phantom.iter_mut() {
                        p.x = mul(p.x, scale);
                        p.y = mul(p.y, scale);
                    }
                }
                let delta_base = self.deltas.len();
                let mut have_deltas = false;
                let base_offset = s.offset();
                if state.vary {
                    let mut flags: u16;
                    let mut count = 0usize;
                    loop {
                        flags = s.read()?;
                        s.skip(2)?;
                        if flags & ARGS_ARE_WORDS != 0 {
                            s.skip(4)?;
                        } else {
                            s.skip(2)?;
                        }
                        if flags & HAVE_SCALE != 0 {
                            s.skip(2)?;
                        } else if flags & HAVE_XY_SCALE != 0 {
                            s.skip(4)?;
                        } else if flags & HAVE_MATRIX != 0 {
                            s.skip(8)?;
                        }
                        count += 1;
                        if flags & MORE_COMPONENTS == 0 {
                            break;
                        }
                    }
                    s.set_offset(base_offset)?;
                    self.deltas.resize(delta_base + count, Point::new(0, 0));
                    if state.proxy.composite_deltas(
                        state.data,
                        state.coords,
                        glyph_id,
                        &mut self.deltas[delta_base..],
                    ) {
                        have_deltas = true;
                    }
                }
                let mut flags: u16;
                let mut i = 0;
                loop {
                    flags = s.read()?;
                    let id = s.read::<u16>()?;
                    let phantom = state.phantom;
                    let start_point = self.scaled.len();
                    self.load(state, id, recurse + 1)?;
                    let end_point = self.scaled.len();
                    if flags & USE_MY_METRICS == 0 {
                        state.phantom = phantom;
                    }
                    let x: i32;
                    let y: i32;
                    let mut have_offset = false;
                    if flags & ARGS_ARE_XY_VALUES != 0 {
                        if flags & ARGS_ARE_WORDS != 0 {
                            x = s.read::<i16>()? as i32;
                            y = s.read::<i16>()? as i32;
                        } else {
                            x = s.read::<i8>()? as i32;
                            y = s.read::<i8>()? as i32;
                        }
                        have_offset = true;
                    } else if flags & ARGS_ARE_WORDS != 0 {
                        x = s.read::<u16>()? as i32;
                        y = s.read::<u16>()? as i32;
                    } else {
                        x = s.read::<u8>()? as i32;
                        y = s.read::<u8>()? as i32;
                    }
                    let mut xx = 0x10000;
                    let mut xy = 0;
                    let mut yx = 0;
                    let mut yy = 0x10000;
                    let have_xform = flags & HAVE_TRANSFORM != 0;
                    if flags & HAVE_SCALE != 0 {
                        xx = s.read::<i16>()? as i32 * 4;
                        yy = xx;
                    } else if flags & HAVE_XY_SCALE != 0 {
                        xx = s.read::<i16>()? as i32 * 4;
                        yy = s.read::<i16>()? as i32 * 4;
                    } else if flags & HAVE_MATRIX != 0 {
                        xx = s.read::<i16>()? as i32 * 4;
                        yx = s.read::<i16>()? as i32 * 4;
                        xy = s.read::<i16>()? as i32 * 4;
                        yy = s.read::<i16>()? as i32 * 4;
                    }
                    let xx = xx;
                    let xy = xy;
                    let yx = yx;
                    let yy = yy;
                    if have_xform {
                        for p in &mut self.scaled[start_point..end_point] {
                            let (x, y) = transform(p.x, p.y, xx, xy, yx, yy);
                            p.x = x;
                            p.y = y;
                        }
                    }
                    let (dx, dy) = if have_offset {
                        let (mut dx, mut dy) = (x, y);
                        if have_xform
                            && (flags & (SCALED_COMPONENT_OFFSET | UNSCALED_COMPONENT_OFFSET)
                                == SCALED_COMPONENT_OFFSET)
                        {
                            dx = mul(dx, hypot(xx, xy));
                            dy = mul(dy, hypot(yy, yx));
                        }
                        if have_deltas {
                            let d = self.deltas[delta_base + i];
                            dx += d.x;
                            dy += d.y;
                        }
                        if state.have_scale {
                            dx = mul(dx, state.scale);
                            dy = mul(dy, state.scale);
                            if state.hint && flags & ROUND_XY_TO_GRID != 0 {
                                dy = round(dy);
                            }
                        }
                        (dx, dy)
                    } else {
                        let (a1, a2) = (x as usize, y as usize);
                        let pi1 = point_base + a1;
                        let pi2 = start_point + a2;
                        if pi1 >= self.scaled.len() || pi2 >= self.scaled.len() {
                            return None;
                        }
                        let p1 = self.scaled[pi1];
                        let p2 = self.scaled[pi2];
                        (p1.x - p2.x, p1.y - p2.y)
                    };
                    if dx != 0 || dy != 0 {
                        for p in &mut self.scaled[start_point..end_point] {
                            p.x += dx;
                            p.y += dy;
                        }
                    }
                    if flags & MORE_COMPONENTS == 0 {
                        break;
                    }
                    i += 1;
                }
                let mut ins: &[u8] = &[];
                if flags & HAVE_INSTRUCTIONS != 0 {
                    let ins_len = s.read::<u16>()? as usize;
                    ins = s.read_bytes(ins_len)?;
                }
                self.deltas.resize(delta_base, Point::new(0, 0));
                if state.hint && !ins.is_empty() {
                    self.push_phantom(state);
                    self.unscaled.clear();
                    self.unscaled.extend_from_slice(&self.scaled[point_base..]);
                    self.original.clear();
                    self.original.extend_from_slice(&self.scaled[point_base..]);
                    let point_end = self.scaled.len();
                    for p in &mut self.scaled[point_end - 4..] {
                        p.x = round(p.x);
                        p.y = round(p.y);
                    }
                    for t in &mut self.tags[point_base..] {
                        *t &= !(0x08 | 0x10);
                    }
                    self.hint(state, point_base, contour_base, ins, true);
                    self.scaled.truncate(self.scaled.len() - 4);
                    self.tags.truncate(self.tags.len() - 4);
                }
                Some(())
            }
            Equal => Some(()),
        }
    }
}

/// Hinting
impl Scaler {
    fn hint(
        &mut self,
        state: &mut ScalerState,
        point_base: usize,
        contour_base: usize,
        ins: &[u8],
        is_composite: bool,
    ) -> bool {
        use super::hint::HinterMode;
        let slot = match state.slot {
            Some(slot) => slot,
            None => {
                match self.cache.prepare(
                    state.id,
                    state.data,
                    state.proxy,
                    state.coords,
                    state.ppem,
                    state.scale,
                    HinterMode::Modern,
                ) {
                    Some(slot) => {
                        state.slot = Some(slot);
                        slot
                    }
                    None => {
                        state.hint = false;
                        return false;
                    }
                }
            }
        };
        self.cache.hint(
            state.data,
            state.proxy,
            state.coords,
            slot,
            &mut self.unscaled[..],
            &mut self.original[..],
            &mut self.scaled[..],
            &mut self.tags[..],
            &mut self.contours[..],
            &mut state.phantom[..],
            point_base,
            contour_base,
            ins,
            is_composite,
        );
        true
    }
}

/// Per-component setup.
impl Scaler {
    fn setup(&mut self, state: &mut ScalerState, bounds: [i16; 4], glyph_id: u16, depth: u8) {
        let metrics = state.metrics.materialize_glyph_metrics(
            &FontRef {
                data: state.data,
                offset: 0,
                key: CacheKey(0),
            },
            state.coords,
        );
        let lsb = metrics.lsb(glyph_id) as i16;
        let advance = metrics.advance_width(glyph_id) as i32;
        let vadvance = 0;
        let tsb = 0;
        state.phantom[0].x = (bounds[0] - lsb) as i32;
        state.phantom[0].y = 0;
        state.phantom[1].x = state.phantom[0].x + advance as i32;
        state.phantom[1].y = 0;
        state.phantom[2].x = advance as i32 / 2;
        state.phantom[2].y = (bounds[3] + tsb) as i32;
        state.phantom[3].x = advance as i32 / 2;
        state.phantom[3].y = state.phantom[2].y - vadvance;
        if depth == 0 && state.have_scale {
            state.xmin = mul(bounds[0] as i32, state.scale);
            state.xmax = mul(bounds[2] as i32, state.scale);
            state.lsb = mul(lsb as i32, state.scale);
        }
        state.advance = mul(advance, state.scale);
    }

    fn push_phantom(&mut self, state: &mut ScalerState) {
        for i in 0..4 {
            self.scaled.push(state.phantom[i]);
            self.tags.push(0);
        }
    }

    fn save_phantom(&mut self, state: &mut ScalerState, point_base: usize, point_count: usize) {
        for i in 0..4 {
            state.phantom[3 - i] = self.scaled[point_base + point_count - i - 1];
        }
    }
}

pub struct ScalerState<'a> {
    pub data: &'a [u8],
    pub id: u64,
    pub coords: &'a [i16],
    pub proxy: &'a GlyfProxy,
    pub metrics: &'a MetricsProxy,
    pub slot: Option<CacheSlot>,
    pub have_scale: bool,
    pub ppem: u16,
    pub scale: i32,
    pub hint: bool,
    pub vary: bool,
    pub xmin: i32,
    pub xmax: i32,
    pub lsb: i32,
    pub advance: i32,
    pub phantom: [Point; 4],
}

impl<'a> ScalerState<'a> {
    pub fn new(
        data: &'a [u8],
        id: u64,
        coords: &'a [i16],
        proxy: &'a GlyfProxy,
        metrics: &'a MetricsProxy,
        size: f32,
        hint: bool,
    ) -> Self {
        let size = size.abs();
        let ppem = size as u16;
        let upem = metrics.units_per_em();
        let (have_scale, scale) = if size != 0. && upem != 0 {
            (true, div((size * 64.) as i32, upem as i32))
        } else {
            (false, 0)
        };
        Self {
            data,
            id,
            coords,
            proxy,
            metrics,
            slot: None,
            have_scale,
            ppem,
            scale,
            hint,
            vary: proxy.axis_count != 0 && !coords.is_empty() && proxy.gvar != 0,
            xmin: 0,
            xmax: 0,
            lsb: 0,
            advance: 0,
            phantom: Default::default(),
        }
    }
}

fn hypot(mut a: i32, mut b: i32) -> i32 {
    a = a.abs();
    b = b.abs();
    if a > b {
        a + ((3 * b) >> 3)
    } else {
        b + ((3 * a) >> 3)
    }
}

fn transform(x: i32, y: i32, xx: i32, xy: i32, yx: i32, yy: i32) -> (i32, i32) {
    let scale = 0x10000;
    (
        muldiv(x, xx, scale) + muldiv(y, xy, scale),
        muldiv(x, yx, scale) + muldiv(y, yy, scale),
    )
}
