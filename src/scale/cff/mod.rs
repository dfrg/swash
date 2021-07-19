/*!
PostScript outlines.

*/

#[allow(clippy::module_inception)]
mod cff;
mod hint;

pub use cff::{Cff, CffProxy, Glyph, GlyphSink};
pub use hint::HinterState;

use super::{internal, TRACE};

use crate::font::FontRef;

pub struct Scaler {
    entries: Vec<Entry>,
    max_entries: usize,
    epoch: u64,
}

impl Scaler {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
            epoch: 0,
        }
    }

    pub fn scale(
        &mut self,
        font: &FontRef,
        id: u64,
        coords: &[i16],
        proxy: &CffProxy,
        scale: f32,
        hint: bool,
        glyph_id: u16,
        sink: &mut impl GlyphSink,
    ) -> Option<()> {
        let cff = proxy.materialize(font);
        let glyph = cff.get(glyph_id)?;
        if hint {
            let dict = glyph.subfont_index();
            let state = self.entry(id, dict, coords, scale, &glyph);
            if glyph.path(scale, coords, Some(state), sink) {
                Some(())
            } else {
                None
            }
        } else if glyph.path(scale, coords, None, sink) {
            Some(())
        } else {
            None
        }
    }

    fn entry(
        &mut self,
        id: u64,
        dict: u16,
        coords: &[i16],
        scale: f32,
        glyph: &Glyph,
    ) -> &HinterState {
        let epoch = self.epoch;
        let (found, index) = self.find_entry(id, dict, coords, scale);
        if found {
            let entry = &mut self.entries[index];
            entry.epoch = epoch;
            &entry.state
        } else {
            self.epoch += 1;
            let state = HinterState::new(glyph, scale, coords);
            if index == self.entries.len() {
                self.entries.push(Entry {
                    epoch,
                    id,
                    dict,
                    state,
                    coords: Vec::from(coords),
                    scale,
                });
                &self.entries[index].state
            } else {
                let entry = &mut self.entries[index];
                entry.epoch = epoch;
                entry.id = id;
                entry.dict = dict;
                entry.state = state;
                entry.coords.clear();
                entry.coords.extend_from_slice(coords);
                entry.scale = scale;
                &entry.state
            }
        }
    }

    fn find_entry(&self, id: u64, dict: u16, coords: &[i16], scale: f32) -> (bool, usize) {
        let mut lowest_epoch = self.epoch;
        let mut lowest_index = 0;
        let vary = !coords.is_empty();
        for (i, entry) in self.entries.iter().enumerate() {
            if entry.id == id
                && entry.dict == dict
                && entry.scale == scale
                && (!vary || (coords == &entry.coords[..]))
            {
                return (true, i);
            }
            if entry.epoch < lowest_epoch {
                lowest_epoch = entry.epoch;
                lowest_index = i;
            }
        }
        if self.entries.len() < self.max_entries {
            lowest_index = self.entries.len();
        }
        (false, lowest_index)
    }
}

struct Entry {
    epoch: u64,
    id: u64,
    dict: u16,
    state: HinterState,
    coords: Vec<i16>,
    scale: f32,
}
