//! PostScript hinting.

const TRACE: bool = false;

use read_fonts::tables::postscript::{charstring::CommandSink, dict::Blues};
use read_fonts::types::Fixed;

// "Default values for OS/2 typoAscender/Descender.."
// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psblues.h#L98>
const ICF_TOP: Fixed = Fixed::from_i32(880);
const ICF_BOTTOM: Fixed = Fixed::from_i32(-120);

// <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psblues.h#L141>
const MAX_OTHER_BLUES: usize = 5;
const MAX_BLUE_ZONES: usize = 12;

const MAX_STEM_HINTS: usize = 48;
const MAX_HINTS: usize = MAX_STEM_HINTS * 2;

const HINT_MASK_SIZE: usize = (MAX_STEM_HINTS + 7) / 4;

const EPSILON: Fixed = Fixed::from_bits(1);

/// Parameters used to generate the stem and counter zones for the hinting
/// algorithm.
#[derive(Clone)]
pub(crate) struct HintParams {
    pub blues: Blues,
    pub family_blues: Blues,
    pub other_blues: Blues,
    pub family_other_blues: Blues,
    pub blue_scale: Fixed,
    pub blue_shift: Fixed,
    pub blue_fuzz: Fixed,
    pub language_group: i32,
}

impl Default for HintParams {
    fn default() -> Self {
        Self {
            blues: Blues::default(),
            other_blues: Blues::default(),
            family_blues: Blues::default(),
            family_other_blues: Blues::default(),
            // See <https://learn.microsoft.com/en-us/typography/opentype/spec/cff2#table-16-private-dict-operators>
            blue_scale: Fixed::from_f64(0.039625),
            blue_shift: Fixed::from_i32(7),
            blue_fuzz: Fixed::ONE,
            language_group: 0,
        }
    }
}

/// Hinting state for a PostScript subfont.
///
/// Note that hinter states depend on the scale, subfont index and
/// variation coordinates of a glyph. They can be retained and reused
/// if those values remain the same.
#[derive(Copy, Clone)]
pub(crate) struct HintState {
    scale: Fixed,
    blue_scale: Fixed,
    // These will be used in later code.
    #[allow(dead_code)]
    blue_shift: Fixed,
    #[allow(dead_code)]
    blue_fuzz: Fixed,
    language_group: u8,
    supress_overshoot: bool,
    do_em_box_hints: bool,
    boost: Fixed,
    darken_y: Fixed,
    zones: [BlueZone; MAX_BLUE_ZONES],
    zone_count: usize,
}

impl HintState {
    pub fn new(params: &HintParams, scale: Fixed) -> Self {
        let mut state = Self {
            scale,
            blue_scale: params.blue_scale,
            blue_shift: params.blue_shift,
            blue_fuzz: params.blue_fuzz,
            language_group: params.language_group as u8,
            supress_overshoot: false,
            do_em_box_hints: false,
            boost: Fixed::ZERO,
            darken_y: Fixed::ZERO,
            zones: [BlueZone::default(); MAX_BLUE_ZONES],
            zone_count: 0,
        };
        state.build_zones(params);
        state
    }

