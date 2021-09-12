use std::fs::read;

use swash::{
    text::{analyze, Codepoint, Script},
    FontRef,
};

pub fn shape_text(
    font: &str,
    font_size: usize,
    features: &[(&str, u16)],
    variations: &[(&str, f32)],
    input: &str,
    show_advance: bool,
    show_name: bool,
) -> Vec<String> {
    let file = read(font).unwrap();
    let font = FontRef::from_offset(&file, 0).unwrap();
    let script = input
        .chars()
        .map(|ch| ch.script())
        .find(|&script| {
            script != Script::Unknown && script != Script::Inherited && script != Script::Common
        })
        .unwrap_or(Script::Latin);

    let mut context = swash::shape::ShapeContext::new();
    let builder = context
        .builder(font)
        .size(font_size as f32)
        .retain_ignorables(true)
        .variations(variations)
        .features(features)
        .script(script);
    let needs_resolution = analyze(input.chars()).any(|x| x.0.bidi_class().needs_resolution());

    let mut advance = 0.0;
    let mut output = Vec::new();
    let mut shaper = builder.build();
    shaper.add_str(input);
    shaper.shape_with(|cluster| {
        cluster.glyphs.iter().for_each(|glyph| {
            let name = if show_name {
                font.glyph_name(glyph.id)
                    .map(|x| x.to_string())
                    .unwrap_or(glyph.id.to_string())
            } else {
                glyph.id.to_string()
            };
            // HarfBuzz format doesn't include glyphs with no advance
            if advance == 0.0 || !show_advance {
                output.push(format!("{}", name));
            } else {
                output.push(format!(
                    "{}@{},{}",
                    name,
                    (glyph.x + advance).round() as i64,
                    glyph.y.round() as i64,
                ));
            }
            advance += glyph.advance;
        });
    });

    // Check if runs need to be reversed.
    if needs_resolution {
        output.reverse();
    }

    output
}

#[macro_export]
macro_rules! shaping_test {
    ($name:ident, $font:expr, $font_size:expr, $features:expr, $variations:expr, $input:expr, $output:expr, $show_advance:expr, $show_name:expr) => {
        #[test]
        fn $name() {
            assert_eq!(
                shaping::shape_text(
                    $font,
                    $font_size,
                    $features,
                    $variations,
                    $input,
                    $show_advance,
                    $show_name,
                ),
                $output
            );
        }
    };
    ($name:ident, $font:expr, $font_size:expr, $features:expr, $variations:expr, $input:expr) => {
        #[test]
        fn $name() {
            shaping::shape_text(
                $font,
                $font_size,
                $features,
                $variations,
                $input,
                false,
                false,
            );
        }
    };
}
