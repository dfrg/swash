/*!
Scaling, hinting and rasterization of visual glyph representations.
*/

const TRACE: bool = false;

pub mod image;
pub mod outline;

mod bitmap;
mod cff;
mod color;
mod glyf;
mod proxy;

use image::*;
use outline::*;

use super::internal;
use super::{cache::FontCache, FontRef, GlyphId, NormalizedCoord, setting::TagAndValue};
use proxy::*;
use zeno::{Format, Origin, Style, Mask, Placement, Scratch, Point, Transform, Vector};
use core::borrow::Borrow;

pub(crate) use bitmap::decode_png;

/// Index of a color palette.
pub type PaletteIndex = u16;

/// Index of a bitmap strike.
pub type StrikeIndex = u32;

/// Bitmap strike selection mode.
#[derive(Copy, Clone, Debug)]
pub enum StrikeWith {
    /// Load a bitmap only if the exact size is available.
    ExactSize,
    /// Load a bitmap of the best available size.
    BestFit,
    /// Load a bitmap from the specified strike.
    Index(StrikeIndex),
}

/// Glyph sources for the renderer.
#[derive(Copy, Clone, Debug)]
pub enum Source {
    /// Scalable outlines.
    Outline,
    /// Layered color scalable outlines.
    ColorOutline(PaletteIndex),
    /// Embedded alpha bitmaps.
    Bitmap(StrikeWith),
    /// Embedded color bitmaps.
    ColorBitmap(StrikeWith),
}

impl Default for Source {
    fn default() -> Self {
        Self::Outline
    }
}

/// Context that manages caches and scratch buffers for scaling.
pub struct ScaleContext {
    fonts: FontCache<ScalerProxy>,
    state: State,
    coords: Vec<i16>,
}

struct State {
    glyf_scaler: glyf::Scaler,
    cff_scaler: cff::Scaler,
    scratch0: Vec<u8>,
    scratch1: Vec<u8>,
    outline: Outline,
    rcx: Scratch,
}

impl ScaleContext {
    /// Creates a new scaling context.
    pub fn new() -> Self {
        Self::with_max_entries(8)
    }

    /// Creates a new scaling context with the specified maximum number of
    /// cache entries.
    pub fn with_max_entries(max_entries: usize) -> Self {
        let max_entries = max_entries.min(64).max(1);
        Self {
            fonts: FontCache::new(max_entries),
            state: State {
                glyf_scaler: glyf::Scaler::new(max_entries),
                cff_scaler: cff::Scaler::new(max_entries),
                scratch0: Vec::new(),
                scratch1: Vec::new(),
                outline: Outline::new(),
                rcx: Scratch::new(),
            },
            coords: Vec::new(),
        }
    }

    /// Creates a new builder for constructing a scaler with this context
    /// and the specified font.
    pub fn builder<'a>(&'a mut self, font: impl Into<FontRef<'a>>) -> ScalerBuilder<'a> {
        ScalerBuilder::new(self, font)
    }
}

/// Builder for configuring a scaler.
pub struct ScalerBuilder<'a> {
    state: &'a mut State,
    font: FontRef<'a>,
    proxy: &'a ScalerProxy,
    id: u64,
    coords: &'a mut Vec<i16>,
    size: f32,
    hint: bool,
}

impl<'a> ScalerBuilder<'a> {
    fn new(context: &'a mut ScaleContext, font: impl Into<FontRef<'a>>) -> Self {
        let font = font.into();
        let (id, proxy) = context
            .fonts
            .get(&font, |font| ScalerProxy::from_font(font));
        Self {
            state: &mut context.state,
            font,
            proxy,
            id,
            coords: &mut context.coords,
            size: 0.,
            hint: false,
        }
    }

    /// Specifies the font size in pixels per em.
    pub fn size(mut self, ppem: f32) -> Self {
        self.size = ppem.max(0.);
        self
    }

    /// Specifies whether to apply hinting to outlines.
    pub fn hint(mut self, yes: bool) -> Self {
        self.hint = yes;
        self
    }

