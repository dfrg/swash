use super::internal::{fixed::Fixed, Array, Bytes};

pub fn cvar_tuples<'a>(
    data: &'a [u8],
    cvar: u32,
    coords: &'a [i16],
    axis_count: u16,
) -> Option<Tuples<'a>> {
    if cvar == 0 {
        return None;
    }
    let b = Bytes::with_offset(data, cvar as usize)?;
    Some(TupleStore::from_cvar(b, axis_count).tuples(coords))
}

pub fn gvar_tuples<'a>(
    data: &'a [u8],
    gvar: u32,
    coords: &'a [i16],
    glyph_id: u16,
) -> Option<Tuples<'a>> {
    if gvar == 0 {
        return None;
    }
    let b = Bytes::with_offset(data, gvar as usize)?;
    TupleStore::from_gvar(b, glyph_id).map(|store| store.tuples(coords))
}

/// Tuple variation store.
#[derive(Copy, Clone)]
pub struct TupleStore<'a> {
    data: Bytes<'a>,
    shared_coords: SharedCoords<'a>,
    tuple_count: usize,
    shared_point_numbers: Option<PointNumbers<'a>>,
    data_offset: usize,
    is_cvar: bool,
    offset: usize,
}

impl<'a> TupleStore<'a> {
    fn from_cvar(b: Bytes<'a>, axis_count: u16) -> Self {
        let mut tuple_count = b.read_or_default::<u16>(4) as usize;
        let mut data_offset = b.read_or_default::<u16>(6) as usize;
        let shared_point_numbers = if tuple_count & 0x8000 != 0 {
            if let Some(nums) = PointNumbers::new(b, data_offset) {
                data_offset += nums.data_size();
                Some(nums)
            } else {
                tuple_count = 0;
                None
            }
        } else {
            None
        };
        tuple_count = tuple_count as usize & 0xFFF;
        Self::new(
            b,
            SharedCoords {
                data: b,
                offset: 0,
                count: 0,
                axis_count: axis_count as usize,
            },
            tuple_count,
            shared_point_numbers,
            data_offset,
            true,
            8,
        )
    }

    fn from_gvar(b: Bytes<'a>, glyph_id: u16) -> Option<Self> {
        let axis_count = b.read::<u16>(4)? as usize;
        let shared_coord_count = b.read::<u16>(6)? as usize;
        let offset_to_coord = b.read::<u32>(8)? as usize;
        let glyph_count = b.read::<u16>(12)?;
        if glyph_id >= glyph_count {
            return None;
        }
        let flags = b.read::<u16>(14)?;
        let offset_to_data = b.read::<u32>(16)? as usize;
        let offsets_base = 20usize;
        let idx = glyph_id as usize;
        let mut range = if flags & 1 != 0 {
            b.read::<u32>(offsets_base + idx * 4)? as usize
                ..b.read::<u32>(offsets_base + (idx + 1) * 4)? as usize
        } else {
            b.read::<u16>(offsets_base + idx * 2)? as usize * 2
                ..b.read::<u16>(offsets_base + (idx + 1) * 2)? as usize * 2
        };
        let len = range.len();
        if len == 0 || !b.check_range(range.start, len) {
            return None;
        }
        range.start += offset_to_data;
        range.end += offset_to_data;
        let shared_coords = SharedCoords {
            data: b,
            offset: offset_to_coord,
            count: shared_coord_count,
            axis_count,
        };
        let b = Bytes::new(b.data().get(range)?);
        let tuple_count = b.read::<u16>(0)?;
        let mut data_offset = b.read::<u16>(2)? as usize;
        let shared_point_numbers = if tuple_count & 0x8000 != 0 {
            let nums = PointNumbers::new(b, data_offset)?;
            data_offset += nums.data_size();
            Some(nums)
        } else {
            None
        };
        let tuple_count = tuple_count as usize & 0xFFF;
        Some(Self::new(
            b,
            shared_coords,
            tuple_count,
            shared_point_numbers,
            data_offset,
            false,
            4,
        ))
    }

    fn new(
        data: Bytes<'a>,
        shared_coords: SharedCoords<'a>,
        tuple_count: usize,
        shared_point_numbers: Option<PointNumbers<'a>>,
        data_offset: usize,
        is_cvar: bool,
        offset: usize,
    ) -> Self {
        Self {
            data,
            shared_coords,
            tuple_count,
            shared_point_numbers,
            data_offset,
            is_cvar,
            offset,
        }
    }

