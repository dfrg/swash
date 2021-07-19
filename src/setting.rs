use super::{tag_from_bytes, tag_from_str_lossy, Tag};

/// Setting combining a tag and a value for features and variations.
#[derive(Copy, Clone, Default, Debug)]
pub struct Setting<T: Copy> {
    /// The tag that identifies the setting.
    pub tag: Tag,
    /// The value for the setting.
    pub value: T,
}

impl<T: Copy> From<(Tag, T)> for Setting<T> {
    fn from(v: (Tag, T)) -> Self {
        Self {
            tag: v.0,
            value: v.1,
        }
    }
}

impl<T: Copy> From<&(Tag, T)> for Setting<T> {
    fn from(v: &(Tag, T)) -> Self {
        Self {
            tag: v.0,
            value: v.1,
        }
    }
}

impl<T: Copy> From<&([u8; 4], T)> for Setting<T> {
    fn from(v: &([u8; 4], T)) -> Self {
        Self {
            tag: tag_from_bytes(&v.0),
            value: v.1,
        }
    }
}

impl<T: Copy> From<&(&[u8; 4], T)> for Setting<T> {
    fn from(v: &(&[u8; 4], T)) -> Self {
        Self {
            tag: tag_from_bytes(v.0),
            value: v.1,
        }
    }
}

impl<T: Copy> From<(&str, T)> for Setting<T> {
    fn from(v: (&str, T)) -> Self {
        Self {
            tag: tag_from_str_lossy(v.0),
            value: v.1,
        }
    }
}

impl<T: Copy> From<&(&str, T)> for Setting<T> {
    fn from(v: &(&str, T)) -> Self {
        Self {
            tag: tag_from_str_lossy(v.0),
            value: v.1,
        }
    }
}

impl<T> PartialEq for Setting<T>
where
    T: Copy + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag && self.value == other.value
    }
}
