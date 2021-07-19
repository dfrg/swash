use super::internal::{
    fixed::{mul, Fixed},
    glyf::*,
    var::Fvar,
    Array, RawFont,
};
use crate::{FontRef, GlyphId};

use super::{var, Point};

#[derive(Copy, Clone)]
pub struct GlyfProxy {
    pub glyf: u32,
    pub loca: u32,
    pub cvt: (u32, u32),
    pub fpgm: (u32, u32),
    pub prep: (u32, u32),
    pub cvar: u32,
    pub gvar: u32,
    pub max_storage: u16,
    pub max_stack: u16,
    pub max_fdefs: u16,
    pub max_idefs: u16,
    pub max_twilight: u16,
    pub axis_count: u16,
    pub loca_fmt: u8,
}

impl GlyfProxy {
    pub fn from_font(font: &FontRef) -> Option<Self> {
        let maxp = font.maxp()?;
        let head = font.head()?;
        let loca_fmt = head.index_to_location_format() as u8;
        let glyf = font.table_offset(GLYF);
        let loca = font.table_offset(LOCA);
        if glyf == 0 || loca == 0 || loca_fmt > 1 {
            return None;
        }
        let cvt = font.table_range(CVT_).unwrap_or((0, 0));
        let fpgm = font.table_range(FPGM).unwrap_or((0, 0));
        let prep = font.table_range(PREP).unwrap_or((0, 0));
        let cvar = font.table_offset(CVAR);
        let gvar = font.table_offset(GVAR);
        let axis_count = Fvar::from_font(font).map(|f| f.axis_count()).unwrap_or(0);
        Some(Self {
            glyf,
            loca,
            loca_fmt,
            cvt,
            fpgm,
            prep,
            cvar,
            gvar,
            max_storage: maxp.max_storage(),
            max_stack: maxp.max_stack_depth(),
            max_fdefs: maxp.max_function_definitions(),
            max_idefs: maxp.max_instruction_definitions(),
            // Four phantom points.
            max_twilight: maxp.max_twilight_points() + 4,
            axis_count,
        })
    }

    pub fn fpgm<'a>(&self, data: &'a [u8]) -> &'a [u8] {
        data.get(self.fpgm.0 as usize..self.fpgm.1 as usize)
            .unwrap_or(&[])
    }

    pub fn prep<'a>(&self, data: &'a [u8]) -> &'a [u8] {
        data.get(self.prep.0 as usize..self.prep.1 as usize)
            .unwrap_or(&[])
    }

    pub fn glyph_data<'a>(&self, data: &'a [u8], glyph_id: GlyphId) -> Option<&'a [u8]> {
        get(data, self.loca_fmt, self.loca, self.glyf, glyph_id)
    }

    pub fn cvt(
        &self,
        data: &[u8],
        scale: Option<i32>,
        coords: &[i16],
        values: &mut Vec<i32>,
    ) -> Option<()> {
        if self.cvt.0 == 0 {
            return Some(());
        }
        let cvt = data.get(self.cvt.0 as usize..self.cvt.1 as usize)?;
        let entries = Array::<i16>::new(cvt);
        let len = entries.len();
        if values.len() < len {
            values.resize(len, 0);
        }
        for (a, b) in entries.iter().zip(values.iter_mut()) {
            *b = a as i32
        }
        if !coords.is_empty() && self.cvar != 0 {
            if let Some(tuples) = var::cvar_tuples(data, self.cvar, coords, self.axis_count) {
                for deltas in tuples {
                    for (index, delta, _) in deltas {
                        if let Some(value) = values.get_mut(index) {
                            *value += delta.to_i32();
                        }
                    }
                }
            }
        }
        if let Some(scale) = scale {
            for v in values.iter_mut() {
                *v = mul(*v, scale);
            }
        }
        Some(())
    }

    pub fn deltas(
        &self,
        data: &[u8],
        coords: &[i16],
        glyph_id: u16,
        points: &[Point],
        tags: &mut [u8],
        contours: &[u16],
        accum: &mut [Point],
        deltas: &mut [Point],
    ) -> bool {
        const HAS_DELTA_TAG: u8 = 4;
        if let Some(tuples) = var::gvar_tuples(data, self.gvar, coords, glyph_id) {
            let len = points.len();
            if len > tags.len() || len > deltas.len() || len > accum.len() {
                return false;
            }
            let tags = &mut tags[..len];
            let accum = &mut accum[..len];
            let deltas = &mut deltas[..len];
            for (d, t) in deltas.iter_mut().zip(tags.iter_mut()) {
                *d = Point::default();
                *t &= !HAS_DELTA_TAG;
            }
            for tuple_deltas in tuples {
                if tuple_deltas.full_coverage() {
                    for (index, x, y) in tuple_deltas {
                        if let Some(point) = deltas.get_mut(index) {
                            point.x += x.0;
                            point.y += y.0;
                        }
                    }
                } else {
                    for p in accum.iter_mut() {
                        *p = Point::default();
                    }
                    for (index, x, y) in tuple_deltas {
                        if let Some(tag) = tags.get_mut(index) {
                            *tag |= HAS_DELTA_TAG;
                        }
                        if let Some(point) = accum.get_mut(index) {
                            point.x += x.0;
                            point.y += y.0;
                        }
                    }
                    let mut next_start = 0;
                    for end in contours.iter() {
                        let start = next_start;
                        let end = *end as usize;
                        next_start = end + 1;
                        if start >= len || end >= len {
                            continue;
                        }
                        let mut idx = start;
                        while idx <= end && tags[idx] & HAS_DELTA_TAG == 0 {
                            idx += 1;
                        }
                        if idx <= end {
                            let first_delta = idx;
                            let mut cur_delta = idx;
                            idx += 1;
                            while idx <= end {
                                if tags[idx] & HAS_DELTA_TAG != 0 {
                                    interpolate(
                                        cur_delta + 1,
                                        idx - 1,
                                        cur_delta,
                                        idx,
                                        points,
                                        accum,
                                    );
                                    cur_delta = idx;
                                }
                                idx += 1;
                            }
                            if cur_delta == first_delta {
                                let d = accum[cur_delta];
                                for a in accum[start..=end].iter_mut() {
                                    *a = d;
                                }
                            } else {
                                interpolate(
                                    cur_delta + 1,
                                    end,
                                    cur_delta,
                                    first_delta,
                                    points,
                                    accum,
                                );
                                if first_delta > 0 {
                                    interpolate(
                                        start,
                                        first_delta - 1,
                                        cur_delta,
                                        first_delta,
                                        points,
                                        accum,
                                    );
                                }
                            }
                        }
                    }
                    for ((d, t), a) in deltas.iter_mut().zip(tags.iter_mut()).zip(accum.iter()) {
                        *t &= !HAS_DELTA_TAG;
                        d.x += a.x;
                        d.y += a.y;
                    }
                }
            }
            for d in deltas.iter_mut() {
                d.x = Fixed(d.x).round().to_i32();
                d.y = Fixed(d.y).round().to_i32();
            }
            return true;
        }
        false
    }

    pub fn composite_deltas(
        &self,
        data: &[u8],
        coords: &[i16],
        glyph_id: u16,
        deltas: &mut [Point],
    ) -> bool {
        if let Some(tuples) = var::gvar_tuples(data, self.gvar, coords, glyph_id) {
            for delta in deltas.iter_mut() {
                *delta = Point::default();
            }
            for tuple_deltas in tuples {
                for (index, x, y) in tuple_deltas {
                    if let Some(point) = deltas.get_mut(index) {
                        point.x += x.round().to_i32();
                        point.y += y.round().to_i32();
                    }
                }
            }
            return true;
        }
        false
    }
}

