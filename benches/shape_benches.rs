use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use std::path::Path;
use swash::shape::{Direction, ShapeContext};
use swash::text::Script;
use swash::FontRef;

/// Represents a single benchmark case.
struct TestCase {
    font_path: String,
    text_path: String,
    script: Script,
    direction: Direction,
}

/// Criterion benchmark entry point.
fn bench_shape(criterion: &mut Criterion) {
    let tests = default_tests();

    for test in tests {
        let font_data = fs::read(&test.font_path)
            .unwrap_or_else(|e| panic!("Failed to read font '{}': {}", test.font_path, e));
        let font =
            FontRef::from_index(&font_data, 0).expect("Failed to create FontRef from font data");

        let text_content = fs::read_to_string(&test.text_path)
            .unwrap_or_else(|e| panic!("Failed to read text '{}': {}", test.text_path, e));

        let font_name = Path::new(&test.font_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&test.font_path);

        let text_name = Path::new(&test.text_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&test.text_path);

        let mut bench_id = format!("BM_Shape/{}/{}", font_name, text_name);
        bench_id.push_str("/swash");

        const FONT_SIZE: f32 = 24.0;
        let mut shape_context = ShapeContext::new();

        criterion.bench_function(&bench_id, |bencher| {
            bencher.iter(|| {
                for line in text_content.lines().filter(|l| !l.trim().is_empty()) {
                    let shaper_builder = shape_context
                        .builder(font)
                        .script(test.script)
                        .direction(test.direction)
                        .size(FONT_SIZE);

                    let mut shaper = shaper_builder.build();
                    shaper.add_str(line);
                    shaper.shape_with(|_glyph_cluster| {
                        // Output intentionally ignored for benchmarking
                    });
                }
            });
        });
    }
}

/// Defines the font/text/script/direction combinations to test.
fn default_tests() -> Vec<TestCase> {
    vec![
        TestCase {
            font_path: "resources/fonts/NotoNastaliqUrdu-Regular.ttf".into(),
            text_path: "resources/texts/fa-thelittleprince.txt".into(),
            script: Script::Arabic,
            direction: Direction::RightToLeft,
        },
        TestCase {
            font_path: "resources/fonts/NotoNastaliqUrdu-Regular.ttf".into(),
            text_path: "resources/texts/fa-words.txt".into(),
            script: Script::Arabic,
            direction: Direction::RightToLeft,
        },
        TestCase {
            font_path: "resources/fonts/Amiri-Regular.ttf".into(),
            text_path: "resources/texts/fa-thelittleprince.txt".into(),
            script: Script::Arabic,
            direction: Direction::RightToLeft,
        },
        // Devanagari test temporarily disabled due to crash
        /*TestCase {
            font_path: "resources/fonts/NotoSansDevanagari-Regular.ttf".into(),
            text_path: "resources/texts/hi-words.txt".into(),
            script: Script::Devanagari,
            direction: Direction::LeftToRight,
        },*/
        TestCase {
            font_path: "resources/fonts/Roboto-Regular.ttf".into(),
            text_path: "resources/texts/en-thelittleprince.txt".into(),
            script: Script::Latin,
            direction: Direction::LeftToRight,
        },
        TestCase {
            font_path: "resources/fonts/Roboto-Regular.ttf".into(),
            text_path: "resources/texts/en-words.txt".into(),
            script: Script::Latin,
            direction: Direction::LeftToRight,
        },
        TestCase {
            font_path: "resources/fonts/SourceSerifVariable-Roman.ttf".into(),
            text_path: "resources/texts/react-dom.txt".into(),
            script: Script::Latin,
            direction: Direction::LeftToRight,
        },
    ]
}

criterion_group!(benches, bench_shape);
criterion_main!(benches);
