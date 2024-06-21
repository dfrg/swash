use image::{ImageBuffer, RgbaImage};
use swash::scale::{Render, ScaleContext, Source, StrikeWith};
use swash::shape::ShapeContext;
use swash::text::cluster::{CharCluster, Parser, Token};
use swash::text::Script;
use swash::zeno::Format;
use swash::FontRef;

const CANVAS_WIDTH: usize = 700;
const CANVAS_HEIGHT: usize = 300;

const FONT_SIZE: f32 = 24.0;

const PATH_FONT: &str = "./examples/assets/Roboto/Roboto-Regular.ttf";
const PATH_OUTPUT_IMAGE: &str = "./examples/minimal.png";

fn main() {
    println!("Loading font data...");

    let font_data = std::fs::read(PATH_FONT).unwrap();
    let font = FontRef::from_index(&font_data, 0).unwrap();

    println!("Drawing text...");

    let mut pixel_data = vec![255; CANVAS_WIDTH * CANVAS_HEIGHT * 4]; // Initialize with white

    let lines = vec![
        "Hello world!",
        "This is another line of text.",
        "And... here's another one.",
    ];
    for (idx, line) in lines.iter().enumerate() {
        let pos_x = 20;
        let pos_y = 30 + idx as u32 * 30;
        render_text(line, pos_x, pos_y, &font, &mut pixel_data);
    }

    println!("Saving image...");

    let img: RgbaImage =
        ImageBuffer::from_vec(CANVAS_WIDTH as u32, CANVAS_HEIGHT as u32, pixel_data)
            .expect("Failed to create image buffer");

    img.save(PATH_OUTPUT_IMAGE).expect("Failed to save image");

    println!("Done");
}

fn render_text(
    text_content: &str,
    pos_x: u32,
    pos_y: u32,
    font: &swash::FontRef,
    pixel_data: &mut Vec<u8>,
) {
    let mut shape_context = ShapeContext::new();
    let mut shaper = shape_context
        .builder(*font)
        .script(Script::Latin)
        .size(FONT_SIZE)
        .features(&[("dlig", 1)])
        .build();

    // We'll need the character map for our font
    let charmap = font.charmap();
    // And some storage for the cluster we're working with
    let mut cluster = CharCluster::new();
    // Now we build a cluster parser which takes a script and
    // an iterator that yields a Token per character
    let mut parser = Parser::new(
        Script::Latin,
        text_content.char_indices().map(|(i, ch)| Token {
            // The character
            ch,
            // Offset of the character in code units
            offset: i as u32,
            // Length of the character in code units
            len: ch.len_utf8() as u8,
            // Character information
            info: ch.into(),
            // Pass through user data
            data: 0,
        }),
    );
    // Loop over all of the clusters
    while parser.next(&mut cluster) {
        // Map all of the characters in the cluster
        // to nominal glyph identifiers
        cluster.map(|ch| charmap.map(ch));
        // Add the cluster to the shaper
        shaper.add_cluster(&cluster);
    }

    let mut scale_context = ScaleContext::new();

    let mut run_offset_x: f32 = 0.0;

    shaper.shape_with(|cluster| {
        cluster.glyphs.iter().for_each(|glyph| {
            let mut scaler = scale_context
                .builder(*font)
                .size(FONT_SIZE)
                .hint(true)
                .build();
            let image = Render::new(&[
                Source::ColorOutline(0),
                Source::ColorBitmap(StrikeWith::BestFit),
                Source::Outline,
            ])
            .format(Format::Subpixel)
            .render(&mut scaler, glyph.id)
            .unwrap();

            let glyph_image_data = image.data.as_slice();

            let glyph_x_min: i32 = image.placement.left;
            let glyph_x_max: i32 = image.placement.left + image.placement.width as i32;
            let glyph_y_min: i32 = image.placement.top;
            let glyph_y_max: i32 = image.placement.top + image.placement.height as i32;

            for glyph_x in glyph_x_min..glyph_x_max {
                for glyph_y in glyph_y_min..glyph_y_max {
                    // Get the value of this pixel
                    let glyph_byte_offset = (glyph_x - glyph_x_min
                        + (glyph_y - glyph_y_min) * image.placement.width as i32)
                        * 4;

                    if glyph_byte_offset < 0 {
                        eprintln!("Glyph byte offset is less than 0, cannot lookup pixel data");
                        panic!("Invalid glyph byte offset");
                    }

                    let glyph_byte_offset = glyph_byte_offset as usize;

                    let pixel_r = 255 - glyph_image_data[glyph_byte_offset];
                    let pixel_g = 255 - glyph_image_data[glyph_byte_offset + 1];
                    let pixel_b = 255 - glyph_image_data[glyph_byte_offset + 2];

                    if pixel_r == 255 && pixel_g == 255 && pixel_b == 255 {
                        // Blank pixel, skip
                        continue;
                    }

                    // Copy the value onto the canvas, at an offset position
                    let canvas_x = glyph_x + pos_x as i32 + run_offset_x.round() as i32; // TODO: I think we might be missing a glyph offset
                    let canvas_y = glyph_y + pos_y as i32 - (glyph_y_min * 2);

                    let canvas_byte_offset = (canvas_y * (CANVAS_WIDTH as i32) + canvas_x) * 4;

                    if canvas_byte_offset < 0 {
                        eprintln!("canvas byte offset is less than 0, cannot set pixel data");
                        panic!("Invalid canvas byte offset");
                    }

                    let canvas_byte_offset = canvas_byte_offset as usize;

                    pixel_data[canvas_byte_offset] = pixel_r;
                    pixel_data[canvas_byte_offset + 1] = pixel_g;
                    pixel_data[canvas_byte_offset + 2] = pixel_b;
                }
            }

            run_offset_x += glyph.advance.round();
        });
    });
}
