use swash::text::{analyze, Codepoint, Script};

pub fn shape(font: &str, font_size: usize, variations: &[(&str, f32)], input: &str) -> String {
    // Open the font source file and create a `FontRef` from it.
    let file = std::fs::read(font).unwrap();
    let font = swash::FontRef::from_offset(&file, 0).unwrap();

    // Other supplementary information for the `Shaper`.
    let script = input
        .chars()
        .map(|ch| ch.script())
        .find(|&script| {
            script != Script::Unknown && script != Script::Inherited && script != Script::Common
        })
        .unwrap_or(Script::Latin);
    let needs_resolution = analyze(input.chars()).any(|x| x.0.bidi_class().needs_resolution());

    // Create the shaper.
    let mut shaper_context = swash::shape::ShapeContext::new();
    let builder = shaper_context
        .builder(font)
        .size(font_size as f32)
        .variations(variations)
        .script(script);
    let mut shaper = builder.build();
    shaper.add_str(input);

    // Iterate over `Shaper` output to create an output that matches what HarfBuzz expects.
    // `advance` needs to be top level to be retained through all the clusters.
    let mut advance = 0.0;
    let mut output = Vec::new();
    shaper.shape_with(|cluster| {
        // Iterate over each glyph in the cluster, format them. and push them
        // to `output`.
        for glyph in cluster.glyphs {
            let font_name = font
                .glyph_name(glyph.id)
                .and_then(|name| Some(name.to_string()))
                .unwrap_or(format!("gid{}", glyph.id));

            // HarfBuzz omits the glyph's position (`@{},{}`) if there is no advance.
            if advance == 0.0 {
                output.push(font_name);
            } else {
                output.push(format!(
                    "{}@{},{}",
                    font_name,
                    (glyph.x + advance).round() as i64,
                    glyph.y.round() as i64,
                ));
            }
            advance += glyph.advance;
        }
    });

    // If the text is a right-to-left run, it needs to be reversed.
    // NOTE: slightly inaccurate due to glyph vs. cluster tracking on `swash`'s part,
    // but it will be fixed before this PR is merged.
    if needs_resolution {
        output.reverse();
    }

    // Finally format everything to match HarfBuzz, e.g. `[A|B@x,y|C@x,y]`.
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

// pub fn shape_aot(font: &str, font_size: usize, variations: &[(&str, f32)], input: &str) -> String {
//     let font_size = if font_size == 0 {
//         75
//     } else {
//         font_size
//     };
//     // Open the font source file and create a `FontRef` from it.
//     let file = std::fs::read(font).unwrap();
//     let font = swash::FontRef::from_offset(&file, 0).unwrap();

//     // Other supplementary information for the `Shaper`.
//     let script = input
//         .chars()
//         .map(|ch| ch.script())
//         .find(|&script| {
//             script != Script::Unknown && script != Script::Inherited && script != Script::Common
//         })
//         .unwrap_or(Script::Latin);
//     let needs_resolution = analyze(input.chars()).any(|x| x.0.bidi_class().needs_resolution());

//     // Create the shaper.
//     let mut shaper_context = swash::shape::ShapeContext::new();
//     let builder = shaper_context
//         .builder(font)
//         .size(font_size as f32)
//         .variations(variations)
//         .script(script);
//     let mut shaper = builder.build();
//     shaper.add_str(input);

//     // Iterate over `Shaper` output to create an output that matches what HarfBuzz expects.
//     // `advance` needs to be top level to be retained through all the clusters.
//     let mut advance = 0.0;
//     let mut output = Vec::new();
//     shaper.shape_with(|cluster| {
//         // Iterate over each glyph in the cluster, format them. and push them
//         // to `output`.
//         for glyph in cluster.glyphs {
//             let font_name = glyph.id.to_string();

//             // HarfBuzz omits the glyph's position (`@{},{}`) if there is no advance.
//             if advance == 0.0 {
//                 output.push(font_name);
//             } else {
//                 output.push(format!(
//                     "{}@{},{}",
//                     font_name,
//                     (glyph.x + advance).round() as i64,
//                     glyph.y.round() as i64,
//                 ));
//             }
//             advance += glyph.advance;
//         }
//     });

//     // If the text is a right-to-left run, it needs to be reversed.
//     // NOTE: slightly inaccurate due to glyph vs. cluster tracking on `swash`'s part,
//     // but it will be fixed before this PR is merged.
//     if needs_resolution {
//         output.reverse();
//     }
//     panic!(output.len().to_string());
//     // Finally format everything to match HarfBuzz, e.g. `[A|B@x,y|C@x,y]`.
//     format!(
//         "[{}]",
//         output
//             .iter()
//             .enumerate()
//             .map(|(idx, glyph)| {
//                 if idx == 0 {
//                     glyph.to_string()
//                 } else {
//                     format!("|{}", glyph)
//                 }
//             })
//             .collect::<String>()
//     )
// }