    /// Initialize zones from the set of blues values.
    ///
    /// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psblues.c#L66>
    fn build_zones(&mut self, params: &HintParams) {
        self.do_em_box_hints = false;
        // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psblues.c#L141>
        match (self.language_group, params.blues.values().len()) {
            (1, 2) => {
                let blues = params.blues.values();
                if blues[0].0 < ICF_BOTTOM
                    && blues[0].1 < ICF_BOTTOM
                    && blues[1].0 > ICF_TOP
                    && blues[1].1 > ICF_TOP
                {
                    // FreeType generates synthetic hints here. We'll do it
                    // later when building the hint map.
                    self.do_em_box_hints = true;
                    return;
                }
            }
            (1, 0) => {
                self.do_em_box_hints = true;
                return;
            }
            _ => {}
        }
        let mut zones = [BlueZone::default(); MAX_BLUE_ZONES];
        let mut max_zone_height = Fixed::ZERO;
        let mut zone_count = 0usize;
        // Copy blues and other blues to a combined array of top and bottom zones.
        for blue in params.blues.values() {
            // FreeType loads blues as integers and then expands to 16.16
            // at initialization. We load them as 16.16 so floor them here
            // to ensure we match.
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psblues.c#L190>
            let bottom = blue.0.floor();
            let top = blue.1.floor();
            let zone_height = top - bottom;
            if zone_height < Fixed::ZERO {
                // Reject zones with negative height
                continue;
            }
            max_zone_height = max_zone_height.max(zone_height);
            let zone = &mut zones[zone_count];
            zone.cs_bottom_edge = bottom;
            zone.cs_top_edge = top;
            if zone_count == 0 {
                // First blue value is bottom zone
                zone.is_bottom = true;
                zone.cs_flat_edge = top;
            } else {
                // Adjust both edges of top zone upward by twice darkening amount
                zone.cs_top_edge += twice(self.darken_y);
                zone.cs_bottom_edge += twice(self.darken_y);
                // Remaining blue values are top zones
                zone.is_bottom = false;
                zone.cs_flat_edge = zone.cs_bottom_edge;
            }
            zone_count += 1;
        }
        for blue in params.other_blues.values().iter().take(MAX_OTHER_BLUES) {
            let bottom = blue.0.floor();
            let top = blue.1.floor();
            let zone_height = top - bottom;
            if zone_height < Fixed::ZERO {
                // Reject zones with negative height
                continue;
            }
            max_zone_height = max_zone_height.max(zone_height);
            let zone = &mut zones[zone_count];
            // All "other" blues are bottom zone
            zone.is_bottom = true;
            zone.cs_bottom_edge = bottom;
            zone.cs_top_edge = top;
            zone.cs_flat_edge = top;
            zone_count += 1;
        }
        // Adjust for family blues
        let units_per_pixel = Fixed::ONE / self.scale;
        for zone in &mut zones[..zone_count] {
            let flat = zone.cs_flat_edge;
            let mut min_diff = Fixed::MAX;
            if zone.is_bottom {
                // In a bottom zone, the top edge is the flat edge.
                // Search family other blues for bottom zones. Look for the
                // closest edge that is within the one pixel threshold.
                for blue in params.family_other_blues.values() {
                    let family_flat = blue.1;
                    let diff = (flat - family_flat).abs();
                    if diff < min_diff && diff < units_per_pixel {
                        zone.cs_flat_edge = family_flat;
                        min_diff = diff;
                        if diff == Fixed::ZERO {
                            break;
                        }
                    }
                }
                // Check the first member of family blues, which is a bottom
                // zone
                if !params.family_blues.values().is_empty() {
                    let family_flat = params.family_blues.values()[0].1;
                    let diff = (flat - family_flat).abs();
                    if diff < min_diff && diff < units_per_pixel {
                        zone.cs_flat_edge = family_flat;
                    }
                }
            } else {
                // In a top zone, the bottom edge is the flat edge.
                // Search family blues for top zones, skipping the first, which
                // is a bottom zone. Look for closest family edge that is
                // within the one pixel threshold.
                for blue in params.family_blues.values().iter().skip(1) {
                    let family_flat = blue.0 + twice(self.darken_y);
                    let diff = (flat - family_flat).abs();
                    if diff < min_diff && diff < units_per_pixel {
                        zone.cs_flat_edge = family_flat;
                        min_diff = diff;
                        if diff == Fixed::ZERO {
                            break;
                        }
                    }
                }
            }
        }
        if max_zone_height > Fixed::ZERO && self.blue_scale > (Fixed::ONE / max_zone_height) {
            // Clamp at maximum scale
            self.blue_scale = Fixed::ONE / max_zone_height;
        }
        // Suppress overshoot and boost blue zones at small sizes
        if self.scale < self.blue_scale {
            self.supress_overshoot = true;
            self.boost =
                Fixed::from_f64(0.6) - Fixed::from_f64(0.6).mul_div(self.scale, self.blue_scale);
            // boost must remain less than 0.5, or baseline could go negative
            self.boost = self.boost.min(Fixed::from_bits(0x7FFF));
        }
        if self.darken_y != Fixed::ZERO {
            self.boost = Fixed::ZERO;
        }
        // Set device space alignment for each zone; apply boost amount before
        // rounding flat edge
        let scale = self.scale;
        let boost = self.boost;
        for zone in &mut zones[..zone_count] {
            let boost = if zone.is_bottom { -boost } else { boost };
            zone.ds_flat_edge = (zone.cs_flat_edge * scale + boost).round();
        }
        self.zones = zones;
        self.zone_count = zone_count;
    }

