mod hint;
mod outlines;

pub(crate) use outlines::Outlines;

use super::Outline;
use read_fonts::types::{F2Dot14, GlyphId};

pub struct SubfontCache {
    entries: Vec<Entry>,
    max_entries: usize,
    epoch: u64,
}

impl SubfontCache {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
            epoch: 0,
        }
    }

    pub fn scale(
        &mut self,
        outlines: &outlines::Outlines,
        id: u64,
        glyph_id: u16,
        size: f32,
        coords: &[i16],
        hint: bool,
        outline: &mut Outline,
    ) -> Option<()> {
        let epoch = self.epoch;
        let gid = GlyphId::new(glyph_id);
        let subfont_index = outlines.subfont_index(gid);
        let (found, entry_index) = self.find_entry(id, subfont_index, coords, size);
        let (subfont, coords) = if found {
            let entry = &mut self.entries[entry_index];
            entry.epoch = epoch;
            (&entry.subfont, &entry.coords)
        } else {
            self.epoch += 1;
            let epoch = self.epoch;
            if entry_index == self.entries.len() {
                let coords: Vec<F2Dot14> = coords.iter().map(|x| F2Dot14::from_bits(*x)).collect();
                let subfont = outlines.subfont(subfont_index, size, &coords).ok()?;
                self.entries.push(Entry {
                    id,
                    epoch,
                    subfont,
                    subfont_index,
                    size,
                    coords,
                });
                let entry = &self.entries[entry_index];
                (&entry.subfont, &entry.coords)
            } else {
                let entry = &mut self.entries[entry_index];
                entry.id = u64::MAX;
                entry.epoch = epoch;
                entry.coords.clear();
                entry
                    .coords
                    .extend(coords.iter().map(|x| F2Dot14::from_bits(*x)));
                entry.subfont = outlines.subfont(subfont_index, size, &entry.coords).ok()?;
                entry.id = id;
                entry.subfont_index = subfont_index;
                entry.size = size;
                (&entry.subfont, &entry.coords)
            }
        };
        outlines
            .draw(subfont, gid, coords, hint, &mut OutlineBuilder(outline))
            .ok()?;
        Some(())
    }

    fn find_entry(&self, id: u64, index: u32, coords: &[i16], size: f32) -> (bool, usize) {
        let mut lowest_epoch = self.epoch;
        let mut lowest_index = 0;
        for (i, entry) in self.entries.iter().enumerate() {
            if entry.id == id
                && entry.subfont_index == index
                && entry.size == size
                && coords
                    .iter()
                    .map(|x| F2Dot14::from_bits(*x))
                    .eq(entry.coords.iter().copied())
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
    subfont: outlines::Subfont,
    subfont_index: u32,
    size: f32,
    coords: Vec<F2Dot14>,
}

struct OutlineBuilder<'a>(&'a mut Outline);

impl read_fonts::types::Pen for OutlineBuilder<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.0.move_to((x, y).into());
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.0.line_to((x, y).into());
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.0.quad_to((cx0, cy0).into(), (x, y).into());
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.0
            .curve_to((cx0, cy0).into(), (cx1, cy1).into(), (x, y).into());
    }

    fn close(&mut self) {
        self.0.close();
    }
}
