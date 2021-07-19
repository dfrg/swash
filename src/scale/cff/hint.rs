use super::cff::{DictionarySink, Glyph, GlyphSink};
use super::internal::fixed::{muldiv, Fixed};

use super::TRACE;

/// Hinting state for a compact font format glyph.
///
/// Note that hinter states depend on the scale, subfont index and
/// variation coordinates of a glyph. They can be retained and reused
/// if those values remain the same.
#[derive(Copy, Clone)]
pub struct HinterState {
    scale: Fixed,
    fscale: f32,
    blues: BlueValues,
    other_blues: BlueValues,
    family_blues: BlueValues,
    family_other_blues: BlueValues,
    blue_scale: Fixed,
    blue_shift: Fixed,
    blue_fuzz: Fixed,
    language_group: u8,
    supress_overshoot: bool,
    em_box_hints: bool,
    boost: Fixed,
    zones: [BlueZone; MAX_BLUE_ZONES],
    zone_count: u16,
}

impl HinterState {
    /// Creates a new hinting state for the specified glyph, scale and
    /// variation coordinates.
    pub fn new(glyph: &Glyph, scale: f32, coords: &[i16]) -> Self {
        let mut state = Self {
            scale: Fixed::from_f32(scale),
            fscale: scale,
            blues: BlueValues::new(),
            other_blues: BlueValues::new(),
            family_blues: BlueValues::new(),
            family_other_blues: BlueValues::new(),
            blue_scale: DEFAULT_BLUE_SCALE,
            blue_shift: DEFAULT_BLUE_SHIFT,
            blue_fuzz: DEFAULT_BLUE_FUZZ,
            language_group: 0,
            supress_overshoot: false,
            em_box_hints: false,
            boost: Fixed(0),
            zones: [BlueZone::default(); MAX_BLUE_ZONES],
            zone_count: 0,
        };
        glyph.eval_private_dict(coords, &mut state);
        state.initialize_zones();
        state
    }

    /// Returns the current scaling factor of the hinter state.
    pub fn scale(&self) -> f32 {
        self.fscale
    }

    fn initialize_zones(&mut self) {
        self.em_box_hints = false;
        if self.language_group == 1 {
            if self.blues.len == 2 {
                let v = self.blues.values();
                if v[0].0 < ICF_BOTTOM
                    && v[0].1 < ICF_BOTTOM
                    && v[1].0 > ICF_TOP
                    && v[1].1 > ICF_TOP
                {
                    self.em_box_hints = true;
                }
            } else if self.blues.len == 0 {
                self.em_box_hints = true;
            }
        }
        if self.em_box_hints {
            return;
        }
        let mut zones = [BlueZone::default(); MAX_BLUE_ZONES];
        let darken_y = Fixed(0);
        let mut max_height = Fixed(0);
        let mut z = 0usize;
        for blue in self.blues.values() {
            let bottom = blue.0;
            let top = blue.1;
            let height = top - bottom;
            if height.0 < 0 {
                continue;
            }
            if height > max_height {
                max_height = height;
            }
            let zone = &mut zones[z];
            zone.bottom = bottom;
            zone.top = top;
            if z == 0 {
                zone.is_bottom = true;
                zone.flat = top;
            } else {
                zone.top += Fixed(2) * darken_y;
                zone.bottom += Fixed(2) * darken_y;
                zone.is_bottom = false;
                zone.flat = zone.bottom;
            }
            z += 1;
        }
        let units_per_pixel = Fixed::ONE / self.scale;
        for blue in self.other_blues.values() {
            let bottom = blue.0;
            let top = blue.1;
            let height = top - bottom;
            if height.0 < 0 {
                continue;
            }
            if height > max_height {
                max_height = height;
            }
            let zone = &mut zones[z];
            zone.bottom = bottom;
            zone.top = top;
            zone.is_bottom = true;
            zone.flat = top;
            z += 1;
        }
        for zone in &mut zones[..z] {
            let flat = zone.flat;
            let mut min_diff = Fixed::MAX;
            if zone.is_bottom {
                for blue in self.family_other_blues.values() {
                    let family_flat = blue.1;
                    let diff = (flat - family_flat).abs();
                    if diff < min_diff && diff < units_per_pixel {
                        zone.flat = family_flat;
                        min_diff = diff;
                        if diff.0 == 0 {
                            break;
                        }
                    }
                }
                if self.family_blues.len > 0 {
                    let family_flat = self.family_blues.values[0].1;
                    let diff = (flat - family_flat).abs();
                    if diff < min_diff && diff < units_per_pixel {
                        zone.flat = family_flat;
                    }
                }
            } else {
                for blue in self.family_blues.values().iter().skip(1) {
                    let family_flat = blue.0 + Fixed(2) * darken_y;
                    let diff = (flat - family_flat).abs();
                    if diff < min_diff && diff < units_per_pixel {
                        zone.flat = family_flat;
                        min_diff = diff;
                        if diff.0 == 0 {
                            break;
                        }
                    }
                }
            }
        }
        if max_height.0 > 0 && self.blue_scale > (Fixed::ONE / max_height) {
            self.blue_scale = Fixed::ONE / max_height;
        }
        if self.scale < self.blue_scale {
            self.supress_overshoot = true;
            self.boost = Fixed::from_f32(0.6)
                - Fixed(muldiv(
                    Fixed::from_f32(0.6).0,
                    self.scale.0,
                    self.blue_scale.0,
                ));
            if self.boost.0 > 0x7FFF {
                self.boost.0 = 0x7FFF;
            }
        }
        if darken_y.0 != 0 {
            self.boost.0 = 0;
        }
        let scale = self.scale;
        let boost = self.boost;
        for zone in &mut zones[..z] {
            let boost = if zone.is_bottom { -boost } else { boost };
            zone.ds_flat = (zone.flat * scale + boost).round();
        }
        self.zones = zones;
        self.zone_count = z as u16;
    }