fn interpolate(
    p1: usize,
    p2: usize,
    ref1: usize,
    ref2: usize,
    points: &[Point],
    deltas: &mut [Point],
) {
    if p1 > p2 {
        return;
    }
    let (ref1, ref2) = if points[ref1].x > points[ref2].x {
        (ref2, ref1)
    } else {
        (ref1, ref2)
    };
    let in1 = Fixed::from_i32(points[ref1].x);
    let in2 = Fixed::from_i32(points[ref2].x);
    let out1 = Fixed(deltas[ref1].x);
    let out2 = Fixed(deltas[ref2].x);
    if in1 == in2 {
        if out1 == out2 {
            for delta in deltas[p1..=p2].iter_mut() {
                delta.x = out1.0;
            }
        } else {
            for delta in deltas[p1..=p2].iter_mut() {
                delta.x = 0;
            }
        }
    } else {
        for p in p1..=p2 {
            let t = Fixed::from_i32(points[p].x);
            if t <= in1 {
                deltas[p].x = out1.0;
            } else if t >= in2 {
                deltas[p].x = out2.0;
            } else {
                let f = (t - in1) / (in2 - in1);
                deltas[p].x = ((Fixed::ONE - f) * out1 + f * out2).0;
            }
        }
    }
    // Repeat for y
    let (ref1, ref2) = if points[ref1].y > points[ref2].y {
        (ref2, ref1)
    } else {
        (ref1, ref2)
    };
    let in1 = Fixed::from_i32(points[ref1].y);
    let in2 = Fixed::from_i32(points[ref2].y);
    let out1 = Fixed(deltas[ref1].y);
    let out2 = Fixed(deltas[ref2].y);
    if in1 == in2 {
        if out1 == out2 {
            for delta in deltas[p1..=p2].iter_mut() {
                delta.y = out1.0;
            }
        } else {
            for delta in deltas[p1..=p2].iter_mut() {
                delta.y = 0;
            }
        }
    } else {
        for p in p1..=p2 {
            let t = Fixed::from_i32(points[p].y);
            if t <= in1 {
                deltas[p].y = out1.0;
            } else if t >= in2 {
                deltas[p].y = out2.0;
            } else {
                let f = (t - in1) / (in2 - in1);
                deltas[p].y = ((Fixed::ONE - f) * out1 + f * out2).0;
            }
        }
    }
}
