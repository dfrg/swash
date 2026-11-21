use super::HintingTarget;
use alloc::vec::Vec;
use skrifa::{
    instance::{NormalizedCoord, Size},
    outline::{
        HintingInstance, HintingMode, LcdLayout, OutlineGlyphCollection, OutlineGlyphFormat,
    },
};

/// We keep this small to enable a simple LRU cache with a linear
/// search. Regenerating hinting data is low to medium cost so it's fine
/// to redo it occasionally.
const MAX_CACHED_HINT_INSTANCES: usize = 8;

pub(crate) struct HintingKey<'a> {
    pub id: [u64; 2],
    pub outlines: &'a OutlineGlyphCollection<'a>,
    pub size: Size,
    pub coords: &'a [NormalizedCoord],
    pub target: HintingTarget,
}

impl<'a> HintingKey<'a> {
    fn new_instance(&self) -> Option<HintingInstance> {
        HintingInstance::new(self.outlines, self.size, self.coords, hinting_mode(self.target))
            .ok()
    }
}

fn hinting_mode(target: HintingTarget) -> HintingMode {
    // preserve_linear_metrics is always requested so that hinting never
    // changes advance widths: swash reports linearly scaled metrics and
    // callers position glyphs based on them.
    match target {
        HintingTarget::HorizontalLcd => HintingMode::Smooth {
            lcd_subpixel: Some(LcdLayout::Horizontal),
            preserve_linear_metrics: true,
        },
        HintingTarget::Grayscale => HintingMode::Smooth {
            lcd_subpixel: None,
            preserve_linear_metrics: true,
        },
    }
}

#[derive(Default)]
pub(super) struct HintingCache {
    // Split caches because the instance type can reuse internal memory when
    // reconfigured for the same format.
    glyf_entries: Vec<HintingEntry>,
    cff_entries: Vec<HintingEntry>,
    other_entries: Vec<HintingEntry>,
    serial: u64,
}

impl HintingCache {
    pub(super) fn get(&mut self, key: &HintingKey) -> Option<&HintingInstance> {
        let entries = match key.outlines.format()? {
            OutlineGlyphFormat::Glyf => &mut self.glyf_entries,
            OutlineGlyphFormat::Cff | OutlineGlyphFormat::Cff2 => &mut self.cff_entries,
            #[allow(unreachable_patterns)]
            _ => &mut self.other_entries,
        };
        let (entry_ix, is_current) = find_hinting_entry(entries, key)?;
        let entry = entries.get_mut(entry_ix)?;
        self.serial += 1;
        entry.serial = self.serial;
        if !is_current {
            entry.id = key.id;
            entry.size = key.size;
            entry.coords.clear();
            entry.coords.extend_from_slice(key.coords);
            entry.target = key.target;
            entry
                .instance
                .reconfigure(key.outlines, key.size, key.coords, hinting_mode(key.target))
                .ok()?;
        }
        Some(&entry.instance)
    }
}

struct HintingEntry {
    id: [u64; 2],
    size: Size,
    coords: Vec<NormalizedCoord>,
    target: HintingTarget,
    instance: HintingInstance,
    serial: u64,
}

fn find_hinting_entry(entries: &mut Vec<HintingEntry>, key: &HintingKey) -> Option<(usize, bool)> {
    let mut found_serial = u64::MAX;
    let mut found_index = 0;
    for (ix, entry) in entries.iter().enumerate() {
        if entry.id == key.id
            && entry.size == key.size
            && entry.coords == key.coords
            && entry.target == key.target
        {
            return Some((ix, true));
        }
        if entry.serial < found_serial {
            found_serial = entry.serial;
            found_index = ix;
        }
    }
    if entries.len() < MAX_CACHED_HINT_INSTANCES {
        let instance = key.new_instance()?;
        let ix = entries.len();
        entries.push(HintingEntry {
            id: key.id,
            size: key.size,
            coords: key.coords.to_vec(),
            target: key.target,
            instance,
            serial: 0,
        });
        Some((ix, true))
    } else {
        Some((found_index, false))
    }
}
