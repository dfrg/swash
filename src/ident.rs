/*!
Helpers for uniquely identifying fonts.
*/

use super::internal::{Bytes, raw_tag};
use super::FontRef;

/// Monotonically increasing identifier for a font.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct Key(pub(crate) u64);

impl Key {
    /// Allocates a new font key.
    pub fn new() -> Self {
        use core::sync::atomic::{AtomicU64, Ordering};
        static KEY: AtomicU64 = AtomicU64::new(1);
        Self(KEY.fetch_add(1, Ordering::Relaxed))
    }

    /// Returns the underlying value of the key.
    pub fn value(self) -> u64 {
        self.0
    }
}

/// Heuristic identifier for a font.
#[derive(Default)]
pub struct Fingerprint {
    len: u32,
    samples: [Sample; 2],
    num_samples: u32,
}

impl Fingerprint {
    /// Creates a new fingerprint for the specified font.
    pub fn from_font(font: &FontRef) -> Option<Self> {
        let mut this = Fingerprint::default();
        this.len = font.data.len().checked_sub(font.offset as usize)? as u32;
        let b = Bytes::with_offset(font.data, font.offset as usize)?;
        let count = b.read_u16(4)? as usize;
        let mut num_samples = 0u32;
        for i in 0..count {
            if num_samples == 2 {
                break;
            }
            let recbase = 12 + i * 16;
            let tag = b.read_u32(recbase)?;
            match tag {
                COLR | NAME | CFF_ | CFF2 | GLYF | GSUB | GPOS | MORX | KERX | KERN | CBDT
                | EBDT | SBIX => {}
                _ => continue,
            }
            let bytes = b.read_bytes(recbase, 16)?;
            let sample = &mut this.samples[num_samples as usize];
            (&mut sample.bytes[..]).copy_from_slice(bytes);
            sample.offset = font.offset + recbase as u32;
            num_samples += 1;
        }
        this.num_samples = num_samples;
        Some(this)
    }

    /// Returns true if the specified font matches the fingerprint.
    #[inline]
    pub fn test(&self, font: &FontRef) -> bool {
        if let Some(len) = font.data.len().checked_sub(font.offset as usize) {
            self.test_len(font, len as u32).unwrap_or(false)
        } else { 
            false
        }
    }

    #[inline]
    pub(crate) fn test_len(&self, font: &FontRef, len: u32) -> Option<bool> {
        if len != self.len {
            return Some(false);
        }
        let b = Bytes::new(font.data);
        for sample in &self.samples[..self.num_samples as usize] {
            if b.read_bytes(sample.offset as usize, 16)? != &sample.bytes[..] {
                return Some(false);
            }
        }
        Some(true)
    }    
}

#[derive(Default)]
struct Sample {
    offset: u32,
    bytes: [u8; 16],
}

const NAME: u32 = raw_tag(b"name");
const CFF_: u32 = raw_tag(b"CFF ");
const CFF2: u32 = raw_tag(b"CFF2");
const GLYF: u32 = raw_tag(b"glyf");
const GSUB: u32 = raw_tag(b"GSUB");
const GPOS: u32 = raw_tag(b"GPOS");
const MORX: u32 = raw_tag(b"morx");
const KERX: u32 = raw_tag(b"kerx");
const KERN: u32 = raw_tag(b"kern");
const COLR: u32 = raw_tag(b"COLR");
const CBDT: u32 = raw_tag(b"CBDT");
const EBDT: u32 = raw_tag(b"EBDT");
const SBIX: u32 = raw_tag(b"sbix");

