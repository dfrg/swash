use swash::text::{analyze, Codepoint, Script};

pub fn shape(font: &str, font_size: usize, variations: &[(&str, f32)], input: &str) -> String {
    // Open file, open font, get necessary data for `ShaperBuilder` etc.
    let file = std::fs::read(font).unwrap();
    let font = swash::FontRef::from_offset(&file, 0).unwrap();
<<<<<<< HEAD
    let mut context = swash::shape::ShapeContext::new();
=======
    let font_size = if font_size == 0 {
        75
    } else {
        font_size
    };

    // Other supplementary information for the `Shaper`.
>>>>>>> d799474 (wip)
    let script = input
        .chars()
        .map(|ch| ch.script())
        .find(|&script| {
            script != Script::Unknown && script != Script::Inherited && script != Script::Common
        })
        .unwrap_or(Script::Latin);
    let builder = context
        .builder(font)
        .size(font_size as f32)
        .variations(variations)
        .script(script);
    let needs_resolution = analyze(input.chars()).any(|x| x.0.bidi_class().needs_resolution());

    let mut advance = 0.0;
    let mut output = Vec::new();
    let mut shaper = builder.build();
    shaper.add_str(input);
    shaper.shape_with(|cluster| {
        cluster.glyphs.iter().for_each(|glyph| {
            // HarfBuzz format doesn't include glyphs with no advance
            if advance == 0.0 {
                output.push(format!(
                    "{}",
                    font.glyph_name(glyph.id)
                        .map(|x| x.to_string())
                        .unwrap_or(format!("gid{}", glyph.id))
                ));
            } else {
                output.push(format!(
                    "{}@{},{}",
                    font.glyph_name(glyph.id)
                        .map(|x| x.to_string())
                        .unwrap_or(format!("gid{}", glyph.id)),
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

<<<<<<< HEAD
    // Format the returned glyphs to match the HarfBuzz format.
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
=======
    // Finally format everything to match HarfBuzz, e.g. `[A|B@x,y|C@x,y]`.
    if output.len() == 1 && output.first().unwrap() == "*" {
        output.first().unwrap().to_string()
    } else {
        format!(
            "[{}]",
            output
                .iter()
                .enumerate()
                .map(|(idx, glyph)| {
                    if idx == 0 {
                        glyph.to_string()
                    } else {
                        format!("|{}", glyph)
                    }
                })
                .collect::<String>()
        )
    }
>>>>>>> d799474 (wip)
}