    /// Returns an iterator over the tuples in this store for the specified
    /// normalized variation coordinates.
    pub fn tuples(&self, coords: &'a [i16]) -> Tuples<'a> {
        Tuples {
            data: self.data,
            shared_coords: self.shared_coords,
            tuple_count: self.tuple_count,
            shared_point_numbers: self.shared_point_numbers,
            data_offset: self.data_offset,
            is_cvar: self.is_cvar,
            offset: self.offset,
            coords,
            cur: 0,
        }
    }
}

/// Iterator over the tuples in a tuple variation store.
#[derive(Copy, Clone)]
pub struct Tuples<'a> {
    data: Bytes<'a>,
    shared_coords: SharedCoords<'a>,
    tuple_count: usize,
    shared_point_numbers: Option<PointNumbers<'a>>,
    data_offset: usize,
    is_cvar: bool,
    offset: usize,
    coords: &'a [i16],
    cur: usize,
}

impl<'a> Tuples<'a> {
    fn next_tuple(&mut self) -> Option<Tuple<'a>> {
        let d = &self.data;
        let size = d.read::<u16>(self.offset)? as usize;
        let index = d.read::<u16>(self.offset + 2)?;
        let axis_count = self.shared_coords.axis_count;
        self.offset += 4;
        let peak = if index & 0x8000 != 0 {
            let embedded = d.read_array::<i16>(self.offset, axis_count)?;
            self.offset += 2 * axis_count;
            embedded
        } else {
            self.shared_coords.get(index as usize & 0xFFF)?
        };
        let intermediate = if index & 0x4000 != 0 {
            let intermidate = d.read_array::<i16>(self.offset, axis_count)?;
            self.offset += 2 * axis_count;
            let end = d.read_array::<i16>(self.offset, axis_count)?;
            self.offset += 2 * axis_count;
            (intermidate, end)
        } else {
            (Array::new(&[]), Array::new(&[]))
        };
        let mut offset = self.data_offset;
        let end = offset + size;
        let point_numbers = if index & 0x2000 != 0 {
            let nums = PointNumbers::new(self.data, self.data_offset)?;
            offset += nums.data_size;
            nums
        } else {
            self.shared_point_numbers?
        };
        self.data_offset += size;
        self.cur += 1;
        Some(Tuple {
            data: self.data,
            point_numbers,
            peak,
            intermediate,
            offset,
            end,
            is_cvar: self.is_cvar,
        })
    }
}

impl<'a> Iterator for Tuples<'a> {
    type Item = Deltas<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.cur > self.tuple_count {
                return None;
            }
            self.cur += 1;
            let tuple = self.next_tuple()?;
            let scalar = tuple.compute_scalar(self.coords);
            if scalar == Fixed::ZERO {
                continue;
            }
            let deltas = tuple.deltas(scalar);
            if deltas.len == 0 {
                continue;
            }
            return Some(deltas);
        }
    }
}

/// Represents a single region of applicability within a font variation space.
#[derive(Copy, Clone)]
struct Tuple<'a> {
    data: Bytes<'a>,
    point_numbers: PointNumbers<'a>,
    peak: Array<'a, i16>,
    intermediate: (Array<'a, i16>, Array<'a, i16>),
    offset: usize,
    end: usize,
    is_cvar: bool,
}

impl<'a> Tuple<'a> {
    /// Returns a scaling factor based on the specified variation coordinates
    /// for the deltas associated with this region.
    #[allow(clippy::needless_range_loop)]
    fn compute_scalar(&self, coords: &[i16]) -> Fixed {
        const ZERO: Fixed = Fixed::ZERO;
        const ONE: Fixed = Fixed::ONE;
        let mut scalar = ONE;
        let len = coords.len();
        if len != self.peak.len() {
            return ZERO;
        }
        let inter_start = self.intermediate.0;
        let inter_end = self.intermediate.1;
        let has_intermediate = inter_start.len() != 0;
        for i in 0..len {
            let peak = Fixed::from_f2dot14(self.peak.get_or(i, 0));
            let coord = Fixed::from_f2dot14(coords[i]);
            if peak == ZERO || peak == coord {
                continue;
            }
            if coord == ZERO {
                return ZERO;
            }
            if !has_intermediate {
                if coord < peak.min(ZERO) || coord > peak.max(ZERO) {
                    return ZERO;
                }
                scalar = scalar * coord / peak;
            } else {
                let start = Fixed::from_f2dot14(inter_start.get_or(i, 0));
                let end = Fixed::from_f2dot14(inter_end.get_or(i, 0));
                if coord <= start || coord >= end {
                    return ZERO;
                }
                if coord < peak {
                    scalar = scalar * (coord - start) / (peak - start);
                } else {
                    scalar = scalar * (end - coord) / (end - peak);
                }
            }
        }
        scalar
    }

