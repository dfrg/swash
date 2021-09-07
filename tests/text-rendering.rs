mod shaping;
use shaping::shape_text;

macro_rules! shaping_test {
    ($name:ident, $font:expr, $font_size:expr, $features:expr, $variations:expr, $input:expr, $output:expr) => {
        #[test]
        fn $name() {
            assert_eq!(
                shape_text($font, $font_size, $features, $variations, $input),
                $output
            );
        }
    };
    ($name:ident, $font:expr, $font_size:expr, $features:expr, $variations:expr, $input:expr) => {
        #[test]
        fn $name() {
            shape_text($font, $font_size, $features, $variations, $input);
        }
    };
}
shaping_test!(
    avar_1_1,
    "tests/fonts/text-rendering/TestAVAR.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 100.0)],
    "⨁",
    &["gid1"]
);
shaping_test!(
    avar_1_2,
    "tests/fonts/text-rendering/TestAVAR.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 150.0)],
    "⨁",
    &["gid1"]
);
shaping_test!(
    avar_1_3,
    "tests/fonts/text-rendering/TestAVAR.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 200.0)],
    "⨁",
    &["gid1"]
);
shaping_test!(
    avar_1_4,
    "tests/fonts/text-rendering/TestAVAR.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 250.0)],
    "⨁",
    &["gid1"]
);
shaping_test!(
    avar_1_5,
    "tests/fonts/text-rendering/TestAVAR.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 300.0)],
    "⨁",
    &["gid1"]
);
shaping_test!(
    avar_1_6,
    "tests/fonts/text-rendering/TestAVAR.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 350.0)],
    "⨁",
    &["gid1"]
);
shaping_test!(
    avar_1_7,
    "tests/fonts/text-rendering/TestAVAR.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 400.0)],
    "⨁",
    &["gid1"]
);
shaping_test!(
    avar_1_8,
    "tests/fonts/text-rendering/TestAVAR.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 450.0)],
    "⨁",
    &["gid1"]
);
shaping_test!(
    avar_1_9,
    "tests/fonts/text-rendering/TestAVAR.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 500.0)],
    "⨁",
    &["gid1"]
);
shaping_test!(
    avar_1_10,
    "tests/fonts/text-rendering/TestAVAR.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 550.0)],
    "⨁",
    &["gid1"]
);
shaping_test!(
    avar_1_11,
    "tests/fonts/text-rendering/TestAVAR.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 600.0)],
    "⨁",
    &["gid1"]
);
shaping_test!(
    avar_1_12,
    "tests/fonts/text-rendering/TestAVAR.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 650.0)],
    "⨁",
    &["gid1"]
);
shaping_test!(
    avar_1_13,
    "tests/fonts/text-rendering/TestAVAR.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 700.0)],
    "⨁",
    &["gid1"]
);
shaping_test!(
    avar_1_14,
    "tests/fonts/text-rendering/TestAVAR.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 750.0)],
    "⨁",
    &["gid1"]
);
shaping_test!(
    avar_1_15,
    "tests/fonts/text-rendering/TestAVAR.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 800.0)],
    "⨁",
    &["gid1"]
);
shaping_test!(
    avar_1_16,
    "tests/fonts/text-rendering/TestAVAR.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 850.0)],
    "⨁",
    &["gid1"]
);
shaping_test!(
    avar_1_17,
    "tests/fonts/text-rendering/TestAVAR.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 900.0)],
    "⨁",
    &["gid1"]
);
shaping_test!(
    cff_1_1,
    "tests/fonts/text-rendering/FDArrayTest257.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "A",
    &["gid66"]
);
shaping_test!(
    cff_1_2,
    "tests/fonts/text-rendering/FDArrayTest257.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ℝ",
    &["gid30"]
);
shaping_test!(
    cff_1_3,
    "tests/fonts/text-rendering/FDArrayTest257.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "⓪",
    &["gid235"]
);
shaping_test!(
    cff_1_4,
    "tests/fonts/text-rendering/FDArrayTest257.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "①",
    &["gid97"]
);
shaping_test!(
    cff_1_5,
    "tests/fonts/text-rendering/FDArrayTest257.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "②",
    &["gid98"]
);
shaping_test!(
    cff_1_6,
    "tests/fonts/text-rendering/FDArrayTest257.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "仿",
    &["gid256"]
);
shaping_test!(
    cff_1_7,
    "tests/fonts/text-rendering/FDArrayTest257.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "Ａ",
    &["gid34"]
);
shaping_test!(
    cff_1_8,
    "tests/fonts/text-rendering/FDArrayTest257.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "𐄳",
    &["gid52"]
);
shaping_test!(
    cff_1_9,
    "tests/fonts/text-rendering/FDArrayTest257.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "𝓐",
    &["gid209"]
);
shaping_test!(
    cff_1_10,
    "tests/fonts/text-rendering/FDArrayTest257.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "🌺",
    &["gid59"]
);
shaping_test!(
    cff_1_11,
    "tests/fonts/text-rendering/FDArrayTest257.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "🌻",
    &["gid60"]
);
shaping_test!(
    cff_1_12,
    "tests/fonts/text-rendering/FDArrayTest257.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "💧",
    &["gid168"]
);
shaping_test!(
    cff_1_13,
    "tests/fonts/text-rendering/FDArrayTest257.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "🥝",
    &["gid94"]
);
shaping_test!(
    cff_2_1,
    "tests/fonts/text-rendering/FDArrayTest65535.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "A",
    &["gid66"]
);
shaping_test!(
    cff_2_2,
    "tests/fonts/text-rendering/FDArrayTest65535.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ℝ",
    &["gid8478"]
);
shaping_test!(
    cff_2_3,
    "tests/fonts/text-rendering/FDArrayTest65535.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "⓪",
    &["gid9451"]
);
shaping_test!(
    cff_2_4,
    "tests/fonts/text-rendering/FDArrayTest65535.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "①",
    &["gid9313"]
);
shaping_test!(
    cff_2_5,
    "tests/fonts/text-rendering/FDArrayTest65535.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "②",
    &["gid9314"]
);
shaping_test!(
    cff_2_6,
    "tests/fonts/text-rendering/FDArrayTest65535.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "仿",
    &["gid20224"]
);
shaping_test!(
    cff_2_7,
    "tests/fonts/text-rendering/FDArrayTest65535.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "Ａ",
    &["gid65314"]
);
shaping_test!(
    cff_2_8,
    "tests/fonts/text-rendering/FDArrayTest65535.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "𐄳",
    &["gid308"]
);
shaping_test!(
    cff_2_9,
    "tests/fonts/text-rendering/FDArrayTest65535.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "𝓐",
    &["gid54481"]
);
shaping_test!(
    cff_2_10,
    "tests/fonts/text-rendering/FDArrayTest65535.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "🌺",
    &["gid62267"]
);
shaping_test!(
    cff_2_11,
    "tests/fonts/text-rendering/FDArrayTest65535.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "🌻",
    &["gid62268"]
);
shaping_test!(
    cff_2_12,
    "tests/fonts/text-rendering/FDArrayTest65535.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "💧",
    &["gid62632"]
);
shaping_test!(
    cff_2_13,
    "tests/fonts/text-rendering/FDArrayTest65535.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "🥝",
    &["gid63838"]
);
shaping_test!(
    cff2_1_1,
    "tests/fonts/text-rendering/AdobeVFPrototype-Subset.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 100.0)],
    "$",
    &["dollar"]
);
shaping_test!(
    cff2_1_2,
    "tests/fonts/text-rendering/AdobeVFPrototype-Subset.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 200.0)],
    "$",
    &["dollar"]
);
shaping_test!(
    cff2_1_3,
    "tests/fonts/text-rendering/AdobeVFPrototype-Subset.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 300.0)],
    "$",
    &["dollar"]
);
shaping_test!(
    cff2_1_4,
    "tests/fonts/text-rendering/AdobeVFPrototype-Subset.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 400.0)],
    "$",
    &["dollar"]
);
shaping_test!(
    cff2_1_5,
    "tests/fonts/text-rendering/AdobeVFPrototype-Subset.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 500.0)],
    "$",
    &["dollar"]
);
shaping_test!(
    cff2_1_6,
    "tests/fonts/text-rendering/AdobeVFPrototype-Subset.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 600.0)],
    "$",
    &["dollar"]
);
shaping_test!(
    cff2_1_7,
    "tests/fonts/text-rendering/AdobeVFPrototype-Subset.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 700.0)],
    "$",
    &["dollar"]
);
shaping_test!(
    cff2_1_8,
    "tests/fonts/text-rendering/AdobeVFPrototype-Subset.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 800.0)],
    "$",
    &["dollar.nostroke"]
);
shaping_test!(
    cff2_1_9,
    "tests/fonts/text-rendering/AdobeVFPrototype-Subset.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 900.0)],
    "$",
    &["dollar.nostroke"]
);
shaping_test!(
    cmap_1_1,
    "tests/fonts/text-rendering/TestCMAP14.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "芦",
    &["uni82A6_uE0100"]
);
shaping_test!(
    cmap_1_2,
    "tests/fonts/text-rendering/TestCMAP14.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "芦\u{e0100}",
    &["uni82A6_uE0100"]
);
shaping_test!(
    cmap_1_3,
    "tests/fonts/text-rendering/TestCMAP14.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "芦\u{e0101}",
    &["uni82A6_uE0101"]
);
shaping_test!(
    cmap_1_4,
    "tests/fonts/text-rendering/TestCMAP14.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "芦\u{e0102}",
    &["uni82A6_uE0100"]
);
shaping_test!(
    cmap_2_1,
    "tests/fonts/text-rendering/TestCMAP14.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "≩",
    &["uni2269"]
);
shaping_test!(
    cmap_2_2,
    "tests/fonts/text-rendering/TestCMAP14.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "≩\u{fe00}",
    &["uni2269FE00"]
);
shaping_test!(
    cmap_3_1,
    "tests/fonts/text-rendering/TestCMAPMacTurkish.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "“",
    &["gid200"]
);
shaping_test!(
    cmap_3_2,
    "tests/fonts/text-rendering/TestCMAPMacTurkish.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "A",
    &["gid34"]
);
shaping_test!(
    cmap_3_3,
    "tests/fonts/text-rendering/TestCMAPMacTurkish.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "B",
    &["gid35"]
);
shaping_test!(
    cmap_3_4,
    "tests/fonts/text-rendering/TestCMAPMacTurkish.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "Ç",
    &["gid126"]
);
shaping_test!(
    cmap_3_5,
    "tests/fonts/text-rendering/TestCMAPMacTurkish.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "Ğ",
    &["gid176"]
);
shaping_test!(
    cmap_3_6,
    "tests/fonts/text-rendering/TestCMAPMacTurkish.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "I",
    &["gid42"]
);
shaping_test!(
    cmap_3_7,
    "tests/fonts/text-rendering/TestCMAPMacTurkish.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "İ",
    &["gid178"]
);
shaping_test!(
    cmap_3_8,
    "tests/fonts/text-rendering/TestCMAPMacTurkish.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "Ö",
    &["gid140"]
);
shaping_test!(
    cmap_3_9,
    "tests/fonts/text-rendering/TestCMAPMacTurkish.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "Ş",
    &["gid181"]
);
shaping_test!(
    cmap_3_10,
    "tests/fonts/text-rendering/TestCMAPMacTurkish.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "Ü",
    &["gid145"]
);
shaping_test!(
    cmap_3_11,
    "tests/fonts/text-rendering/TestCMAPMacTurkish.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "”",
    &["gid201"]
);
shaping_test!(
    cmap_3_12,
    "tests/fonts/text-rendering/TestCMAPMacTurkish.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "a",
    &["gid66"]
);
shaping_test!(
    cmap_3_13,
    "tests/fonts/text-rendering/TestCMAPMacTurkish.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "b",
    &["gid67"]
);
shaping_test!(
    cmap_3_14,
    "tests/fonts/text-rendering/TestCMAPMacTurkish.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ç",
    &["gid154"]
);
shaping_test!(
    cmap_3_15,
    "tests/fonts/text-rendering/TestCMAPMacTurkish.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ğ",
    &["gid177"]
);
shaping_test!(
    cmap_3_16,
    "tests/fonts/text-rendering/TestCMAPMacTurkish.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ı",
    &["gid222"]
);
shaping_test!(
    cmap_3_17,
    "tests/fonts/text-rendering/TestCMAPMacTurkish.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "i",
    &["gid74"]
);
shaping_test!(
    cmap_3_18,
    "tests/fonts/text-rendering/TestCMAPMacTurkish.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ö",
    &["gid168"]
);
shaping_test!(
    cmap_3_19,
    "tests/fonts/text-rendering/TestCMAPMacTurkish.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ş",
    &["gid182"]
);
shaping_test!(
    cmap_3_20,
    "tests/fonts/text-rendering/TestCMAPMacTurkish.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ü",
    &["gid174"]
);
shaping_test!(
    cvar_1_1,
    "tests/fonts/text-rendering/TestCVARGVARTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 72.0)],
    "hon",
    &["uni0068", "uni006F@595,0", "uni006E@1126,0"]
);
shaping_test!(
    cvar_1_2,
    "tests/fonts/text-rendering/TestCVARGVARTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 72.0)],
    "hon",
    &["uni0068", "uni006F@635,0", "uni006E@1212,0"]
);
shaping_test!(
    cvar_1_3,
    "tests/fonts/text-rendering/TestCVARGVARTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 72.0)],
    "hon",
    &["uni0068", "uni006F@691,0", "uni006E@1331,0"]
);
shaping_test!(
    cvar_2_1,
    "tests/fonts/text-rendering/TestCVARGVAROne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 72.0)],
    "hon",
    &["uni0068", "uni006F@595,0", "uni006E@1126,0"]
);
shaping_test!(
    cvar_2_2,
    "tests/fonts/text-rendering/TestCVARGVAROne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 72.0)],
    "hon",
    &["uni0068", "uni006F@635,0", "uni006E@1212,0"]
);
shaping_test!(
    cvar_2_3,
    "tests/fonts/text-rendering/TestCVARGVAROne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 72.0)],
    "hon",
    &["uni0068", "uni006F@691,0", "uni006E@1331,0"]
);
shaping_test!(
    glyf_1_1,
    "tests/fonts/text-rendering/TestGLYFOne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ģ",
    &["gcommaabove"]
);
shaping_test!(
    gpos_1_1,
    "tests/fonts/text-rendering/TestGPOSOne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ĄJ",
    &["Aogonek", "J@732,0"]
);
shaping_test!(
    gpos_1_2,
    "tests/fonts/text-rendering/TestGPOSOne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "Ąg",
    &["Aogonek", "g@692,0"]
);
shaping_test!(
    gpos_1_3,
    "tests/fonts/text-rendering/TestGPOSOne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "Ąģ",
    &["Aogonek", "gcommaabove@692,0"]
);
shaping_test!(
    gpos_1_4,
    "tests/fonts/text-rendering/TestGPOSOne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "Ąj",
    &["Aogonek", "j@752,0"]
);
shaping_test!(
    gpos_1_5,
    "tests/fonts/text-rendering/TestGPOSOne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "Ąȷ",
    &["Aogonek", "dotlessj@752,0"]
);
shaping_test!(
    gpos_1_6,
    "tests/fonts/text-rendering/TestGPOSOne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "Qȷ",
    &["Q", "dotlessj@734,0"]
);
shaping_test!(
    gpos_1_7,
    "tests/fonts/text-rendering/TestGPOSOne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ąj",
    &["aogonek", "j@588,0"]
);
shaping_test!(
    gpos_1_8,
    "tests/fonts/text-rendering/TestGPOSOne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ąȷ",
    &["aogonek", "dotlessj@588,0"]
);
shaping_test!(
    gpos_1_9,
    "tests/fonts/text-rendering/TestGPOSOne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "gȷ",
    &["g", "dotlessj@563,0"]
);
shaping_test!(
    gpos_1_10,
    "tests/fonts/text-rendering/TestGPOSOne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ģȷ",
    &["gcommaabove", "dotlessj@563,0"]
);
shaping_test!(
    gpos_1_11,
    "tests/fonts/text-rendering/TestGPOSOne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ıȷ",
    &["dotlessi", "dotlessj@334,0"]
);
shaping_test!(
    gpos_1_12,
    "tests/fonts/text-rendering/TestGPOSOne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ųȷ",
    &["uogonek", "dotlessj@656,0"]
);
shaping_test!(
    gpos_1_13,
    "tests/fonts/text-rendering/TestGPOSOne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "vȷ",
    &["v", "dotlessj@587,0"]
);
shaping_test!(
    gpos_1_14,
    "tests/fonts/text-rendering/TestGPOSOne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "Va",
    &["V", "a@594,0"]
);
shaping_test!(
    gpos_1_15,
    "tests/fonts/text-rendering/TestGPOSOne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "Vá",
    &["V", "aacute@594,0"]
);
shaping_test!(
    gpos_1_16,
    "tests/fonts/text-rendering/TestGPOSOne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "Vą",
    &["V", "aogonek@594,0"]
);
shaping_test!(
    gpos_1_17,
    "tests/fonts/text-rendering/TestGPOSOne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "Vf",
    &["V", "f@634,0"]
);
shaping_test!(
    gpos_1_18,
    "tests/fonts/text-rendering/TestGPOSOne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "Vﬂ",
    &["V", "fl@634,0"]
);
shaping_test!(
    gpos_1_19,
    "tests/fonts/text-rendering/TestGPOSOne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "V.",
    &["V", "period@504,0"]
);
shaping_test!(
    gpos_2_1,
    "tests/fonts/text-rendering/TestGPOSTwo.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "◯",
    &["uni25EF"]
);
shaping_test!(
    gpos_2_2,
    "tests/fonts/text-rendering/TestGPOSTwo.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "☼",
    &["sun"]
);
shaping_test!(
    gpos_2_3,
    "tests/fonts/text-rendering/TestGPOSTwo.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "◯☼",
    &["uni25EF", "sun"]
);
shaping_test!(
    gpos_3_1,
    "tests/fonts/text-rendering/TestShapeEthi.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ለ",
    &["uni1208"]
);
shaping_test!(
    gpos_3_2,
    "tests/fonts/text-rendering/TestShapeEthi.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ለ\u{135e}",
    &["uni1208", "uni135E@303,0"]
);
shaping_test!(
    gpos_3_3,
    "tests/fonts/text-rendering/TestShapeEthi.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ለ\u{135f}",
    &["uni1208", "uni135F@303,0"]
);
shaping_test!(
    gpos_3_4,
    "tests/fonts/text-rendering/TestShapeEthi.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ለ\u{135d}",
    &["uni1208", "uni135D@303,0"]
);
shaping_test!(
    gpos_4_1,
    "tests/fonts/text-rendering/TestGPOSThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "u\u{308}\u{301}",
    &["u", "uni0308@529,-31", "acutecomb@537,138"]
);
shaping_test!(
    gpos_4_2,
    "tests/fonts/text-rendering/TestGPOSThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "u\u{308}\u{304}",
    &["u", "uni0308@529,-31", "uni0304@526,138"]
);
shaping_test!(
    gpos_4_3,
    "tests/fonts/text-rendering/TestGPOSThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "u\u{308}\u{308}",
    &["u", "uni0308@529,-31", "uni0308@529,138"]
);
shaping_test!(
    gpos_4_4,
    "tests/fonts/text-rendering/TestGPOSThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "u\u{308}\u{308}\u{308}",
    &["u", "uni0308@529,-31", "uni0308@529,138", "uni0308@529,307"]
);
shaping_test!(
    gpos_5_1,
    "tests/fonts/text-rendering/TestGPOSFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 100.0)],
    "ش\u{652}",
    &["uni0652@663,144", "uni0634"]
);
shaping_test!(
    gpos_5_2,
    "tests/fonts/text-rendering/TestGPOSFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 300.0)],
    "ش\u{652}",
    &["uni0652@680,165", "uni0634"]
);
shaping_test!(
    gpos_5_3,
    "tests/fonts/text-rendering/TestGPOSFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 600.0)],
    "ش\u{652}",
    &["uni0652@730,246", "uni0634"]
);
shaping_test!(
    gpos_5_4,
    "tests/fonts/text-rendering/TestGPOSFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 700.0)],
    "ش\u{652}",
    &["uni0652@750,282", "uni0634"]
);
shaping_test!(
    gpos_5_5,
    "tests/fonts/text-rendering/TestGPOSFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 900.0)],
    "ش\u{652}",
    &["uni0652@784,351", "uni0634"]
);
shaping_test!(
    gsub_1_1,
    "tests/fonts/text-rendering/TestGSUBOne.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "a a",
    &["a.alt", "space@500,0", "a@1000,0"]
);
shaping_test!(
    gsub_2_1,
    "tests/fonts/text-rendering/TestShapeEthi.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "፳",
    &["uni1373"]
);
shaping_test!(
    gsub_2_2,
    "tests/fonts/text-rendering/TestShapeEthi.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "፫",
    &["uni136B"]
);
shaping_test!(
    gsub_2_3,
    "tests/fonts/text-rendering/TestShapeEthi.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "፵",
    &["uni1375"]
);
shaping_test!(
    gsub_2_4,
    "tests/fonts/text-rendering/TestShapeEthi.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "፭",
    &["uni136D"]
);
shaping_test!(
    gsub_2_5,
    "tests/fonts/text-rendering/TestShapeEthi.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "፳፫",
    &["uni1373.init", "uni136B.fina@621,0"]
);
shaping_test!(
    gsub_2_6,
    "tests/fonts/text-rendering/TestShapeEthi.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "፵፭",
    &["uni1375.init", "uni136D.fina@662,0"]
);
shaping_test!(
    gsub_2_7,
    "tests/fonts/text-rendering/TestShapeEthi.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "፻",
    &["uni137B"]
);
shaping_test!(
    gsub_2_8,
    "tests/fonts/text-rendering/TestShapeEthi.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "፳፫፻",
    &["uni1373.init", "uni136B.medi@621,0", "uni137B.fina@1102,0"]
);
shaping_test!(
    gsub_2_9,
    "tests/fonts/text-rendering/TestShapeEthi.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "፳፫፻፳፫",
    &[
        "uni1373.init",
        "uni136B.medi@621,0",
        "uni137B.medi@1102,0",
        "uni1373.medi@1489,0",
        "uni136B.fina@2110,0"
    ]
);
shaping_test!(
    gsub_2_10,
    "tests/fonts/text-rendering/TestShapeEthi.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "፳፫፻፵፭",
    &[
        "uni1373.init",
        "uni136B.medi@621,0",
        "uni137B.medi@1102,0",
        "uni1375.medi@1489,0",
        "uni136D.fina@2157,0"
    ]
);
shaping_test!(
    gsub_2_11,
    "tests/fonts/text-rendering/TestShapeEthi.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "፵፭፻፳፫",
    &[
        "uni1375.init",
        "uni136D.medi@662,0",
        "uni137B.medi@1203,0",
        "uni1373.medi@1590,0",
        "uni136B.fina@2211,0"
    ]
);
shaping_test!(
    gsub_3_1,
    "tests/fonts/text-rendering/TestGSUBThree.ttf",
    75,
    &[],
    &[],
    "lol"
);
shaping_test!(
    gvar_1_1,
    "tests/fonts/text-rendering/TestGVAROne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 300.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_1_2,
    "tests/fonts/text-rendering/TestGVAROne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 350.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_1_3,
    "tests/fonts/text-rendering/TestGVAROne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 400.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_1_4,
    "tests/fonts/text-rendering/TestGVAROne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 450.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_1_5,
    "tests/fonts/text-rendering/TestGVAROne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 500.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_1_6,
    "tests/fonts/text-rendering/TestGVAROne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 550.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_1_7,
    "tests/fonts/text-rendering/TestGVAROne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 600.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_1_8,
    "tests/fonts/text-rendering/TestGVAROne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 650.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_1_9,
    "tests/fonts/text-rendering/TestGVAROne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 700.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_2_1,
    "tests/fonts/text-rendering/TestGVARTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 300.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_2_2,
    "tests/fonts/text-rendering/TestGVARTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 350.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_2_3,
    "tests/fonts/text-rendering/TestGVARTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 400.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_2_4,
    "tests/fonts/text-rendering/TestGVARTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 450.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_2_5,
    "tests/fonts/text-rendering/TestGVARTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 500.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_2_6,
    "tests/fonts/text-rendering/TestGVARTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 550.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_2_7,
    "tests/fonts/text-rendering/TestGVARTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 600.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_2_8,
    "tests/fonts/text-rendering/TestGVARTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 650.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_2_9,
    "tests/fonts/text-rendering/TestGVARTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 700.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_3_1,
    "tests/fonts/text-rendering/TestGVARThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 300.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_3_2,
    "tests/fonts/text-rendering/TestGVARThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 350.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_3_3,
    "tests/fonts/text-rendering/TestGVARThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 400.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_3_4,
    "tests/fonts/text-rendering/TestGVARThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 450.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_3_5,
    "tests/fonts/text-rendering/TestGVARThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 500.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_3_6,
    "tests/fonts/text-rendering/TestGVARThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 550.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_3_7,
    "tests/fonts/text-rendering/TestGVARThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 600.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_3_8,
    "tests/fonts/text-rendering/TestGVARThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 650.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_3_9,
    "tests/fonts/text-rendering/TestGVARThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 700.0)],
    "彌",
    &["gid2"]
);
shaping_test!(
    gvar_4_1,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.0)],
    "🦎",
    &["gid5"]
);
shaping_test!(
    gvar_4_2,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.1)],
    "🦎",
    &["gid5"]
);
shaping_test!(
    gvar_4_3,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.2)],
    "🦎",
    &["gid5"]
);
shaping_test!(
    gvar_4_4,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.3)],
    "🦎",
    &["gid5"]
);
shaping_test!(
    gvar_4_5,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.4)],
    "🦎",
    &["gid5"]
);
shaping_test!(
    gvar_4_6,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.5)],
    "🦎",
    &["gid5"]
);
shaping_test!(
    gvar_4_7,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.6)],
    "🦎",
    &["gid5"]
);
shaping_test!(
    gvar_4_8,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.7)],
    "🦎",
    &["gid5"]
);
shaping_test!(
    gvar_4_9,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.8)],
    "🦎",
    &["gid5"]
);
shaping_test!(
    gvar_4_10,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.9)],
    "🦎",
    &["gid5"]
);
shaping_test!(
    gvar_4_11,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 1.0)],
    "🦎",
    &["gid5"]
);
shaping_test!(
    gvar_5_1,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", -1.0)],
    "🌝",
    &["gid15"]
);
shaping_test!(
    gvar_5_2,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", -0.8)],
    "🌝",
    &["gid15"]
);
shaping_test!(
    gvar_5_3,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", -0.6)],
    "🌝",
    &["gid15"]
);
shaping_test!(
    gvar_5_4,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", -0.4)],
    "🌝",
    &["gid15"]
);
shaping_test!(
    gvar_5_5,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", -0.2)],
    "🌝",
    &["gid15"]
);
shaping_test!(
    gvar_5_6,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.0)],
    "🌝",
    &["gid15"]
);
shaping_test!(
    gvar_5_7,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.2)],
    "🌝",
    &["gid15"]
);
shaping_test!(
    gvar_5_8,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.4)],
    "🌝",
    &["gid15"]
);
shaping_test!(
    gvar_5_9,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.6)],
    "🌝",
    &["gid15"]
);
shaping_test!(
    gvar_5_10,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.8)],
    "🌝",
    &["gid15"]
);
shaping_test!(
    gvar_5_11,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 1.0)],
    "🌝",
    &["gid15"]
);
shaping_test!(
    gvar_6_1,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.0)],
    "🐢",
    &["gid12"]
);
shaping_test!(
    gvar_6_2,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.1)],
    "🐢",
    &["gid12"]
);
shaping_test!(
    gvar_6_3,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.2)],
    "🐢",
    &["gid12"]
);
shaping_test!(
    gvar_6_4,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.3)],
    "🐢",
    &["gid12"]
);
shaping_test!(
    gvar_6_5,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.4)],
    "🐢",
    &["gid12"]
);
shaping_test!(
    gvar_6_6,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.5)],
    "🐢",
    &["gid12"]
);
shaping_test!(
    gvar_6_7,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.6)],
    "🐢",
    &["gid12"]
);
shaping_test!(
    gvar_6_8,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.7)],
    "🐢",
    &["gid12"]
);
shaping_test!(
    gvar_6_9,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.8)],
    "🐢",
    &["gid12"]
);
shaping_test!(
    gvar_6_10,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.9)],
    "🐢",
    &["gid12"]
);
shaping_test!(
    gvar_6_11,
    "tests/fonts/text-rendering/Zycon.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 1.0)],
    "🐢",
    &["gid12"]
);
shaping_test!(
    gvar_7_1,
    "tests/fonts/text-rendering/TestGVARFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 150.0)],
    "OIO",
    &["uni004F", "uni0049@706,0", "uni004F@1072,0"]
);
shaping_test!(
    gvar_7_2,
    "tests/fonts/text-rendering/TestGVARFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 200.0)],
    "OIO",
    &["uni004F", "uni0049@707,0", "uni004F@1074,0"]
);
shaping_test!(
    gvar_7_3,
    "tests/fonts/text-rendering/TestGVARFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 250.0)],
    "OIO",
    &["uni004F", "uni0049@707,0", "uni004F@1075,0"]
);
shaping_test!(
    gvar_7_4,
    "tests/fonts/text-rendering/TestGVARFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 300.0)],
    "OIO",
    &["uni004F", "uni0049@707,0", "uni004F@1076,0"]
);
shaping_test!(
    gvar_7_5,
    "tests/fonts/text-rendering/TestGVARFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 350.0)],
    "OIO",
    &["uni004F", "uni0049@707,0", "uni004F@1077,0"]
);
shaping_test!(
    gvar_7_6,
    "tests/fonts/text-rendering/TestGVARFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 400.0)],
    "OIO",
    &["uni004F", "uni0049@707,0", "uni004F@1078,0"]
);
shaping_test!(
    gvar_7_7,
    "tests/fonts/text-rendering/TestGVARFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 450.0)],
    "OIO",
    &["uni004F", "uni0049@706,0", "uni004F@1079,0"]
);
shaping_test!(
    gvar_8_1,
    "tests/fonts/text-rendering/TestGVAREight.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.0)],
    "H",
    &["H"]
);
shaping_test!(
    gvar_8_2,
    "tests/fonts/text-rendering/TestGVAREight.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", -0.2)],
    "H",
    &["H"]
);
shaping_test!(
    gvar_8_3,
    "tests/fonts/text-rendering/TestGVAREight.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", -0.4)],
    "H",
    &["H"]
);
shaping_test!(
    gvar_8_4,
    "tests/fonts/text-rendering/TestGVAREight.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", -0.6)],
    "H",
    &["H"]
);
shaping_test!(
    gvar_8_5,
    "tests/fonts/text-rendering/TestGVAREight.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", -0.8)],
    "H",
    &["H"]
);
shaping_test!(
    gvar_8_6,
    "tests/fonts/text-rendering/TestGVAREight.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", -1.0)],
    "H",
    &["H"]
);
shaping_test!(
    gvar_9_1,
    "tests/fonts/text-rendering/TestGVARNine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", -1.0)],
    "A",
    &["A"]
);
shaping_test!(
    gvar_9_2,
    "tests/fonts/text-rendering/TestGVARNine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", -0.5)],
    "A",
    &["A"]
);
shaping_test!(
    gvar_9_3,
    "tests/fonts/text-rendering/TestGVARNine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.0)],
    "A",
    &["A"]
);
shaping_test!(
    gvar_9_4,
    "tests/fonts/text-rendering/TestGVARNine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.5)],
    "A",
    &["A"]
);
shaping_test!(
    gvar_9_5,
    "tests/fonts/text-rendering/TestGVARNine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.6)],
    "A",
    &["A"]
);
shaping_test!(
    gvar_9_6,
    "tests/fonts/text-rendering/TestGVARNine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.7)],
    "A",
    &["A"]
);
shaping_test!(
    gvar_9_7,
    "tests/fonts/text-rendering/TestGVARNine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.8)],
    "A",
    &["A"]
);
shaping_test!(
    gvar_9_8,
    "tests/fonts/text-rendering/TestGVARNine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.9)],
    "A",
    &["A"]
);
shaping_test!(
    gvar_9_9,
    "tests/fonts/text-rendering/TestGVARNine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.944444)],
    "A",
    &["A"]
);
shaping_test!(
    gvar_9_10,
    "tests/fonts/text-rendering/TestGVARNine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 1.0)],
    "A",
    &["A"]
);
shaping_test!(
    hvar_1_1,
    "tests/fonts/text-rendering/TestHVAROne.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.0)],
    "ABC",
    &["A", "B@520,0", "C@1094,0"]
);
shaping_test!(
    hvar_1_2,
    "tests/fonts/text-rendering/TestHVAROne.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 200.0)],
    "ABC",
    &["A", "B@533,0", "C@1115,0"]
);
shaping_test!(
    hvar_1_3,
    "tests/fonts/text-rendering/TestHVAROne.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 400.0)],
    "ABC",
    &["A", "B@546,0", "C@1135,0"]
);
shaping_test!(
    hvar_1_4,
    "tests/fonts/text-rendering/TestHVAROne.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 600.0)],
    "ABC",
    &["A", "B@558,0", "C@1155,0"]
);
shaping_test!(
    hvar_1_5,
    "tests/fonts/text-rendering/TestHVAROne.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 800.0)],
    "ABC",
    &["A", "B@571,0", "C@1175,0"]
);
shaping_test!(
    hvar_1_6,
    "tests/fonts/text-rendering/TestHVAROne.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 1000.0)],
    "ABC",
    &["A", "B@584,0", "C@1196,0"]
);
shaping_test!(
    hvar_2_1,
    "tests/fonts/text-rendering/TestHVARTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 0.0)],
    "AB",
    &["uni0041", "uni0042@450,0"]
);
shaping_test!(
    hvar_2_2,
    "tests/fonts/text-rendering/TestHVARTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 200.0)],
    "AB",
    &["uni0041", "uni0042@515,0"]
);
shaping_test!(
    hvar_2_3,
    "tests/fonts/text-rendering/TestHVARTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 400.0)],
    "AB",
    &["uni0041", "uni0042@584,0"]
);
shaping_test!(
    hvar_2_4,
    "tests/fonts/text-rendering/TestHVARTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 600.0)],
    "AB",
    &["uni0041", "uni0042@673,0"]
);
shaping_test!(
    hvar_2_5,
    "tests/fonts/text-rendering/TestHVARTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 800.0)],
    "AB",
    &["uni0041", "uni0042@761,0"]
);
shaping_test!(
    hvar_2_6,
    "tests/fonts/text-rendering/TestHVARTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0), ("variations", 1000.0)],
    "AB",
    &["uni0041", "uni0042@850,0"]
);
shaping_test!(
    kern_1_1,
    "tests/fonts/text-rendering/TestKERNOne.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ıTuTuTı",
    &[
        "dotlessi",
        "T",
        "u@400,0",
        "T@600,0",
        "u@1000,0",
        "T@1200,0",
        "dotlessi@1600,0"
    ]
);
shaping_test!(
    kern_2_1,
    "tests/fonts/text-rendering/TestKERNOne.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "uııTııTııu",
    &[
        "u",
        "dotlessi@400,0",
        "dotlessi@1100,0",
        "T@1100,0",
        "dotlessi@1500,0",
        "dotlessi@2200,0",
        "T@2200,0",
        "dotlessi@2600,0",
        "dotlessi@3300,0",
        "u@3500,0"
    ]
);
shaping_test!(
    morx_1_1,
    "tests/fonts/text-rendering/TestMORXOne.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABC",
    &["A.alt", "B@1000,0", "C.alt@2000,0"]
);
shaping_test!(
    morx_10_1,
    "tests/fonts/text-rendering/TestMORXTen.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABABAB",
    &["A", "B@638,0", "A@1288,0", "B@1926,0", "B@2576,0", "A@3226,0"]
);
shaping_test!(
    morx_11_1,
    "tests/fonts/text-rendering/TestMORXEleven.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "BABBAABX",
    &["B", "A@650,0", "B@1288,0", "B@1938,0", "A@2588,0", "X@3226,0", "A@3812,0", "B@4450,0"]
);
shaping_test!(
    morx_12_1,
    "tests/fonts/text-rendering/TestMORXTwelve.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "XABCX1",
    &[
        "X",
        "C@598,0",
        "A@1230,0",
        "B@1868,0",
        "X@2518,0",
        "one@3116,0"
    ]
);
shaping_test!(
    morx_12_2,
    "tests/fonts/text-rendering/TestMORXTwelve.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "XABCX2",
    &[
        "X",
        "C@598,0",
        "A@1230,0",
        "B@1868,0",
        "X@2518,0",
        "two@3116,0"
    ]
);
shaping_test!(
    morx_12_3,
    "tests/fonts/text-rendering/TestMORXTwelve.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "XABCX3",
    &[
        "X",
        "B@598,0",
        "C@1248,0",
        "A@1880,0",
        "X@2518,0",
        "three@3116,0"
    ]
);
shaping_test!(
    morx_13_1,
    "tests/fonts/text-rendering/TestMORXThirteen.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABCDE",
    &["B", "C@626,0", "D@1222,0", "E@1896,0", "A@2452,0"]
);
shaping_test!(
    morx_14_1,
    "tests/fonts/text-rendering/TestMORXFourteen.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABCDE",
    &["B", "C@626,0", "D@1222,0", "E@1896,0", "A@2452,0"]
);
shaping_test!(
    morx_14_2,
    "tests/fonts/text-rendering/TestMORXFourteen.ttf",
    75,
    &[],
    &[],
    "ABBBCCCDDDBCDCE"
);
shaping_test!(
    morx_16_1,
    "tests/fonts/text-rendering/TestMORXSixteen.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABCDE",
    &["B", "C@626,0", "D@1222,0", "E@1896,0", "A@2452,0"]
);
shaping_test!(
    morx_17_1,
    "tests/fonts/text-rendering/TestMORXSeventeen.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "AB",
    &["B", "A@626,0"]
);
shaping_test!(
    morx_18_1,
    "tests/fonts/text-rendering/TestMORXEighteen.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABCDE",
    &["A", "B.alt@639,0", "C@1639,0", "D.alt1@2235,0", "E@3235,0"]
);
shaping_test!(
    morx_18_2,
    "tests/fonts/text-rendering/TestMORXEighteen.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABBBDE",
    &[
        "A",
        "B@639,0",
        "B@1265,0",
        "B.alt@1891,0",
        "D.alt1@2891,0",
        "E@3891,0"
    ]
);
shaping_test!(
    morx_18_3,
    "tests/fonts/text-rendering/TestMORXEighteen.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABDE",
    &["A", "B.alt@639,0", "D.alt1@1639,0", "E@2639,0"]
);
shaping_test!(
    morx_18_4,
    "tests/fonts/text-rendering/TestMORXEighteen.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABE",
    &["A", "B@639,0", "E@1265,0"]
);
shaping_test!(
    morx_19_1,
    "tests/fonts/text-rendering/TestMORXEighteen.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ACDE",
    &["A.alt", "C@1000,0", "D.alt1@1596,0", "E@2596,0"]
);
shaping_test!(
    morx_19_2,
    "tests/fonts/text-rendering/TestMORXEighteen.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "D",
    &["D.alt"]
);
shaping_test!(
    morx_2_1,
    "tests/fonts/text-rendering/TestMORXTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "OOOABXYZCDOOO⓿",
    &[
        "O",
        "O@418,0",
        "O@836,0",
        "A@1254,0",
        "B@2084,0",
        "X@2914,0",
        "Y@3744,0",
        "Z@4574,0",
        "C@5404,0",
        "D@6234,0",
        "O@7064,0",
        "O@7482,0",
        "O@7900,0",
        "zero@8318,0"
    ]
);
shaping_test!(
    morx_2_2,
    "tests/fonts/text-rendering/TestMORXTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "OOOABXYZCDOOO➊",
    &[
        "O",
        "O@418,0",
        "O@836,0",
        "B@1254,0",
        "X@2084,0",
        "Y@2914,0",
        "Z@3744,0",
        "C@4574,0",
        "D@5404,0",
        "A@6234,0",
        "O@7064,0",
        "O@7482,0",
        "O@7900,0",
        "one@8318,0"
    ]
);
shaping_test!(
    morx_2_3,
    "tests/fonts/text-rendering/TestMORXTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "OOOABXYZCDOOO➋",
    &[
        "O",
        "O@418,0",
        "O@836,0",
        "D@1254,0",
        "A@2084,0",
        "B@2914,0",
        "X@3744,0",
        "Y@4574,0",
        "Z@5404,0",
        "C@6234,0",
        "O@7064,0",
        "O@7482,0",
        "O@7900,0",
        "two@8318,0"
    ]
);
shaping_test!(
    morx_2_4,
    "tests/fonts/text-rendering/TestMORXTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "OOOABXYZCDOOO3",
    &[
        "O",
        "O@418,0",
        "O@836,0",
        "D@1254,0",
        "B@2084,0",
        "X@2914,0",
        "Y@3744,0",
        "Z@4574,0",
        "C@5404,0",
        "A@6234,0",
        "O@7064,0",
        "O@7482,0",
        "O@7900,0",
        "three@8318,0"
    ]
);
shaping_test!(
    morx_2_5,
    "tests/fonts/text-rendering/TestMORXTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "OOOABXYZCDOOO4",
    &[
        "O",
        "O@418,0",
        "O@836,0",
        "X@1254,0",
        "Y@2084,0",
        "Z@2914,0",
        "C@3744,0",
        "D@4574,0",
        "A@5404,0",
        "B@6234,0",
        "O@7064,0",
        "O@7482,0",
        "O@7900,0",
        "four@8318,0"
    ]
);
shaping_test!(
    morx_2_6,
    "tests/fonts/text-rendering/TestMORXTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "OOOABXYZCDOOO5",
    &[
        "O",
        "O@418,0",
        "O@836,0",
        "X@1254,0",
        "Y@2084,0",
        "Z@2914,0",
        "C@3744,0",
        "D@4574,0",
        "B@5404,0",
        "A@6234,0",
        "O@7064,0",
        "O@7482,0",
        "O@7900,0",
        "five@8318,0"
    ]
);
shaping_test!(
    morx_2_7,
    "tests/fonts/text-rendering/TestMORXTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "OOOABXYZCDOOO6",
    &[
        "O",
        "O@418,0",
        "O@836,0",
        "C@1254,0",
        "D@2084,0",
        "A@2914,0",
        "B@3744,0",
        "X@4574,0",
        "Y@5404,0",
        "Z@6234,0",
        "O@7064,0",
        "O@7482,0",
        "O@7900,0",
        "six@8318,0"
    ]
);
shaping_test!(
    morx_2_8,
    "tests/fonts/text-rendering/TestMORXTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "OOOABXYZCDOOO7",
    &[
        "O",
        "O@418,0",
        "O@836,0",
        "D@1254,0",
        "C@2084,0",
        "A@2914,0",
        "B@3744,0",
        "X@4574,0",
        "Y@5404,0",
        "Z@6234,0",
        "O@7064,0",
        "O@7482,0",
        "O@7900,0",
        "seven@8318,0"
    ]
);
shaping_test!(
    morx_2_9,
    "tests/fonts/text-rendering/TestMORXTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "OOOABXYZCDOOO8",
    &[
        "O",
        "O@418,0",
        "O@836,0",
        "C@1254,0",
        "D@2084,0",
        "B@2914,0",
        "X@3744,0",
        "Y@4574,0",
        "Z@5404,0",
        "A@6234,0",
        "O@7064,0",
        "O@7482,0",
        "O@7900,0",
        "eight@8318,0"
    ]
);
shaping_test!(
    morx_2_10,
    "tests/fonts/text-rendering/TestMORXTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "OOOABXYZCDOOO9",
    &[
        "O",
        "O@418,0",
        "O@836,0",
        "D@1254,0",
        "C@2084,0",
        "B@2914,0",
        "X@3744,0",
        "Y@4574,0",
        "Z@5404,0",
        "A@6234,0",
        "O@7064,0",
        "O@7482,0",
        "O@7900,0",
        "nine@8318,0"
    ]
);
shaping_test!(
    morx_2_11,
    "tests/fonts/text-rendering/TestMORXTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "OOOABXYZCDOOO➓",
    &[
        "O",
        "O@418,0",
        "O@836,0",
        "D@1254,0",
        "X@2084,0",
        "Y@2914,0",
        "Z@3744,0",
        "C@4574,0",
        "A@5404,0",
        "B@6234,0",
        "O@7064,0",
        "O@7482,0",
        "O@7900,0",
        "one_zero@8318,0"
    ]
);
shaping_test!(
    morx_2_12,
    "tests/fonts/text-rendering/TestMORXTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "OOOABXYZCDOOO⓫",
    &[
        "O",
        "O@418,0",
        "O@836,0",
        "D@1254,0",
        "X@2084,0",
        "Y@2914,0",
        "Z@3744,0",
        "C@4574,0",
        "B@5404,0",
        "A@6234,0",
        "O@7064,0",
        "O@7482,0",
        "O@7900,0",
        "one_one@8318,0"
    ]
);
shaping_test!(
    morx_2_13,
    "tests/fonts/text-rendering/TestMORXTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "OOOABXYZCDOOO⓬",
    &[
        "O",
        "O@418,0",
        "O@836,0",
        "C@1254,0",
        "D@2084,0",
        "X@2914,0",
        "Y@3744,0",
        "Z@4574,0",
        "A@5404,0",
        "B@6234,0",
        "O@7064,0",
        "O@7482,0",
        "O@7900,0",
        "one_two@8318,0"
    ]
);
shaping_test!(
    morx_2_14,
    "tests/fonts/text-rendering/TestMORXTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "OOOABXYZCDOOO⓭",
    &[
        "O",
        "O@418,0",
        "O@836,0",
        "C@1254,0",
        "D@2084,0",
        "X@2914,0",
        "Y@3744,0",
        "Z@4574,0",
        "B@5404,0",
        "A@6234,0",
        "O@7064,0",
        "O@7482,0",
        "O@7900,0",
        "one_three@8318,0"
    ]
);
shaping_test!(
    morx_2_15,
    "tests/fonts/text-rendering/TestMORXTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "OOOABXYZCDOOO⓮",
    &[
        "O",
        "O@418,0",
        "O@836,0",
        "D@1254,0",
        "C@2084,0",
        "X@2914,0",
        "Y@3744,0",
        "Z@4574,0",
        "A@5404,0",
        "B@6234,0",
        "O@7064,0",
        "O@7482,0",
        "O@7900,0",
        "one_four@8318,0"
    ]
);
shaping_test!(
    morx_2_16,
    "tests/fonts/text-rendering/TestMORXTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "OOOABXYZCDOOO⓯",
    &[
        "O",
        "O@418,0",
        "O@836,0",
        "D@1254,0",
        "C@2084,0",
        "X@2914,0",
        "Y@3744,0",
        "Z@4574,0",
        "B@5404,0",
        "A@6234,0",
        "O@7064,0",
        "O@7482,0",
        "O@7900,0",
        "one_five@8318,0"
    ]
);
shaping_test!(
    morx_20_1,
    "tests/fonts/text-rendering/TestMORXTwenty.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABCDE",
    &["A", "B@639,0", "C.alt@1265,0", "D@2265,0", "E.alt1@2939,0"]
);
shaping_test!(
    morx_20_2,
    "tests/fonts/text-rendering/TestMORXTwenty.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABC",
    &["A", "B@639,0", "C.alt@1265,0"]
);
shaping_test!(
    morx_20_3,
    "tests/fonts/text-rendering/TestMORXTwenty.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABE",
    &["A", "B.alt@639,0", "E.alt1@1639,0"]
);
shaping_test!(
    morx_20_4,
    "tests/fonts/text-rendering/TestMORXTwenty.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "AE",
    &["A.alt", "E.alt1@1000,0"]
);
shaping_test!(
    morx_20_5,
    "tests/fonts/text-rendering/TestMORXTwenty.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "EE",
    &["E", "E@556,0"]
);
shaping_test!(
    morx_20_6,
    "tests/fonts/text-rendering/TestMORXTwenty.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "A",
    &["A.alt"]
);
shaping_test!(
    morx_20_7,
    "tests/fonts/text-rendering/TestMORXTwenty.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "E",
    &["E"]
);
shaping_test!(
    morx_21_1,
    "tests/fonts/text-rendering/TestMORXTwentyone.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABCDE",
    &["A", "B.alt@639,0", "C@1639,0", "D@2235,0", "E@2909,0"]
);
shaping_test!(
    morx_22_1,
    "tests/fonts/text-rendering/TestMORXTwentytwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "A",
    &["C"]
);
shaping_test!(
    morx_23_1,
    "tests/fonts/text-rendering/TestMORXTwentythree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABCDE",
    &["E", "E@556,0", "E@1112,0", "E@1668,0", "E@2224,0"]
);
shaping_test!(
    morx_24_1,
    "tests/fonts/text-rendering/TestMORXTwentyfour.ttf",
    75,
    &[],
    &[],
    "ABCDE"
);
shaping_test!(
    morx_25_1,
    "tests/fonts/text-rendering/TestMORXTwentyfive.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABCDE",
    &[
        "A.alt",
        "B.alt@1000,0",
        "C.alt@2000,0",
        "D.alt@3000,0",
        "E.alt@4000,0"
    ]
);
shaping_test!(
    morx_25_2,
    "tests/fonts/text-rendering/TestMORXTwentyfive.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "EBCDA",
    &["E", "B@556,0", "C@1182,0", "D@1778,0", "A@2452,0"]
);
shaping_test!(
    morx_25_3,
    "tests/fonts/text-rendering/TestMORXTwentyfive.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "CBABC",
    &[
        "C",
        "B@596,0",
        "A.alt@1222,0",
        "B.alt@2222,0",
        "C.alt@3222,0"
    ]
);
shaping_test!(
    morx_25_4,
    "tests/fonts/text-rendering/TestMORXTwentyfive.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABC",
    &["A.alt", "B.alt@1000,0", "C.alt@2000,0"]
);
shaping_test!(
    morx_25_5,
    "tests/fonts/text-rendering/TestMORXTwentyfive.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "CBA",
    &["C", "B@596,0", "A@1222,0"]
);
shaping_test!(
    morx_25_6,
    "tests/fonts/text-rendering/TestMORXTwentyfive.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "AB",
    &["A.alt", "B.alt@1000,0"]
);
shaping_test!(
    morx_25_7,
    "tests/fonts/text-rendering/TestMORXTwentyfive.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "BA",
    &["B", "A@626,0"]
);
shaping_test!(
    morx_25_8,
    "tests/fonts/text-rendering/TestMORXTwentyfive.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "A",
    &["A"]
);
shaping_test!(
    morx_25_9,
    "tests/fonts/text-rendering/TestMORXTwentyfive.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "B",
    &["B"]
);
shaping_test!(
    morx_26_1,
    "tests/fonts/text-rendering/TestMORXTwentysix.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "AB",
    &["A", "B@639,0"]
);
shaping_test!(
    morx_26_2,
    "tests/fonts/text-rendering/TestMORXTwentysix.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "B",
    &["B.alt"]
);
shaping_test!(
    morx_27_1,
    "tests/fonts/text-rendering/TestMORXTwentyseven.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "AEB",
    &["A_E_B"]
);
shaping_test!(
    morx_27_2,
    "tests/fonts/text-rendering/TestMORXTwentyseven.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "AEC",
    &["A_E_C"]
);
shaping_test!(
    morx_27_3,
    "tests/fonts/text-rendering/TestMORXTwentyseven.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "AED",
    &["A_E_D"]
);
shaping_test!(
    morx_28_1,
    "tests/fonts/text-rendering/TestMORXTwentyeight.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "AED",
    &["A_E_D"]
);
shaping_test!(
    morx_28_2,
    "tests/fonts/text-rendering/TestMORXTwentyeight.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "AxED",
    &["A_E_D", "x@1394,0"]
);
shaping_test!(
    morx_28_3,
    "tests/fonts/text-rendering/TestMORXTwentyeight.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "AEyD",
    &["A_E_D", "y@1394,0"]
);
shaping_test!(
    morx_28_4,
    "tests/fonts/text-rendering/TestMORXTwentyeight.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "AxEyD",
    &["A_E_D", "x@1394,0", "y@1923,0"]
);
shaping_test!(
    morx_28_5,
    "tests/fonts/text-rendering/TestMORXTwentyeight.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "AxxxEyyyD",
    &["A_E_D", "x@1394,0", "x@1923,0", "x@2452,0", "y@2981,0", "y@3491,0", "y@4001,0"]
);
shaping_test!(
    morx_29_1,
    "tests/fonts/text-rendering/TestMORXTwentynine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRMMXXMMYYAZZ",
    &[
        "P", "Q@333,0", "R@699,0", "M@1050,0", "M@1880,0", "X@2710,0", "X@3074,0", "M@3438,0",
        "I@4268,0", "N@5098,0", "S@5928,0", "M@6758,0", "Y@7588,0", "Y@7920,0", "A@8252,0",
        "Z@9082,0", "Z@9404,0"
    ]
);
shaping_test!(
    morx_29_2,
    "tests/fonts/text-rendering/TestMORXTwentynine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRMMXXMMYYBZZ",
    &[
        "P", "Q@333,0", "R@699,0", "M@1050,0", "M@1880,0", "X@2710,0", "X@3074,0", "M@3438,0",
        "M@4268,0", "I@5098,0", "N@5928,0", "S@6758,0", "Y@7588,0", "Y@7920,0", "B@8252,0",
        "Z@9082,0", "Z@9404,0"
    ]
);
shaping_test!(
    morx_29_3,
    "tests/fonts/text-rendering/TestMORXTwentynine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRMMXXMMYYCZZ",
    &[
        "P", "Q@333,0", "R@699,0", "M@1050,0", "M@1880,0", "X@2710,0", "X@3074,0", "M@3438,0",
        "M@4268,0", "Y@5098,0", "Y@5430,0", "I@5762,0", "N@6592,0", "S@7422,0", "C@8252,0",
        "Z@9082,0", "Z@9404,0"
    ]
);
shaping_test!(
    morx_29_4,
    "tests/fonts/text-rendering/TestMORXTwentynine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRMMXXMMYYDZZ",
    &[
        "P", "Q@333,0", "R@699,0", "M@1050,0", "M@1880,0", "X@2710,0", "X@3074,0", "M@3438,0",
        "M@4268,0", "Y@5098,0", "Y@5430,0", "D@5762,0", "I@6592,0", "N@7422,0", "S@8252,0",
        "Z@9082,0", "Z@9404,0"
    ]
);
shaping_test!(
    morx_3_1,
    "tests/fonts/text-rendering/TestMORXThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABXCD0",
    &[
        "A",
        "B@363,0",
        "X@722,0",
        "C@1086,0",
        "D@1402,0",
        "zero@1793,0"
    ]
);
shaping_test!(
    morx_3_2,
    "tests/fonts/text-rendering/TestMORXThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABXCD1",
    &[
        "A",
        "B@363,0",
        "X@722,0",
        "C@1086,0",
        "D@1402,0",
        "one@1793,0"
    ]
);
shaping_test!(
    morx_3_3,
    "tests/fonts/text-rendering/TestMORXThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABXCD2",
    &[
        "A",
        "B@363,0",
        "X@722,0",
        "C@1086,0",
        "D@1402,0",
        "two@1793,0"
    ]
);
shaping_test!(
    morx_3_4,
    "tests/fonts/text-rendering/TestMORXThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABXCD3",
    &[
        "A",
        "B@363,0",
        "X@722,0",
        "C@1086,0",
        "D@1402,0",
        "three@1793,0"
    ]
);
shaping_test!(
    morx_3_5,
    "tests/fonts/text-rendering/TestMORXThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABXCD4",
    &[
        "A",
        "B@363,0",
        "X@722,0",
        "C@1086,0",
        "D@1402,0",
        "four@1793,0"
    ]
);
shaping_test!(
    morx_3_6,
    "tests/fonts/text-rendering/TestMORXThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABXCD5",
    &[
        "A",
        "B@363,0",
        "X@722,0",
        "C@1086,0",
        "D@1402,0",
        "five@1793,0"
    ]
);
shaping_test!(
    morx_3_7,
    "tests/fonts/text-rendering/TestMORXThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABXCD6",
    &[
        "A",
        "B@363,0",
        "X@722,0",
        "C@1086,0",
        "D@1402,0",
        "six@1793,0"
    ]
);
shaping_test!(
    morx_3_8,
    "tests/fonts/text-rendering/TestMORXThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABXCD7",
    &[
        "A",
        "B@363,0",
        "X@722,0",
        "C@1086,0",
        "D@1402,0",
        "seven@1793,0"
    ]
);
shaping_test!(
    morx_3_9,
    "tests/fonts/text-rendering/TestMORXThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABXCD8",
    &[
        "A",
        "B@363,0",
        "X@722,0",
        "C@1086,0",
        "D@1402,0",
        "eight@1793,0"
    ]
);
shaping_test!(
    morx_3_10,
    "tests/fonts/text-rendering/TestMORXThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABXCD9",
    &[
        "A",
        "B@363,0",
        "X@722,0",
        "C@1086,0",
        "D@1402,0",
        "nine@1793,0"
    ]
);
shaping_test!(
    morx_3_11,
    "tests/fonts/text-rendering/TestMORXThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABXCD➓",
    &[
        "A",
        "B@363,0",
        "X@722,0",
        "C@1086,0",
        "D@1402,0",
        "one_zero@1793,0"
    ]
);
shaping_test!(
    morx_3_12,
    "tests/fonts/text-rendering/TestMORXThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABXCD⓫",
    &[
        "A",
        "B@363,0",
        "X@722,0",
        "C@1086,0",
        "D@1402,0",
        "one_one@1793,0"
    ]
);
shaping_test!(
    morx_3_13,
    "tests/fonts/text-rendering/TestMORXThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABXCD⓬",
    &[
        "A",
        "B@363,0",
        "X@722,0",
        "C@1086,0",
        "D@1402,0",
        "one_two@1793,0"
    ]
);
shaping_test!(
    morx_3_14,
    "tests/fonts/text-rendering/TestMORXThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABXCD⓭",
    &[
        "A",
        "B@363,0",
        "X@722,0",
        "C@1086,0",
        "D@1402,0",
        "one_three@1793,0"
    ]
);
shaping_test!(
    morx_3_15,
    "tests/fonts/text-rendering/TestMORXThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABXCD⓮",
    &[
        "A",
        "B@363,0",
        "X@722,0",
        "C@1086,0",
        "D@1402,0",
        "one_four@1793,0"
    ]
);
shaping_test!(
    morx_3_16,
    "tests/fonts/text-rendering/TestMORXThree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABXCD⓯",
    &[
        "A",
        "B@363,0",
        "X@722,0",
        "C@1086,0",
        "D@1402,0",
        "one_five@1793,0"
    ]
);
shaping_test!(
    morx_30_1,
    "tests/fonts/text-rendering/TestMORXTwentynine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRMMXXXAYYAZZ",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "M@1050,0",
        "I@1880,0",
        "N@2710,0",
        "S@3540,0",
        "I@4370,0",
        "N@5200,0",
        "S@6030,0",
        "M@6860,0",
        "X@7690,0",
        "X@8054,0",
        "X@8418,0",
        "A@8782,0",
        "Y@9612,0",
        "Y@9944,0",
        "A@10276,0",
        "Z@11106,0",
        "Z@11428,0"
    ]
);
shaping_test!(
    morx_30_2,
    "tests/fonts/text-rendering/TestMORXTwentynine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRMMXXXAYYBZZ",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "M@1050,0",
        "I@1880,0",
        "I@2710,0",
        "N@3540,0",
        "S@4370,0",
        "N@5200,0",
        "S@6030,0",
        "M@6860,0",
        "X@7690,0",
        "X@8054,0",
        "X@8418,0",
        "A@8782,0",
        "Y@9612,0",
        "Y@9944,0",
        "B@10276,0",
        "Z@11106,0",
        "Z@11428,0"
    ]
);
shaping_test!(
    morx_30_3,
    "tests/fonts/text-rendering/TestMORXTwentynine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRMMXXXBYYAZZ",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "M@1050,0",
        "I@1880,0",
        "N@2710,0",
        "S@3540,0",
        "M@4370,0",
        "I@5200,0",
        "N@6030,0",
        "S@6860,0",
        "X@7690,0",
        "X@8054,0",
        "X@8418,0",
        "B@8782,0",
        "Y@9612,0",
        "Y@9944,0",
        "A@10276,0",
        "Z@11106,0",
        "Z@11428,0"
    ]
);
shaping_test!(
    morx_30_4,
    "tests/fonts/text-rendering/TestMORXTwentynine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRMMXXXBYYBZZ",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "M@1050,0",
        "M@1880,0",
        "I@2710,0",
        "N@3540,0",
        "S@4370,0",
        "I@5200,0",
        "N@6030,0",
        "S@6860,0",
        "X@7690,0",
        "X@8054,0",
        "X@8418,0",
        "B@8782,0",
        "Y@9612,0",
        "Y@9944,0",
        "B@10276,0",
        "Z@11106,0",
        "Z@11428,0"
    ]
);
shaping_test!(
    morx_31_1,
    "tests/fonts/text-rendering/TestMORXThirtyone.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "XXAYYAZZ",
    &[
        "I", "N@830,0", "I@1660,0", "N@2490,0", "S@3320,0", "S@4150,0", "X@4980,0", "X@5344,0",
        "A@5708,0", "Y@6538,0", "Y@6870,0", "A@7202,0", "Z@8032,0", "Z@8354,0"
    ]
);
shaping_test!(
    morx_31_2,
    "tests/fonts/text-rendering/TestMORXThirtyone.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "XXAYYBYY",
    &[
        "I", "N@830,0", "S@1660,0", "I@2490,0", "N@3320,0", "S@4150,0", "X@4980,0", "X@5344,0",
        "A@5708,0", "Y@6538,0", "Y@6870,0", "B@7202,0", "Y@8032,0", "Y@8364,0"
    ]
);
shaping_test!(
    morx_31_3,
    "tests/fonts/text-rendering/TestMORXThirtyone.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "XXBYYAZZ",
    &[
        "X", "I@364,0", "I@1194,0", "N@2024,0", "S@2854,0", "N@3684,0", "S@4514,0", "X@5344,0",
        "B@5708,0", "Y@6538,0", "Y@6870,0", "A@7202,0", "Z@8032,0", "Z@8354,0"
    ]
);
shaping_test!(
    morx_31_4,
    "tests/fonts/text-rendering/TestMORXThirtyone.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "XXBYYBZZ",
    &[
        "X", "I@364,0", "N@1194,0", "I@2024,0", "N@2854,0", "S@3684,0", "S@4514,0", "X@5344,0",
        "B@5708,0", "Y@6538,0", "Y@6870,0", "B@7202,0", "Z@8032,0", "Z@8354,0"
    ]
);
shaping_test!(
    morx_31_5,
    "tests/fonts/text-rendering/TestMORXThirtyone.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "MPQRAXYZA",
    &[
        "I", "N@830,0", "S@1660,0", "M@2490,0", "I@3320,0", "N@4150,0", "S@4980,0", "P@5810,0",
        "Q@6143,0", "R@6509,0", "A@6860,0", "X@7690,0", "Y@8054,0", "Z@8386,0", "A@8708,0"
    ]
);
shaping_test!(
    morx_31_6,
    "tests/fonts/text-rendering/TestMORXThirtyone.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "MPQRAXYZB",
    &[
        "I", "N@830,0", "S@1660,0", "M@2490,0", "P@3320,0", "I@3653,0", "N@4483,0", "S@5313,0",
        "Q@6143,0", "R@6509,0", "A@6860,0", "X@7690,0", "Y@8054,0", "Z@8386,0", "B@8708,0"
    ]
);
shaping_test!(
    morx_31_7,
    "tests/fonts/text-rendering/TestMORXThirtyone.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "MPQRBXYZA",
    &[
        "M", "I@830,0", "N@1660,0", "S@2490,0", "I@3320,0", "N@4150,0", "S@4980,0", "P@5810,0",
        "Q@6143,0", "R@6509,0", "B@6860,0", "X@7690,0", "Y@8054,0", "Z@8386,0", "A@8708,0"
    ]
);
shaping_test!(
    morx_31_8,
    "tests/fonts/text-rendering/TestMORXThirtyone.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "MPQRBXYZB",
    &[
        "M", "I@830,0", "N@1660,0", "S@2490,0", "P@3320,0", "I@3653,0", "N@4483,0", "S@5313,0",
        "Q@6143,0", "R@6509,0", "B@6860,0", "X@7690,0", "Y@8054,0", "Z@8386,0", "B@8708,0"
    ]
);
shaping_test!(
    morx_32_1,
    "tests/fonts/text-rendering/TestMORXThirtytwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "A",
    &["I", "N@830,0", "S@1660,0", "A@2490,0"]
);
shaping_test!(
    morx_32_2,
    "tests/fonts/text-rendering/TestMORXThirtytwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "XAY",
    &["I", "N@830,0", "S@1660,0", "X@2490,0", "A@2854,0", "Y@3684,0"]
);
shaping_test!(
    morx_32_3,
    "tests/fonts/text-rendering/TestMORXThirtytwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "B",
    &["B", "I@830,0", "N@1660,0", "S@2490,0"]
);
shaping_test!(
    morx_32_4,
    "tests/fonts/text-rendering/TestMORXThirtytwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "XBY",
    &["X", "I@364,0", "N@1194,0", "S@2024,0", "B@2854,0", "Y@3684,0"]
);
shaping_test!(
    morx_33_1,
    "tests/fonts/text-rendering/TestMORXThirtythree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ha",
    &["h", "a@618,0", "h@1179,0", "a@1797,0"]
);
shaping_test!(
    morx_33_2,
    "tests/fonts/text-rendering/TestMORXThirtythree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "haha",
    &["h", "a@618,0", "h@1179,0", "a@1797,0", "h@2358,0", "a@2976,0", "h@3537,0", "a@4155,0"]
);
shaping_test!(
    morx_33_3,
    "tests/fonts/text-rendering/TestMORXThirtythree.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ah",
    &["a", "h@561,0"]
);
shaping_test!(
    morx_34_1,
    "tests/fonts/text-rendering/TestMORXThirtyfour.ttf",
    75,
    &[],
    &[],
    "ha"
);
shaping_test!(
    morx_35_1,
    "tests/fonts/text-rendering/TestMORXThirtyfive.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "A",
    &["A", "B@639,0", "C@1265,0", "E@1861,0"]
);
shaping_test!(
    morx_35_2,
    "tests/fonts/text-rendering/TestMORXThirtyfive.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "XAY",
    &["X", "A@586,0", "B@1225,0", "C@1851,0", "E@2447,0", "Y@3003,0"]
);
shaping_test!(
    morx_36_1,
    "tests/fonts/text-rendering/TestMORXThirtysix.ttf",
    75,
    &[],
    &[],
    "A"
);
shaping_test!(
    morx_37_1,
    "tests/fonts/text-rendering/TestMORXThirtyseven.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "AB",
    &["A.alt", "B.alt@1000,0"]
);
shaping_test!(
    morx_37_2,
    "tests/fonts/text-rendering/TestMORXThirtyseven.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "BA",
    &["B", "A@650,0"]
);
shaping_test!(
    morx_37_3,
    "tests/fonts/text-rendering/TestMORXThirtyseven.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "אב",
    &["uni05D1", "uni05D0@542,0"]
);
shaping_test!(
    morx_37_4,
    "tests/fonts/text-rendering/TestMORXThirtyseven.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "בא",
    &["uni05D0.alt", "uni05D1.alt@1000,0"]
);
shaping_test!(
    morx_38_1,
    "tests/fonts/text-rendering/TestMORXThirtyeight.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "AB",
    &["A.alt", "B.alt@1000,0"]
);
shaping_test!(
    morx_38_2,
    "tests/fonts/text-rendering/TestMORXThirtyeight.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "BA",
    &["B", "A@650,0"]
);
shaping_test!(
    morx_38_3,
    "tests/fonts/text-rendering/TestMORXThirtyeight.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "אב",
    &["uni05D1.alt", "uni05D0.alt@1000,0"]
);
shaping_test!(
    morx_38_4,
    "tests/fonts/text-rendering/TestMORXThirtyeight.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "בא",
    &["uni05D0", "uni05D1@606,0"]
);
shaping_test!(
    morx_39_1,
    "tests/fonts/text-rendering/TestMORXThirtynine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "AB",
    &["A", "B@639,0"]
);
shaping_test!(
    morx_39_2,
    "tests/fonts/text-rendering/TestMORXThirtynine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "BA",
    &["B.alt", "A.alt@1000,0"]
);
shaping_test!(
    morx_39_3,
    "tests/fonts/text-rendering/TestMORXThirtynine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "אב",
    &["uni05D1.alt", "uni05D0.alt@1000,0"]
);
shaping_test!(
    morx_39_4,
    "tests/fonts/text-rendering/TestMORXThirtynine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "בא",
    &["uni05D0", "uni05D1@606,0"]
);
shaping_test!(
    morx_4_1,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRAXYZ1",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "X@1880,0",
        "Y@2244,0",
        "Z@2576,0",
        "one@2898,0"
    ]
);
shaping_test!(
    morx_4_2,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRAXYZ2",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "X@1880,0",
        "Y@2244,0",
        "Z@2576,0",
        "two@2898,0"
    ]
);
shaping_test!(
    morx_4_3,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRADXYZ3",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "D@1050,0",
        "A@1880,0",
        "X@2710,0",
        "Y@3074,0",
        "Z@3406,0",
        "three@3728,0"
    ]
);
shaping_test!(
    morx_4_4,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABXYZ4",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "B@1880,0",
        "X@2710,0",
        "Y@3074,0",
        "Z@3406,0",
        "four@3728,0"
    ]
);
shaping_test!(
    morx_4_5,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABXYZ5",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "B@1050,0",
        "A@1880,0",
        "X@2710,0",
        "Y@3074,0",
        "Z@3406,0",
        "five@3728,0"
    ]
);
shaping_test!(
    morx_4_6,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABXYZ6",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "B@1880,0",
        "X@2710,0",
        "Y@3074,0",
        "Z@3406,0",
        "six@3728,0"
    ]
);
shaping_test!(
    morx_4_7,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABXYZ7",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "B@1050,0",
        "A@1880,0",
        "X@2710,0",
        "Y@3074,0",
        "Z@3406,0",
        "seven@3728,0"
    ]
);
shaping_test!(
    morx_4_8,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRACDXYZ8",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "C@1050,0",
        "D@1880,0",
        "A@2710,0",
        "X@3540,0",
        "Y@3904,0",
        "Z@4236,0",
        "eight@4558,0"
    ]
);
shaping_test!(
    morx_4_9,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRACDXYZ9",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "D@1050,0",
        "C@1880,0",
        "A@2710,0",
        "X@3540,0",
        "Y@3904,0",
        "Z@4236,0",
        "nine@4558,0"
    ]
);
shaping_test!(
    morx_4_10,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABDXYZ➓",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "D@1050,0",
        "A@1880,0",
        "B@2710,0",
        "X@3540,0",
        "Y@3904,0",
        "Z@4236,0",
        "one_zero@4558,0"
    ]
);
shaping_test!(
    morx_4_11,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABDXYZ⓫",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "D@1050,0",
        "B@1880,0",
        "A@2710,0",
        "X@3540,0",
        "Y@3904,0",
        "Z@4236,0",
        "one_one@4558,0"
    ]
);
shaping_test!(
    morx_4_12,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABCDXYZ⓬",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "C@1050,0",
        "D@1880,0",
        "A@2710,0",
        "B@3540,0",
        "X@4370,0",
        "Y@4734,0",
        "Z@5066,0",
        "one_two@5388,0"
    ]
);
shaping_test!(
    morx_4_13,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABCDXYZ⓭",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "C@1050,0",
        "D@1880,0",
        "B@2710,0",
        "A@3540,0",
        "X@4370,0",
        "Y@4734,0",
        "Z@5066,0",
        "one_three@5388,0"
    ]
);
shaping_test!(
    morx_4_14,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABCDXYZ⓮",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "D@1050,0",
        "C@1880,0",
        "A@2710,0",
        "B@3540,0",
        "X@4370,0",
        "Y@4734,0",
        "Z@5066,0",
        "one_four@5388,0"
    ]
);
shaping_test!(
    morx_4_15,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABCDXYZ⓯",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "D@1050,0",
        "C@1880,0",
        "B@2710,0",
        "A@3540,0",
        "X@4370,0",
        "Y@4734,0",
        "Z@5066,0",
        "one_five@5388,0"
    ]
);
shaping_test!(
    morx_40_1,
    "tests/fonts/text-rendering/TestMORXForty.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "AB",
    &["A", "B@639,0"]
);
shaping_test!(
    morx_40_2,
    "tests/fonts/text-rendering/TestMORXForty.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "BA",
    &["B.alt", "A.alt@1000,0"]
);
shaping_test!(
    morx_40_3,
    "tests/fonts/text-rendering/TestMORXForty.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "אב",
    &["uni05D1", "uni05D0@542,0"]
);
shaping_test!(
    morx_40_4,
    "tests/fonts/text-rendering/TestMORXForty.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "בא",
    &["uni05D0.alt", "uni05D1.alt@1000,0"]
);
shaping_test!(
    morx_41_1,
    "tests/fonts/text-rendering/TestMORXFourtyone.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ac",
    &["a_c"]
);
shaping_test!(
    morx_41_2,
    "tests/fonts/text-rendering/TestMORXFourtyone.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "bc",
    &["b_c"]
);
shaping_test!(
    morx_41_3,
    "tests/fonts/text-rendering/TestMORXFourtyone.ttf",
    75,
    &[],
    &[],
    "cc"
);
shaping_test!(
    morx_41_4,
    "tests/fonts/text-rendering/TestMORXFourtyone.ttf",
    75,
    &[],
    &[],
    "abcc"
);
shaping_test!(
    morx_5_1,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRAXYZ3",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "X@1880,0",
        "Y@2244,0",
        "Z@2576,0",
        "three@2898,0"
    ]
);
shaping_test!(
    morx_5_2,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRAXYZ4",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "X@1880,0",
        "Y@2244,0",
        "Z@2576,0",
        "four@2898,0"
    ]
);
shaping_test!(
    morx_5_3,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRAXYZ5",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "X@1880,0",
        "Y@2244,0",
        "Z@2576,0",
        "five@2898,0"
    ]
);
shaping_test!(
    morx_5_4,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRAXYZ6",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "X@1880,0",
        "Y@2244,0",
        "Z@2576,0",
        "six@2898,0"
    ]
);
shaping_test!(
    morx_5_5,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRAXYZ7",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "X@1880,0",
        "Y@2244,0",
        "Z@2576,0",
        "seven@2898,0"
    ]
);
shaping_test!(
    morx_5_6,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRAXYZ8",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "X@1880,0",
        "Y@2244,0",
        "Z@2576,0",
        "eight@2898,0"
    ]
);
shaping_test!(
    morx_5_7,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABXYZ8",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "B@1880,0",
        "X@2710,0",
        "Y@3074,0",
        "Z@3406,0",
        "eight@3728,0"
    ]
);
shaping_test!(
    morx_5_8,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRAXYZ9",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "X@1880,0",
        "Y@2244,0",
        "Z@2576,0",
        "nine@2898,0"
    ]
);
shaping_test!(
    morx_5_9,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABXYZ9",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "B@1880,0",
        "X@2710,0",
        "Y@3074,0",
        "Z@3406,0",
        "nine@3728,0"
    ]
);
shaping_test!(
    morx_5_10,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRAXYZ➓",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "X@1880,0",
        "Y@2244,0",
        "Z@2576,0",
        "one_zero@2898,0"
    ]
);
shaping_test!(
    morx_5_11,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABXYZ➓",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "B@1880,0",
        "X@2710,0",
        "Y@3074,0",
        "Z@3406,0",
        "one_zero@3728,0"
    ]
);
shaping_test!(
    morx_5_12,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRAXYZ⓫",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "X@1880,0",
        "Y@2244,0",
        "Z@2576,0",
        "one_one@2898,0"
    ]
);
shaping_test!(
    morx_5_13,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABXYZ⓫",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "B@1880,0",
        "X@2710,0",
        "Y@3074,0",
        "Z@3406,0",
        "one_one@3728,0"
    ]
);
shaping_test!(
    morx_5_14,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRAXYZ⓬",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "X@1880,0",
        "Y@2244,0",
        "Z@2576,0",
        "one_two@2898,0"
    ]
);
shaping_test!(
    morx_5_15,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABXYZ⓬",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "B@1880,0",
        "X@2710,0",
        "Y@3074,0",
        "Z@3406,0",
        "one_two@3728,0"
    ]
);
shaping_test!(
    morx_5_16,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABCXYZ⓬",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "B@1880,0",
        "C@2710,0",
        "X@3540,0",
        "Y@3904,0",
        "Z@4236,0",
        "one_two@4558,0"
    ]
);
shaping_test!(
    morx_5_17,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRAXYZ⓭",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "X@1880,0",
        "Y@2244,0",
        "Z@2576,0",
        "one_three@2898,0"
    ]
);
shaping_test!(
    morx_5_18,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABXYZ⓭",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "B@1880,0",
        "X@2710,0",
        "Y@3074,0",
        "Z@3406,0",
        "one_three@3728,0"
    ]
);
shaping_test!(
    morx_5_19,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABCXYZ⓭",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "B@1880,0",
        "C@2710,0",
        "X@3540,0",
        "Y@3904,0",
        "Z@4236,0",
        "one_three@4558,0"
    ]
);
shaping_test!(
    morx_5_20,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRAXYZ⓮",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "X@1880,0",
        "Y@2244,0",
        "Z@2576,0",
        "one_four@2898,0"
    ]
);
shaping_test!(
    morx_5_21,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABXYZ⓮",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "B@1880,0",
        "X@2710,0",
        "Y@3074,0",
        "Z@3406,0",
        "one_four@3728,0"
    ]
);
shaping_test!(
    morx_5_22,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABCXYZ⓮",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "B@1880,0",
        "C@2710,0",
        "X@3540,0",
        "Y@3904,0",
        "Z@4236,0",
        "one_four@4558,0"
    ]
);
shaping_test!(
    morx_5_23,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRAXYZ⓯",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "X@1880,0",
        "Y@2244,0",
        "Z@2576,0",
        "one_five@2898,0"
    ]
);
shaping_test!(
    morx_5_24,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABXYZ⓯",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "B@1880,0",
        "X@2710,0",
        "Y@3074,0",
        "Z@3406,0",
        "one_five@3728,0"
    ]
);
shaping_test!(
    morx_5_25,
    "tests/fonts/text-rendering/TestMORXFour.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "PQRABCXYZ⓯",
    &[
        "P",
        "Q@333,0",
        "R@699,0",
        "A@1050,0",
        "B@1880,0",
        "C@2710,0",
        "X@3540,0",
        "Y@3904,0",
        "Z@4236,0",
        "one_five@4558,0"
    ]
);
shaping_test!(
    morx_6_1,
    "tests/fonts/text-rendering/TestMORXTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "OOOABCDEFGOOO3141",
    &[
        "O",
        "O@418,0",
        "O@836,0",
        "E@1254,0",
        "F@2084,0",
        "A@2914,0",
        "G@3744,0",
        "B@4574,0",
        "C@5404,0",
        "D@6234,0",
        "O@7064,0",
        "O@7482,0",
        "O@7900,0",
        "three@8318,0",
        "one@9168,0",
        "four@10018,0",
        "one@10868,0"
    ]
);
shaping_test!(
    morx_7_1,
    "tests/fonts/text-rendering/TestMORXTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "OBCD1",
    &["B", "C@830,0", "D@1660,0", "O@2490,0", "one@2908,0"]
);
shaping_test!(
    morx_8_1,
    "tests/fonts/text-rendering/TestMORXEight.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "0ABC",
    &["zero", "A@914,0", "B@1552,0", "C@2202,0"]
);
shaping_test!(
    morx_8_2,
    "tests/fonts/text-rendering/TestMORXEight.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "1ABC",
    &["one", "B@914,0", "C@1564,0", "A@2196,0"]
);
shaping_test!(
    morx_8_3,
    "tests/fonts/text-rendering/TestMORXEight.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "2ABC",
    &["two", "C@914,0", "A@1546,0", "B@2184,0"]
);
shaping_test!(
    morx_9_1,
    "tests/fonts/text-rendering/TestMORXNine.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ABXAB",
    &["B", "A@650,0", "X@1288,0", "A@1874,0", "B@2512,0"]
);
shaping_test!(
    sfnt_1_1,
    "tests/fonts/text-rendering/TestSFNTOne.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "A",
    &["A"]
);
shaping_test!(
    sfnt_1_2,
    "tests/fonts/text-rendering/TestSFNTOne.otf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "B",
    &["B"]
);
shaping_test!(
    sfnt_2_1,
    "tests/fonts/text-rendering/TestSFNTTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "A",
    &["A"]
);
shaping_test!(
    sfnt_2_2,
    "tests/fonts/text-rendering/TestSFNTTwo.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "B",
    &["B"]
);
shaping_test!(
    sharan_1_1,
    "tests/fonts/text-rendering/TestShapeAran.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "لسان",
    &[
        "OneDotEnclNS@398,-1",
        "NoonxSep",
        "AlefFin@861,0",
        "SeenMed.inT2outT1@1125,0",
        "sp0@1664,0",
        "LamIni.outT2@1664,223"
    ]
);
shaping_test!(
    sharan_1_2,
    "tests/fonts/text-rendering/TestShapeAran.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "یونیکوڈ",
    &[
        "TahSmallNS@118,-213",
        "DalSep",
        "WawFin.cut@300,0",
        "KafMed.outT3@573,206",
        "TwoDotsBelowNS@1115,220",
        "BehxMed.inT2outT1@903,304",
        "OneDotAboveNS@1271,-71",
        "sp1@1170,0",
        "BehxIni.outT2@1170,449",
        "WawFin.inD2@1387,0",
        "TwoDotsBelowNS@1867,1",
        "sp0@1758,0",
        "BehxIni.outD2WQ@1758,323"
    ]
);
shaping_test!(
    sharan_1_3,
    "tests/fonts/text-rendering/TestShapeAran.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "فونٹ",
    &[
        "TahSmallNS@595,-331",
        "BehxFin.soft",
        "OneDotAboveNS@1163,-182",
        "sp0@1184,0",
        "BehxIni.outT2B@1184,300",
        "WawFin.inD2alt@1340,0",
        "OneDotAboveNS@1784,108",
        "sp0@1599,0",
        "FehxIni.outD2WQ@1599,237"
    ]
);
shaping_test!(
    sharan_1_4,
    "tests/fonts/text-rendering/TestShapeAran.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ٹائپ فیس",
    &[
        "SeenFin",
        "TwoDotsBelowNS@1216,269",
        "BehxMed.inT1outT2SeenWide@1041,455",
        "OneDotAboveNS@1454,224",
        "sp0@1271,0",
        "FehxIni@1271,490",
        "space@1584,0",
        "ThreeDotsDownBelowNS@2290,-159",
        "BehxFin.soft@1715,0",
        "HamzaAboveNS@2878,-201",
        "sp0@2899,0",
        "BehxIni.outT2B@2899,300",
        "AlefFin.narrow@3056,0",
        "TahSmallNS@3442,-420",
        "sp0@3295,0",
        "BehxIni.A@3295,0"
    ]
);
shaping_test!(
    sharan_1_5,
    "tests/fonts/text-rendering/TestShapeAran.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "فن خطاطی",
    &[
        "YehxFin",
        "sp0@521,0",
        "TahIni.outD2@521,380",
        "AlefFin@1119,0",
        "TahMed.inD1outT1@1382,0",
        "OneDotAboveNS@2081,-47",
        "sp0@1451,0",
        "HahIni.outD1@1451,36",
        "space@2326,0",
        "OneDotEnclNS@2855,-2",
        "NoonxFin@2458,0",
        "OneDotAboveNS@3361,188",
        "sp0@3208,0",
        "FehxIni.outT2N@3208,336"
    ]
);
shaping_test!(
    sharan_1_6,
    "tests/fonts/text-rendering/TestShapeAran.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "نستعلیق",
    &[
        "TwoDotsAboveNS@519,-199",
        "QafxFin.cut",
        "TwoDotsBelowNS@977,141",
        "BehxMed.inT2outD2WQ@692,272",
        "LamMed.outT2@1023,434",
        "AinMed.inT3outT1@1301,507",
        "TwoDotsAboveNS@1785,209",
        "BehxMed.inT2outT3@1563,603",
        "SeenMed.inT2outT2@1865,735",
        "OneDotAboveNS@2574,670",
        "sp0@2434,0",
        "BehxIni.outT2tall@2434,952"
    ]
);
shaping_test!(
    shbali_1_1,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓ\u{1b38}\u{1b00}",
    &["gid23", "gid60@1113,0", "gid4@1064,0"]
);
shaping_test!(
    shbali_1_2,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬕ᭄ᬖ\u{1b02}",
    &["gid25", "gid132@1092,0", "gid6@942,0"]
);
shaping_test!(
    shbali_1_3,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬘᬻ",
    &["gid28", "gid62@796,0", "gid57@794,0"]
);
shaping_test!(
    shbali_1_4,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬙᭀ",
    &["gid66", "gid29@483,0", "gid57@1536,0"]
);
shaping_test!(
    shbali_1_5,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬚᬿ",
    &["gid67", "gid30@483,0"]
);
shaping_test!(
    shbali_1_6,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬔ\u{1b36}",
    &["gid24", "gid58@828,0"]
);
shaping_test!(
    shbali_1_7,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓ᭄ᬓ\u{1b01}",
    &["gid23", "gid129@1111,0", "gid5@1064,0"]
);
shaping_test!(
    shbali_1_8,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓ᭄ᬛ\u{1b01}",
    &["gid23", "gid137@1111,0", "gid5@1379,181"]
);
shaping_test!(
    shbali_1_9,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓ᭄ᬦ\u{1b03}",
    &["gid23", "gid148@1111,0", "gid7@991,0"]
);
shaping_test!(
    shbali_1_10,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓ᭄ᬓ\u{1b38}",
    &["gid23", "gid129@1111,0", "gid60@1111,-488"]
);
shaping_test!(
    shbali_1_11,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓ᭄ᬓ\u{1b3c}",
    &["gid23", "gid129@1111,0", "gid70@1128,0", "gid170@1113,0"]
);
shaping_test!(
    shbali_1_12,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓ᭄ᬓᬽ",
    &[
        "gid23",
        "gid129@1111,0",
        "gid70@1128,0",
        "gid170@1113,0",
        "gid57@1111,0"
    ]
);
shaping_test!(
    shbali_1_13,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓᬾ",
    &["gid66", "gid23@483,0"]
);
shaping_test!(
    shbali_1_14,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓ\u{1b36}ᬾ",
    &["gid23", "gid58@1064,0", "gid66@1111,0", "gid128@1594,0"]
);
shaping_test!(
    shbali_1_15,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓ\u{1b38}ᬾ",
    &["gid23", "gid60@1113,0", "gid66@1111,0", "gid128@1594,0"]
);
shaping_test!(
    shbali_1_16,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓ᭄ᬕᬾ",
    &["gid66", "gid23@483,0", "gid131@1594,0"]
);
shaping_test!(
    shbali_1_17,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓᭀ",
    &["gid66", "gid23@483,0", "gid57@1594,0"]
);
shaping_test!(
    shbali_1_18,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓᬾ",
    &["gid66", "gid23@483,0"]
);
shaping_test!(
    shbali_1_19,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓᬾ\u{1b36}",
    &["gid66", "gid23@483,0", "gid58@1548,0"]
);
shaping_test!(
    shbali_1_20,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓᬾ\u{1b38}",
    &["gid66", "gid23@483,0", "gid60@1597,0"]
);
shaping_test!(
    shbali_1_21,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓ᭄ᬕᬾ",
    &["gid66", "gid23@483,0", "gid131@1594,0"]
);
shaping_test!(
    shbali_1_22,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓᭀ",
    &["gid66", "gid23@483,0", "gid57@1594,0"]
);
shaping_test!(
    shbali_2_1,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓ᭄ᬧᬾ",
    &["gid66", "gid23@483,0", "gid149@1594,0"]
);
shaping_test!(
    shbali_2_2,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓ᭄ᬨᬿ",
    &["gid67", "gid23@483,0", "gid150@1594,0"]
);
shaping_test!(
    shbali_2_3,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓ᭄ᬱᬾ",
    &["gid66", "gid23@483,0", "gid159@1594,0"]
);
shaping_test!(
    shbali_2_4,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓ᭄ᬲᬾ",
    &["gid66", "gid23@483,0", "gid60@1597,0", "gid149@1594,0"]
);
shaping_test!(
    shbali_2_5,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓ᭄ᭊᬾ",
    &["gid66", "gid23@483,0", "gid60@1597,0", "gid165@1594,0"]
);
shaping_test!(
    shbali_2_6,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬛ᭄ᬓ",
    &["gid181", "gid129@1064,-195"]
);
shaping_test!(
    shbali_2_7,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬛ᭄ᬓᬾ",
    &["gid66", "gid181@483,0", "gid129@1548,-195"]
);
shaping_test!(
    shbali_2_8,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬛ᭄ᬓ\u{1b38}\u{1b00}",
    &[
        "gid181",
        "gid129@1064,-195",
        "gid60@1064,-684",
        "gid4@855,0"
    ]
);
shaping_test!(
    shbali_2_9,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓ᭄ᬓ\u{1b38}",
    &["gid23", "gid129@1111,0", "gid60@1111,-488"]
);
shaping_test!(
    shbali_2_10,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓ᭄ᬛ\u{1b39}",
    &["gid23", "gid137@1111,0", "gid61@1261,-488"]
);
shaping_test!(
    shbali_2_11,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓ᭄ᬱ\u{1b3a}",
    &["gid23", "gid159@1111,0", "gid62@1753,0"]
);
shaping_test!(
    shbali_2_12,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᬓ᭄ᭅ\u{1b38}",
    &["gid23", "gid162@1111,0", "gid60@1111,-488"]
);
shaping_test!(
    shbali_3_1,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "᭦\u{1b6b}",
    &["gid102", "gid107@560,-10"]
);
shaping_test!(
    shbali_3_2,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "᭦\u{1b6c}",
    &["gid102", "gid108@573,49"]
);
shaping_test!(
    shbali_3_3,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "᭦\u{1b6d}",
    &["gid102", "gid109@652,-10"]
);
shaping_test!(
    shbali_3_4,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "᭦\u{1b6e}",
    &["gid102", "gid110@652,-98"]
);
shaping_test!(
    shbali_3_5,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "᭦\u{1b6f}",
    &["gid102", "gid111@667,-10"]
);
shaping_test!(
    shbali_3_6,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "᭦\u{1b70}",
    &["gid102", "gid112@667,-10"]
);
shaping_test!(
    shbali_3_7,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "᭦\u{1b71}",
    &["gid102", "gid113@667,-10"]
);
shaping_test!(
    shbali_3_8,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "᭦\u{1b72}",
    &["gid102", "gid114@667,-10"]
);
shaping_test!(
    shbali_3_9,
    "tests/fonts/text-rendering/NotoSansBalinese-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "᭦\u{1b73}",
    &["gid102", "gid115@599,-10"]
);
shaping_test!(
    shknda_1_1,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಲ\u{ccd}ಲ\u{cbf}",
    &["knLI", "knLAc2@757,0"]
);
shaping_test!(
    shknda_1_2,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಟ\u{ccd}ಸ\u{ccd}",
    &["knTT", "knSAc2@1021,0"]
);
shaping_test!(
    shknda_1_3,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಳ\u{cbf}",
    &["knLLI"]
);
shaping_test!(
    shknda_1_4,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಡ\u{cbf}",
    &["knDDI"]
);
shaping_test!(
    shknda_1_5,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಮ\u{cc6}",
    &["knME"]
);
shaping_test!(
    shknda_1_6,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ರ\u{cbf}",
    &["knRI"]
);
shaping_test!(
    shknda_1_7,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಖ\u{ccd}ಯ\u{cc6}",
    &["knKHE", "knYAc2@846,0"]
);
shaping_test!(
    shknda_1_8,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಫ\u{ccd}ರ\u{cbf}",
    &["knPHI", "knRAc2@735,0"]
);
shaping_test!(
    shknda_1_9,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ನ\u{cc6}",
    &["knNE"]
);
shaping_test!(
    shknda_1_10,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಗ\u{cbf}",
    &["knGI"]
);
shaping_test!(
    shknda_1_11,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಷ\u{ccd}ಟ\u{cbf}",
    &["knSSI", "knTTAc2@746,0"]
);
shaping_test!(
    shknda_1_12,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಯ\u{cbf}ಂ",
    &["knYI", "knAnusvara@1252,0"]
);
shaping_test!(
    shknda_1_13,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಚೀ",
    &["knCI", "knLengthmark@766,0"]
);
shaping_test!(
    shknda_1_14,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ನ\u{cbf}",
    &["knNI"]
);
shaping_test!(
    shknda_1_15,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಗ\u{ccd}ಲ\u{cbf}",
    &["knGI", "knLAc2@621,0"]
);
shaping_test!(
    shknda_1_16,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಷ\u{cbf}",
    &["knSSI"]
);
shaping_test!(
    shknda_1_17,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಗ\u{cc6}",
    &["knGE"]
);
shaping_test!(
    shknda_1_18,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ದ\u{ccd}ವ\u{cbf}",
    &["knDI", "knVAc2@740,0"]
);
shaping_test!(
    shknda_1_19,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ತೀ",
    &["knTI", "knLengthmark@613,0"]
);
shaping_test!(
    shknda_1_20,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಮ\u{cbf}",
    &["knMI"]
);
shaping_test!(
    shknda_1_21,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಲ\u{cbf}",
    &["knLI"]
);
shaping_test!(
    shknda_1_22,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಗ\u{cbf}",
    &["knGI"]
);
shaping_test!(
    shknda_1_23,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ನ\u{ccd}",
    &["knN"]
);
shaping_test!(
    shknda_1_24,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಬ\u{cbf}",
    &["knBI"]
);
shaping_test!(
    shknda_1_25,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಲ\u{cbf}",
    &["knLI"]
);
shaping_test!(
    shknda_1_26,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ನ\u{ccd}ನ\u{cbf}ಂ",
    &["knNI", "knNAc2@678,0", "knAnusvara@755,0"]
);
shaping_test!(
    shknda_1_27,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಲ\u{ccd}ಲ\u{cbf}",
    &["knLI", "knLAc2@757,0"]
);
shaping_test!(
    shknda_1_28,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಧ\u{cbf}",
    &["knDHI"]
);
shaping_test!(
    shknda_1_29,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಪ\u{ccc}",
    &["knPA.base", "knmAU@739,0"]
);
shaping_test!(
    shknda_1_30,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ವ\u{cbf}ಂ",
    &["knVI", "knAnusvara@749,0"]
);
shaping_test!(
    shknda_1_31,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಡ\u{cbf}",
    &["knDDI"]
);
shaping_test!(
    shknda_1_32,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಟ\u{cbf}",
    &["knTTI"]
);
shaping_test!(
    shknda_1_33,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ನ\u{cbf}",
    &["knNI"]
);
shaping_test!(
    shknda_1_34,
    "tests/fonts/text-rendering/NotoSerifKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಧ\u{cbf}",
    &["knDHI"]
);
shaping_test!(
    shknda_2_1,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ನ\u{ccd}ನಾ",
    &["gid150", "gid57@711,0", "gid116@1160,0"]
);
shaping_test!(
    shknda_2_2,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ನ\u{ccd}ನಾ",
    &["gid150", "gid57@711,0", "gid116@1160,0"]
);
shaping_test!(
    shknda_2_3,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ತ\u{ccd}ತಾ",
    &["gid146", "gid57@623,0", "gid112@1071,0"]
);
shaping_test!(
    shknda_2_4,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಟ\u{ccd}ಟಾ",
    &["gid141", "gid57@815,0", "gid107@1264,0"]
);
shaping_test!(
    shknda_2_5,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಡೋಂಗ\u{cbf}",
    &[
        "gid249",
        "gid61@768,0",
        "gid71@1513,0",
        "gid4@1925,0",
        "gid207@2475,0"
    ]
);
shaping_test!(
    shknda_2_6,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಜ\u{cbf}\u{cbc}\u{cd5}ಬ\u{cc6}ನ\u{ccd}",
    &[
        "gid211",
        "gid55@652,0",
        "gid71@776,0",
        "gid259@1188,0",
        "gid186@1994,0"
    ]
);
shaping_test!(
    shknda_2_7,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಜಾ\u{cbc}ಕ\u{cbf}ರ\u{ccd}",
    &[
        "gid139",
        "gid57@776,0",
        "gid55@652,0",
        "gid205@1225,0",
        "gid193@1799,0"
    ]
);
shaping_test!(
    shknda_2_8,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಇನ\u{ccd}ಫ\u{ccd}ಲ\u{cc6}ಕ\u{ccd}ಷನಲ\u{ccd}",
    &[
        "gid8",
        "gid256@711,0",
        "gid118@1422,0",
        "gid335@1591,0",
        "gid282@1978,0",
        "gid39@2552,0",
        "gid195@3263,0"
    ]
);
shaping_test!(
    shknda_2_9,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಇನ\u{ccd}ಫ\u{ccd}ಲ\u{cc6}ಕ\u{ccd}ಷನ\u{ccd}",
    &[
        "gid8",
        "gid256@711,0",
        "gid118@1422,0",
        "gid335@1591,0",
        "gid282@1978,0",
        "gid186@2552,0"
    ]
);
shaping_test!(
    shknda_2_10,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ದಟ\u{ccd}ಸ\u{ccd}",
    &["gid37", "gid177@765,0", "gid130@1814,0"]
);
shaping_test!(
    shknda_2_11,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಎಕ\u{ccd}ಸ\u{ccd}",
    &["gid14", "gid167@787,0", "gid130@1596,0"]
);
shaping_test!(
    shknda_2_12,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಮಾರ\u{ccd}ಚ\u{ccd}",
    &["gid155", "gid57@1156,0", "gid172@1605,0", "gid94@2718,0"]
);
shaping_test!(
    shknda_2_13,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಟ\u{cc6}ಕ\u{ccd}ಸ\u{ccd}ಟ\u{ccd}",
    &["gid247", "gid167@815,0", "gid130@1624,0", "gid317@1792,0"]
);
shaping_test!(
    shknda_2_14,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಬುಕ\u{ccd}ಸ\u{ccd}",
    &["gid42", "gid60@801,0", "gid167@1165,0", "gid130@1974,0"]
);
shaping_test!(
    shknda_2_15,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಸಾಫ\u{ccd}ಟ\u{ccd}",
    &["gid163", "gid57@709,0", "gid188@1158,0", "gid107@2184,0"]
);
shaping_test!(
    shknda_2_16,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಜಸ\u{ccd}ಟ\u{ccd}",
    &["gid27", "gid200@776,0", "gid107@1720,0"]
);
shaping_test!(
    shknda_3_1,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಕೋಂ",
    &["gid239", "gid61@574,0", "gid71@1319,0", "gid4@1731,0"]
);
shaping_test!(
    shknda_3_2,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಖೋಂ",
    &["gid240", "gid61@865,0", "gid71@1610,0", "gid4@2022,0"]
);
shaping_test!(
    shknda_3_3,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಗೋಂ",
    &["gid241", "gid61@648,0", "gid71@1393,0", "gid4@1805,0"]
);
shaping_test!(
    shknda_3_4,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಘೋಂ",
    &["gid242", "gid279@997,0", "gid71@1742,0", "gid4@2153,0"]
);
shaping_test!(
    shknda_3_5,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಙೋಂ",
    &["gid24", "gid67@737,0", "gid71@1718,0", "gid4@2130,0"]
);
shaping_test!(
    shknda_3_6,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಚೋಂ",
    &["gid243", "gid61@795,0", "gid71@1540,0", "gid4@1952,0"]
);
shaping_test!(
    shknda_3_7,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಛೋಂ",
    &["gid244", "gid61@843,0", "gid71@1588,0", "gid4@2000,0"]
);
shaping_test!(
    shknda_3_8,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಜೋಂ",
    &["gid245", "gid61@776,0", "gid71@1522,0", "gid4@1933,0"]
);
shaping_test!(
    shknda_3_9,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಝೋಂ",
    &["gid246", "gid61@1379,0", "gid71@2124,0", "gid4@2536,0"]
);
shaping_test!(
    shknda_3_10,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಞೋಂ",
    &["gid29", "gid67@968,0", "gid71@1949,0", "gid4@2360,0"]
);
shaping_test!(
    shknda_3_11,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಟೋಂ",
    &["gid247", "gid61@815,0", "gid71@1560,0", "gid4@1972,0"]
);
shaping_test!(
    shknda_3_12,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಠೋಂ",
    &["gid248", "gid61@651,0", "gid71@1397,0", "gid4@1808,0"]
);
shaping_test!(
    shknda_3_13,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಡೋಂ",
    &["gid249", "gid61@768,0", "gid71@1513,0", "gid4@1925,0"]
);
shaping_test!(
    shknda_3_14,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಢೋಂ",
    &["gid250", "gid61@768,0", "gid71@1513,0", "gid4@1925,0"]
);
shaping_test!(
    shknda_3_15,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಣೋಂ",
    &["gid251", "gid61@867,0", "gid71@1612,0", "gid4@2023,0"]
);
shaping_test!(
    shknda_3_16,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ತೋಂ",
    &["gid252", "gid61@623,0", "gid71@1368,0", "gid4@1779,0"]
);
shaping_test!(
    shknda_3_17,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಥೋಂ",
    &["gid253", "gid61@765,0", "gid71@1510,0", "gid4@1921,0"]
);
shaping_test!(
    shknda_3_18,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ದೋಂ",
    &["gid254", "gid61@765,0", "gid71@1510,0", "gid4@1921,0"]
);
shaping_test!(
    shknda_3_19,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಧೋಂ ",
    &[
        "gid255",
        "gid61@765,0",
        "gid71@1510,0",
        "gid4@1921,0",
        "gid3@2472,0"
    ]
);
shaping_test!(
    shknda_3_20,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ನೋಂ",
    &["gid256", "gid61@711,0", "gid71@1456,0", "gid4@1868,0"]
);
shaping_test!(
    shknda_3_21,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಪೋಂ",
    &["gid257", "gid275@792,0", "gid71@1434,0", "gid4@1846,0"]
);
shaping_test!(
    shknda_3_22,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಫೋಂ",
    &["gid258", "gid277@792,0", "gid71@1434,0", "gid4@1846,0"]
);
shaping_test!(
    shknda_3_23,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಬೋಂ",
    &["gid259", "gid61@806,0", "gid71@1551,0", "gid4@1963,0"]
);
shaping_test!(
    shknda_3_24,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಭೋಂ",
    &["gid260", "gid61@806,0", "gid71@1551,0", "gid4@1963,0"]
);
shaping_test!(
    shknda_3_25,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಮೋಂ",
    &["gid280", "gid71@1539,0", "gid4@1951,0"]
);
shaping_test!(
    shknda_3_26,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಯೋಂ",
    &["gid281", "gid71@1712,0", "gid4@2124,0"]
);
shaping_test!(
    shknda_3_27,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ರೋಂ",
    &["gid263", "gid61@651,0", "gid71@1397,0", "gid4@1808,0"]
);
shaping_test!(
    shknda_3_28,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಱೋಂ",
    &["gid47", "gid67@831,0", "gid71@1812,0", "gid4@2223,0"]
);
shaping_test!(
    shknda_3_29,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಲೋಂ",
    &["gid264", "gid61@769,0", "gid71@1514,0", "gid4@1925,0"]
);
shaping_test!(
    shknda_3_30,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ವೋಂ",
    &["gid266", "gid275@794,0", "gid71@1437,0", "gid4@1848,0"]
);
shaping_test!(
    shknda_3_31,
    "tests/fonts/text-rendering/NotoSansKannada-Regular.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ಆ\u{ccd}ಯಕ\u{ccd}ಷ\u{cbf}ಸ\u{ccd}\u{200c}",
    &[
        "gid7",
        "gid122@838,0",
        "gid285@1098,0",
        "gid200@1672,0",
        "gid3@2694,0"
    ]
);
shaping_test!(
    shlana_1_1,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠ\u{1a6b}",
    &["uni1A20", "uni1A6B@762,0"]
);
shaping_test!(
    shlana_1_2,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨣ\u{1a74}",
    &["uni1A23", "uni1A74@592,0"]
);
shaping_test!(
    shlana_1_3,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨧ\u{1a62}",
    &["uni1A27", "uni1A62@592,0"]
);
shaping_test!(
    shlana_1_4,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨲ\u{1a60}ᩅ\u{1a6b}ᩡ",
    &[
        "uni1A32",
        "uni1A601A45@762,0",
        "uni1A6B@762,0",
        "uni1A61@933,0"
    ]
);
shaping_test!(
    shlana_1_5,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨷ\u{1a60}ᩅ\u{1a6b}",
    &["uni1A37", "uni1A601A45@592,0", "uni1A6B@592,0"]
);
shaping_test!(
    shlana_1_6,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠ\u{1a60}ᩅ",
    &["uni1A20", "uni1A601A45@762,0"]
);
shaping_test!(
    shlana_1_7,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨡ\u{1a6c}\u{1a74}",
    &["uni1A21", "uni1A6C@592,-98", "uni1A74@592,0"]
);
shaping_test!(
    shlana_1_8,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠ\u{1a6c}",
    &["uni1A20", "uni1A6C.wide@933,0"]
);
shaping_test!(
    shlana_1_9,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨦᩡ",
    &["uni1A26", "uni1A61@592,0"]
);
shaping_test!(
    shlana_1_10,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠᩣ",
    &["uni1A20", "uni1A63@933,0"]
);
shaping_test!(
    shlana_1_11,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨴᩤ",
    &["uni1A34", "uni1A64@592,0"]
);
shaping_test!(
    shlana_1_12,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩌᩣ\u{1a74}",
    &["uni1A4C", "uni1A74@762,0", "uni1A63@933,0"]
);
shaping_test!(
    shlana_1_13,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨣᩤ\u{1a74}",
    &["uni1A23", "uni1A74@592,0", "uni1A64@592,0"]
);
shaping_test!(
    shlana_1_14,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨳ\u{1a65}",
    &["uni1A33", "uni1A65@592,0"]
);
shaping_test!(
    shlana_1_15,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨺ\u{1a66} ",
    &["uni1A3A", "uni1A66@738,0", "space@592,0"]
);
shaping_test!(
    shlana_1_16,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨩ\u{1a67}",
    &["uni1A29", "uni1A67@592,0"]
);
shaping_test!(
    shlana_1_17,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨾ\u{1a68}",
    &["uni1A3E.v2", "uni1A68@592,0"]
);
shaping_test!(
    shlana_1_18,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨵ\u{1a69}",
    &["uni1A35", "uni1A69@592,0"]
);
shaping_test!(
    shlana_1_19,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨦ\u{1a6a}",
    &["uni1A26", "uni1A6A@592,0"]
);
shaping_test!(
    shlana_1_20,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨲᩮᩡ",
    &["uni1A6E", "uni1A32@592,0", "uni1A61@1524,0"]
);
shaping_test!(
    shlana_1_21,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨽᩮ",
    &["uni1A6E", "uni1A3D@592,0"]
);
shaping_test!(
    shlana_1_22,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨤᩯᩡ",
    &["uni1A6F", "uni1A24@1096,0", "uni1A61@1688,0"]
);
shaping_test!(
    shlana_1_23,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨧᩯ",
    &["uni1A6F", "uni1A27@1096,0"]
);
shaping_test!(
    shlana_1_24,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨸᩮ\u{1a6c}\u{1a65}ᩡ",
    &[
        "uni1A6E",
        "uni1A38@592,0",
        "uni1A6C@1184,0",
        "uni1A65@1098,0",
        "uni1A61@1184,0"
    ]
);
shaping_test!(
    shlana_1_25,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨶᩮ\u{1a6c}\u{1a65}",
    &[
        "uni1A6E",
        "uni1A36@592,0",
        "uni1A6C@1184,0",
        "uni1A65@1184,0"
    ]
);
shaping_test!(
    shlana_1_26,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠᩮ\u{1a6c}\u{1a68}ᩡ",
    &[
        "uni1A6E",
        "uni1A20@592,0",
        "uni1A6C.wide@1524,0",
        "uni1A68@1354,0",
        "uni1A61@1524,0"
    ]
);
shaping_test!(
    shlana_1_27,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠᩮ\u{1a6c}\u{1a68}",
    &[
        "uni1A6E",
        "uni1A20@592,0",
        "uni1A6C.wide@1524,0",
        "uni1A68@1354,0"
    ]
);
shaping_test!(
    shlana_1_28,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩁᩮ\u{1a62}ᩣ",
    &[
        "uni1A6E",
        "uni1A41@592,0",
        "uni1A62@1184,0",
        "uni1A63@1184,0"
    ]
);
shaping_test!(
    shlana_1_29,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨾ\u{1a73}",
    &["uni1A3E.v2", "uni1A73@592,0"]
);
shaping_test!(
    shlana_1_30,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠᩮᩣ",
    &["uni1A6E", "uni1A20@592,0", "uni1A63@1524,0"]
);
shaping_test!(
    shlana_1_31,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨹ\u{1a60}ᨿᩮᩡ",
    &[
        "uni1A6E",
        "uni1A39@592,0",
        "uni1A601A3F@1324,0",
        "uni1A61@1551,0"
    ]
);
shaping_test!(
    shlana_1_32,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨻ\u{1a60}ᨿᩮ",
    &["uni1A6E", "uni1A3B@592,0", "uni1A601A3F@1524,0"]
);
shaping_test!(
    shlana_1_33,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠ\u{1a60}ᨿ",
    &["uni1A20", "uni1A601A3F@933,0"]
);
shaping_test!(
    shlana_1_34,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨾᩮ\u{1a6c}\u{1a65}ᩋᩡ",
    &[
        "uni1A6E",
        "uni1A3E.v2@592,0",
        "uni1A6C@1184,0",
        "uni1A65@1184,0",
        "uni1A4B@1324,0",
        "uni1A61@1916,0"
    ]
);
shaping_test!(
    shlana_1_35,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠ\u{1a56}ᩮ\u{1a6c}\u{1a65}ᩋ",
    &[
        "uni1A6E",
        "uni1A20@592,0",
        "uni1A56.wide@1524,0",
        "uni1A6C.wide@1524,-547",
        "uni1A65@1524,0",
        "uni1A4B@1524,0"
    ]
);
shaping_test!(
    shlana_1_36,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈᩰᩡ",
    &["uni1A70", "uni1A48@592,0", "uni1A61@1184,0"]
);
shaping_test!(
    shlana_1_37,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨾᩰ",
    &["uni1A70", "uni1A3E.v2@592,0"]
);
shaping_test!(
    shlana_1_38,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨪᩰ\u{1a6c}ᩡ",
    &[
        "uni1A70",
        "uni1A2A@592,0",
        "uni1A6C@1184,0",
        "uni1A61@1184,0"
    ]
);
shaping_test!(
    shlana_1_39,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨩ\u{1a62}\u{1a60}ᨿ",
    &["uni1A29", "uni1A62@592,0", "uni1A601A3F@592,0"]
);
shaping_test!(
    shlana_1_40,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨶᩲ",
    &["uni1A72", "uni1A36@592,0"]
);
shaping_test!(
    shlana_1_41,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨢᩱ",
    &["uni1A71", "uni1A22@592,0"]
);
shaping_test!(
    shlana_1_42,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨴᩱ\u{1a60}ᨿ",
    &["uni1A71", "uni1A34@592,0", "uni1A601A3F@1184,0"]
);
shaping_test!(
    shlana_1_43,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠᩮ\u{1a6c}\u{1a68}ᩡ",
    &[
        "uni1A6E",
        "uni1A20@592,0",
        "uni1A6C.wide@1524,0",
        "uni1A68@1354,0",
        "uni1A61@1524,0"
    ]
);
shaping_test!(
    shlana_1_44,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠᩮ\u{1a6c}\u{1a68}",
    &[
        "uni1A6E",
        "uni1A20@592,0",
        "uni1A6C.wide@1524,0",
        "uni1A68@1354,0"
    ]
);
shaping_test!(
    shlana_1_45,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠᩰ\u{1a62}",
    &["uni1A70", "uni1A20@592,0", "uni1A62@1354,0"]
);
shaping_test!(
    shlana_1_46,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈ\u{1a58}",
    &["uni1A48", "uni1A58@592,0"]
);
shaping_test!(
    shlana_1_47,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨴ\u{1a62}\u{1a60}ᨦ",
    &["uni1A34", "uni1A62@592,0", "uni1A601A26@592,0"]
);
shaping_test!(
    shlana_1_48,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩌ\u{1a65}\u{1a74} ",
    &["uni1A4C", "uni1A65@762,0", "uni1A74@1211,0", "space@933,0"]
);
shaping_test!(
    shlana_1_49,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠ\u{1a65}\u{1a60}ᨦ",
    &["uni1A20", "uni1A65@762,0", "uni1A601A26@762,0"]
);
shaping_test!(
    shlana_1_50,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠ\u{1a62}\u{1a60}ᨾ",
    &["uni1A20", "uni1A62@762,0", "uni1A601A3E@762,0"]
);
shaping_test!(
    shlana_1_51,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠ\u{1a62}ᨾ",
    &["uni1A20", "uni1A62@762,0", "uni1A3E.v2@933,0"]
);
shaping_test!(
    shlana_1_52,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨯᩭ",
    &["uni1A2F", "uni1A6D@592,0"]
);
shaping_test!(
    shlana_10_1,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩓᩯ",
    &["uni1A6F", "uni1A53@1096,0"]
);
shaping_test!(
    shlana_10_2,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩐᩣ",
    &["uni1A50", "uni1A63@592,0"]
);
shaping_test!(
    shlana_10_3,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩐ\u{1a75}ᩣ",
    &["uni1A50", "uni1A75@738,0", "uni1A63@592,0"]
);
shaping_test!(
    shlana_10_4,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨯ\u{1a6a}ᩕᩣ",
    &["uni1A55", "uni1A2F@227,0", "uni1A6A@818,0", "uni1A63@818,0"]
);
shaping_test!(
    shlana_10_5,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨯᩮ\u{1a6c}\u{1a65}ᩁᨹ\u{1a6b}\u{1a56}ᨣ\u{1a69}ᨱ\u{1a7a}",
    &[
        "uni1A6E",
        "uni1A2F@592,0",
        "uni1A6C@1184,-188",
        "uni1A65@1184,0",
        "uni1A41@1184,0",
        "uni1A39@1775,0",
        "uni1A6B@2367,0",
        "uni1A56@2367,0",
        "uni1A23@2508,0",
        "uni1A69@3100,0",
        "uni1A31@3100,0",
        "uni1A7A@3861,0"
    ]
);
shaping_test!(
    shlana_10_6,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨻᩣᨷᩰ\u{1a56}",
    &[
        "uni1A3B",
        "uni1A63@933,0",
        "uni1A70@1524,0",
        "uni1A37@2116,0",
        "uni1A56@2708,0"
    ]
);
shaping_test!(
    shlana_10_7,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨶ\u{1a76}ᩭ",
    &["uni1A36", "uni1A76@592,0", "uni1A6D@592,0"]
);
shaping_test!(
    shlana_10_8,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨹ\u{1a65}\u{1a56}\u{1a69}\u{1a75}\u{1a7b}",
    &[
        "uni1A39",
        "uni1A65@592,0",
        "uni1A56@592,0",
        "uni1A69@592,-547",
        "uni1A75@592,357",
        "uni1A78@864,357"
    ]
);
shaping_test!(
    shlana_10_9,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩉ\u{1a56}\u{1a60}ᩅᨦ",
    &[
        "uni1A49",
        "uni1A56.wide@933,0",
        "uni1A601A45@762,-547",
        "uni1A26@933,0"
    ]
);
shaping_test!(
    shlana_10_10,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩉ\u{1a56}\u{1a60}ᩅᩣ",
    &[
        "uni1A49",
        "uni1A56.wide@933,0",
        "uni1A601A45@762,-547",
        "uni1A63@933,0"
    ]
);
shaping_test!(
    shlana_10_11,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨻᩕ\u{1a60}ᨿᩮᩡ",
    &[
        "uni1A6E",
        "uni1A55@592,0",
        "uni1A3B@818,0",
        "uni1A601A3F@1751,0",
        "uni1A61@1978,0"
    ]
);
shaping_test!(
    shlana_10_12,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠ\u{1a69}\u{1a76}ᩣ\u{1a60}ᨶ\u{1a60}ᨦ",
    &[
        "uni1A20",
        "uni1A69@762,0",
        "uni1A76@762,0",
        "uni1A63@933,0",
        "uni1A601A36@1524,0",
        "uni1A601A26@1524,-367"
    ]
);
shaping_test!(
    shlana_10_13,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩋ\u{1a62}\u{1a60}ᨭ\u{1a5b}",
    &["uni1A4B", "uni1A62@592,0", "uni1A601A2D1A5B@592,0"]
);
shaping_test!(
    shlana_10_14,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩆ\u{1a62}ᨠ\u{1a60}ᨯ\u{1a65}\u{1a7a}",
    &[
        "uni1A46",
        "uni1A62@592,0",
        "uni1A20@681,0",
        "uni1A601A2F@1442,0",
        "uni1A65@1442,0",
        "uni1A7A@1442,357"
    ]
);
shaping_test!(
    shlana_10_15,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩆ\u{1a62}ᨠ\u{1a60}ᨯ\u{1a65}\u{1a7c}",
    &[
        "uni1A46",
        "uni1A62@592,0",
        "uni1A20@681,0",
        "uni1A601A2F@1442,0",
        "uni1A65@1442,0",
        "uni1A7C@1442,357"
    ]
);
shaping_test!(
    shlana_10_16,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨾᩉᩣᩉ\u{1a65}ᨦ\u{1a60}ᨣ\u{1a69}\u{1a7a}",
    &[
        "uni1A3E.v2",
        "uni1A49@732,0",
        "uni1A63@1665,0",
        "uni1A49@2257,0",
        "uni1A65@3019,0",
        "uni1A26@3189,0",
        "uni1A601A23@3781,0",
        "uni1A69@3781,-547",
        "uni1A7A@3781,0"
    ]
);
shaping_test!(
    shlana_10_17,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨾᩉᩣᩉ\u{1a65}ᨦ\u{1a60}ᨣ\u{1a69}\u{1a7c}",
    &[
        "uni1A3E.v2",
        "uni1A49@732,0",
        "uni1A63@1665,0",
        "uni1A49@2257,0",
        "uni1A65@3019,0",
        "uni1A26@3189,0",
        "uni1A601A23@3781,0",
        "uni1A69@3781,-547",
        "uni1A7C@3781,0"
    ]
);
shaping_test!(
    shlana_10_18,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩆᩣᩈ\u{1a60}ᨲᩕ\u{1a7a}",
    &[
        "uni1A46",
        "uni1A63@681,0",
        "uni1A55@1272,0",
        "uni1A48@1499,0",
        "uni1A601A32@2091,0",
        "uni1A7A@2091,0"
    ]
);
shaping_test!(
    shlana_10_19,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈᩣᩈ\u{1a60}ᨲᩕ\u{1a7c}",
    &[
        "uni1A48",
        "uni1A63@592,0",
        "uni1A55@1184,0",
        "uni1A48@1410,0",
        "uni1A601A32@2002,0",
        "uni1A7C@2002,0"
    ]
);
shaping_test!(
    shlana_10_20,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩁ\u{1a6a}\u{1a60}ᨷ",
    &["uni1A41", "uni1A601A37@592,0", "uni1A6A@725,-367"]
);
shaping_test!(
    shlana_10_21,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨻ\u{1a66}\u{1a75}\u{1a60}ᨶ\u{1a6c}\u{1a76}ᨦ",
    &[
        "uni1A3B",
        "uni1A66@762,0",
        "uni1A75@762,357",
        "uni1A601A36@762,0",
        "uni1A6C@762,-367",
        "uni1A76@1087,357",
        "uni1A26@933,0"
    ]
);
shaping_test!(
    shlana_10_22,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨸ\u{1a62}ᩣ",
    &["uni1A38", "uni1A62@506,0", "uni1A63@592,0"]
);
shaping_test!(
    shlana_10_23,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩃ\u{1a6a}\u{1a62}",
    &["uni1A43", "uni1A6A@762,0", "uni1A62@762,0"]
);
shaping_test!(
    shlana_10_24,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨯ\u{1a6c}\u{1a62}",
    &["uni1A2F", "uni1A6C@592,-188", "uni1A62@592,0"]
);
shaping_test!(
    shlana_10_25,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨯ\u{1a6c}\u{1a62}ᩡ",
    &[
        "uni1A2F",
        "uni1A6C@592,-188",
        "uni1A62@592,0",
        "uni1A61@592,0"
    ]
);
shaping_test!(
    shlana_10_26,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨻ\u{1a60}ᩅ\u{1a62}ᩡ",
    &[
        "uni1A3B",
        "uni1A601A45@762,0",
        "uni1A62@762,0",
        "uni1A61@933,0"
    ]
);
shaping_test!(
    shlana_10_27,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨲᩯ\u{1a60}ᨶ\u{1a6c}\u{1a74}\u{1a75}",
    &[
        "uni1A6F",
        "uni1A32@1096,0",
        "uni1A601A36@1857,0",
        "uni1A6C@1857,-367",
        "uni1A74@1857,0",
        "uni1A75@1857,357"
    ]
);
shaping_test!(
    shlana_10_28,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨲ\u{1a6c}\u{1a74}\u{1a75}\u{34f}ᩯ\u{1a60}ᨶ",
    &[
        "uni1A6F",
        "uni1A32@1096,0",
        "uni1A6C.wide@2028,0",
        "uni1A74@2028,0",
        "uni1A75@2028,357",
        "uni1A601A36@1857,-367"
    ]
);
shaping_test!(
    shlana_10_29,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨲ\u{1a6c}\u{1a74}\u{1a75}ᩯ\u{1a60}ᨶ",
    &[
        "uni1A6F",
        "uni1A32@1096,0",
        "uni1A6C.wide@2028,0",
        "uni1A74@2028,0",
        "uni1A75@2028,357",
        "uni1A601A36@1857,-367"
    ]
);
shaping_test!(
    shlana_10_30,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈ\u{1a60}ᨶ\u{1a6b}\u{1a7b}",
    &[
        "uni1A48",
        "uni1A601A36@592,0",
        "uni1A6B@592,0",
        "uni1A78@592,357"
    ]
);
shaping_test!(
    shlana_10_31,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠᨾ\u{1a5b}\u{1a66}",
    &["uni1A20", "uni1A3E1A5B@933,0", "uni1A66@1549,0"]
);
shaping_test!(
    shlana_10_32,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨶ\u{1a65}ᨻ\u{1a5b}ᩣ\u{1a60}ᨶ",
    &[
        "uni1A36",
        "uni1A65@592,0",
        "uni1A3B1A5B@592,0",
        "uni1A63@1524,0",
        "uni1A601A36@2116,0"
    ]
);
shaping_test!(
    shlana_10_33,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨵᨾ\u{1a5c}\u{1a65}ᨠ",
    &[
        "uni1A35",
        "uni1A3E.v2@592,0",
        "uni1A5C@1184,0",
        "uni1A65@1184,0",
        "uni1A20@1324,0"
    ]
);
shaping_test!(
    shlana_10_34,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈᨵ\u{1a69}\u{1a60}ᨷ",
    &[
        "uni1A48",
        "uni1A35@592,0",
        "uni1A601A37@1184,0",
        "uni1A69@1317,-367"
    ]
);
shaping_test!(
    shlana_10_35,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩅ\u{1a65}ᩈᩮ\u{1a60}ᩈ",
    &[
        "uni1A45",
        "uni1A65@592,0",
        "uni1A6E@592,0",
        "uni1A48@1184,0",
        "uni1A601A48@1775,0"
    ]
);
shaping_test!(
    shlana_10_36,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨢ\u{1a76}ᩣ",
    &["uni1A22", "uni1A76@738,0", "uni1A63@592,0"]
);
shaping_test!(
    shlana_10_37,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈᩣᩈᨶᩣ",
    &[
        "uni1A48",
        "uni1A63@592,0",
        "uni1A48@1184,0",
        "uni1A361A63@1775,0"
    ]
);
shaping_test!(
    shlana_10_38,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈᩣᩈ\u{1a60}ᨶ\u{1a7b}ᩣ",
    &[
        "uni1A48",
        "uni1A63@592,0",
        "uni1A48@1184,0",
        "uni1A601A36@1775,0",
        "uni1A78@1775,0",
        "uni1A63@1775,0"
    ]
);
shaping_test!(
    shlana_10_39,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈ\u{1a60}ᨶ\u{1a7b}ᩮ\u{1a62}\u{1a76}ᩣ",
    &[
        "uni1A6E",
        "uni1A48@592,0",
        "uni1A601A36@1184,0",
        "uni1A78@1184,0",
        "uni1A621A76@1599,0",
        "uni1A63@1184,0"
    ]
);
shaping_test!(
    shlana_10_40,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈ\u{1a60}ᨶ\u{1a7b}ᩮ\u{1a62}\u{1a62}\u{1a76}ᩣ",
    &[
        "uni1A6E",
        "uni1A48@592,0",
        "uni1A601A36@1184,0",
        "uni1A78@1184,0",
        "uni1A621A621A76@1599,0",
        "uni1A63@1184,0"
    ]
);
shaping_test!(
    shlana_10_41,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨲ\u{1a66}\u{34f}ᩣ\u{1a60}ᨿ",
    &[
        "uni1A32",
        "uni1A66@762,0",
        "uni1A63@933,0",
        "uni1A601A3F@1524,0"
    ]
);
shaping_test!(
    shlana_10_42,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩉ\u{1a60}ᨾ\u{1a75}ᩣ\u{1a74}\u{1a7b}",
    &[
        "uni1A49",
        "uni1A601A3E@762,0",
        "uni1A74@762,0",
        "uni1A75@762,357",
        "uni1A63@933,0",
        "uni1A78@1524,0"
    ]
);
shaping_test!(
    shlana_10_43,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩃᩮ\u{1a5e}",
    &["uni1A6E", "uni1A43@592,0", "uni1A5E@1354,0"]
);
shaping_test!(
    shlana_10_44,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩋᨶᩣᨳᨷ\u{1a65}ᨱ\u{1a60}ᨯ\u{1a65}ᨠᩈ\u{1a5e}",
    &[
        "uni1A4B",
        "uni1A361A63@592,0",
        "uni1A33@1184,0",
        "uni1A37@1775,0",
        "uni1A65@2367,0",
        "uni1A31@2367,0",
        "uni1A601A2F@3129,0",
        "uni1A65@3129,0",
        "uni1A20@3300,0",
        "uni1A48@4232,0",
        "uni1A5E@4824,0"
    ]
);
shaping_test!(
    shlana_10_45,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨯᩣ\u{1a74}\u{1a7b}",
    &[
        "uni1A2F",
        "uni1A74@592,0",
        "uni1A63@592,0",
        "uni1A78@1184,0"
    ]
);
shaping_test!(
    shlana_10_46,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩉ\u{1a60}ᨾ\u{1a74}\u{1a75}ᩣ\u{1a7b}",
    &[
        "uni1A49",
        "uni1A601A3E@762,0",
        "uni1A74@762,0",
        "uni1A75@762,357",
        "uni1A63@933,0",
        "uni1A78@1524,0"
    ]
);
shaping_test!(
    shlana_10_47,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨯ\u{1a74}ᩣ\u{1a7b}",
    &[
        "uni1A2F",
        "uni1A74@592,0",
        "uni1A63@592,0",
        "uni1A78@1184,0"
    ]
);
shaping_test!(
    shlana_2_1,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "᪓\u{1a60}ᨴ",
    &["uni1A93", "uni1A601A34@592,0"]
);
shaping_test!(
    shlana_2_2,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨲ\u{1a75}ᩣ\u{1a60}ᨦ\u{1a7b}",
    &[
        "uni1A32",
        "uni1A75@762,0",
        "uni1A63@933,0",
        "uni1A601A26@1524,0",
        "uni1A78@1524,0"
    ]
);
shaping_test!(
    shlana_2_3,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨳ\u{1a60}ᨶ\u{1a7b}\u{1a6b}ᩁ",
    &[
        "uni1A33",
        "uni1A601A36@592,0",
        "uni1A78@592,0",
        "uni1A6B@954,0",
        "uni1A41@592,0"
    ]
);
shaping_test!(
    shlana_2_4,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨡ\u{1a62}\u{1a76}\u{1a7b}\u{1a6c}ᨦ",
    &[
        "uni1A21",
        "uni1A621A76@592,0",
        "uni1A78@1007,0",
        "uni1A6C@592,-98",
        "uni1A26@592,0"
    ]
);
shaping_test!(
    shlana_2_5,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "᪭ᩣ",
    &["uni1AAD", "uni1A63@592,0"]
);
shaping_test!(
    shlana_2_6,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩉ\u{1a60}ᨶ\u{1a66}",
    &["uni1A49", "uni1A601A36@762,0", "uni1A66@762,0"]
);
shaping_test!(
    shlana_2_7,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨤ\u{1a60}ᩅᩯ\u{1a76}ᩁ",
    &[
        "uni1A6F",
        "uni1A24@1096,0",
        "uni1A601A45@1688,0",
        "uni1A76@1688,0",
        "uni1A41@1688,0"
    ]
);
shaping_test!(
    shlana_2_8,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩉ\u{1a60}ᩅ\u{1a6b}",
    &["uni1A49", "uni1A601A45@762,0", "uni1A6B@762,0"]
);
shaping_test!(
    shlana_2_9,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨯ\u{1a62}\u{1a75}ᨦ\u{1a60}ᨶ\u{1a66}\u{1a76}",
    &[
        "uni1A2F",
        "uni1A621A75@592,0",
        "uni1A26@592,0",
        "uni1A601A36@1184,0",
        "uni1A66@1184,0",
        "uni1A76@1184,357"
    ]
);
shaping_test!(
    shlana_2_10,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩉ\u{1a65}\u{1a60}ᨶ",
    &["uni1A49", "uni1A65@762,0", "uni1A601A36@762,0"]
);
shaping_test!(
    shlana_2_11,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨲᩣ\u{1a60}ᨾ",
    &["uni1A32", "uni1A63@933,0", "uni1A601A3E@1524,0"]
);
shaping_test!(
    shlana_2_12,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨻ\u{1a60}ᨿᩣ\u{1a60}ᨵ\u{1a65}",
    &[
        "uni1A3B",
        "uni1A601A3F@933,0",
        "uni1A63@1159,0",
        "uni1A601A35@1751,0",
        "uni1A65@1751,0"
    ]
);
shaping_test!(
    shlana_2_13,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨸ\u{1a60}ᩃ\u{1a60}ᨿ\u{1a75}ᩁ",
    &[
        "uni1A38",
        "uni1A601A43@592,0",
        "uni1A601A3F@818,0",
        "uni1A75@1049,0",
        "uni1A41@1045,0"
    ]
);
shaping_test!(
    shlana_2_14,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨾᩯ\u{1a76}\u{1a60}ᨶ\u{1a60}ᩅ\u{1a75}ᩣ",
    &[
        "uni1A6F",
        "uni1A3E.v2@1096,0",
        "uni1A76@1688,0",
        "uni1A601A36@1688,0",
        "uni1A601A45@1688,-367",
        "uni1A75@2013,0",
        "uni1A63@1828,0"
    ]
);
shaping_test!(
    shlana_2_15,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈ\u{1a60}ᩅᩯ\u{1a75}",
    &[
        "uni1A6F",
        "uni1A48@1096,0",
        "uni1A601A45@1688,0",
        "uni1A75@1688,0"
    ]
);
shaping_test!(
    shlana_2_16,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈ\u{1a75}ᩯ\u{1a60}ᩅ",
    &[
        "uni1A6F",
        "uni1A48@1096,0",
        "uni1A75@1688,0",
        "uni1A601A45@1688,0"
    ]
);
shaping_test!(
    shlana_2_17,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨿ\u{1a6a}",
    &["uni1A3F", "uni1A6A@762,0"]
);
shaping_test!(
    shlana_2_18,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨾ\u{1a66}",
    &["uni1A3E.v2", "uni1A66@592,0"]
);
shaping_test!(
    shlana_2_19,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩉ\u{1a60}ᨾ\u{1a6a}",
    &["uni1A49", "uni1A601A3E@762,0", "uni1A6A@762,-367"]
);
shaping_test!(
    shlana_2_20,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩉ\u{1a60}ᨾ\u{1a66}",
    &["uni1A49", "uni1A601A3E@762,0", "uni1A66@762,0"]
);
shaping_test!(
    shlana_2_21,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨹ\u{1a60}ᩅ\u{1a6b}",
    &["uni1A39", "uni1A601A45@592,0", "uni1A6B@592,0"]
);
shaping_test!(
    shlana_2_22,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩉ\u{1a60}ᩃ\u{1a6c}\u{1a74}\u{1a75}",
    &[
        "uni1A49",
        "uni1A601A43@933,0",
        "uni1A6C@1060,-547",
        "uni1A74@1164,0",
        "uni1A75@1164,357"
    ]
);
shaping_test!(
    shlana_2_23,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨾᩣ",
    &["uni1A3E.v2", "uni1A63@732,0"]
);
shaping_test!(
    shlana_2_24,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩉᩱ\u{1a75}",
    &["uni1A71", "uni1A49@592,0", "uni1A75@1354,0"]
);
shaping_test!(
    shlana_2_25,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩅ\u{1a60}ᨿᨦ",
    &["uni1A45", "uni1A601A3F@592,0", "uni1A26@818,0"]
);
shaping_test!(
    shlana_2_26,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩉᩣ\u{1a60}ᨾ",
    &["uni1A49", "uni1A63@933,0", "uni1A601A3E@1524,0"]
);
shaping_test!(
    shlana_2_27,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨯᩣ\u{1a74}",
    &["uni1A2F", "uni1A74@592,0", "uni1A63@592,0"]
);
shaping_test!(
    shlana_2_28,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩉ\u{1a60}ᨾᩣ",
    &["uni1A49", "uni1A601A3E@762,0", "uni1A63@933,0"]
);
shaping_test!(
    shlana_2_29,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠᩕᩣ\u{1a60}ᨸ",
    &[
        "uni1A55",
        "uni1A20@227,0",
        "uni1A63@1159,0",
        "uni1A601A37@1751,0"
    ]
);
shaping_test!(
    shlana_2_30,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨻᩕ\u{1a75}ᩣ\u{1a74}",
    &[
        "uni1A55",
        "uni1A3B@227,0",
        "uni1A74@988,0",
        "uni1A75@988,357",
        "uni1A63@1159,0"
    ]
);
shaping_test!(
    shlana_2_31,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠᩕ\u{1a6c}ᨦ",
    &[
        "uni1A55",
        "uni1A20@227,0",
        "uni1A6C.wide@1159,0",
        "uni1A26@1159,0"
    ]
);
shaping_test!(
    shlana_2_32,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈᩕ\u{1a6b}ᨾ\u{1a60}ᨱ\u{1a7a}",
    &[
        "uni1A55",
        "uni1A48@227,0",
        "uni1A6B@818,0",
        "uni1A3E.v2@818,0",
        "uni1A601A31.narrow@1410,0",
        "uni1A7A@1410,0"
    ]
);
shaping_test!(
    shlana_2_33,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈᩕ\u{1a60}ᩅ\u{1a6b}ᨾ",
    &[
        "uni1A55",
        "uni1A48@227,0",
        "uni1A601A45@818,0",
        "uni1A6B@818,0",
        "uni1A3E.v2@818,0"
    ]
);
shaping_test!(
    shlana_2_34,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩉ\u{1a60}ᨾ\u{1a75}ᩣ\u{1a74}",
    &[
        "uni1A49",
        "uni1A601A3E@762,0",
        "uni1A74@762,0",
        "uni1A75@762,357",
        "uni1A63@933,0"
    ]
);
shaping_test!(
    shlana_2_35,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩉ\u{1a60}ᨾᩮ\u{1a6c}\u{1a68}ᨦ",
    &[
        "uni1A6E",
        "uni1A49@592,0",
        "uni1A601A3E@1524,0",
        "uni1A6C@1524,-367",
        "uni1A68@1524,0",
        "uni1A26@1524,0"
    ]
);
shaping_test!(
    shlana_2_36,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩉ\u{1a60}ᨿᩮ\u{1a6c}\u{1a68}ᨦ",
    &[
        "uni1A6E",
        "uni1A49@592,0",
        "uni1A601A3F@1524,0",
        "uni1A6C@1729,-367",
        "uni1A68@1755,0",
        "uni1A26@1751,0"
    ]
);
shaping_test!(
    shlana_2_37,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩉ\u{1a60}ᩃᩣ\u{1a60}ᨿ",
    &[
        "uni1A49",
        "uni1A601A43@933,0",
        "uni1A63@1159,0",
        "uni1A601A3F@1751,0"
    ]
);
shaping_test!(
    shlana_3_1,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠ\u{1a6c}\u{1a62}ᩃ\u{1a60}ᨼ\u{1a7a}",
    &[
        "uni1A20",
        "uni1A6C.wide@933,0",
        "uni1A62@762,0",
        "uni1A43@933,0",
        "antler@933,0",
        "uni1A601A3C.wide@1865,0",
        "uni1A7A@1964,0"
    ]
);
shaping_test!(
    shlana_3_2,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠ\u{1a6c}\u{1a62}ᩃ\u{1a7a}\u{1a60}ᨼ",
    &[
        "uni1A20",
        "uni1A6C.wide@933,0",
        "uni1A62@762,0",
        "uni1A43@933,0",
        "antler@933,0",
        "uni1A7A@1964,0",
        "uni1A601A3C.wide@1865,0"
    ]
);
shaping_test!(
    shlana_3_3,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠ\u{1a6c}\u{1a62}ᩃ\u{1a60}\u{1a7a}ᨼ",
    &[
        "uni1A20",
        "uni1A6C.wide@933,0",
        "uni1A62@762,0",
        "uni1A43@933,0",
        "antler@933,0",
        "uni1A7A@1964,0",
        "uni1A601A3C.wide@1865,0"
    ]
);
shaping_test!(
    shlana_3_4,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠᩕᩣ\u{1a60}ᨼ",
    &[
        "uni1A55",
        "uni1A20@227,0",
        "uni1A63@1159,0",
        "antler@1159,0",
        "uni1A601A3C@1751,0"
    ]
);
shaping_test!(
    shlana_3_5,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨴ\u{1a6c}ᨼ\u{1a60}ᨼ\u{1a66}\u{1a75}",
    &[
        "uni1A34",
        "uni1A6C@592,0",
        "uni1A3C@592,0",
        "antler@771,0",
        "uni1A601A3C.wide@1524,0",
        "uni1A66@1558,0",
        "uni1A75@1558,357"
    ]
);
shaping_test!(
    shlana_3_6,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠᨽ\u{1a5a}",
    &["uni1A20", "uni1A3D@933,0", "uni1A5A@1865,0"]
);
shaping_test!(
    shlana_3_7,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈᨱ\u{1a5b}ᩣ\u{1a60}ᨶ",
    &[
        "uni1A48",
        "uni1A311A5B@592,0",
        "uni1A63@1524,0",
        "uni1A601A36@2116,0"
    ]
);
shaping_test!(
    shlana_3_8,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩁᨭ\u{1a5b}ᨷᩣ\u{1a60}ᩃ",
    &[
        "uni1A41",
        "uni1A2D1A5B@592,0",
        "uni1A37@1184,0",
        "uni1A63@1775,0",
        "uni1A601A43@2367,0"
    ]
);
shaping_test!(
    shlana_3_9,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩁ\u{1a62}ᨭ\u{1a5b}ᨷᩣ\u{1a60}ᩃ",
    &[
        "uni1A41",
        "uni1A62@592,0",
        "uni1A2D1A5B@592,0",
        "uni1A37@1184,0",
        "uni1A63@1775,0",
        "uni1A601A43@2367,0"
    ]
);
shaping_test!(
    shlana_3_10,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈᨻ\u{1a5b}",
    &["uni1A48", "uni1A3B1A5B@592,0"]
);
shaping_test!(
    shlana_3_11,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩋᨾ\u{1a5b}",
    &["uni1A4B", "uni1A3E1A5B@592,0"]
);
shaping_test!(
    shlana_3_12,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩁᩣᨩᨽ\u{1a62}\u{1a60}ᨮ",
    &[
        "uni1A41",
        "uni1A63@592,0",
        "uni1A29@1184,0",
        "uni1A3D@1775,0",
        "uni1A62@2537,0",
        "uni1A601A2E@2537,0"
    ]
);
shaping_test!(
    shlana_3_13,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨷ\u{1a62}ᨱ\u{1a60}ᨻᨷ\u{1a69}ᩁ\u{1a69}ᩈ",
    &[
        "uni1A37",
        "uni1A62@592,0",
        "uni1A31@592,0",
        "uni1A601A3B.wide@1524,0",
        "uni1A37@1524,0",
        "uni1A69@2116,0",
        "uni1A41@2116,0",
        "uni1A69@2708,0",
        "uni1A48@2708,0"
    ]
);
shaping_test!(
    shlana_4_1,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨴ\u{1a58}\u{1a60}ᩃᩣ\u{1a60}ᨿ",
    &[
        "uni1A34",
        "uni1A58@592,0",
        "uni1A601A43@592,0",
        "uni1A63@818,0",
        "uni1A601A3F@1410,0"
    ]
);
shaping_test!(
    shlana_4_2,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈ\u{1a58}ᨥᩮᩣ",
    &[
        "uni1A48",
        "uni1A6E@592,0",
        "uni1A25@1184,0",
        "uni1A58@1945,0",
        "uni1A63@2117,0"
    ]
);
shaping_test!(
    shlana_4_3,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩁ\u{1a58}ᩈ\u{1a66}",
    &[
        "uni1A41",
        "uni1A58@592,0",
        "uni1A48@592,0",
        "uni1A66@1184,0"
    ]
);
shaping_test!(
    shlana_5_1,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨶᩣ\u{1a74}",
    &["uni1A361A63", "uni1A74@592,0"]
);
shaping_test!(
    shlana_5_2,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨾᨶᩮᩣ",
    &["uni1A3E.v2", "uni1A6E@732,0", "uni1A361A63@1324,0"]
);
shaping_test!(
    shlana_5_3,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨶᩮ\u{1a62}ᩣ",
    &["uni1A6E", "uni1A361A63@592,0", "uni1A62@1184,0"]
);
shaping_test!(
    shlana_5_4,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨶᩣ\u{1a60}ᨿ",
    &["uni1A361A63", "uni1A601A3F@592,0"]
);
shaping_test!(
    shlana_5_5,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨶ\u{1a75}ᩣ\u{1a60}ᨶ",
    &["uni1A361A63", "uni1A75@592,0", "uni1A601A36@592,0"]
);
shaping_test!(
    shlana_5_6,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩍᨶ\u{1a60}ᨴᩣ",
    &["uni1A4D", "uni1A361A63@933,0", "uni1A601A34@1524,0"]
);
shaping_test!(
    shlana_5_7,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩋ\u{1a6b}ᨶ\u{1a60}ᨲᩕᩣ\u{1a60}ᨿ",
    &[
        "uni1A4B",
        "uni1A6B@592,0",
        "uni1A55@592,0",
        "uni1A361A63@818,0",
        "uni1A601A32@1410,0",
        "uni1A601A3F@1410,0"
    ]
);
shaping_test!(
    shlana_5_8,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨶ\u{1a76}ᩣ\u{1a74}",
    &["uni1A361A63", "uni1A74@592,0", "uni1A76@592,357"]
);
shaping_test!(
    shlana_5_9,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨶ\u{1a60}ᩅᩣ\u{1a60}ᨷ",
    &["uni1A361A63", "uni1A601A45@592,0", "uni1A601A37@592,0"]
);
shaping_test!(
    shlana_5_10,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨴᩤ\u{1a74}ᨶ\u{1a60}ᩅ\u{200c}ᩣ\u{1a60}ᨿ",
    &[
        "uni1A34",
        "uni1A74@592,0",
        "uni1A64@592,0",
        "uni1A36@818,0",
        "uni1A601A45@1410,0",
        "uni1A63@1410,0",
        "uni1A601A3F@2002,0"
    ]
);
shaping_test!(
    shlana_5_11,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨲ\u{1a75}ᩣ\u{1a74}ᨶ\u{1a60}ᩅᩣ\u{1a60}ᨿ",
    &[
        "uni1A32",
        "uni1A74@762,0",
        "uni1A75@762,357",
        "uni1A63@933,0",
        "uni1A361A63@1524,0",
        "uni1A601A45@2116,0",
        "uni1A601A3F@2116,0"
    ]
);
shaping_test!(
    shlana_5_12,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨶ\u{200c}ᩣ",
    &["uni1A36", "uni1A63@592,0"]
);
shaping_test!(
    shlana_5_13,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩉ\u{1a60}ᨶ\u{1a76}ᩣ",
    &[
        "uni1A49",
        "uni1A601A36@762,0",
        "uni1A76@762,0",
        "uni1A63@933,0"
    ]
);
shaping_test!(
    shlana_6_1,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠᩯ\u{1a62}\u{1a60}ᩈ",
    &[
        "uni1A6F",
        "uni1A20@1096,0",
        "uni1A62@1857,0",
        "uni1A601A48@2028,0"
    ]
);
shaping_test!(
    shlana_6_2,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨴᩕᩯ\u{1a60}ᨠᨴᩮ\u{1a6c}\u{1a65}ᩁ\u{1a7a}",
    &[
        "uni1A6F",
        "uni1A55@1096,0",
        "uni1A34@1322,0",
        "uni1A601A20@1914,0",
        "uni1A6E@1914,0",
        "uni1A34@2506,0",
        "uni1A6C@3098,0",
        "uni1A65@3098,0",
        "uni1A41@3098,0",
        "uni1A7A@3689,0"
    ]
);
shaping_test!(
    shlana_6_3,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨶᩰ\u{1a6b}\u{1a76}\u{1a60}ᨲ",
    &[
        "uni1A70",
        "uni1A36@592,0",
        "uni1A6B@1184,0",
        "uni1A76@1184,357",
        "uni1A601A32@1184,0"
    ]
);
shaping_test!(
    shlana_6_4,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨷᩕᩰᨴ\u{1a66}\u{1a60}ᨶ",
    &[
        "uni1A70",
        "uni1A55@592,0",
        "uni1A37@818,0",
        "uni1A34@1410,0",
        "uni1A66@2002,0",
        "uni1A601A36@2002,0"
    ]
);
shaping_test!(
    shlana_6_5,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨼ\u{1a65}ᩅ\u{1a60}ᩈ\u{1a7a}",
    &[
        "uni1A3C",
        "uni1A65@859,0",
        "uni1A45@933,0",
        "uni1A601A48@1524,0",
        "uni1A7A@1755,0"
    ]
);
shaping_test!(
    shlana_6_6,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈᨲᩯᨾ\u{1a60}ᨷ\u{1a7a}",
    &[
        "uni1A48",
        "uni1A6F@592,0",
        "uni1A32@1688,0",
        "uni1A3E.v2@2620,0",
        "uni1A601A37@3353,0",
        "uni1A7A@3584,0"
    ]
);
shaping_test!(
    shlana_6_7,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈᩮ\u{1a65}ᩁ\u{1a7a}\u{1a60}ᨷ",
    &[
        "uni1A6E",
        "uni1A48@592,0",
        "uni1A65@1184,0",
        "uni1A41@1184,0",
        "uni1A7A@1775,0",
        "uni1A601A37@1775,0"
    ]
);
shaping_test!(
    shlana_7_1,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨴ\u{1a62}\u{1a75}ᩗᩣ",
    &[
        "uni1A34",
        "uni1A621A75@592,0",
        "uni1A57@592,0",
        "uni1A63@818,0"
    ]
);
shaping_test!(
    shlana_7_2,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨡᨶ\u{1a60}ᨵᩣ",
    &["uni1A21", "uni1A361A63@592,0", "uni1A601A35@1184,0"]
);
shaping_test!(
    shlana_7_3,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨣ\u{1a76}\u{1a60}ᨯ\u{1a66}\u{a0}",
    &[
        "uni1A23",
        "uni1A76@592,0",
        "uni1A601A2F@592,0",
        "uni1A66@1095,0",
        "space_nb@592,0"
    ]
);
shaping_test!(
    shlana_7_4,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨷ\u{1a76}\u{1a60}ᨾᩣ",
    &[
        "uni1A37",
        "uni1A76@592,0",
        "uni1A601A3E@592,0",
        "uni1A63@592,0"
    ]
);
shaping_test!(
    shlana_7_5,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨷ\u{1a60}\u{1a76}ᨾᩣ",
    &[
        "uni1A37",
        "uni1A76@592,0",
        "uni1A601A3E@592,0",
        "uni1A63@592,0"
    ]
);
shaping_test!(
    shlana_7_6,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨷ\u{1a76}\u{1a60}ᨯᩣ\u{1a60}ᨿ",
    &[
        "uni1A37",
        "uni1A76@592,0",
        "uni1A601A2F@592,0",
        "uni1A63@592,0",
        "uni1A601A3F@1184,0"
    ]
);
shaping_test!(
    shlana_7_7,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨧ\u{1a62}\u{1a60}ᩅᩤ",
    &[
        "uni1A27",
        "uni1A62@592,0",
        "uni1A601A45@592,0",
        "uni1A64@592,0"
    ]
);
shaping_test!(
    shlana_7_8,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈᩮ\u{1a60}ᩓ\u{1a60}ᩅ",
    &[
        "uni1A6E",
        "uni1A48@592,0",
        "uni1A601A53@1184,0",
        "uni1A601A45@1311,-547"
    ]
);
shaping_test!(
    shlana_7_9,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨴᩯ\u{1a60}ᨶ\u{1a73}",
    &[
        "uni1A6F",
        "uni1A34@1096,0",
        "uni1A601A36@1688,0",
        "uni1A73@1688,0"
    ]
);
shaping_test!(
    shlana_7_10,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩓ\u{1a60}ᨾᩣ",
    &["uni1A53", "uni1A601A3E@762,0", "uni1A63@933,0"]
);
shaping_test!(
    shlana_7_11,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨠᩮ\u{1a60}ᩈᩣ",
    &[
        "uni1A6E",
        "uni1A20@592,0",
        "uni1A601A48@1524,0",
        "uni1A63@1751,0"
    ]
);
shaping_test!(
    shlana_7_12,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨻᩱ\u{1a60}ᨾᩣ",
    &[
        "uni1A71",
        "uni1A3B@592,0",
        "uni1A601A3E@1354,0",
        "uni1A63@1524,0"
    ]
);
shaping_test!(
    shlana_7_13,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈᩮ\u{1a60}ᩅ\u{1a76}ᩤ",
    &[
        "uni1A6E",
        "uni1A48@592,0",
        "uni1A601A45@1184,0",
        "uni1A76@1184,0",
        "uni1A64@1184,0"
    ]
);
shaping_test!(
    shlana_7_14,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩅᩮ\u{1a60}ᩃᩣ",
    &[
        "uni1A6E",
        "uni1A45@592,0",
        "uni1A601A43@1184,0",
        "uni1A63@1410,0"
    ]
);
shaping_test!(
    shlana_7_15,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨵᩤ\u{1a60}ᨲ\u{1a69}",
    &[
        "uni1A35",
        "uni1A64@592,0",
        "uni1A601A32@863,0",
        "uni1A69@863,-547"
    ]
);
shaping_test!(
    shlana_7_16,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨩ\u{1a60}ᩓ",
    &["uni1A29", "uni1A601A53@592,0"]
);
shaping_test!(
    shlana_7_17,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨻᩭ\u{1a60}ᩅ\u{1a7b}ᩣ",
    &[
        "uni1A3B",
        "uni1A6D@933,0",
        "uni1A601A45@1137,-547",
        "uni1A78@1204,195",
        "uni1A63@1159,0"
    ]
);
shaping_test!(
    shlana_7_18,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈ\u{1a6b}\u{1a60}ᨦᩣ\u{1a60}ᨶ",
    &[
        "uni1A48",
        "uni1A6B@592,0",
        "uni1A601A26@592,0",
        "uni1A63@592,0",
        "uni1A601A36@1184,0"
    ]
);
shaping_test!(
    shlana_8_1,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨻ\u{1a6c}\u{1a73}\u{1a75}",
    &[
        "uni1A3B",
        "uni1A6C.wide@933,0",
        "uni1A73@762,0",
        "uni1A75@762,447"
    ]
);
shaping_test!(
    shlana_8_2,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩈᨷ\u{1a77}ᩣ\u{1a60}ᨿ",
    &[
        "uni1A48",
        "uni1A37@592,0",
        "uni1A77@1184,0",
        "uni1A63@1184,0",
        "uni1A601A3F@1775,0"
    ]
);
shaping_test!(
    shlana_8_3,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩅ\u{1a60}ᨿ\u{1a59}",
    &["uni1A45", "uni1A601A3F@592,0", "uni1A59@823,0"]
);
shaping_test!(
    shlana_8_4,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨣ\u{1a6a}\u{1a7a}",
    &["uni1A23", "uni1A6A@592,0", "uni1A7A@592,0"]
);
shaping_test!(
    shlana_8_5,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "᪁᪂\u{a0}ᨻ\u{1a62}\u{1a60}ᨶ\u{1a7b}ᩣ",
    &[
        "uni1A81",
        "uni1A82@592,0",
        "space_nb@1184,0",
        "uni1A3B@1501,0",
        "uni1A62@2263,0",
        "uni1A601A36@2263,0",
        "uni1A78@2263,357",
        "uni1A63@2434,0"
    ]
);
shaping_test!(
    shlana_8_6,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨻᩱ\u{1a7b}ᩣ\u{1a60}ᨿ",
    &[
        "uni1A71",
        "uni1A3B@592,0",
        "uni1A78@1354,0",
        "uni1A63@1524,0",
        "uni1A601A3F@2116,0"
    ]
);
shaping_test!(
    shlana_8_7,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨩ\u{1a60}ᨿ\u{1a59}ᨲ\u{1a69}\u{1a74}",
    &[
        "uni1A29",
        "uni1A601A3F@592,0",
        "uni1A59@823,0",
        "uni1A32@818,0",
        "uni1A69@1580,0",
        "uni1A74@1580,0"
    ]
);
shaping_test!(
    shlana_8_8,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩅᨲ\u{1a5b}\u{1a69}",
    &["uni1A45", "uni1A321A5B@592,0", "uni1A69@1184,-734"]
);
shaping_test!(
    shlana_8_9,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩅᨲ\u{1a60}ᨳ\u{1a69}",
    &[
        "uni1A45",
        "uni1A32@592,0",
        "uni1A601A33@1354,0",
        "uni1A69@1354,-547"
    ]
);
shaping_test!(
    shlana_8_10,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨯ\u{1a60}ᨿ\u{1a74}",
    &["uni1A2F", "uni1A601A3F@592,0", "uni1A74@823,0"]
);
shaping_test!(
    shlana_8_11,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩉ\u{1a60}ᨶ\u{1a66}\u{1a62}\u{1a76}",
    &[
        "uni1A49",
        "uni1A601A36@762,0",
        "uni1A66@762,0",
        "uni1A621A76@1265,0"
    ]
);
shaping_test!(
    shlana_8_12,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩁᩮᩂ\u{1a60}ᨠ",
    &[
        "uni1A6E",
        "uni1A41@592,0",
        "uni1A42@1184,0",
        "uni1A601A20@1775,-547"
    ]
);
shaping_test!(
    shlana_8_13,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᩁ\u{1a60}ᨿ\u{1a7a}",
    &["uni1A41", "uni1A601A3F@592,0", "uni1A7A@823,0"]
);
shaping_test!(
    shlana_9_1,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨣ\u{1a74}ᨷ\u{1a74}\u{1a75}",
    &[
        "uni1A23",
        "uni1A74@592,0",
        "uni1A37@592,0",
        "uni1A74@1184,0",
        "uni1A75@1184,357"
    ]
);
shaping_test!(
    shlana_9_2,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨣ\u{1a74}ᨷ\u{1a74}",
    &[
        "uni1A23",
        "uni1A74@592,0",
        "uni1A37@592,0",
        "uni1A74@1184,0"
    ]
);
shaping_test!(
    shlana_9_3,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨣ\u{1a5d}\u{1a74}\u{1a75}",
    &[
        "uni1A23",
        "uni1A5D@592,0",
        "uni1A74@592,0",
        "uni1A75@592,357"
    ]
);
shaping_test!(
    shlana_9_4,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨣ\u{1a74}\u{1a5d}\u{1a75}",
    &[
        "uni1A23",
        "uni1A74@592,0",
        "uni1A5D@592,0",
        "uni1A75@592,357"
    ]
);
shaping_test!(
    shlana_9_5,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨣ\u{1a5d}\u{1a74}",
    &["uni1A23", "uni1A5D@592,0", "uni1A74@592,0"]
);
shaping_test!(
    shlana_9_6,
    "tests/fonts/text-rendering/TestShapeLana.ttf",
    1000,
    &[("ned", 1), ("remove-default-ignorables", 1)],
    &[("font-funcs", 0.0)],
    "ᨣ\u{1a74}\u{1a5d}",
    &["uni1A23", "uni1A74@592,0", "uni1A5D@592,0"]
);