    fn capture(&self, bottom: &mut Hint, top: &mut Hint) -> bool {
        let fuzz = self.blue_fuzz;
        let mut captured = false;
        let mut adjustment = Fixed::ZERO;
        for zone in &self.zones[..self.zone_count] {
            if zone.is_bottom
                && bottom.is_bottom()
                && (zone.cs_bottom_edge - fuzz) <= bottom.coord
                && bottom.coord <= (zone.cs_top_edge + fuzz)
            {
                adjustment = if self.supress_overshoot {
                    zone.ds_flat_edge
                } else if zone.cs_top_edge - bottom.coord >= self.blue_shift {
                    bottom.ds_coord.round().min(zone.ds_flat_edge - Fixed::ONE)
                } else {
                    bottom.ds_coord.round()
                } - bottom.ds_coord;
                captured = true;
                break;
            }
            if !zone.is_bottom
                && top.is_top()
                && (zone.cs_bottom_edge - fuzz) <= top.coord
                && top.coord <= (zone.cs_top_edge + fuzz)
            {
                adjustment = if self.supress_overshoot {
                    zone.ds_flat_edge
                } else if top.coord - zone.cs_bottom_edge >= self.blue_shift {
                    top.ds_coord.round().max(zone.ds_flat_edge + Fixed::ONE)
                } else {
                    top.ds_coord.round()
                } - top.ds_coord;
                captured = true;
                break;
            }
        }
        if captured {
            if bottom.is_valid() {
                bottom.ds_coord += adjustment;
                bottom.lock();
            }
            if top.is_valid() {
                top.ds_coord += adjustment;
                top.lock();
            }
        }
        captured
    }
}

pub(crate) struct Hinter<'a, S> {
    state: &'a HintState,
    sink: &'a mut S,
    stem_hints: [StemHint; MAX_STEM_HINTS],
    stem_count: u8,
    mask: HintMask,
    initial_map: HintMap,
    map: HintMap,
}

impl<'a, S: CommandSink> Hinter<'a, S> {
    pub fn new(state: &'a HintState, sink: &'a mut S) -> Self {
        Self {
            state,
            sink,
            stem_hints: [StemHint::default(); MAX_STEM_HINTS],
            stem_count: 0,
            mask: HintMask::all(),
            initial_map: HintMap::new(),
            map: HintMap::new(),
        }
    }

    fn hint(&mut self, coord: Fixed) -> Fixed {
        if !self.map.valid {
            self.build_hint_map(Some(self.mask), Fixed::ZERO);
        }
        Fixed::from_bits(self.map.apply(self.state.scale, coord).to_bits() >> 6)
    }

    #[inline(always)]
    fn scale(&self, coord: Fixed) -> Fixed {
        Fixed::from_bits((coord * self.state.scale).to_bits() >> 6)
    }