    /// Adds variation settings to the scaler.
    pub fn variations<I>(self, settings: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<TagAndValue<f32>>,
    {
        if self.proxy.coord_count != 0 {
            let vars = self.font.variations();
            self.coords.resize(vars.len(), 0);
            for setting in settings {
                let setting = setting.into();
                for var in vars.clone() {
                    if var.tag() == setting.tag {
                        let value = var.normalize(setting.value);
                        self.coords.get_mut(var.index()).map(|c| *c = value);
                    }
                }
            }
        }
        self
    }    

    /// Specifies the variation settings in terms of normalized coordinates.
    pub fn normalized_coords<I>(self, coords: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<NormalizedCoord>,
    {
        self.coords.clear();
        self.coords.extend(coords.into_iter().map(|c| *c.borrow()));
        self
    }

    /// Builds a scaler for the current configuration.
    pub fn build(self) -> Scaler<'a> {
        let upem = self.proxy.metrics.units_per_em();
        let scale = if self.size != 0. && upem != 0 {
            self.size / upem as f32
        } else {
            1.
        };
        Scaler {
            state: self.state,
            font: self.font,
            proxy: self.proxy,
            id: self.id,
            coords: &self.coords[..],
            size: self.size,
            scale,
            hint: self.hint,
            glyf_state: None,
        }
    }
}

/// Scales, hints and rasterizes visual representations of glyphs.
pub struct Scaler<'a> {
    state: &'a mut State,
    font: FontRef<'a>,
    proxy: &'a ScalerProxy,
    id: u64,
    coords: &'a [i16],
    size: f32,
    scale: f32,
    hint: bool,
    glyf_state: Option<glyf::ScalerState<'a>>,
}

impl<'a> Scaler<'a> {
    /// Returns true if scalable glyph outlines are available.
    pub fn has_outlines(&self) -> bool {
        match self.proxy.outlines {
            OutlinesProxy::None => false,
            _ => true,
        }
    }

    /// Scales an outline for the specified glyph into the provided outline.
    pub fn scale_outline_into(&mut self, glyph_id: GlyphId, outline: &mut Outline) -> bool {
        outline.clear();
        self.scale_outline_impl(glyph_id, None, Some(outline))
    }

    /// Scales an outline for the specified glyph.
    pub fn scale_outline(&mut self, glyph_id: GlyphId) -> Option<Outline> {
        let mut outline = Outline::new();
        if self.scale_outline_into(glyph_id, &mut outline) {
            Some(outline)
        } else {
            None
        }
    }

    /// Returns true if scalable color glyph outlines are available.
    pub fn has_color_outlines(&self) -> bool {
        self.proxy.color.colr != 0 && self.proxy.color.cpal != 0
    }

    /// Scales a color outline for the specified glyph into the provided outline.
    pub fn scale_color_outline_into(&mut self, glyph_id: GlyphId, outline: &mut Outline) -> bool {
        outline.clear();
        if !self.has_color_outlines() {
            return false;
        }
        let layers = match self.proxy.color.layers(self.font.data, glyph_id) {
            Some(layers) => layers,
            _ => return false,
        };
        for i in 0..layers.len() {
            let layer = match layers.get(i) {
                Some(layer) => layer,
                _ => return false,
            };
            if !self.scale_outline_impl(layer.glyph_id, layer.color_index, Some(outline)) {
                return false;
            }
        }
        outline.set_color(true);
        true
    }

    /// Scales a color outline for the specified glyph.
    pub fn scale_color_outline(&mut self, glyph_id: GlyphId) -> Option<Outline> {
        let mut outline = Outline::new();
        if self.scale_color_outline_into(glyph_id, &mut outline) {
            Some(outline)
        } else {
            None
        }
    }

