use crate::internal::raw_tag;

use super::{
    super::{metrics::MetricsProxy, strike::BitmapStrikesProxy, FontRef},
    color::ColorProxy,
    glyf::GlyfProxy,
};

#[derive(Copy, Clone)]
pub enum OutlinesProxy {
    None,
    Cff,
    Glyf(GlyfProxy),
}

#[derive(Copy, Clone)]
pub struct ScalerProxy {
    pub metrics: MetricsProxy,
    pub color: ColorProxy,
    pub outlines: OutlinesProxy,
    pub bitmaps: BitmapStrikesProxy,
    pub coord_count: u16,
}

impl ScalerProxy {
    pub fn from_font(font: &FontRef) -> Self {
        let outlines = if let Some(glyf) = GlyfProxy::from_font(font) {
            OutlinesProxy::Glyf(glyf)
        } else if font.table(raw_tag(b"CFF ")).is_some() || font.table(raw_tag(b"CFF2")).is_some() {
            OutlinesProxy::Cff
        } else {
            OutlinesProxy::None
        };
        Self {
            metrics: MetricsProxy::from_font(font),
            color: ColorProxy::from_font(font),
            outlines,
            bitmaps: BitmapStrikesProxy::from_font(font),
            coord_count: font.variations().len() as u16,
        }
    }
}