    fn add_stem(&mut self, min: Fixed, max: Fixed) {
        let index = self.stem_count as usize;
        if index >= MAX_STEM_HINTS || self.map.valid {
            return;
        }
        let stem = &mut self.stem_hints[index];
        stem.min = min;
        stem.max = max;
        stem.used = false;
        stem.ds_min = Fixed::ZERO;
        stem.ds_max = Fixed::ZERO;
        self.stem_count = index as u8 + 1;
    }

    fn build_hint_map(&mut self, mask: Option<HintMask>, origin: Fixed) {
        self.map.build(
            self.state,
            mask,
            Some(&mut self.initial_map),
            &mut self.stem_hints[..self.stem_count as usize],
            origin,
            false,
        );
    }
}

impl<'a, S: CommandSink> CommandSink for Hinter<'a, S> {
    fn hstem(&mut self, y: Fixed, dy: Fixed) {
        self.add_stem(y, dy);
    }

    fn hint_mask(&mut self, mask: &[u8]) {
        let mut hint_mask = HintMask::new();
        hint_mask.set_mask(mask);
        if TRACE {
            //println!("Got hintmask: {:?}", mask);
        }
        if hint_mask != self.mask {
            self.mask = hint_mask;
            self.map.valid = false;
        }
    }

    fn counter_mask(&mut self, mask: &[u8]) {
        let mut hint_mask = HintMask::new();
        hint_mask.set_mask(mask);
        let mut map = HintMap::new();
        map.build(
            self.state,
            Some(hint_mask),
            Some(&mut self.initial_map),
            &mut self.stem_hints[..self.stem_count as usize],
            Fixed::ZERO,
            false,
        );
    }

    fn move_to(&mut self, x: Fixed, y: Fixed) {
        let x = self.scale(x);
        let y = self.hint(y);
        self.sink.move_to(x, y);
    }

    fn line_to(&mut self, x: Fixed, y: Fixed) {
        let x = self.scale(x);
        let y = self.hint(y);
        self.sink.line_to(x, y);
    }

    fn curve_to(&mut self, cx1: Fixed, cy1: Fixed, cx2: Fixed, cy2: Fixed, x: Fixed, y: Fixed) {
        let cx1 = self.scale(cx1);
        let cy1 = self.hint(cy1);
        let cx2 = self.scale(cx2);
        let cy2 = self.hint(cy2);
        let x = self.scale(x);
        let y = self.hint(y);
        self.sink.curve_to(cx1, cy1, cx2, cy2, x, y);
    }

    fn close(&mut self) {
        self.sink.close();
    }
}

/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psblues.h#L129>
#[derive(Copy, Clone, PartialEq, Default, Debug)]
struct BlueZone {
    is_bottom: bool,
    cs_bottom_edge: Fixed,
    cs_top_edge: Fixed,
    cs_flat_edge: Fixed,
    ds_flat_edge: Fixed,
}

#[derive(Copy, Clone, Default)]
struct StemHint {
    used: bool,
    min: Fixed,
    max: Fixed,
    ds_min: Fixed,
    ds_max: Fixed,
}

// Hint flags
const GHOST_BOTTOM: u8 = 0x1;
const GHOST_TOP: u8 = 0x2;
const PAIR_BOTTOM: u8 = 0x4;
const PAIR_TOP: u8 = 0x8;
const LOCKED: u8 = 0x10;
const SYNTHETIC: u8 = 0x20;

/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/80a507a6b8e3d2906ad2c8ba69329bd2fb2a85ef/src/psaux/psblues.h#L118>
#[derive(Copy, Clone, Default)]
struct Hint {
    flags: u8,
    index: u8,
    coord: Fixed,
    ds_coord: Fixed,
    scale: Fixed,
}

impl Hint {
    fn is_valid(&self) -> bool {
        self.flags != 0
    }

    fn is_bottom(&self) -> bool {
        self.flags & (GHOST_BOTTOM | PAIR_BOTTOM) != 0
    }

    fn is_top(&self) -> bool {
        self.flags & (GHOST_TOP | PAIR_TOP) != 0
    }