    fn scale_outline_impl(
        &mut self,
        glyph_id: GlyphId,
        color_index: Option<u16>,
        outline: Option<&mut Outline>,
    ) -> bool {
        let outline = match outline {
            Some(x) => x,
            _ => &mut self.state.outline,
        };
        match &self.proxy.outlines {
            OutlinesProxy::None => false,
            OutlinesProxy::Cff(proxy) => {
                outline.begin_layer(color_index);
                let mut builder = CffBuilder { outline };
                if self
                    .state
                    .cff_scaler
                    .scale(
                        &self.font,
                        self.id,
                        &self.coords,
                        proxy,
                        self.scale,
                        self.hint,
                        glyph_id,
                        &mut builder,
                    )
                    .is_some()
                {
                    outline.maybe_close();
                    outline.finish();
                    true
                } else {
                    false
                }
            }
            OutlinesProxy::Glyf(proxy) => {
                if self.glyf_state.is_none() {
                    self.glyf_state = Some(glyf::ScalerState::new(
                        self.font.data,
                        self.id,
                        &self.coords,
                        proxy,
                        &self.proxy.metrics,
                        self.size,
                        self.hint,
                    ));
                }
                let state = self.glyf_state.as_mut().unwrap();
                outline.begin_layer(color_index);
                if self.state.glyf_scaler.scale(state, glyph_id).is_some() {
                    let scaler = &self.state.glyf_scaler;
                    fill_outline(
                        outline,
                        &scaler.scaled,
                        &scaler.contours,
                        &scaler.tags,
                        self.size != 0.,
                    );
                    outline.maybe_close();
                    outline.finish();
                    true
                } else {
                    false
                }
            }
        }
    }

    fn scale_color_outline_impl(&mut self, glyph_id: GlyphId) -> bool {
        if !self.has_color_outlines() {
            return false;
        }
        let layers = match self.proxy.color.layers(self.font.data, glyph_id) {
            Some(layers) => layers,
            _ => return false,
        };
        self.state.outline.clear();
        for i in 0..layers.len() {
            let layer = match layers.get(i) {
                Some(layer) => layer,
                _ => return false,
            };
            if !self.scale_outline_impl(layer.glyph_id, layer.color_index, None) {
                return false;
            }
        }
        true
    }

    /// Returns true if alpha bitmaps are available.
    pub fn has_bitmaps(&self) -> bool {
        self.proxy.bitmaps.has_alpha()
    }

    /// Scales a bitmap for the specified glyph and mode into the provided image.
    pub fn scale_bitmap_into(
        &mut self,
        glyph_id: u16,
        strike: StrikeWith,
        image: &mut Image,
    ) -> bool {
        self.scale_bitmap_impl(glyph_id, false, strike, image) == Some(true)
    }

    /// Scales a bitmap for the specified glyph and mode.
    pub fn scale_bitmap(&mut self, glyph_id: u16, strike: StrikeWith) -> Option<Image> {
        let mut image = Image::new();
        if self.scale_bitmap_into(glyph_id, strike, &mut image) {
            Some(image)
        } else {
            None
        }
    }

    /// Returns true if color bitmaps are available.
    pub fn has_color_bitmaps(&self) -> bool {
        self.proxy.bitmaps.has_color()
    }

    /// Scales a color bitmap for the specified glyph and mode into the provided image.
    pub fn scale_color_bitmap_into(
        &mut self,
        glyph_id: u16,
        strike: StrikeWith,
        image: &mut Image,
    ) -> bool {
        self.scale_bitmap_impl(glyph_id, true, strike, image) == Some(true)
    }

    /// Scales a color bitmap for the specified glyph and mode.
    pub fn scale_color_bitmap(&mut self, glyph_id: u16, strike: StrikeWith) -> Option<Image> {
        let mut image = Image::new();
        if self.scale_color_bitmap_into(glyph_id, strike, &mut image) {
            Some(image)
        } else {
            None
        }
    }

