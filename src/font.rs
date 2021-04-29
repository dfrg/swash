use super::internal::{raw_data, RawFont};
use super::ident::Key;

/// Reference to the content of a font file.
#[derive(Copy, Clone)]
pub struct FontDataRef<'a> {
    data: &'a [u8],
    len: usize,
}

impl<'a> FontDataRef<'a> {
    /// Creates font data from the specified bytes. Returns `None` if the bytes
    /// cannot trivially be determined to represent a font.
    pub fn new(data: &'a [u8]) -> Option<Self> {
        if !raw_data::is_font(data, 0) && !raw_data::is_collection(data) {
            None
        } else {
            Some(Self {
                data,
                len: raw_data::count(data) as usize,
            })
        }
    }

    /// Returns true if the data represents a font collection.
    pub fn is_collection(&self) -> bool {
        raw_data::is_collection(self.data)
    }

    /// Returns the underlying data.
    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    /// Returns the number of available fonts.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the font at the specified index.
    pub fn get(&self, index: usize) -> Option<FontRef<'a>> {
        FontRef::new(self.data, raw_data::offset(self.data, index as u32)?)
    }

    /// Returns an iterator over the available fonts.
    pub fn fonts(&self) -> Fonts<'a> {
        Fonts {
            data: *self,
            pos: 0,
        }
    }
}

/// Reference to a font.
#[derive(Copy, Clone)]
pub struct FontRef<'a> {
    /// Content of a font file containing the font.
    pub data: &'a [u8],
    /// Offset to the table directory of the font.
    pub offset: u32,
    /// Key for identifying a font in various caches.
    pub key: Key,
}

impl<'a> FontRef<'a> {
    /// Creates a new font from the specified font data and offset to the
    /// table directory. Returns `None` if the offset is out of bounds or the
    /// data at the offset does not represent a table directory.
    pub fn new(data: &'a [u8], offset: u32) -> Option<Self> {
        if !raw_data::is_font(data, offset) {
            None
        } else {
            Some(Self { data, offset, key: Key::new(), })
        }
    }
}

impl<'a> From<FontRef<'a>> for (&'a [u8], u32) {
    fn from(f: FontRef<'a>) -> Self {
        (f.data, f.offset)
    }
}

impl<'a> From<&FontRef<'a>> for (&'a [u8], u32) {
    fn from(f: &FontRef<'a>) -> Self {
        (f.data, f.offset)
    }
}

impl<'a> RawFont<'a> for FontRef<'a> {
    fn data(&self) -> &'a [u8] {
        self.data
    }

    fn offset(&self) -> u32 {
        self.offset
    }
}

/// Iterator over a collection of fonts.
pub struct Fonts<'a> {
    data: FontDataRef<'a>,
    pos: usize,
}

impl<'a> Iterator for Fonts<'a> {
    type Item = FontRef<'a>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.data.len - self.pos) as usize;
        (remaining, Some(remaining))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let pos = self.pos.checked_add(n)?;
        self.pos = pos.checked_add(1)?;
        self.data.get(pos)
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.data.len {
            None
        } else {
            let pos = self.pos;
            self.pos += 1;
            self.data.get(pos)
        }
    }
}

impl<'a> ExactSizeIterator for Fonts<'a> {
    fn len(&self) -> usize {
        (self.data.len - self.pos) as usize
    }
}

impl<'a> IntoIterator for FontDataRef<'a> {
    type IntoIter = Fonts<'a>;
    type Item = FontRef<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.fonts()
    }
}