    fn is_pair(&self) -> bool {
        self.flags & (PAIR_BOTTOM | PAIR_TOP) != 0
    }

    fn is_pair_top(&self) -> bool {
        self.flags & PAIR_TOP != 0
    }

    fn is_locked(&self) -> bool {
        self.flags & LOCKED != 0
    }

    fn is_synthetic(&self) -> bool {
        self.flags & SYNTHETIC != 0
    }

    fn lock(&mut self) {
        self.flags |= LOCKED
    }

    fn setup(
        &mut self,
        stem: &StemHint,
        index: u8,
        origin: Fixed,
        scale: Fixed,
        darken_y: Fixed,
        is_bottom: bool,
    ) {
        let width = stem.max - stem.min;
        if width == Fixed::from_i32(-21) {
            if is_bottom {
                self.coord = stem.max;
                self.flags = GHOST_BOTTOM;
            } else {
                self.flags = 0;
            }
        } else if width == Fixed::from_i32(-20) {
            if !is_bottom {
                self.coord = stem.min;
                self.flags = GHOST_TOP;
            } else {
                self.flags = 0;
            }
        } else if width < Fixed::ZERO {
            if is_bottom {
                self.coord = stem.max;
                self.flags = PAIR_BOTTOM;
            } else {
                self.coord = stem.min;
                self.flags = PAIR_TOP;
            }
        } else if is_bottom {
            self.coord = stem.min;
            self.flags = PAIR_BOTTOM;
        } else {
            self.coord = stem.max;
            self.flags = PAIR_TOP;
        }
        if self.is_top() {
            self.coord += twice(darken_y);
        }
        self.coord += origin;
        self.scale = scale;
        self.index = index;
        if self.flags != 0 && stem.used {
            if self.is_top() {
                self.ds_coord = stem.ds_max;
            } else {
                self.ds_coord = stem.ds_min;
            }
            self.lock();
        } else {
            self.ds_coord = self.coord * scale;
        }
    }
}

#[derive(Copy, Clone)]
struct HintMap {
    hints: [Hint; MAX_HINTS],
    len: usize,
    valid: bool,
}

impl HintMap {
    fn new() -> Self {
        Self {
            hints: [Hint::default(); MAX_HINTS],
            len: 0,
            valid: false,
        }
    }

    fn clear(&mut self) {
        self.len = 0;
        self.valid = false;
    }

    fn apply(&self, scale: Fixed, coord: Fixed) -> Fixed {
        if self.len == 0 {
            return coord * scale;
        }
        let limit = self.len - 1;
        let mut i = 0;
        while i < limit && coord >= self.hints[i + 1].coord {
            i += 1;
        }
        while i > 0 && coord < self.hints[i].coord {
            i -= 1;
        }
        let hint = &self.hints[i];
        if i == 0 && coord < self.hints[0].coord {
            ((coord - self.hints[0].coord) * scale) + self.hints[0].ds_coord
        } else {
            ((coord - hint.coord) * hint.scale) + hint.ds_coord
        }
    }