    fn scale_bitmap_impl(
        &mut self,
        glyph_id: GlyphId,
        color: bool,
        strike: StrikeWith,
        image: &mut Image,
    ) -> Option<bool> {
        image.clear();
        let size = self.size;
        let mut strikes = if color {
            self.proxy.bitmaps.materialize_color(&self.font)
        } else {
            self.proxy.bitmaps.materialize_alpha(&self.font)
        };
        let bitmap = match strike {
            StrikeWith::ExactSize => {
                if self.size == 0. {
                    None
                } else {
                    strikes
                        .find_by_exact_ppem(size as u16, glyph_id)?
                        .get(glyph_id)
                }
            }
            StrikeWith::BestFit => {
                if self.size == 0. {
                    None
                } else {
                    strikes
                        .find_by_nearest_ppem(size as u16, glyph_id)?
                        .get(glyph_id)
                }
            }
            StrikeWith::Index(i) => strikes
                .nth(i as usize)
                .and_then(|strike| strike.get(glyph_id)),
        }?;
        if bitmap.ppem == 0 {
            return None;
        }
        let (_, _, bufsize) = bitmap.scaled_size(size);
        image.data.resize(bufsize, 0);
        self.state.scratch0.clear();
        self.state.scratch1.clear();
        let mut w = bitmap.width;
        let mut h = bitmap.height;
        let scale = size / bitmap.ppem as f32;
        image.placement = if size != 0. && scale != 1. {
            self.state
                .scratch0
                .resize(bitmap.format.buffer_size(w, h), 0);
            w = (w as f32 * scale) as u32;
            h = (h as f32 * scale) as u32;
            image.data.resize(bitmap.format.buffer_size(w, h), 0);
            if !bitmap.decode(Some(&mut self.state.scratch1), &mut self.state.scratch0) {
                return None;
            }
            if !bitmap::resize(
                &self.state.scratch0,
                bitmap.width,
                bitmap.height,
                bitmap.format.channels(),
                &mut image.data,
                w,
                h,
                bitmap::Filter::Mitchell,
                Some(&mut self.state.scratch1),
            ) {
                return None;
            }
            let left = (bitmap.left as f32 * scale) as i32;
            let top = (bitmap.top as f32 * scale) as i32;
            Placement {
                left,
                top,
                width: w,
                height: h,
            }
        } else {
            image.data.resize(bitmap.format.buffer_size(w, h), 0);
            if !bitmap.decode(Some(&mut self.state.scratch1), &mut image.data) {
                return None;
            }
            Placement {
                left: bitmap.left,
                top: bitmap.top,
                width: w,
                height: h,
            }
        };
        image.source = match color {
            true => Source::ColorBitmap(strike),
            false => Source::Bitmap(strike),
        };
        image.content = match bitmap.format.channels() {
            1 => Content::Mask,
            _ => Content::Color,
        };
        // let mut advance = bitmap.advance() as f32;
        // if options.size != 0. && options.size as u16 != bitmap.ppem() {
        //     advance *= options.size / bitmap.ppem() as f32;
        // }
        Some(true)
    }
}

/// Builder type for rendering a glyph into an image.
pub struct Render<'a> {
    sources: &'a [Source],
    format: Format,
    offset: Point,
    transform: Option<Transform>,
    embolden: f32,
    foreground: [u8; 4],
    style: Style<'a>,
}

impl<'a> Render<'a> {
    /// Creates a new builder for configuring rendering using the specified
    /// prioritized list of sources.
    pub fn new(sources: &'a [Source]) -> Self {
        Self {
            sources,
            format: Format::Alpha,
            offset: Point::new(0., 0.),
            transform: None,
            embolden: 0.,
            foreground: [128, 128, 128, 255],
            style: Style::default(),
        }
    }

    /// Specifies the target format for rasterizing an outline. Default is
    /// [`Format::Alpha`](zeno::Format::Alpha).
    pub fn format(&mut self, format: Format) -> &mut Self {
        self.format = format;
        self
    }

    /// Specifies the path style to use when rasterizing an outline. Default is
    /// [`Fill::NonZero`](zeno::Fill::NonZero).
    pub fn style(&mut self, style: impl Into<Style<'a>>) -> &mut Self {
        self.style = style.into();
        self
    }

    /// Specifies an additional offset to apply when rasterizing an outline.
    /// Default is `(0, 0)`.
    pub fn offset(&mut self, offset: Vector) -> &mut Self {
        self.offset = offset;
        self
    }

    /// Specifies a transformation matrix to apply when rasterizing an
    /// outline. Default is `None`.
    pub fn transform(&mut self, transform: Option<Transform>) -> &mut Self {
        self.transform = transform;
        self
    }

    /// Specifies the strength of a faux bold transform to apply when
    /// rasterizing an outline. Default is `0`.
    pub fn embolden(&mut self, strength: f32) -> &mut Self {
        self.embolden = strength;
        self
    }

