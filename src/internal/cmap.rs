//! Character to glyph mapping table.

use super::{raw_tag, Array, Bytes, RawFont, RawTag, Stream};

pub const CMAP: RawTag = raw_tag(b"cmap");

/// Finds a suitable character map subtable for the specified font.
pub fn subtable<'a>(font: impl RawFont<'a>) -> Option<(u32, u8, bool)> {
    let cmap = font.table_offset(CMAP);
    if cmap == 0 {
        return None;
    }
    let mut s = Stream::with_offset(font.data(), cmap as usize)?;
    s.skip(2)?;
    let len = s.read_u16()? as usize;
    let b = Bytes::new(s.data());
    let mut best = None;
    for _ in 0..len {
        let platform = s.read_u16()?;
        let encoding = s.read_u16()?;
        let offset = s.read_u32()?;
        let format = b.read_u16(offset as usize)? as u8;
        if format != 4 && format != 12 {
            continue;
        }
        let offset = cmap.checked_add(offset)?;
        if is_symbol(platform, encoding) {
            return Some((offset, format, true));
        } else if format == 12 && is_unicode(platform, encoding) {
            best = Some((offset, format, false));
        } else if best.is_none() && is_unicode(platform, encoding) {
            best = Some((offset, format, false));
        }
    }
    best
}

/// Maps a codepoint to a glyph identifier.
pub fn map(data: &[u8], subtable: u32, format: u8, codepoint: u32) -> Option<u16> {
    if subtable == 0 {
        return None;
    }
    let b = Bytes::with_offset(data, subtable as usize)?;
    if format == 4 {
        if codepoint >= 65535 {
            return None;
        }
        let c = codepoint as u16;
        let segcount_x2 = b.read_u16(6)? as usize;
        let segcount = segcount_x2 / 2;
        b.ensure_range(0, 16 + segcount_x2 * 4)?;
        let end_codes_offset = 14;
        let start_codes_offset = end_codes_offset + segcount_x2 + 2;
        let mut l = 0;
        let mut h = segcount;
        while l < h {
            let i = (l + h) / 2;
            let i2 = i * 2;
            let start = unsafe { b.read_unchecked::<u16>(start_codes_offset + i2) };
            if c < start {
                h = i;
            } else if c > unsafe { b.read_unchecked::<u16>(end_codes_offset + i2) } {
                l = i + 1;
            } else {
                let deltas_offset = start_codes_offset + segcount_x2;
                let ranges_offset = deltas_offset + segcount_x2;
                let mut range_base = ranges_offset + i2;
                let range = unsafe { b.read_unchecked::<u16>(range_base) as usize };
                let delta = unsafe { b.read_unchecked::<i16>(deltas_offset + i2) as i32 };
                if range == 0 {
                    return Some((codepoint as i32 + delta) as u16);
                }
                range_base += range;
                let diff = (c - start) as usize * 2;
                let id = b.read::<u16>(range_base + diff).unwrap_or(0);
                return if id != 0 {
                    Some((id as i32 + delta as i32) as u16)
                } else {
                    Some(0)
                };
            }
        }
    } else if format == 12 {
        let base = 16;
        let len = b.read::<u32>(base - 4).unwrap_or(0) as usize;
        b.ensure_range(base, len * 12)?;
        let mut l = 0;
        let mut h = len;
        while l < h {
            let i = (l + h) / 2;
            let rec = base + i * 12;
            let start = unsafe { b.read_unchecked::<u32>(rec) };
            if codepoint < start {
                h = i;
            } else if codepoint > unsafe { b.read_unchecked::<u32>(rec + 4) } {
                l = i + 1;
            } else {
                let delta = unsafe { b.read_unchecked::<u32>(rec + 8) };
                return Some((codepoint - start + delta) as u16);
            }
        }
    }
    None
}

/// Enumerates all codepoint/glyph pairs in the table.
pub fn enumerate(data: &[u8], subtable: u32, mut f: impl FnMut(u32, u16)) {
    if subtable == 0 {
        return;
    }
    let b = if let Some(b) = Bytes::with_offset(data, subtable as usize) {
        b
    } else {
        return;
    };
    let format = b.read_or_default::<u16>(0);
    if format == 4 {
        let segcount_x2 = b.read::<u16>(6).unwrap_or(0) as usize;
        let segcount = segcount_x2 / 2;
        if !b.check_range(0, 16 + segcount_x2 * 4) {
            return;
        }
        let end_code_offset = 14;
        let start_code_offset = end_code_offset + segcount_x2 + 2;
        let deltas_offset = start_code_offset + segcount_x2;
        let ranges_offset = deltas_offset + segcount_x2;
        let start_codes = b
            .read_array::<u16>(start_code_offset, segcount)
            .unwrap_or_else(|| Array::new(&[]));
        let end_codes = b
            .read_array::<u16>(end_code_offset, segcount)
            .unwrap_or_else(|| Array::new(&[]));
        let deltas = b
            .read_array::<i16>(deltas_offset, segcount)
            .unwrap_or_else(|| Array::new(&[]));
        for (i, ((start, end), delta)) in start_codes
            .iter()
            .zip(end_codes.iter())
            .zip(deltas.iter())
            .enumerate()
        {
            let mut range_base = ranges_offset + i * 2;
            if let Some(range) = b.read_u16(range_base) {
                if range == 0 {
                    for codepoint in start..=end {
                        let id = (codepoint as i32 + delta as i32) as u16;
                        if id != 0 {
                            f(codepoint as u32, id);
                        }
                    }
                } else {
                    range_base += range as usize;
                    for codepoint in start..=end {
                        let diff = (codepoint - start) as usize * 2;
                        if let Some(mut id) = b.read::<u16>(range_base + diff) {
                            if id != 0 {
                                id = (id as i32 + delta as i32) as u16;
                                f(codepoint as u32, id);
                            }
                        }
                    }
                }
            }
        }
    } else if format == 12 {
        let base = 16;
        let len = b.read::<u32>(base - 4).unwrap_or(0) as usize;
        if !b.check_range(base, len * 12) {
            return;
        }
        for i in 0..len {
            let rec = base + i * 12;
            let (start, end, offset) = unsafe {
                (
                    b.read_unchecked::<u32>(rec),
                    b.read_unchecked::<u32>(rec + 4),
                    b.read_unchecked::<u32>(rec + 8),
                )
            };
            for codepoint in start..=end {
                let id = (offset + codepoint - start) as u16;
                if id != 0 {
                    f(codepoint, id);
                }
            }
        }
    }
}

fn is_unicode(platform: u16, encoding: u16) -> bool {
    match (platform, encoding) {
        (0, _) => true,
        (3, 1) => true,
        (3, 10) => true,
        _ => false,
    }
}

fn is_symbol(platform: u16, encoding: u16) -> bool {
    platform == 3 && encoding == 0
}