    fn insert(&mut self, bottom: &Hint, top: &Hint, scale: Fixed, initial: Option<&HintMap>) {
        let (is_pair, mut first) = if !bottom.is_valid() {
            (false, *top)
        } else if !top.is_valid() {
            (false, *bottom)
        } else {
            (true, *bottom)
        };
        let mut second = *top;
        if is_pair && top.coord < bottom.coord {
            return;
        }
        let count = if is_pair { 2 } else { 1 };
        if self.len + count > MAX_HINTS {
            return;
        }
        if TRACE {
            // println!(
            //     "  Got hint at {} ({})",
            //     first.coord.to_f64(),
            //     first.ds_coord.to_f64()
            // );
            // if is_pair {
            //     println!(
            //         "  Got hint at {} ({})",
            //         second.coord.to_f64(),
            //         second.ds_coord.to_f64()
            //     );
            // }
        }
        let mut insertion_index = 0;
        while insertion_index < self.len {
            if self.hints[insertion_index].coord >= first.coord {
                break;
            }
            insertion_index += 1;
        }
        if insertion_index < self.len {
            let current = &self.hints[insertion_index];
            if (current.coord == first.coord)
                || (is_pair && current.coord <= second.coord)
                || current.is_pair_top()
            {
                return;
            }
        }
        if !first.is_locked() {
            if let Some(initial) = initial {
                if is_pair {
                    let mid = initial.apply(scale, first.coord + half(second.coord - first.coord));
                    let half = half(second.coord - first.coord) * scale;
                    first.ds_coord = mid - half;
                    second.ds_coord = mid + half;
                } else {
                    first.ds_coord = initial.apply(scale, first.coord);
                }
            }
        }
        if insertion_index > 0 && first.ds_coord < self.hints[insertion_index - 1].ds_coord {
            return;
        }
        if insertion_index < self.len
            && ((is_pair && second.ds_coord > self.hints[insertion_index].ds_coord)
                || first.ds_coord > self.hints[insertion_index].ds_coord)
        {
            return;
        }
        if insertion_index != self.len {
            let mut src_index = self.len - 1;
            let mut dst_index = self.len + count - 1;
            loop {
                self.hints[dst_index] = self.hints[src_index];
                if src_index == insertion_index {
                    break;
                }
                src_index -= 1;
                dst_index -= 1;
            }
        }
        self.hints[insertion_index] = first;
        if is_pair {
            self.hints[insertion_index + 1] = second;
        }
        if TRACE {
            // println!(
            //     "  Inserting hint at {} ({})",
            //     first.coord.to_f64(),
            //     first.ds_coord.to_f64()
            // );
            // if is_pair {
            //     println!(
            //         "  Inserting hint at {} ({})",
            //         second.coord.to_f64(),
            //         second.ds_coord.to_f64()
            //     );
            // }
        }
        self.len += count;
    }

    fn adjust(&mut self) {
        let mut saved = [(0usize, Fixed::ZERO); MAX_HINTS];
        let mut saved_count = 0usize;
        let mut i = 0;
        let limit = self.len;
        while i < limit {
            let is_pair = self.hints[i].is_pair();
            let j = if is_pair { i + 1 } else { i };
            if !self.hints[i].is_locked() {
                let frac_down = self.hints[i].ds_coord.fract();
                let frac_up = self.hints[j].ds_coord.fract();
                let down_move_down = Fixed::ZERO - frac_down;
                let up_move_down = Fixed::ZERO - frac_up;
                let down_move_up = if frac_down == Fixed::ZERO {
                    Fixed::ZERO
                } else {
                    Fixed::ONE - frac_down
                };
                let up_move_up = if frac_up == Fixed::ZERO {
                    Fixed::ZERO
                } else {
                    Fixed::ONE - frac_up
                };
                let move_up = down_move_up.min(up_move_up);
                let move_down = down_move_down.max(up_move_down);
                const MIN_COUNTER: Fixed = Fixed::from_bits(0x8000);
                let mut save_edge = false;
                let adjustment;
                if j >= self.len - 1
                    || self.hints[j + 1].ds_coord
                        >= (self.hints[j].ds_coord + move_up + MIN_COUNTER)
                {
                    if i == 0
                        || self.hints[i - 1].ds_coord
                            <= (self.hints[i].ds_coord + move_down - MIN_COUNTER)
                    {
                        adjustment = if -move_down < move_up {
                            move_down
                        } else {
                            move_up
                        };
                    } else {
                        adjustment = move_up;
                    }
                } else if i == 0
                    || self.hints[i - 1].ds_coord
                        <= (self.hints[i].ds_coord + move_down - MIN_COUNTER)
                {
                    adjustment = move_down;
                    save_edge = move_up < -move_down;
                } else {
                    adjustment = Fixed::ZERO;
                    save_edge = true;
                }
                if save_edge && j < self.len - 1 && !self.hints[j + 1].is_locked() {
                    saved[saved_count] = (j, move_up - adjustment);
                    saved_count += 1;
                }
                self.hints[i].ds_coord += adjustment;
                if is_pair {
                    self.hints[j].ds_coord += adjustment;
                }
            }
            if i > 0 && self.hints[i].coord != self.hints[i - 1].coord {
                let a = self.hints[i];
                let b = self.hints[i - 1];
                self.hints[i - 1].scale = (a.ds_coord - b.ds_coord) / (a.coord - b.coord);
            }
            if is_pair {
                if self.hints[j].coord != self.hints[j - 1].coord {
                    let a = self.hints[j];
                    let b = self.hints[j - 1];
                    self.hints[j - 1].scale = (a.ds_coord - b.ds_coord) / (a.coord - b.coord);
                }
                i += 1;
            }
            i += 1;
        }
        for i in (0..saved_count).rev() {
            let (j, adjustment) = saved[i];
            if self.hints[j + 1].ds_coord
                >= (self.hints[j].ds_coord + adjustment + Fixed::from_f64(0.5))
            {
                self.hints[j].ds_coord += adjustment;
                if self.hints[j].is_pair() {
                    self.hints[j - 1].ds_coord += adjustment;
                }
            }
        }
    }