    fn capture(&self, bottom: &mut Hint, top: &mut Hint) -> bool {
        let fuzz = self.blue_fuzz;
        let mut captured = false;
        let mut adjustment = Fixed(0);
        for zone in &self.zones[..self.zone_count as usize] {
            if zone.is_bottom
                && bottom.is_bottom()
                && (zone.bottom - fuzz) <= bottom.coord
                && bottom.coord <= (zone.top + fuzz)
            {
                adjustment = if self.supress_overshoot {
                    zone.ds_flat
                } else if zone.top - bottom.coord >= self.blue_shift {
                    bottom.ds_coord.round().min(zone.ds_flat - Fixed::ONE)
                } else {
                    bottom.ds_coord.round()
                } - bottom.ds_coord;
                captured = true;
                break;
            }
            if !zone.is_bottom
                && top.is_top()
                && (zone.bottom - fuzz) <= top.coord
                && top.coord <= (zone.top + fuzz)
            {
                adjustment = if self.supress_overshoot {
                    zone.ds_flat
                } else if top.coord - zone.bottom >= self.blue_shift {
                    top.ds_coord.round().max(zone.ds_flat + Fixed::ONE)
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

    // pub(super) fn dump_blues(&self) {
    //     for (i, zone) in (&self.zones[..self.zone_count as usize]).iter().enumerate() {
    //         println!(
    //             "[{}] {} t: {} b: {} f: {} ds: {}",
    //             i,
    //             if zone.is_bottom { "B" } else { "T" },
    //             zone.top.0,
    //             zone.bottom.0,
    //             zone.flat.0,
    //             zone.ds_flat.0,
    //         );
    //     }
    // }
}

pub struct Hinter<'a, 'b, Sink> {
    state: &'a HinterState,
    sink: &'b mut Sink,
    stem_hints: [StemHint; MAX_STEM_HINTS],
    stem_count: u8,
    mask: HintMask,
    initial_map: HintMap,
    map: HintMap,
}

impl<'a, 'b, Sink: GlyphSink> Hinter<'a, 'b, Sink> {
    pub fn new(state: &'a HinterState, sink: &'b mut Sink) -> Self {
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

    fn hint(&mut self, coord: f32) -> f32 {
        if !self.map.valid {
            self.build_hint_map(Some(self.mask), Fixed(0));
        }
        let f = (coord * 65536.) as i32;
        let mapped = self.map.map(self.state.scale, Fixed(f));
        ((mapped.0 >> 10) as f32) / 64.
    }

    #[inline(always)]
    fn scale(&self, coord: f32) -> f32 {
        ((Fixed::from_f32(coord) * self.state.scale).0 >> 10) as f32 / 64.
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
        stem.ds_min = Fixed(0);
        stem.ds_max = Fixed(0);
        self.stem_count = index as u8 + 1;
    }

    fn build_hint_map(&mut self, mask: Option<HintMask>, origin: Fixed) {
        self.map.build(
            &self.state,
            mask,
            Some(&mut self.initial_map),
            &mut self.stem_hints[..self.stem_count as usize],
            origin,
            false,
        );
    }
}

impl<'a, 'b, Sink: GlyphSink> GlyphSink for Hinter<'a, 'b, Sink> {
    fn hstem(&mut self, y: f32, dy: f32) {
        let y = (y * 65536.) as i32;
        let dy = (dy * 65536.) as i32;
        self.add_stem(Fixed(y), Fixed(dy));
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
            &self.state,
            Some(hint_mask),
            Some(&mut self.initial_map),
            &mut self.stem_hints[..self.stem_count as usize],
            Fixed(0),
            false,
        );
    }

    fn move_to(&mut self, x: f32, y: f32) {
        let x = self.scale(x);
        let y = self.hint(y);
        self.sink.move_to(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let x = self.scale(x);
        let y = self.hint(y);
        self.sink.line_to(x, y);
    }

    fn curve_to(&mut self, cx1: f32, cy1: f32, cx2: f32, cy2: f32, x: f32, y: f32) {
        let cx1 = self.scale(cx1);
        let cy1 = self.hint(cy1);
        let cx2 = self.scale(cx2);
        let cy2 = self.hint(cy2);
        let x = self.scale(x);
        let y = self.hint(y);
        self.sink.curve_to(cx1, cy1, cx2, cy2, x, y);
    }

    fn close(&mut self) {}
}

const ICF_TOP: Fixed = Fixed::from_i32(880);
const ICF_BOTTOM: Fixed = Fixed::from_i32(-120);

const MAX_BLUE_VALUES: usize = 7;
const MAX_BLUE_ZONES: usize = 12;
const MAX_STEM_HINTS: usize = 48;
const MAX_HINTS: usize = MAX_STEM_HINTS * 2;

const HINT_MASK_SIZE: usize = (MAX_STEM_HINTS + 7) / 4;

const DEFAULT_BLUE_SCALE: Fixed = Fixed((0.039625 * 65536. + 0.5) as i32);
const DEFAULT_BLUE_SHIFT: Fixed = Fixed::from_i32(7);
const DEFAULT_BLUE_FUZZ: Fixed = Fixed::ONE;

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
        } else if width < Fixed(0) {
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
            self.coord += Fixed::from_i32(2) * darken_y;
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

    fn map(&self, scale: Fixed, coord: Fixed) -> Fixed {
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
            //     first.coord.to_f32(),
            //     first.ds_coord.to_f32()
            // );
            // if is_pair {
            //     println!(
            //         "  Got hint at {} ({})",
            //         second.coord.to_f32(),
            //         second.ds_coord.to_f32()
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
            if let Some(ref initial) = initial {
                if is_pair {
                    let mid = initial.map(scale, (second.coord + first.coord) / 2);
                    let half = (second.coord - first.coord) / 2 * scale;
                    first.ds_coord = mid - half;
                    second.ds_coord = mid + half;
                } else {
                    first.ds_coord = initial.map(scale, first.coord);
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
            //     first.coord.to_f32(),
            //     first.ds_coord.to_f32()
            // );
            // if is_pair {
            //     println!(
            //         "  Inserting hint at {} ({})",
            //         second.coord.to_f32(),
            //         second.ds_coord.to_f32()
            //     );
            // }
        }
        self.len += count;
    }

    fn adjust(&mut self) {
        let mut saved = [(0usize, Fixed(0)); MAX_HINTS];
        let mut saved_count = 0usize;
        let mut i = 0;
        let limit = self.len;
        while i < limit {
            let is_pair = self.hints[i].is_pair();
            let j = if is_pair { i + 1 } else { i };
            if !self.hints[i].is_locked() {
                let frac_down = self.hints[i].ds_coord.fract();
                let frac_up = self.hints[j].ds_coord.fract();
                let down_move_down = Fixed(0) - frac_down;
                let up_move_down = Fixed(0) - frac_up;
                let down_move_up = if frac_down == Fixed(0) {
                    Fixed(0)
                } else {
                    Fixed::ONE - frac_down
                };
                let up_move_up = if frac_up == Fixed(0) {
                    Fixed(0)
                } else {
                    Fixed::ONE - frac_up
                };
                let move_up = Fixed(down_move_up.0.min(up_move_up.0));
                let move_down = Fixed(down_move_down.0.max(up_move_down.0));
                const MIN_COUNTER: Fixed = Fixed(0x8000);
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
                    adjustment = Fixed(0);
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
                >= (self.hints[j].ds_coord + adjustment + Fixed::from_f32(0.5))
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
        state: &HinterState,
        mask: Option<HintMask>,
        mut initial_map: Option<&mut HintMap>,
        stems: &mut [StemHint],
        origin: Fixed,
        initial: bool,
    ) {
        let scale = state.scale;
        let darken_y = Fixed(0);
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
        if state.em_box_hints {
            let mut bottom = Hint::default();
            bottom.coord = ICF_BOTTOM - Fixed(1);
            bottom.ds_coord = (bottom.coord * scale).round() - Fixed::from_f32(0.5);
            bottom.scale = scale;
            bottom.flags = GHOST_BOTTOM | LOCKED | SYNTHETIC;
            let mut top = Hint::default();
            top.coord = ICF_TOP + Fixed(1);
            top.ds_coord = (top.coord * scale).round() + Fixed::from_f32(0.5);
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
            if self.len == 0 || self.hints[0].coord.0 > 0 || self.hints[self.len - 1].coord.0 < 0 {
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
        //         hint.coord.to_f32(),
        //         hint.ds_coord.to_f32() / hint.scale.to_f32(),
        //         hint.scale.to_f32() * 65536.,
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

#[derive(Copy, Clone, PartialEq)]
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

#[derive(Copy, Clone, Default)]
struct BlueZone {
    is_bottom: bool,
    bottom: Fixed,
    top: Fixed,
    flat: Fixed,
    ds_flat: Fixed,
}

#[derive(Copy, Clone, Default)]
struct BlueValues {
    values: [(Fixed, Fixed); MAX_BLUE_VALUES],
    len: u32,
}

impl BlueValues {
    fn new() -> Self {
        Self::default()
    }

    fn set(&mut self, values: &[f32]) {
        let len = values.len().min(MAX_BLUE_VALUES * 2);
        for i in 0..len / 2 {
            let a = values[i * 2];
            let b = values[i * 2 + 1];
            self.values[i] = (Fixed::from_i32(a as i32), Fixed::from_i32(b as i32));
        }
        self.len = len as u32 / 2;
    }

    fn values(&self) -> &[(Fixed, Fixed)] {
        &self.values[..self.len as usize]
    }
}

impl DictionarySink for HinterState {
    fn blue_values(&mut self, values: &[f32]) {
        self.blues.set(values);
    }

    fn family_blues(&mut self, values: &[f32]) {
        self.family_blues.set(values);
    }

    fn other_blues(&mut self, values: &[f32]) {
        self.other_blues.set(values);
    }

    fn family_other_blues(&mut self, values: &[f32]) {
        self.family_other_blues.set(values);
    }

    fn blue_scale(&mut self, scale: f32) {
        self.blue_scale = Fixed::from_f32(scale);
    }

    fn blue_shift(&mut self, shift: f32) {
        self.blue_shift = Fixed::from_f32(shift);
    }

    fn blue_fuzz(&mut self, fuzz: f32) {
        self.blue_fuzz = Fixed::from_f32(fuzz);
    }

    fn language_group(&mut self, group: u32) {
        self.language_group = group as u8;
    }
}