    /// Returns an iterator over the deltas for this region.
    fn deltas(&self, scalar: Fixed) -> Deltas<'a> {
        if let Some(deltas) = Deltas::new(self, scalar) {
            deltas
        } else {
            Deltas {
                data: self.data,
                pts: self.point_numbers.iter(),
                len: 0,
                cur: 0,
                xs: 0,
                ys: 0,
                xrun: (0, 0, false, false),
                yrun: (0, 0, false, false),
                scalar,
            }
        }
    }
}

/// Iterator over the delta values of a particular region.
#[derive(Copy, Clone)]
pub struct Deltas<'a> {
    data: Bytes<'a>,
    pts: PointNumbersIter<'a>,
    len: usize,
    cur: usize,
    xs: usize,
    ys: usize,
    xrun: (u8, u8, bool, bool),
    yrun: (u8, u8, bool, bool),
    scalar: Fixed,
}

impl<'a> Deltas<'a> {
    fn new(parent: &Tuple<'a>, scalar: Fixed) -> Option<Self> {
        let d = &parent.data;
        let len = if parent.point_numbers.all {
            let len = Self::count_deltas(d, parent.offset, parent.end)?;
            if parent.is_cvar {
                len
            } else {
                len / 2
            }
        } else {
            parent.point_numbers.len as usize
        };
        let xs = parent.offset;
        let ys = if !parent.is_cvar {
            Self::compute_ys_offset(d, parent.offset, parent.end, len)?
        } else {
            0
        };
        Some(Self {
            data: parent.data,
            pts: parent.point_numbers.iter(),
            len,
            cur: 0,
            xs,
            ys,
            xrun: (0, 0, false, false),
            yrun: (0, 0, false, false),
            scalar,
        })
    }

    /// Returns true if this iterator provides deltas for all points.
    pub fn full_coverage(&self) -> bool {
        self.pts.parent.len == 0
    }

    fn count_deltas(d: &Bytes, offset: usize, end: usize) -> Option<usize> {
        let mut offs = offset;
        let mut n = 0;
        while offs < end {
            let control = d.read::<u8>(offs)?;
            offs += 1;
            let count = (control as usize & 0x3F) + 1;
            n += count;
            if control & 0x80 == 0 {
                let words = control & 0x40 != 0;
                if words {
                    offs += count as usize * 2;
                } else {
                    offs += count as usize;
                }
            }
        }
        Some(n)
    }

    fn compute_ys_offset(d: &Bytes, offset: usize, end: usize, x_count: usize) -> Option<usize> {
        let mut offs = offset;
        let mut n = 0;
        while offs < end {
            let control = d.read::<u8>(offs)?;
            offs += 1;
            let count = (control as usize & 0x3F) + 1;
            n += count;
            if control & 0x80 == 0 {
                let words = control & 0x40 != 0;
                if words {
                    offs += count as usize * 2;
                } else {
                    offs += count as usize;
                }
            }
            if n == x_count {
                return Some(offs);
            }
        }
        None
    }

