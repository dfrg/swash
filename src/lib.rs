/*!
Font introspection, complex text shaping and glyph rendering.

### Note

Documentation is present but of the "skeleton" variety for the time being. This
will be filled in over the coming weeks. 

### Goals

This crate aims to lay the foundation for a cross-platform, high performance set
of components for beautiful typography. In particular, this library focuses on
fonts and operations that are directly applicable to them. For the most part, the
desire is to be unopinionated with respect to resource management, higher level
layout and lower level rendering.

### Non goals

Due to the intention of being generally useful and easy to integrate, the following
areas of related interest are specifically avoided:

- Text layout. This is highly application specific and the requirements for both
    features and performance differ greatly among web browsers, word processors,
    text editors, game engines, etc. There is a sibling crate in development that
    does provide general purpose text layout based on this library and will be
    released shortly.

- Composition. Like layout, this is also application specific in addition to being
    hardware dependent. Glyph catching, geometry batching and rendering all belong
    here and should integrate well with the application and the hardware environment.
    There is also a sibling crate that attempts to provide these services that will
    be released in the future.

### General features

- Simple borrowed font representation that imposes no requirements on resource
    management leading to...
- Thread friendly architecture. Acceleration structures are completely separate from
    font data and can be retained per thread, thrown away and rematerialized at any
    time
- Zero transient heap allocations. All scratch buffers and caches are maintained by
    contexts. Resources belonging to evicted cache entries are immediately reused

### Introspection

- Enumerating font collections (ttc/otc)
- Localized strings including names and other metadata
- Font variation axes and named instances
- Comprehensive font and per-glyph metrics (with synthesized vertical metrics if not provided)
- Primary attributes (stretch, weight, style) with support for synthesis suggestions
    based on available variations and scaler transforms (faux bold and oblique)
- Color palettes
- Embedded color and alpha bitmap strikes
- Character to nominal glyph identifier mapping with support for enumerating all pairs
- Writing systems: provides a list of all supported script/language pairs
    and their associated typographic features
- All introspection is zero allocation and no copy

### Complex text shaping

- Full support for OpenType advanced typography (GSUB/GPOS)
- Partial support for Apple advanced typography: glyph metamorphosis (morx) is fully
    supported while the current extended kerning support (kerx) covers most common
    cases (kerning and mark positioning)
- Full support for variable fonts including positioning and feature substitutions
- Implementation of the Universal Shaping Engine for complex scripts such as Devangari, Malayalam, etc.
- Arabic joining including Urdu style climbing runs
- Basic shaping support: ligatures, marks, kerning, etc.
- Enable/disable individual features with argument support for activating alternates
    such as _swashes_...
- Pre-shaping cluster parsing with an iterative mapping technique (including normalization) allowing for
    sophisticated font fallback mechanisms without the expense of heuristically shaping runs
- Shaper output is structured by cluster including original source ranges and provides simple
    identification of ligatures and complex multi-glyph clusters
- Pass-through per character user data (a single u32) for accurate association of style
    properties with glyphs
- Pass-through per cluster information for retaining text analysis results such as word
    and line boundaries, whitespace identification and emoji presentation modes
- Shaping of vertical runs is not yet provided but a prototype exists and will be released soon

### Scaling

- Scalable outlines with full variation support (TrueType and Postscript)
- Asymmetric vertical hinting (TrueType and Postscript)
- Horizontal subpixel rendering and fractional positioning
- Full emoji support for Apple (sbix), Google (CBLC/CBDT) and Microsoft (COLR/CPAL)
    formats
- Path effects (stroking and dashing)
- Transforms including synthetic emboldening and affine transformations
- Customizable glyph source prioritization (best fit color bitmap -> exact size alpha bitmap -> outline)

### Text analysis

- Unicode character properties related to layout and shaping
- Character composition and decomposition (canonical and compatible)
- Complex, script aware cluster segmentation
- Single pass, iterator based analysis determines word and line boundaries
    and detects whether bidi resolution is necessary
*/

pub use zeno;

#[macro_use]
mod macros;