    fn build(
        &mut self,
        state: &HintState,
        mask: Option<HintMask>,
        mut initial_map: Option<&mut HintMap>,
        stems: &mut [StemHint],
        origin: Fixed,
        initial: bool,
    ) {
        let scale = state.scale;
        let darken_y = Fixed::ZERO;
        if !initial {
            if let Some(ref mut initial_map) = initial_map {
                if !initial_map.valid {
                    initial_map.build(state, None, None, stems, origin, true);
                }
            }
        }
        let initial_map = initial_map.map(|x| x as &HintMap);
        self.clear();
        let mut mask = mask.unwrap_or_else(HintMask::all);
        if !mask.valid {
            mask = HintMask::all();
        }
        if state.do_em_box_hints {
            let mut bottom = Hint::default();
            bottom.coord = ICF_BOTTOM - EPSILON;
            bottom.ds_coord = (bottom.coord * scale).round() - Fixed::from_f64(0.5);
            bottom.scale = scale;
            bottom.flags = GHOST_BOTTOM | LOCKED | SYNTHETIC;
            let mut top = Hint::default();
            top.coord = ICF_TOP + EPSILON + twice(state.darken_y);
            top.ds_coord = (top.coord * scale).round() + Fixed::from_f64(0.5);
            top.scale = scale;
            top.flags = GHOST_TOP | LOCKED | SYNTHETIC;
            let invalid = Hint::default();
            self.insert(&bottom, &invalid, scale, initial_map);
            self.insert(&invalid, &top, scale, initial_map);
        }
        let mut tmp_mask = mask;
        for (i, stem) in stems.iter().enumerate() {
            if !tmp_mask.get(i) {
                continue;
            }
            let mut bottom = Hint::default();
            let mut top = Hint::default();
            bottom.setup(stem, i as u8, origin, scale, darken_y, true);
            top.setup(stem, i as u8, origin, scale, darken_y, false);
            if bottom.is_locked() || top.is_locked() || state.capture(&mut bottom, &mut top) {
                if initial {
                    self.insert(&bottom, &top, scale, None);
                } else {
                    self.insert(&bottom, &top, scale, initial_map);
                }
                tmp_mask.clear(i);
            }
        }
        if initial {
            if self.len == 0
                || self.hints[0].coord > Fixed::ZERO
                || self.hints[self.len - 1].coord < Fixed::ZERO
            {
                let edge = Hint {
                    flags: GHOST_BOTTOM | LOCKED | SYNTHETIC,
                    scale,
                    ..Default::default()
                };
                let invalid = Hint::default();
                self.insert(&edge, &invalid, scale, None);
            }
        } else {
            for (i, stem) in stems.iter().enumerate() {
                if !tmp_mask.get(i) {
                    continue;
                }
                let mut bottom = Hint::default();
                let mut top = Hint::default();
                bottom.setup(stem, i as u8, origin, scale, darken_y, true);
                top.setup(stem, i as u8, origin, scale, darken_y, false);
                self.insert(&bottom, &top, scale, initial_map);
            }
        }
        self.dump();
        self.adjust();
        self.dump();
        if !initial {
            for i in 0..self.len {
                let hint = &self.hints[i];
                if hint.is_synthetic() {
                    continue;
                }
                let stem = &mut stems[hint.index as usize];
                if hint.is_top() {
                    stem.ds_max = hint.ds_coord;
                } else {
                    stem.ds_min = hint.ds_coord;
                }
                stem.used = true;
            }
            self.valid = true;
        }
        self.valid = true;
    }