    fn next_item(&mut self) -> Option<(usize, Fixed, Fixed)> {
        let d = &self.data;
        let point_index = self.pts.next()?;
        if self.xrun.0 >= self.xrun.1 {
            let control = d.read::<u8>(self.xs)?;
            self.xs += 1;
            self.xrun.0 = 0;
            self.xrun.1 = (control & 0x3F) + 1;
            self.xrun.2 = control & 0x80 != 0;
            self.xrun.3 = control & 0x40 != 0;
        }
        self.xrun.0 += 1;
        let x = if self.xrun.2 {
            0
        } else if self.xrun.3 {
            let offs = self.xs;
            self.xs += 2;
            d.read::<i16>(offs)?
        } else {
            let offs = self.xs;
            self.xs += 1;
            d.read::<i8>(offs)? as i16
        };
        let y = if self.ys != 0 {
            if self.yrun.0 >= self.yrun.1 {
                let control = d.read::<u8>(self.ys)?;
                self.ys += 1;
                self.yrun.0 = 0;
                self.yrun.1 = (control & 0x3F) + 1;
                self.yrun.2 = control & 0x80 != 0;
                self.yrun.3 = control & 0x40 != 0;
            }
            self.yrun.0 += 1;
            if self.yrun.2 {
                0
            } else if self.yrun.3 {
                let offs = self.ys;
                self.ys += 2;
                d.read::<i16>(offs)?
            } else {
                let offs = self.ys;
                self.ys += 1;
                d.read::<i8>(offs)? as i16
            }
        } else {
            0
        };
        let dx = self.scalar * Fixed::from_i32(x as i32);
        let dy = self.scalar * Fixed::from_i32(y as i32);
        Some((point_index as usize, dx, dy))
    }
}

impl<'a> Iterator for Deltas<'a> {
    type Item = (usize, Fixed, Fixed);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.len {
            return None;
        }
        self.cur += 1;
        self.next_item()
    }
}

#[derive(Copy, Clone)]
struct PointNumbers<'a> {
    data: Bytes<'a>,
    offset: usize,
    len: u16,
    all: bool,
    data_size: usize,
}

impl<'a> PointNumbers<'a> {
    fn new(data: Bytes<'a>, offset: usize) -> Option<Self> {
        let base = offset;
        let mut offset = offset;
        let control = data.read::<u8>(offset)? as usize;
        offset += 1;
        if control & 0x7F == 0 {
            return Some(Self {
                data,
                offset,
                len: 0,
                all: true,
                data_size: 1,
            });
        }
        let mut data_size = 1usize;
        let len = if control & 0x80 != 0 {
            data_size += 1;
            offset += 1;
            (control & 0x7F) << 8 | data.read::<u8>(offset - 1)? as usize
        } else {
            control & 0x7F
        };
        let mut points_read = 0;
        while points_read < len {
            let control = data.read::<u8>(base + data_size)?;
            data_size += 1;
            let words = control & 0x80 != 0;
            let count = (control as usize & 0x7F) + 1;
            if words {
                data_size += count * 2;
            } else {
                data_size += count;
            }
            points_read += count;
        }
        Some(Self {
            data,
            offset,
            len: len as u16,
            all: false,
            data_size,
        })
    }

    pub fn data_size(&self) -> usize {
        self.data_size
    }

    fn iter(&self) -> PointNumbersIter<'a> {
        PointNumbersIter {
            parent: *self,
            cur: 0,
            offset: self.offset,
            num: 0,
            run_len: 0,
            run_cur: 0,
            run_words: false,
        }
    }
}

#[derive(Copy, Clone)]
struct PointNumbersIter<'a> {
    parent: PointNumbers<'a>,
    cur: u16,
    offset: usize,
    num: u16,
    run_len: u16,
    run_cur: u16,
    run_words: bool,
}

impl<'a> Iterator for PointNumbersIter<'a> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.parent.all {
            let value = self.num;
            self.num += 1;
            return Some(value);
        }
        if self.cur >= self.parent.len {
            return None;
        }
        let d = &self.parent.data;
        if self.run_cur >= self.run_len {
            let control = d.read::<u8>(self.offset)? as u16;
            self.offset += 1;
            self.run_len = (control & 0x7F) + 1;
            self.run_words = control & 0x80 != 0;
            self.run_cur = 0;
        }
        let offset = self.offset;
        self.num += if self.run_words {
            self.offset += 2;
            d.read::<u16>(offset)?
        } else {
            self.offset += 1;
            d.read::<u8>(offset)? as u16
        };
        self.run_cur += 1;
        self.cur += 1;
        Some(self.num)
    }
}

#[derive(Copy, Clone)]
struct SharedCoords<'a> {
    pub data: Bytes<'a>,
    pub offset: usize,
    pub count: usize,
    pub axis_count: usize,
}

impl<'a> SharedCoords<'a> {
    fn get(&self, index: usize) -> Option<Array<'a, i16>> {
        self.data
            .read_array(self.offset + (self.axis_count * index * 2), self.axis_count)
    }
}