mod attributes;
mod cache;
mod charmap;
mod feature;
mod font;
mod internal;
mod metrics;
mod palette;
mod setting;
mod strike;
mod string;
mod tag;
mod variation;

pub mod ident;
pub mod scale;
pub mod shape;
pub mod text;

pub use attributes::*;
pub use charmap::Charmap;
pub use feature::{Action, Feature, WritingSystem};
pub use font::{FontDataRef, FontRef};
pub use metrics::{GlyphMetrics, Metrics};
pub use palette::{ColorPalette, Usability};
pub use setting::TagAndValue;
pub use strike::BitmapStrike;
pub use string::{LocalizedString, StringId};
pub use tag::{Tag, tag_from_bytes, tag_from_str_lossy};
pub use variation::{Instance, Variation};

/// Collection of various iterators over metadata contained in a font.
pub mod iter {
    pub use super::feature::{Features, WritingSystems};
    pub use super::font::Fonts;
    pub use super::palette::ColorPalettes;
    pub use super::strike::BitmapStrikes;
    pub use super::string::{Chars, LocalizedStrings};
    pub use super::variation::{Instances, Variations};
}   

/// Proxies used to efficiently rematerialize metadata.
pub mod proxy {
    pub use super::strike::BitmapStrikesProxy;
    pub use super::charmap::CharmapProxy;
    pub use super::metrics::MetricsProxy;
    pub use super::variation::VariationsProxy;
}   

use iter::*;
use proxy::BitmapStrikesProxy;

/// Glyph identifier.
pub type GlyphId = u16;

/// Normalized variation coordinate in 2.14 fixed point format.
pub type NormalizedCoord = i16;

impl<'a> FontRef<'a> {
    /// Returns the primary attributes for the font.
    pub fn attributes(&self) -> Attributes {
        Attributes::from_font(self)
    }

    /// Returns an iterator over the localized strings for the font.
    pub fn localized_strings(&self) -> LocalizedStrings<'a> {
        LocalizedStrings::from_font(self)
    }

    /// Returns an iterator over the variations for the font.
    pub fn variations(&self) -> Variations<'a> {
        Variations::from_font(self)
    }

    /// Returns an iterator over the named instances for the font.
    pub fn instances(&self) -> Instances<'a> {
        Instances::from_font(self)
    }

    /// Returns an iterator over writing systems supported by the font.
    pub fn writing_systems(&self) -> WritingSystems<'a> {
        WritingSystems::from_font(self)
    }

    /// Returns an iterator over the features supported by a font.
    pub fn features(&self) -> Features<'a> {
        Features::from_font(self)
    }

    /// Returns metrics for the font and the specified normalized variation
    /// coordinates.
    pub fn metrics(&self, coords: &'a [NormalizedCoord]) -> Metrics {
        Metrics::from_font(self, coords)
    }

    /// Returns glyph metrics for the font and the specified normalized
    /// variation coordinates.
    pub fn glyph_metrics(&self, coords: &'a [NormalizedCoord]) -> GlyphMetrics<'a> {
        GlyphMetrics::from_font(self, coords)
    }

    /// Returns the character map for the font.
    pub fn charmap(&self) -> Charmap<'a> {
        Charmap::from_font(self)
    }

    /// Returns an iterator over the color palettes for the font.
    pub fn color_palettes(&self) -> ColorPalettes<'a> {
        ColorPalettes::from_font(self)
    }

    /// Returns an iterator over the alpha bitmap strikes for the font.
    pub fn alpha_strikes(&self) -> BitmapStrikes<'a> {
        BitmapStrikesProxy::from_font(self).materialize_alpha(self)
    }

    /// Returns an iterator over the color bitmap strikes for the font.
    pub fn color_strikes(&self) -> BitmapStrikes<'a> {
        BitmapStrikesProxy::from_font(self).materialize_color(self)
    }

    /// Returns the table data for the specified tag.
    pub fn table(&self, tag: Tag) -> Option<&'a [u8]> {
        use internal::RawFont;
        let range = self.table_range(tag)?;
        self.data.get(range.0 as usize..range.1 as usize)        
    }
}

