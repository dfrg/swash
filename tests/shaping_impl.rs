pub fn shape(font: &str, font_size: usize, variations: &[(&str, f32)], input: &str) -> String {
    let file = std::fs::read(font).unwrap();
    let font = swash::FontRef::from_offset(&file, 0).unwrap();
    let mut context = swash::shape::ShapeContext::new();
    let builder = context
        .builder(font)
        .size(font_size as f32)
        .variations(variations);
    let bidi_info = unic_bidi::BidiInfo::new(input, None);

    let mut advance = 0.0;
    let mut output = Vec::new();
    let mut shaper = builder.build();
    let glyph_count = shaper.metrics().glyph_count as usize;
    shaper.add_str(input);
    shaper.shape_with(|cluster| {
        output.extend(cluster.glyphs.iter().map(|glyph| {
            let old_advance = advance;
            advance += glyph.advance as f64;
            if old_advance == 0.0 {
                format!(
                    "{}",
                    font.glyph_name(glyph.id)
                        .map(|x| x.to_string())
                        .unwrap_or(format!("gid{}", glyph.id))
                )
            } else {
                format!(
                    "{}@{},{}",
                    font.glyph_name(glyph.id)
                        .map(|x| x.to_string())
                        .unwrap_or(format!("gid{}", glyph.id)),
                    (glyph.x as f64 + f64::ceil(old_advance) ) as usize,
                    glyph.y as usize,
                )
            }
        }));
    });
    if bidi_info.has_rtl() {
        output.reverse();
    }

    let collected: String = output
        .iter()
        .enumerate()
        .map(|(idx, i)| {
            if idx == 0 {
                i.to_owned()
            } else {
                format!("|{}", i)
            }
        })
        .collect();
    format!("[{}]", collected)
}