    /// Specifies an RGBA color to use when rasterizing layers of a color
    /// outline that do not directly reference a palette color. Default is
    /// `[128, 128, 128, 255]`.
    pub fn default_color(&mut self, color: [u8; 4]) -> &mut Self {
        self.foreground = color;
        self
    }

    /// Renders the specified glyph using the current configuration into the
    /// provided image.
    pub fn render_into(
        &self,
        scaler: &mut Scaler,
        glyph_id: GlyphId,
        image: &mut Image,
    ) -> bool {
        for source in self.sources {
            match source {
                Source::Outline => {
                    if !scaler.has_outlines() {
                        continue;
                    }
                    scaler.state.outline.clear();
                    if scaler.scale_outline_impl(glyph_id, None, None) {
                        let state = &mut scaler.state;
                        let rcx = &mut state.rcx;
                        let outline = &mut state.outline;
                        if self.embolden != 0. {
                            outline.embolden(self.embolden, self.embolden);
                        }
                        if let Some(transform) = &self.transform {
                            outline.transform(transform);
                        }
                        let placement = Mask::with_scratch(outline.path(), rcx)
                            .format(self.format)
                            .origin(Origin::BottomLeft)
                            .style(self.style)
                            .render_offset(self.offset)
                            .inspect(|fmt, w, h| {
                                image.data.resize(fmt.buffer_size(w, h), 0);
                            })
                            .render_into(&mut image.data[..], None);
                        image.placement = placement;
                        image.content = if self.format == Format::Alpha {
                            Content::Mask
                        } else {
                            Content::SubpixelMask
                        };
                        image.source = Source::Outline;
                        return true;
                    }
                }
                Source::ColorOutline(palette_index) => {
                    if !scaler.has_color_outlines() {
                        continue;
                    }
                    scaler.state.outline.clear();
                    if scaler.scale_color_outline_impl(glyph_id) {
                        let font = &scaler.font;
                        let proxy = &scaler.proxy;
                        let state = &mut scaler.state;
                        let scratch = &mut state.scratch0;
                        let rcx = &mut state.rcx;
                        let outline = &mut state.outline;
                        // Cool effect, but probably not generally desirable.
                        // Maybe expose a separate option?
                        // if self.embolden != 0. {
                        //     outline.embolden(self.embolden, self.embolden);
                        // }
                        if let Some(transform) = &self.transform {
                            outline.transform(transform);
                        }
                        let palette = proxy.color.palette(font, *palette_index);
                        let mut base_x = 0i32;
                        let mut base_y = 0i32;
                        let mut base_w = 0u32;
                        let mut base_h = 0u32;
                        let mut ok = true;
                        for i in 0..outline.len() {
                            let layer = match outline.get(i) {
                                Some(layer) => layer,
                                _ => {
                                    ok = false;
                                    break;
                                }
                            };
                            scratch.clear();
                            let placement = Mask::with_scratch(layer.path(), rcx)
                                .origin(Origin::BottomLeft)
                                .style(self.style)
                                .render_offset(self.offset)
                                .inspect(|fmt, w, h| {
                                    scratch.resize(fmt.buffer_size(w, h), 0);
                                })
                                .render_into(&mut scratch[..], None);
                            if i == 0 {
                                base_x = placement.left;
                                base_y = placement.top;
                                base_w = placement.width;
                                base_h = placement.height;
                                image.data.resize((base_w * base_h * 4) as usize, 0);
                                image.placement = placement;
                            }
                            let color = layer
                                .color_index()
                                .map(|i| palette.map(|p| p.get(i)))
                                .flatten()
                                .unwrap_or(self.foreground);
                            bitmap::blit(
                                &scratch[..],
                                placement.width,
                                placement.height,
                                placement.left.wrapping_sub(base_x),
                                base_y.wrapping_sub(placement.top),
                                color,
                                &mut image.data,
                                base_w,
                                base_h,
                            );
                        }
                        if ok {
                            image.source = Source::ColorOutline(*palette_index);
                            image.content = Content::Color;
                            return true;
                        }
                    }
                }
                Source::Bitmap(mode) => {
                    if !scaler.has_bitmaps() {
                        continue;
                    }
                    if scaler.scale_bitmap_into(glyph_id, *mode, image) {
                        return true;
                    }
                }
                Source::ColorBitmap(mode) => {
                    if !scaler.has_color_bitmaps() {
                        continue;
                    }
                    if scaler.scale_color_bitmap_into(glyph_id, *mode, image) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Renders the specified glyph using the current configuration.
    pub fn render(&self, scaler: &mut Scaler, glyph_id: GlyphId) -> Option<Image> {
        let mut image = Image::new();
        if self.render_into(scaler, glyph_id, &mut image) {
            Some(image)
        } else {
            None
        }
    }
}

fn fill_outline(
    outline: &mut Outline,
    points: &[glyf::Point],
    contours: &[u16],
    tags: &[u8],
    scaled: bool,
) -> Option<()> {
    use glyf::Point as PointI;
    #[inline(always)]
    fn conv(p: glyf::Point, s: f32) -> Point {
        Point::new(p.x as f32 * s, p.y as f32 * s)
    }
    const TAG_MASK: u8 = 0x3;
    const CONIC: u8 = 0x0;
    const ON: u8 = 0x1;
    const CUBIC: u8 = 0x2;
    let s = if scaled { 1. / 64. } else { 1. };
    for c in 0..contours.len() {
        let mut cur = if c > 0 {
            contours[c - 1] as usize + 1
        } else {
            0
        };
        let mut last = contours[c] as usize;
        if last < cur || last >= points.len() {
            return None;
        }
        let mut v_start = points[cur];
        let v_last = points[last];
        let mut tag = tags[cur] & TAG_MASK;
        if tag == CUBIC {
            return None;
        }
        let mut step_point = true;
        if tag == CONIC {
            if tags[last] & TAG_MASK == ON {
                v_start = v_last;
                last -= 1;
            } else {
                v_start.x = (v_start.x + v_last.x) / 2;
                v_start.y = (v_start.y + v_last.y) / 2;
            }
            step_point = false;
        }
        outline.move_to(conv(v_start, s));
        // let mut do_close = true;
        while cur < last {
            if step_point {
                cur += 1;
            }
            step_point = true;
            tag = tags[cur] & TAG_MASK;
            match tag {
                ON => {
                    outline.line_to(conv(points[cur], s));
                    continue;
                }
                CONIC => {
                    let mut do_close_conic = true;
                    let mut v_control = points[cur];
                    while cur < last {
                        cur += 1;
                        let point = points[cur];
                        tag = tags[cur] & TAG_MASK;
                        if tag == ON {
                            outline.quad_to(conv(v_control, s), conv(point, s));
                            do_close_conic = false;
                            break;
                        }
                        if tag != CONIC {
                            return None;
                        }
                        let v_middle =
                            PointI::new((v_control.x + point.x) / 2, (v_control.y + point.y) / 2);
                        outline.quad_to(conv(v_control, s), conv(v_middle, s));
                        v_control = point;
                    }
                    if do_close_conic {
                        outline.quad_to(conv(v_control, s), conv(v_start, s));
                        //                        do_close = false;
                        break;
                    }
                    continue;
                }
                _ => {
                    if cur + 1 > last || (tags[cur + 1] & TAG_MASK != CUBIC) {
                        return None;
                    }
                    let v1 = conv(points[cur], s);
                    let v2 = conv(points[cur + 1], s);
                    cur += 2;
                    if cur <= last {
                        outline.curve_to(v1, v2, conv(points[cur], s));
                        continue;
                    }
                    outline.curve_to(v1, v2, conv(v_start, s));
                    // do_close = false;
                    break;
                }
            }
        }
        if true {
            outline.maybe_close();
        }
    }
    Some(())
}

struct CffBuilder<'a> {
    outline: &'a mut Outline,
}

impl cff::GlyphSink for CffBuilder<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.outline.move_to(Point::new(x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.outline.line_to(Point::new(x, y));
    }

    fn curve_to(&mut self, cx1: f32, cy1: f32, cx2: f32, cy2: f32, x: f32, y: f32) {
        self.outline
            .curve_to(Point::new(cx1, cy1), Point::new(cx2, cy2), Point::new(x, y));
    }

    fn close(&mut self) {
        self.outline.close();
    }
}
