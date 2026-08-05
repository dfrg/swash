//! Glyph metrics tables.

use super::{raw_tag, Bytes, RawTag};

pub const HMTX: RawTag = raw_tag(b"hmtx");
pub const VMTX: RawTag = raw_tag(b"vmtx");

/// Returns the advance for the specified glyph.
pub fn advance(data: &[u8], xmtx: u32, long_metric_count: u16, glyph_id: u16) -> u16 {
    let b = Bytes::new(data);
    let offset = if glyph_id < long_metric_count {
        glyph_id as usize * 4
    } else {
        long_metric_count.saturating_sub(1) as usize * 4
    };
    b.read_u16(offset + xmtx as usize).unwrap_or(0)
}

/// Returns the side bearing for the specified glyph.
pub fn sb(data: &[u8], xmtx: u32, long_metric_count: u16, glyph_id: u16) -> i16 {
    let b = Bytes::new(data);
    let offset = if glyph_id < long_metric_count {
        glyph_id as usize * 4 + 2
    } else {
        long_metric_count as usize * 4 + (glyph_id - long_metric_count) as usize * 2
    };
    b.read_i16(offset + xmtx as usize).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::advance;

    #[test]
    fn advance_with_zero_long_metrics_does_not_underflow() {
        // A font with numberOfHMetrics == 0 must not underflow
        // `long_metric_count - 1` (debug panic) or wrap to 65535 (release).
        // https://github.com/dfrg/swash/issues/133
        assert_eq!(advance(&[], 0, 0, 0), 0);
        assert_eq!(advance(&[], 0, 0, 5), 0);
    }
}