    fn dump(&self) {
        // if !TRACE {
        //     return;
        // }
        // for i in 0..self.len {
        //     let hint = self.hints[i];
        //     println!(
        //         "[{}] {} {} {} {}{}{}{}{}{}",
        //         hint.index,
        //         hint.coord.to_f64(),
        //         hint.ds_coord.to_f64() / hint.scale.to_f64(),
        //         hint.scale.to_f64() * 65536.,
        //         if hint.is_pair() { "p" } else { "" },
        //         if hint.flags & (GHOST_BOTTOM | GHOST_TOP) != 0 {
        //             "g"
        //         } else {
        //             ""
        //         },
        //         if hint.is_top() { "t" } else { "" },
        //         if hint.is_bottom() { "b" } else { "" },
        //         if hint.is_locked() { "L" } else { "" },
        //         if hint.is_synthetic() { "S" } else { "" },
        //     );
        // }
        // println!("-------------------------------");
    }
}

#[derive(Copy, Clone, PartialEq, Default)]
pub struct HintMask {
    mask: [u8; HINT_MASK_SIZE],
    valid: bool,
}

impl HintMask {
    pub fn new() -> Self {
        Self {
            mask: [0u8; HINT_MASK_SIZE],
            valid: false,
        }
    }

    pub fn all() -> Self {
        Self {
            mask: [0xFF; HINT_MASK_SIZE],
            valid: true,
        }
    }

    fn clear_all(&mut self) {
        self.mask = [0u8; HINT_MASK_SIZE];
        self.valid = true;
    }

    pub fn set_mask(&mut self, mask: &[u8]) {
        self.clear_all();
        if mask.len() > HINT_MASK_SIZE {
            return;
        }
        for (i, b) in mask.iter().enumerate() {
            self.mask[i] = *b;
        }
        self.valid = true;
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set(&mut self, bit: usize) {
        self.mask[bit >> 3] |= 1 << (7 - (bit & 0x7));
    }

    #[inline]
    pub fn clear(&mut self, bit: usize) {
        self.mask[bit >> 3] &= !(1 << (7 - (bit & 0x7)));
    }

    #[inline]
    pub fn get(&self, bit: usize) -> bool {
        self.mask[bit >> 3] & (1 << (7 - (bit & 0x7))) != 0
    }

    // pub fn dump(&self) {
    //     for i in 0..MAX_STEM_HINTS {
    //         print!("{}", self.get(i) as u8);
    //     }
    //     println!();
    //     for b in &self.mask {
    //         print!("{:#8b}", *b)
    //     }
    //     println!();
    // }
}

fn half(value: Fixed) -> Fixed {
    Fixed::from_bits(value.to_bits() / 2)
}

fn twice(value: Fixed) -> Fixed {
    Fixed::from_bits(value.to_bits().wrapping_mul(2))
}
