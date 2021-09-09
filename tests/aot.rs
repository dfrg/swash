mod shaping;
shaping_test!(
    classdef1_1,
    "tests\\fonts\\aot\\classdef1_font4.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}",
    &["17", "18", "19", "20", "21"],
    false,
    false
);
shaping_test!(
    classdef1_empty_1,
    "tests\\fonts\\aot\\classdef1_font2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}",
    &["17", "23", "24", "25", "21"],
    false,
    false
);
shaping_test!(classdef1_multiple_1, "tests\\fonts\\aot\\classdef1_font3.otf", 75, &[("no-clusters", 1)], &[("features", 0.0)], "\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$", &["20", "23", "24", "25", "24", "26", "27", "28", "28", "29", "30", "31", "34", "33", "34", "35", "37", "38", "38", "39"], false, false);
shaping_test!(
    classdef1_single_1,
    "tests\\fonts\\aot\\classdef2_font1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}",
    &["17", "23", "24", "25", "21"],
    false,
    false
);
shaping_test!(
    classdef2_1,
    "tests\\fonts\\aot\\classdef2_font4.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}",
    &["17", "18", "19", "20", "21"],
    false,
    false
);
shaping_test!(
    classdef2_empty_1,
    "tests\\fonts\\aot\\classdef2_font2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}",
    &["17", "23", "24", "25", "21"],
    false,
    false
);
shaping_test!(classdef2_multiple_1, "tests\\fonts\\aot\\classdef2_font3.otf", 75, &[("no-clusters", 1)], &[("features", 0.0)], "\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$", &["20", "23", "24", "25", "24", "26", "27", "28", "28", "29", "30", "31", "34", "33", "34", "35", "37", "38", "38", "39"], false, false);
shaping_test!(
    classdef2_single_1,
    "tests\\fonts\\aot\\classdef2_font1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}",
    &["17", "23", "24", "25", "21"],
    false,
    false
);
shaping_test!(
    cmap0_1,
    "tests\\fonts\\aot\\cmap0_font1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0), ("font-funcs", 0.0)],
    "\u{0}\u{1}34567\u{ffff}",
    &["0", "0", "0", "17", "56", "12", "0", "0"],
    false,
    false
);
shaping_test!(
    cmap10_1,
    "tests\\fonts\\aot\\cmap10_font1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0), ("font-funcs", 0.0)],
    "\u{0}\u{1}鈲\u{109422}\u{109423}\u{109424}\u{109425}\u{ffff}",
    &["0", "0", "0", "0", "26", "27", "32", "0"],
    false,
    false
);
shaping_test!(
    cmap10_2,
    "tests\\fonts\\aot\\cmap10_font2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0), ("font-funcs", 0.0)],
    "\u{0}\u{1}!\"#$%\u{ffff}",
    &["0", "0", "0", "0", "0", "0", "0", "0"],
    false,
    false
);
shaping_test!(cmap12_1, "tests\\fonts\\aot\\cmap12_font1.otf", 75, &[("no-clusters", 1)], &[("features", 0.0), ("font-funcs", 0.0)], "\u{0}\u{1}\u{10}\u{101723}\u{101724}\u{101727}\u{101728}\u{102522}\u{102523}\u{102527}\u{102528}\u{ffff}", &["0", "0", "0", "23", "24", "27", "0", "0", "53", "57", "0", "0"], false, false);
shaping_test!(
    cmap4_1,
    "tests\\fonts\\aot\\cmap4_font1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0), ("font-funcs", 0.0)],
    "\u{0}\u{1}\u{10}\u{11}\u{12}\u{1e}\u{1f}ÇÈÍÒÓ\u{ffff}",
    &["0", "0", "0", "40", "41", "53", "0", "0", "256", "261", "266", "0", "0"],
    false,
    false
);
shaping_test!(
    cmap4_2,
    "tests\\fonts\\aot\\cmap4_font2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0), ("font-funcs", 0.0)],
    "\u{0}\u{1}\u{10}\u{11}\u{12}\u{1e}\u{1f}ÇÈÍÒÓ\u{ffff}",
    &["0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0"],
    false,
    false
);
shaping_test!(
    cmap4_3,
    "tests\\fonts\\aot\\cmap4_font3.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0), ("font-funcs", 0.0)],
    "\u{0}\u{1}\u{10}\u{11}\u{12}\u{1e}\u{1f}ÇÈÍÒÓ\u{ffff}",
    &["0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "65534"],
    false,
    false
);
shaping_test!(
    cmap4_4,
    "tests\\fonts\\aot\\cmap4_font4.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0), ("font-funcs", 0.0)],
    "\u{0}꿇꿈꿉뀫뀬뀭",
    &["0", "0", "44500", "44501", "44599", "44600", "0"],
    false,
    false
);
shaping_test!(
    cmap4_5,
    "tests\\fonts\\aot\\cmap4_font4.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0), ("font-funcs", 0.0)],
    "\u{0}cdǳǴǵϨϩ",
    &["0", "0", "65136", "65535", "0", "1", "500", "0"],
    false,
    false
);
shaping_test!(
    cmap6_1,
    "tests\\fonts\\aot\\cmap6_font1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0), ("font-funcs", 0.0)],
    "\u{0}\u{1}!\"#$%\u{ffff}",
    &["0", "0", "0", "17", "56", "12", "0", "0"],
    false,
    false
);
shaping_test!(
    cmap6_2,
    "tests\\fonts\\aot\\cmap6_font2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0), ("font-funcs", 0.0)],
    "\u{0}\u{1}!\"#$%\u{ffff}",
    &["0", "0", "0", "0", "0", "0", "0", "0"],
    false,
    false
);
shaping_test!(
    gpos1_1_lookupflag_1,
    "tests\\fonts\\aot\\gpos1_1_lookupflag_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}",
    &["17", "18@1500,0", "19@3000,0", "20@4200,0", "21@6000,0"],
    true,
    false
);
shaping_test!(
    gpos1_1_simple_1,
    "tests\\fonts\\aot\\gpos1_1_simple_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}",
    &["17", "18@1300,0", "19@3000,0", "20@4300,0", "21@6000,0"],
    true,
    false
);
shaping_test!(
    gpos1_1_simple_2,
    "tests\\fonts\\aot\\gpos1_1_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}",
    &[
        "17",
        "18@1500,-200",
        "19@3000,0",
        "20@4500,-200",
        "21@6000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos1_1_simple_3,
    "tests\\fonts\\aot\\gpos1_1_simple_f3.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}",
    &["17", "18@1500,0", "19@2800,0", "20@4300,0", "21@5600,0"],
    true,
    false
);
shaping_test!(
    gpos1_2_1,
    "tests\\fonts\\aot\\gpos1_2_font1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}",
    &["17", "18@1300,0", "19@3000,0", "20@4200,0", "21@6000,0"],
    true,
    false
);
shaping_test!(
    gpos1_2_lookupflag_1,
    "tests\\fonts\\aot\\gpos1_2_font2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}",
    &["17", "18@1500,0", "19@3000,0", "20@4200,0", "21@6000,0"],
    true,
    false
);
shaping_test!(
    gpos2_1_1,
    "tests\\fonts\\aot\\gpos2_1_font6.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{11}\u{12}\u{14}\u{11}",
    &[
        "17",
        "18@1300,0",
        "19@3000,-100",
        "17@4500,0",
        "18@5700,0",
        "20@7500,-400",
        "17@9000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos2_1_2,
    "tests\\fonts\\aot\\gpos2_1_font7.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{11}\u{12}\u{14}\u{11}\u{15}\u{16}\u{11}",
    &[
        "17",
        "18@1300,0",
        "19@3000,-100",
        "17@4500,0",
        "18@5700,0",
        "20@7500,-400",
        "17@9000,0",
        "21@10000,0",
        "22@12000,-600",
        "17@13500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos2_1_lookupflag_1,
    "tests\\fonts\\aot\\gpos2_1_lookupflag_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{13}\u{14}\u{11}\u{13}\u{12}\u{14}\u{11}",
    &[
        "17",
        "19@1300,0",
        "20@3000,-100",
        "17@4500,0",
        "19@5800,0",
        "18@7500,0",
        "20@9000,-100",
        "17@10500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos2_1_lookupflag_2,
    "tests\\fonts\\aot\\gpos2_1_lookupflag_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{13}\u{14}\u{11}\u{13}\u{12}\u{14}\u{11}",
    &[
        "17",
        "19@1500,0",
        "20@2800,-100",
        "17@4300,0",
        "19@5800,0",
        "18@7100,0",
        "20@8600,-100",
        "17@10100,0"
    ],
    true,
    false
);
shaping_test!(
    gpos2_1_next_glyph_1,
    "tests\\fonts\\aot\\gpos2_1_next_glyph_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{12}\u{12}\u{12}\u{12}",
    &["18@-100,0", "18@1500,-100", "18@2900,0", "18@4500,-100"],
    true,
    false
);
shaping_test!(
    gpos2_1_next_glyph_2,
    "tests\\fonts\\aot\\gpos2_1_next_glyph_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{12}\u{12}\u{12}\u{12}",
    &["18@-100,0", "18@1400,0", "18@2900,0", "18@4500,0"],
    true,
    false
);
shaping_test!(
    gpos2_1_simple_1,
    "tests\\fonts\\aot\\gpos2_1_simple_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{11}\u{12}\u{14}",
    &[
        "17",
        "18@1300,0",
        "19@3000,-100",
        "17@4500,0",
        "18@6000,0",
        "20@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos2_1_simple_2,
    "tests\\fonts\\aot\\gpos2_1_simple_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}",
    &["17", "18@1500,0"],
    true,
    false
);
shaping_test!(
    gpos2_2_1,
    "tests\\fonts\\aot\\gpos2_2_font1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{11}\u{12}\u{14}",
    &[
        "17",
        "18@1300,0",
        "19@3000,-100",
        "17@4500,0",
        "18@6000,0",
        "20@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos2_2_2,
    "tests\\fonts\\aot\\gpos2_2_font2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{13}\u{14}\u{11}\u{13}\u{12}\u{14}\u{11}",
    &[
        "17",
        "19@1300,0",
        "20@3000,-100",
        "17@4500,0",
        "19@5800,0",
        "18@7500,0",
        "20@9000,-100",
        "17@10500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos2_2_3,
    "tests\\fonts\\aot\\gpos2_2_font3.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{13}\u{14}\u{11}\u{13}\u{12}\u{14}\u{11}",
    &[
        "17",
        "19@1500,0",
        "20@2800,-100",
        "17@4300,0",
        "19@5800,0",
        "18@7100,0",
        "20@8600,-100",
        "17@10100,0"
    ],
    true,
    false
);
shaping_test!(
    gpos2_2_4,
    "tests\\fonts\\aot\\gpos2_2_font4.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{12}\u{12}\u{12}\u{12}",
    &["18@-100,0", "18@1500,-100", "18@2900,0", "18@4500,-100"],
    true,
    false
);
shaping_test!(
    gpos2_2_5,
    "tests\\fonts\\aot\\gpos2_2_font5.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{12}\u{12}\u{12}\u{12}",
    &["18@-100,0", "18@1400,0", "18@2900,0", "18@4500,0"],
    true,
    false
);
shaping_test!(
    gpos3_1,
    "tests\\fonts\\aot\\gpos3_font1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{11}\u{13}\u{11}",
    &["17", "18@1500,0", "17@3000,0", "19@4500,0", "17@6000,0"],
    true,
    false
);
shaping_test!(
    gpos3_2,
    "tests\\fonts\\aot\\gpos3_font3.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{14}\u{11}",
    &["17", "18@1500,0", "20@3000,0", "17@4500,0"],
    true,
    false
);
shaping_test!(
    gpos3_3,
    "tests\\fonts\\aot\\gpos3_font3.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{13}\u{12}\u{11}",
    &["17", "19@1500,0", "18@3000,0", "17@4500,0"],
    true,
    false
);
shaping_test!(
    gpos3_4,
    "tests\\fonts\\aot\\gpos3_font3.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{13}\u{14}\u{11}",
    &["17", "19@1500,0", "20@3000,0", "17@4500,0"],
    true,
    false
);
shaping_test!(
    gpos3_5,
    "tests\\fonts\\aot\\gpos3_font3.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}",
    &["17", "18@1500,0"],
    true,
    false
);
shaping_test!(
    gpos3_6,
    "tests\\fonts\\aot\\gpos3_font3.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{15}\u{15}\u{15}",
    &["17", "18@1500,0", "21@3000,0", "21@4500,0", "21@6000,0"],
    true,
    false
);
shaping_test!(
    gpos4_lookupflag_1,
    "tests\\fonts\\aot\\gpos4_lookupflag_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{11}\u{13}\u{11}",
    &["17", "18@1500,0", "17@3000,0", "19@4500,0", "17@4500,0"],
    true,
    false
);
shaping_test!(
    gpos4_lookupflag_2,
    "tests\\fonts\\aot\\gpos4_lookupflag_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{11}",
    &["17", "18@1500,0", "19@3000,0", "17@3000,0"],
    true,
    false
);
shaping_test!(
    gpos4_simple_1,
    "tests\\fonts\\aot\\gpos4_simple_1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{11}\u{13}\u{11}",
    &["17", "17@1500,0", "19@3000,0", "17@3000,0"],
    true,
    false
);
shaping_test!(
    gpos4_simple_2,
    "tests\\fonts\\aot\\gpos4_simple_1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{19}\u{19}\u{13}\u{11}",
    &["25", "25@1500,0", "19@3000,0", "17@3000,0"],
    true,
    false
);
shaping_test!(
    gpos5_1,
    "tests\\fonts\\aot\\gpos5_font1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{1e}\u{13}\u{1f}\u{11}",
    &["17", "18@1500,0", "19@1400,-80", "17@3000,0"],
    true,
    false
);
shaping_test!(
    gpos6_1,
    "tests\\fonts\\aot\\gpos6_font1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{11}\u{13}\u{11}",
    &["17", "17@1500,0", "19@3000,0", "17@3000,0"],
    true,
    false
);
shaping_test!(
    gpos6_2,
    "tests\\fonts\\aot\\gpos6_font1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{14}\u{14}\u{13}\u{11}",
    &["20", "20", "19", "17"],
    true,
    false
);
shaping_test!(
    gpos7_1_1,
    "tests\\fonts\\aot\\gpos7_1_font1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}",
    &["17", "18@1600,0", "19@3200,0", "20@4800,0", "21@6000,0"],
    true,
    false
);
shaping_test!(
    gpos7_1_2,
    "tests\\fonts\\aot\\gpos7_1_font1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{11}\u{12}\u{13}\u{11}",
    &[
        "17",
        "18@1500,0",
        "17@3000,0",
        "18@4500,0",
        "19@6000,0",
        "17@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos9_1,
    "tests\\fonts\\aot\\gpos9_font1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}",
    &["17", "18@1300,0", "19@3000,0", "20@4300,0", "21@6000,0"],
    true,
    false
);
shaping_test!(
    gpos9_2,
    "tests\\fonts\\aot\\gpos9_font2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}\u{11}",
    &[
        "17",
        "18@1300,0",
        "19@2700,0",
        "20@4300,0",
        "21@5700,0",
        "17@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining1_boundary_1,
    "tests\\fonts\\aot\\gpos_chaining1_boundary_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4500,0",
        "23@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining1_boundary_2,
    "tests\\fonts\\aot\\gpos_chaining1_boundary_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3020,0",
        "22@4500,0",
        "23@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining1_boundary_3,
    "tests\\fonts\\aot\\gpos_chaining1_boundary_f3.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3020,0",
        "22@4500,0",
        "23@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining1_boundary_4,
    "tests\\fonts\\aot\\gpos_chaining1_boundary_f4.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4520,0",
        "23@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining1_lookupflag_1,
    "tests\\fonts\\aot\\gpos_chaining1_lookupflag_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}Z\u{15}[\u{16}\\\u{17}]^\u{18}Z\u{19}[\u{1a}\u{0}",
    &[
        "0",
        "20@1500,0",
        "90@3000,0",
        "21@3000,0",
        "91@4500,0",
        "22@4500,0",
        "92@6000,0",
        "23@6020,0",
        "93@7500,0",
        "94@7500,0",
        "24@7500,0",
        "90@9000,0",
        "25@9000,0",
        "91@10500,0",
        "26@10500,0",
        "0@12000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining1_multiple_subrules_1,
    "tests\\fonts\\aot\\gpos_chaining1_multiple_subrules_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3020,0",
        "22@4500,0",
        "23@6000,0",
        "24@7500,0",
        "0@9000,0",
        "20@10500,0",
        "21@12000,0",
        "22@13520,0",
        "23@15000,0",
        "0@16500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining1_multiple_subrules_2,
    "tests\\fonts\\aot\\gpos_chaining1_multiple_subrules_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4520,0",
        "23@6000,0",
        "24@7500,0",
        "0@9000,0",
        "20@10500,0",
        "21@12000,0",
        "22@13520,0",
        "23@15000,0",
        "0@16500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining1_next_glyph_1,
    "tests\\fonts\\aot\\gpos_chaining1_next_glyph_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3020,0",
        "22@4520,0",
        "23@6020,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining1_simple_1,
    "tests\\fonts\\aot\\gpos_chaining1_simple_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3020,0",
        "22@4520,0",
        "23@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining1_simple_2,
    "tests\\fonts\\aot\\gpos_chaining1_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4520,0",
        "23@6000,0",
        "24@7500,0",
        "25@9000,0",
        "26@10500,0",
        "0@12000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining1_simple_3,
    "tests\\fonts\\aot\\gpos_chaining1_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{0}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4500,0",
        "23@6000,0",
        "24@7500,0",
        "25@9000,0",
        "0@10500,0",
        "0@12000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining1_simple_4,
    "tests\\fonts\\aot\\gpos_chaining1_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4500,0",
        "23@6000,0",
        "24@7500,0",
        "25@9000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining1_simple_5,
    "tests\\fonts\\aot\\gpos_chaining1_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4500,0",
        "23@6000,0",
        "24@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining1_simple_6,
    "tests\\fonts\\aot\\gpos_chaining1_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{0}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &[
        "0",
        "0@1500,0",
        "21@3000,0",
        "22@4500,0",
        "23@6000,0",
        "24@7500,0",
        "25@9000,0",
        "26@10500,0",
        "0@12000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining1_simple_7,
    "tests\\fonts\\aot\\gpos_chaining1_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &[
        "21",
        "22@1500,0",
        "23@3000,0",
        "24@4500,0",
        "25@6000,0",
        "26@7500,0",
        "0@9000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining1_simple_8,
    "tests\\fonts\\aot\\gpos_chaining1_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &[
        "22",
        "23@1500,0",
        "24@3000,0",
        "25@4500,0",
        "26@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining1_simple_9,
    "tests\\fonts\\aot\\gpos_chaining1_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}\u{18}\u{19}\u{1a}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4500,0",
        "0@6000,0",
        "24@7500,0",
        "25@9000,0",
        "26@10500,0",
        "0@12000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining1_simple_10,
    "tests\\fonts\\aot\\gpos_chaining1_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}",
    &["0", "20@1500,0", "21@3000,0", "22@4500,0", "23@6000,0"],
    true,
    false
);
shaping_test!(
    gpos_chaining1_simple_11,
    "tests\\fonts\\aot\\gpos_chaining1_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}",
    &["0", "20@1500,0", "21@3000,0", "22@4500,0"],
    true,
    false
);
shaping_test!(
    gpos_chaining1_successive_1,
    "tests\\fonts\\aot\\gpos_chaining1_successive_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{19}\u{14}\u{15}\u{16}\u{17}\u{18}\u{0}",
    &[
        "0",
        "25@1500,0",
        "20@3000,0",
        "21@4520,0",
        "22@6020,0",
        "23@7500,0",
        "24@9000,0",
        "0@10500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining2_boundary_1,
    "tests\\fonts\\aot\\gpos_chaining2_boundary_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4500,0",
        "23@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining2_boundary_2,
    "tests\\fonts\\aot\\gpos_chaining2_boundary_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3020,0",
        "22@4500,0",
        "23@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining2_boundary_3,
    "tests\\fonts\\aot\\gpos_chaining2_boundary_f3.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3020,0",
        "22@4500,0",
        "23@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining2_boundary_4,
    "tests\\fonts\\aot\\gpos_chaining2_boundary_f4.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4520,0",
        "23@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining2_lookupflag_1,
    "tests\\fonts\\aot\\gpos_chaining2_lookupflag_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}Z\u{15}[\u{16}\\\u{17}]^\u{18}Z\u{19}[\u{1a}\u{0}",
    &[
        "0",
        "20@1500,0",
        "90@3000,0",
        "21@3000,0",
        "91@4500,0",
        "22@4500,0",
        "92@6000,0",
        "23@6020,0",
        "93@7500,0",
        "94@7500,0",
        "24@7500,0",
        "90@9000,0",
        "25@9000,0",
        "91@10500,0",
        "26@10500,0",
        "0@12000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining2_multiple_subrules_1,
    "tests\\fonts\\aot\\gpos_chaining2_multiple_subrules_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3020,0",
        "22@4500,0",
        "23@6000,0",
        "24@7500,0",
        "0@9000,0",
        "20@10500,0",
        "21@12000,0",
        "22@13520,0",
        "23@15000,0",
        "0@16500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining2_multiple_subrules_2,
    "tests\\fonts\\aot\\gpos_chaining2_multiple_subrules_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4520,0",
        "23@6000,0",
        "24@7500,0",
        "0@9000,0",
        "20@10500,0",
        "21@12000,0",
        "22@13520,0",
        "23@15000,0",
        "0@16500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining2_next_glyph_1,
    "tests\\fonts\\aot\\gpos_chaining2_next_glyph_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3020,0",
        "22@4520,0",
        "23@6020,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining2_simple_1,
    "tests\\fonts\\aot\\gpos_chaining2_simple_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3020,0",
        "22@4520,0",
        "23@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining2_simple_2,
    "tests\\fonts\\aot\\gpos_chaining2_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4520,0",
        "23@6000,0",
        "24@7500,0",
        "25@9000,0",
        "26@10500,0",
        "0@12000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining2_simple_3,
    "tests\\fonts\\aot\\gpos_chaining2_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{0}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4500,0",
        "23@6000,0",
        "24@7500,0",
        "25@9000,0",
        "0@10500,0",
        "0@12000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining2_simple_4,
    "tests\\fonts\\aot\\gpos_chaining2_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4500,0",
        "23@6000,0",
        "24@7500,0",
        "25@9000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining2_simple_5,
    "tests\\fonts\\aot\\gpos_chaining2_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4500,0",
        "23@6000,0",
        "24@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining2_simple_6,
    "tests\\fonts\\aot\\gpos_chaining2_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{0}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &[
        "0",
        "0@1500,0",
        "21@3000,0",
        "22@4500,0",
        "23@6000,0",
        "24@7500,0",
        "25@9000,0",
        "26@10500,0",
        "0@12000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining2_simple_7,
    "tests\\fonts\\aot\\gpos_chaining2_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &[
        "21",
        "22@1500,0",
        "23@3000,0",
        "24@4500,0",
        "25@6000,0",
        "26@7500,0",
        "0@9000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining2_simple_8,
    "tests\\fonts\\aot\\gpos_chaining2_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &[
        "22",
        "23@1500,0",
        "24@3000,0",
        "25@4500,0",
        "26@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining2_simple_9,
    "tests\\fonts\\aot\\gpos_chaining2_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}\u{18}\u{19}\u{1a}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4500,0",
        "0@6000,0",
        "24@7500,0",
        "25@9000,0",
        "26@10500,0",
        "0@12000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining2_simple_10,
    "tests\\fonts\\aot\\gpos_chaining2_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}",
    &["0", "20@1500,0", "21@3000,0", "22@4500,0", "23@6000,0"],
    true,
    false
);
shaping_test!(
    gpos_chaining2_simple_11,
    "tests\\fonts\\aot\\gpos_chaining2_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}",
    &["0", "20@1500,0", "21@3000,0", "22@4500,0"],
    true,
    false
);
shaping_test!(
    gpos_chaining2_successive_1,
    "tests\\fonts\\aot\\gpos_chaining2_successive_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{19}\u{14}\u{15}\u{16}\u{17}\u{18}\u{0}",
    &[
        "0",
        "25@1500,0",
        "20@3000,0",
        "21@4520,0",
        "22@6020,0",
        "23@7500,0",
        "24@9000,0",
        "0@10500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining3_boundary_1,
    "tests\\fonts\\aot\\gpos_chaining3_boundary_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4500,0",
        "23@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining3_boundary_2,
    "tests\\fonts\\aot\\gpos_chaining3_boundary_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3020,0",
        "22@4500,0",
        "23@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining3_boundary_3,
    "tests\\fonts\\aot\\gpos_chaining3_boundary_f3.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3020,0",
        "22@4500,0",
        "23@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining3_boundary_4,
    "tests\\fonts\\aot\\gpos_chaining3_boundary_f4.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4520,0",
        "23@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining3_lookupflag_1,
    "tests\\fonts\\aot\\gpos_chaining3_lookupflag_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}Z\u{15}[\u{16}\\\u{17}]^\u{18}Z\u{19}[\u{1a}\u{0}",
    &[
        "0",
        "20@1500,0",
        "90@3000,0",
        "21@3000,0",
        "91@4500,0",
        "22@4500,0",
        "92@6000,0",
        "23@6020,0",
        "93@7500,0",
        "94@7500,0",
        "24@7500,0",
        "90@9000,0",
        "25@9000,0",
        "91@10500,0",
        "26@10500,0",
        "0@12000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining3_next_glyph_1,
    "tests\\fonts\\aot\\gpos_chaining3_next_glyph_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{16}\u{15}\u{16}\u{15}\u{16}\u{15}\u{0}",
    &[
        "0",
        "22@1500,0",
        "21@3020,0",
        "22@4500,0",
        "21@6020,0",
        "22@7500,0",
        "21@9000,0",
        "0@10500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining3_simple_1,
    "tests\\fonts\\aot\\gpos_chaining3_simple_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3020,0",
        "22@4520,0",
        "23@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining3_simple_2,
    "tests\\fonts\\aot\\gpos_chaining3_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4520,0",
        "23@6000,0",
        "24@7500,0",
        "25@9000,0",
        "26@10500,0",
        "0@12000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining3_simple_3,
    "tests\\fonts\\aot\\gpos_chaining3_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{0}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4500,0",
        "23@6000,0",
        "24@7500,0",
        "25@9000,0",
        "0@10500,0",
        "0@12000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining3_simple_4,
    "tests\\fonts\\aot\\gpos_chaining3_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4500,0",
        "23@6000,0",
        "24@7500,0",
        "25@9000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining3_simple_5,
    "tests\\fonts\\aot\\gpos_chaining3_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4500,0",
        "23@6000,0",
        "24@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining3_simple_6,
    "tests\\fonts\\aot\\gpos_chaining3_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{0}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &[
        "0",
        "0@1500,0",
        "21@3000,0",
        "22@4500,0",
        "23@6000,0",
        "24@7500,0",
        "25@9000,0",
        "26@10500,0",
        "0@12000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining3_simple_7,
    "tests\\fonts\\aot\\gpos_chaining3_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &[
        "21",
        "22@1500,0",
        "23@3000,0",
        "24@4500,0",
        "25@6000,0",
        "26@7500,0",
        "0@9000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining3_simple_8,
    "tests\\fonts\\aot\\gpos_chaining3_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &[
        "22",
        "23@1500,0",
        "24@3000,0",
        "25@4500,0",
        "26@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining3_simple_9,
    "tests\\fonts\\aot\\gpos_chaining3_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}\u{18}\u{19}\u{1a}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3000,0",
        "22@4500,0",
        "0@6000,0",
        "24@7500,0",
        "25@9000,0",
        "26@10500,0",
        "0@12000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_chaining3_simple_10,
    "tests\\fonts\\aot\\gpos_chaining3_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}",
    &["0", "20@1500,0", "21@3000,0", "22@4500,0", "23@6000,0"],
    true,
    false
);
shaping_test!(
    gpos_chaining3_simple_11,
    "tests\\fonts\\aot\\gpos_chaining3_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}",
    &["0", "20@1500,0", "21@3000,0", "22@4500,0"],
    true,
    false
);
shaping_test!(
    gpos_chaining3_successive_1,
    "tests\\fonts\\aot\\gpos_chaining3_successive_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{19}\u{14}\u{15}\u{16}\u{17}\u{18}\u{0}",
    &[
        "0",
        "25@1500,0",
        "20@3000,0",
        "21@4520,0",
        "22@6020,0",
        "23@7500,0",
        "24@9000,0",
        "0@10500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context1_boundary_1,
    "tests\\fonts\\aot\\gpos_context1_boundary_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &[
        "0",
        "20@1500,0",
        "20@3000,0",
        "20@4500,0",
        "20@6000,0",
        "20@7500,0",
        "0@9000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context1_boundary_2,
    "tests\\fonts\\aot\\gpos_context1_boundary_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &[
        "0",
        "20@1520,0",
        "20@3020,0",
        "20@4520,0",
        "20@6020,0",
        "20@7520,0",
        "0@9000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context1_expansion_1,
    "tests\\fonts\\aot\\gpos_context1_expansion_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}",
    &["0", "20@1500,0", "21@3000,0", "22@4500,0", "0@6000,0"],
    true,
    false
);
shaping_test!(
    gpos_context1_lookupflag_1,
    "tests\\fonts\\aot\\gpos_context1_lookupflag_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}Z\u{15}[\\\u{16}\u{0}",
    &[
        "0",
        "20@1520,0",
        "90@3000,0",
        "21@3020,0",
        "91@4500,0",
        "92@4500,0",
        "22@4520,0",
        "0@6000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context1_lookupflag_2,
    "tests\\fonts\\aot\\gpos_context1_lookupflag_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}Z\u{15}[\\\u{16}\u{0}",
    &[
        "0",
        "20@1500,0",
        "90@3000,0",
        "21@3020,0",
        "91@4500,0",
        "92@4500,0",
        "22@4500,0",
        "0@6000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context1_multiple_subrules_1,
    "tests\\fonts\\aot\\gpos_context1_multiple_subrules_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}\u{14}\u{15}\u{0}",
    &[
        "0",
        "20@1520,0",
        "21@3000,0",
        "22@4500,0",
        "0@6000,0",
        "20@7500,0",
        "21@9020,0",
        "0@10500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context1_multiple_subrules_2,
    "tests\\fonts\\aot\\gpos_context1_multiple_subrules_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}\u{14}\u{15}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3020,0",
        "22@4500,0",
        "0@6000,0",
        "20@7500,0",
        "21@9020,0",
        "0@10500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context1_next_glyph_1,
    "tests\\fonts\\aot\\gpos_context1_next_glyph_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &[
        "0",
        "20@1520,0",
        "20@3000,0",
        "20@4520,0",
        "20@6000,0",
        "20@7500,0",
        "0@9000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context1_simple_1,
    "tests\\fonts\\aot\\gpos_context1_simple_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}",
    &["0", "20@1520,0", "21@3020,0", "22@4520,0", "0@6000,0"],
    true,
    false
);
shaping_test!(
    gpos_context1_simple_2,
    "tests\\fonts\\aot\\gpos_context1_simple_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{0}\u{14}\u{15}\u{0}",
    &[
        "0",
        "20@1500,0",
        "0@3000,0",
        "20@4500,0",
        "21@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context1_simple_3,
    "tests\\fonts\\aot\\gpos_context1_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &[
        "0",
        "20@1500,0",
        "20@3020,0",
        "20@4500,0",
        "20@6000,0",
        "20@7500,0",
        "0@9000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context1_successive_1,
    "tests\\fonts\\aot\\gpos_context1_successive_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3020,0",
        "22@4520,0",
        "23@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context2_boundary_1,
    "tests\\fonts\\aot\\gpos_context2_boundary_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &[
        "0",
        "20@1500,0",
        "20@3000,0",
        "20@4500,0",
        "20@6000,0",
        "20@7500,0",
        "0@9000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context2_boundary_2,
    "tests\\fonts\\aot\\gpos_context2_boundary_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &[
        "0",
        "20@1520,0",
        "20@3020,0",
        "20@4520,0",
        "20@6020,0",
        "20@7520,0",
        "0@9000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context2_classes_1,
    "tests\\fonts\\aot\\gpos_context2_classes_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{1a}\u{1c}\u{18}\u{0}\u{15}\u{1b}\u{1a}\u{18}\u{0}\u{16}\u{1b}\u{1a}\u{18}",
    &[
        "0",
        "20@1500,0",
        "26@3020,0",
        "28@4500,0",
        "24@6000,0",
        "0@7500,0",
        "21@9000,0",
        "27@10520,0",
        "26@12000,0",
        "24@13500,0",
        "0@15000,0",
        "22@16500,0",
        "27@18000,0",
        "26@19500,0",
        "24@21000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context2_classes_2,
    "tests\\fonts\\aot\\gpos_context2_classes_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{16}\u{1b}\u{1a}\u{18}\u{0}\u{18}\u{18}\u{1d}\u{16}\u{0}\u{16}\u{1b}\u{1a}\u{18}",
    &[
        "0",
        "22@1500,0",
        "27@3020,0",
        "26@4500,0",
        "24@6000,0",
        "0@7500,0",
        "24@9000,0",
        "24@10500,0",
        "29@12020,0",
        "22@13500,0",
        "0@15000,0",
        "22@16500,0",
        "27@18020,0",
        "26@19500,0",
        "24@21000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context2_expansion_1,
    "tests\\fonts\\aot\\gpos_context2_expansion_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}",
    &["0", "20@1500,0", "21@3000,0", "22@4500,0", "0@6000,0"],
    true,
    false
);
shaping_test!(
    gpos_context2_lookupflag_1,
    "tests\\fonts\\aot\\gpos_context2_lookupflag_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}Z\u{15}[\\\u{16}\u{0}",
    &[
        "0",
        "20@1520,0",
        "90@3000,0",
        "21@3020,0",
        "91@4500,0",
        "92@4500,0",
        "22@4520,0",
        "0@6000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context2_lookupflag_2,
    "tests\\fonts\\aot\\gpos_context2_lookupflag_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}Z\u{15}[\\\u{16}\u{0}",
    &[
        "0",
        "20@1500,0",
        "90@3000,0",
        "21@3020,0",
        "91@4500,0",
        "92@4500,0",
        "22@4500,0",
        "0@6000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context2_multiple_subrules_1,
    "tests\\fonts\\aot\\gpos_context2_multiple_subrules_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}\u{14}\u{15}\u{0}",
    &[
        "0",
        "20@1520,0",
        "21@3000,0",
        "22@4500,0",
        "0@6000,0",
        "20@7500,0",
        "21@9020,0",
        "0@10500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context2_multiple_subrules_2,
    "tests\\fonts\\aot\\gpos_context2_multiple_subrules_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}\u{14}\u{15}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3020,0",
        "22@4500,0",
        "0@6000,0",
        "20@7500,0",
        "21@9020,0",
        "0@10500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context2_next_glyph_1,
    "tests\\fonts\\aot\\gpos_context2_next_glyph_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &[
        "0",
        "20@1520,0",
        "20@3000,0",
        "20@4520,0",
        "20@6000,0",
        "20@7500,0",
        "0@9000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context2_simple_1,
    "tests\\fonts\\aot\\gpos_context2_simple_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}",
    &["0", "20@1520,0", "21@3020,0", "22@4520,0", "0@6000,0"],
    true,
    false
);
shaping_test!(
    gpos_context2_simple_2,
    "tests\\fonts\\aot\\gpos_context2_simple_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{0}\u{14}\u{15}\u{0}",
    &[
        "0",
        "20@1500,0",
        "0@3000,0",
        "20@4500,0",
        "21@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context2_simple_3,
    "tests\\fonts\\aot\\gpos_context2_simple_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &[
        "0",
        "20@1500,0",
        "20@3020,0",
        "20@4500,0",
        "20@6000,0",
        "20@7500,0",
        "0@9000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context2_successive_1,
    "tests\\fonts\\aot\\gpos_context2_successive_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3020,0",
        "22@4520,0",
        "23@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context3_boundary_1,
    "tests\\fonts\\aot\\gpos_context3_boundary_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &[
        "0",
        "20@1500,0",
        "20@3000,0",
        "20@4500,0",
        "20@6000,0",
        "20@7500,0",
        "0@9000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context3_boundary_2,
    "tests\\fonts\\aot\\gpos_context3_boundary_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &[
        "0",
        "20@1520,0",
        "20@3020,0",
        "20@4520,0",
        "20@6020,0",
        "20@7520,0",
        "0@9000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context3_lookupflag_1,
    "tests\\fonts\\aot\\gpos_context3_lookupflag_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}Z\u{15}[\\\u{16}\u{0}",
    &[
        "0",
        "20@1520,0",
        "90@3000,0",
        "21@3020,0",
        "91@4500,0",
        "92@4500,0",
        "22@4520,0",
        "0@6000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context3_lookupflag_2,
    "tests\\fonts\\aot\\gpos_context3_lookupflag_f2.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}Z\u{15}[\\\u{16}\u{0}",
    &[
        "0",
        "20@1500,0",
        "90@3000,0",
        "21@3020,0",
        "91@4500,0",
        "92@4500,0",
        "22@4500,0",
        "0@6000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context3_next_glyph_1,
    "tests\\fonts\\aot\\gpos_context3_next_glyph_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &[
        "0",
        "20@1520,0",
        "20@3000,0",
        "20@4520,0",
        "20@6000,0",
        "20@7500,0",
        "0@9000,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context3_simple_1,
    "tests\\fonts\\aot\\gpos_context3_simple_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}",
    &["0", "20@1520,0", "21@3020,0", "22@4520,0", "0@6000,0"],
    true,
    false
);
shaping_test!(
    gpos_context3_simple_2,
    "tests\\fonts\\aot\\gpos_context3_simple_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{0}\u{14}\u{15}\u{0}\u{14}\u{15}\u{16}\u{0}",
    &[
        "0",
        "20@1500,0",
        "0@3000,0",
        "20@4500,0",
        "21@6000,0",
        "0@7500,0",
        "20@9020,0",
        "21@10520,0",
        "22@12020,0",
        "0@13500,0"
    ],
    true,
    false
);
shaping_test!(
    gpos_context3_successive_1,
    "tests\\fonts\\aot\\gpos_context3_successive_f1.otf",
    75,
    &[("no-clusters", 1), ("ned", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &[
        "0",
        "20@1500,0",
        "21@3020,0",
        "22@4520,0",
        "23@6000,0",
        "0@7500,0"
    ],
    true,
    false
);
shaping_test!(
    gsub1_1_lookupflag_1,
    "tests\\fonts\\aot\\gsub1_1_lookupflag_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}",
    &["17", "18", "24", "20", "21"],
    false,
    false
);
shaping_test!(
    gsub1_1_modulo_1,
    "tests\\fonts\\aot\\gsub1_1_modulo_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}",
    &["17", "18", "17", "24", "23", "18", "23", "24"],
    false,
    false
);
shaping_test!(
    gsub1_1_simple_1,
    "tests\\fonts\\aot\\gsub1_1_simple_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}",
    &["17", "23", "24", "20", "21"],
    false,
    false
);
shaping_test!(
    gsub1_2_lookupflag_1,
    "tests\\fonts\\aot\\gsub1_2_lookupflag_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}",
    &["17", "18", "19", "25", "21"],
    false,
    false
);
shaping_test!(
    gsub1_2_simple_1,
    "tests\\fonts\\aot\\gsub1_2_simple_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}",
    &["17", "22", "19", "25", "21"],
    false,
    false
);
shaping_test!(
    gsub2_1_lookupflag_1,
    "tests\\fonts\\aot\\gsub2_1_lookupflag_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{11}",
    &["17", "18", "22", "23", "17"],
    false,
    false
);
shaping_test!(
    gsub2_1_multiple_sequences_1,
    "tests\\fonts\\aot\\gsub2_1_multiple_sequences_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{11}",
    &["17", "20", "21", "22", "23", "17"],
    false,
    false
);
shaping_test!(
    gsub2_1_simple_1,
    "tests\\fonts\\aot\\gsub2_1_simple_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}",
    &["17", "20", "21", "22", "19"],
    false,
    false
);
shaping_test!(
    gsub2_1_simple_2,
    "tests\\fonts\\aot\\gsub2_1_simple_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{12}",
    &["17", "20", "21", "22", "19", "20", "21", "22"],
    false,
    false
);
shaping_test!(
    gsub3_1_lookupflag_1,
    "tests\\fonts\\aot\\gsub3_1_lookupflag_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{12}\u{12}\u{13}\u{13}\u{13}\u{13}\u{11}",
    &["17", "18", "18", "18", "19", "22", "23", "19", "17"],
    false,
    false
);
shaping_test!(
    gsub3_1_multiple_1,
    "tests\\fonts\\aot\\gsub3_1_multiple_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{12}\u{12}\u{12}\u{13}\u{13}\u{13}\u{13}\u{11}",
    &["17", "18", "20", "21", "18", "19", "22", "23", "19", "17"],
    false,
    false
);
shaping_test!(
    gsub3_1_simple_1,
    "tests\\fonts\\aot\\gsub3_1_simple_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{11}\u{12}\u{11}\u{12}\u{11}\u{12}\u{11}\u{12}\u{11}\u{12}\u{11}",
    &["17", "18", "17", "20", "17", "21", "17", "22", "17", "18", "17", "20", "17"],
    false,
    false
);
shaping_test!(
    gsub4_1_lookupflag_1,
    "tests\\fonts\\aot\\gsub4_1_lookupflag_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{18}\u{12}\u{18}\u{13}\u{18}\u{18}\u{14}\u{18}\u{11}\u{12}\u{13}\u{16}\u{14}",
    &["17", "24", "23", "24", "24", "24", "24", "17", "18", "19", "22", "20"],
    false,
    false
);
shaping_test!(
    gsub4_1_multiple_ligatures_1,
    "tests\\fonts\\aot\\gsub4_1_multiple_ligatures_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{11}\u{12}\u{13}\u{16}\u{14}",
    &["17", "23", "17", "24", "22", "20"],
    false,
    false
);
shaping_test!(
    gsub4_1_multiple_ligatures_2,
    "tests\\fonts\\aot\\gsub4_1_multiple_ligatures_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{11}\u{12}\u{13}\u{16}\u{14}",
    &["17", "24", "20", "17", "24", "22", "20"],
    false,
    false
);
shaping_test!(
    gsub4_1_multiple_ligsets_1,
    "tests\\fonts\\aot\\gsub4_1_multiple_ligsets_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{15}\u{14}\u{13}\u{16}",
    &["17", "23", "21", "24", "22"],
    false,
    false
);
shaping_test!(
    gsub4_1_simple_1,
    "tests\\fonts\\aot\\gsub4_1_simple_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{11}\u{12}\u{13}\u{16}\u{14}",
    &["17", "23", "17", "18", "19", "22", "20"],
    false,
    false
);
shaping_test!(
    gsub7_1,
    "tests\\fonts\\aot\\gsub7_font1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}",
    &["17", "23", "24", "20", "21"],
    false,
    false
);
shaping_test!(
    gsub7_2,
    "tests\\fonts\\aot\\gsub7_font2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}",
    &["17", "23", "29", "20", "21"],
    false,
    false
);
shaping_test!(
    gsub_chaining1_boundary_1,
    "tests\\fonts\\aot\\gsub_chaining1_boundary_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "21", "22", "23", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining1_boundary_2,
    "tests\\fonts\\aot\\gsub_chaining1_boundary_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "61", "22", "23", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining1_boundary_3,
    "tests\\fonts\\aot\\gsub_chaining1_boundary_f3.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "61", "22", "23", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining1_boundary_4,
    "tests\\fonts\\aot\\gsub_chaining1_boundary_f4.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "21", "62", "23", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining1_lookupflag_1,
    "tests\\fonts\\aot\\gsub_chaining1_lookupflag_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}Z\u{15}[\u{16}\\\u{17}]^\u{18}Z\u{19}[\u{1a}\u{0}",
    &[
        "0", "20", "90", "21", "91", "22", "92", "63", "93", "94", "24", "90", "25", "91", "26",
        "0"
    ],
    false,
    false
);
shaping_test!(
    gsub_chaining1_multiple_subrules_1,
    "tests\\fonts\\aot\\gsub_chaining1_multiple_subrules_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "61", "22", "23", "24", "0", "20", "21", "62", "23", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining1_multiple_subrules_2,
    "tests\\fonts\\aot\\gsub_chaining1_multiple_subrules_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "21", "62", "23", "24", "0", "20", "21", "62", "23", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining1_next_glyph_1,
    "tests\\fonts\\aot\\gsub_chaining1_next_glyph_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "61", "62", "63", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining1_simple_1,
    "tests\\fonts\\aot\\gsub_chaining1_simple_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "61", "62", "23", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining1_simple_2,
    "tests\\fonts\\aot\\gsub_chaining1_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &["0", "20", "21", "62", "23", "24", "25", "26", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining1_simple_3,
    "tests\\fonts\\aot\\gsub_chaining1_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{0}\u{0}",
    &["0", "20", "21", "22", "23", "24", "25", "0", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining1_simple_4,
    "tests\\fonts\\aot\\gsub_chaining1_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}",
    &["0", "20", "21", "22", "23", "24", "25"],
    false,
    false
);
shaping_test!(
    gsub_chaining1_simple_5,
    "tests\\fonts\\aot\\gsub_chaining1_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}",
    &["0", "20", "21", "22", "23", "24"],
    false,
    false
);
shaping_test!(
    gsub_chaining1_simple_6,
    "tests\\fonts\\aot\\gsub_chaining1_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{0}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &["0", "0", "21", "22", "23", "24", "25", "26", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining1_simple_7,
    "tests\\fonts\\aot\\gsub_chaining1_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &["21", "22", "23", "24", "25", "26", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining1_simple_8,
    "tests\\fonts\\aot\\gsub_chaining1_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &["22", "23", "24", "25", "26", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining1_simple_9,
    "tests\\fonts\\aot\\gsub_chaining1_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}\u{18}\u{19}\u{1a}\u{0}",
    &["0", "20", "21", "22", "0", "24", "25", "26", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining1_simple_10,
    "tests\\fonts\\aot\\gsub_chaining1_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}",
    &["0", "20", "21", "22", "23"],
    false,
    false
);
shaping_test!(
    gsub_chaining1_simple_11,
    "tests\\fonts\\aot\\gsub_chaining1_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}",
    &["0", "20", "21", "22"],
    false,
    false
);
shaping_test!(
    gsub_chaining1_successive_1,
    "tests\\fonts\\aot\\gsub_chaining1_successive_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{19}\u{14}\u{15}\u{16}\u{17}\u{18}\u{0}",
    &["0", "25", "20", "61", "63", "24", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining2_boundary_1,
    "tests\\fonts\\aot\\gsub_chaining2_boundary_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "21", "22", "23", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining2_boundary_2,
    "tests\\fonts\\aot\\gsub_chaining2_boundary_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "61", "22", "23", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining2_boundary_3,
    "tests\\fonts\\aot\\gsub_chaining2_boundary_f3.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "61", "22", "23", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining2_boundary_4,
    "tests\\fonts\\aot\\gsub_chaining2_boundary_f4.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "21", "62", "23", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining2_lookupflag_1,
    "tests\\fonts\\aot\\gsub_chaining2_lookupflag_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}Z\u{15}[\u{16}\\\u{17}]^\u{18}Z\u{19}[\u{1a}\u{0}",
    &[
        "0", "20", "90", "21", "91", "22", "92", "63", "93", "94", "24", "90", "25", "91", "26",
        "0"
    ],
    false,
    false
);
shaping_test!(
    gsub_chaining2_multiple_subrules_1,
    "tests\\fonts\\aot\\gsub_chaining2_multiple_subrules_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "61", "22", "23", "24", "0", "20", "21", "62", "23", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining2_multiple_subrules_2,
    "tests\\fonts\\aot\\gsub_chaining2_multiple_subrules_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "21", "62", "23", "24", "0", "20", "21", "62", "23", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining2_next_glyph_1,
    "tests\\fonts\\aot\\gsub_chaining2_next_glyph_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "61", "62", "63", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining2_simple_1,
    "tests\\fonts\\aot\\gsub_chaining2_simple_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "61", "62", "23", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining2_simple_2,
    "tests\\fonts\\aot\\gsub_chaining2_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &["0", "20", "21", "62", "23", "24", "25", "26", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining2_simple_3,
    "tests\\fonts\\aot\\gsub_chaining2_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{0}\u{0}",
    &["0", "20", "21", "22", "23", "24", "25", "0", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining2_simple_4,
    "tests\\fonts\\aot\\gsub_chaining2_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}",
    &["0", "20", "21", "22", "23", "24", "25"],
    false,
    false
);
shaping_test!(
    gsub_chaining2_simple_5,
    "tests\\fonts\\aot\\gsub_chaining2_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}",
    &["0", "20", "21", "22", "23", "24"],
    false,
    false
);
shaping_test!(
    gsub_chaining2_simple_6,
    "tests\\fonts\\aot\\gsub_chaining2_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{0}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &["0", "0", "21", "22", "23", "24", "25", "26", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining2_simple_7,
    "tests\\fonts\\aot\\gsub_chaining2_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &["21", "22", "23", "24", "25", "26", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining2_simple_8,
    "tests\\fonts\\aot\\gsub_chaining2_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &["22", "23", "24", "25", "26", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining2_simple_9,
    "tests\\fonts\\aot\\gsub_chaining2_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}\u{18}\u{19}\u{1a}\u{0}",
    &["0", "20", "21", "22", "0", "24", "25", "26", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining2_simple_10,
    "tests\\fonts\\aot\\gsub_chaining2_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}",
    &["0", "20", "21", "22", "23"],
    false,
    false
);
shaping_test!(
    gsub_chaining2_simple_11,
    "tests\\fonts\\aot\\gsub_chaining2_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}",
    &["0", "20", "21", "22"],
    false,
    false
);
shaping_test!(
    gsub_chaining2_successive_1,
    "tests\\fonts\\aot\\gsub_chaining2_successive_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{19}\u{14}\u{15}\u{16}\u{17}\u{18}\u{0}",
    &["0", "25", "20", "61", "63", "24", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining3_boundary_1,
    "tests\\fonts\\aot\\gsub_chaining3_boundary_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "21", "22", "23", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining3_boundary_2,
    "tests\\fonts\\aot\\gsub_chaining3_boundary_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "61", "22", "23", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining3_boundary_3,
    "tests\\fonts\\aot\\gsub_chaining3_boundary_f3.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "61", "22", "23", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining3_boundary_4,
    "tests\\fonts\\aot\\gsub_chaining3_boundary_f4.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "21", "62", "23", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining3_lookupflag_1,
    "tests\\fonts\\aot\\gsub_chaining3_lookupflag_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}Z\u{15}[\u{16}\\\u{17}]^\u{18}Z\u{19}[\u{1a}\u{0}",
    &[
        "0", "20", "90", "21", "91", "22", "92", "63", "93", "94", "24", "90", "25", "91", "26",
        "0"
    ],
    false,
    false
);
shaping_test!(
    gsub_chaining3_next_glyph_1,
    "tests\\fonts\\aot\\gsub_chaining3_next_glyph_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{16}\u{15}\u{16}\u{15}\u{16}\u{15}\u{0}",
    &["0", "22", "61", "22", "61", "22", "21", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining3_simple_1,
    "tests\\fonts\\aot\\gsub_chaining3_simple_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "61", "62", "23", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining3_simple_2,
    "tests\\fonts\\aot\\gsub_chaining3_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &["0", "20", "21", "62", "23", "24", "25", "26", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining3_simple_3,
    "tests\\fonts\\aot\\gsub_chaining3_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{0}\u{0}",
    &["0", "20", "21", "22", "23", "24", "25", "0", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining3_simple_4,
    "tests\\fonts\\aot\\gsub_chaining3_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}",
    &["0", "20", "21", "22", "23", "24", "25"],
    false,
    false
);
shaping_test!(
    gsub_chaining3_simple_5,
    "tests\\fonts\\aot\\gsub_chaining3_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{18}",
    &["0", "20", "21", "22", "23", "24"],
    false,
    false
);
shaping_test!(
    gsub_chaining3_simple_6,
    "tests\\fonts\\aot\\gsub_chaining3_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{0}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &["0", "0", "21", "22", "23", "24", "25", "26", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining3_simple_7,
    "tests\\fonts\\aot\\gsub_chaining3_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &["21", "22", "23", "24", "25", "26", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining3_simple_8,
    "tests\\fonts\\aot\\gsub_chaining3_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{16}\u{17}\u{18}\u{19}\u{1a}\u{0}",
    &["22", "23", "24", "25", "26", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining3_simple_9,
    "tests\\fonts\\aot\\gsub_chaining3_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}\u{18}\u{19}\u{1a}\u{0}",
    &["0", "20", "21", "22", "0", "24", "25", "26", "0"],
    false,
    false
);
shaping_test!(
    gsub_chaining3_simple_10,
    "tests\\fonts\\aot\\gsub_chaining3_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}",
    &["0", "20", "21", "22", "23"],
    false,
    false
);
shaping_test!(
    gsub_chaining3_simple_11,
    "tests\\fonts\\aot\\gsub_chaining3_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}",
    &["0", "20", "21", "22"],
    false,
    false
);
shaping_test!(
    gsub_chaining3_successive_1,
    "tests\\fonts\\aot\\gsub_chaining3_successive_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{19}\u{14}\u{15}\u{16}\u{17}\u{18}\u{0}",
    &["0", "25", "20", "61", "63", "24", "0"],
    false,
    false
);
shaping_test!(
    gsub_context1_boundary_1,
    "tests\\fonts\\aot\\gsub_context1_boundary_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &["0", "20", "20", "20", "20", "20", "0"],
    false,
    false
);
shaping_test!(
    gsub_context1_boundary_2,
    "tests\\fonts\\aot\\gsub_context1_boundary_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &["0", "60", "60", "60", "60", "60", "0"],
    false,
    false
);
shaping_test!(
    gsub_context1_expansion_1,
    "tests\\fonts\\aot\\gsub_context1_expansion_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}",
    &["0", "20", "61", "62", "63", "22", "0"],
    false,
    false
);
shaping_test!(
    gsub_context1_lookupflag_1,
    "tests\\fonts\\aot\\gsub_context1_lookupflag_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}Z\u{15}[\\\u{16}\u{0}",
    &["0", "60", "90", "61", "91", "92", "62", "0"],
    false,
    false
);
shaping_test!(
    gsub_context1_lookupflag_2,
    "tests\\fonts\\aot\\gsub_context1_lookupflag_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}Z\u{15}[\\\u{16}\u{0}",
    &["0", "20", "90", "61", "91", "92", "0"],
    false,
    false
);
shaping_test!(
    gsub_context1_multiple_subrules_1,
    "tests\\fonts\\aot\\gsub_context1_multiple_subrules_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}\u{14}\u{15}\u{0}",
    &["0", "60", "21", "22", "0", "20", "61", "0"],
    false,
    false
);
shaping_test!(
    gsub_context1_multiple_subrules_2,
    "tests\\fonts\\aot\\gsub_context1_multiple_subrules_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}\u{14}\u{15}\u{0}",
    &["0", "20", "61", "22", "0", "20", "61", "0"],
    false,
    false
);
shaping_test!(
    gsub_context1_next_glyph_1,
    "tests\\fonts\\aot\\gsub_context1_next_glyph_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &["0", "60", "20", "60", "20", "20", "0"],
    false,
    false
);
shaping_test!(
    gsub_context1_simple_1,
    "tests\\fonts\\aot\\gsub_context1_simple_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}",
    &["0", "60", "61", "62", "0"],
    false,
    false
);
shaping_test!(
    gsub_context1_simple_2,
    "tests\\fonts\\aot\\gsub_context1_simple_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{0}\u{14}\u{15}\u{0}",
    &["0", "20", "0", "20", "21", "0"],
    false,
    false
);
shaping_test!(
    gsub_context1_simple_3,
    "tests\\fonts\\aot\\gsub_context1_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &["0", "20", "60", "20", "20", "20", "0"],
    false,
    false
);
shaping_test!(
    gsub_context1_successive_1,
    "tests\\fonts\\aot\\gsub_context1_successive_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "61", "63", "0"],
    false,
    false
);
shaping_test!(
    gsub_context2_boundary_1,
    "tests\\fonts\\aot\\gsub_context2_boundary_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &["0", "20", "20", "20", "20", "20", "0"],
    false,
    false
);
shaping_test!(
    gsub_context2_boundary_2,
    "tests\\fonts\\aot\\gsub_context2_boundary_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &["0", "60", "60", "60", "60", "60", "0"],
    false,
    false
);
shaping_test!(
    gsub_context2_classes_1,
    "tests\\fonts\\aot\\gsub_context2_classes_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{1a}\u{1c}\u{18}\u{0}\u{15}\u{1b}\u{1a}\u{18}\u{0}\u{16}\u{1b}\u{1a}\u{18}",
    &["0", "20", "66", "28", "24", "0", "21", "67", "26", "24", "0", "22", "27", "26", "24"],
    false,
    false
);
shaping_test!(
    gsub_context2_classes_2,
    "tests\\fonts\\aot\\gsub_context2_classes_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{16}\u{1b}\u{1a}\u{18}\u{0}\u{18}\u{18}\u{1d}\u{16}\u{0}\u{16}\u{1b}\u{1a}\u{18}",
    &["0", "22", "67", "26", "24", "0", "24", "24", "69", "22", "0", "22", "67", "26", "24"],
    false,
    false
);
shaping_test!(
    gsub_context2_expansion_1,
    "tests\\fonts\\aot\\gsub_context2_expansion_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}",
    &["0", "20", "61", "62", "63", "22", "0"],
    false,
    false
);
shaping_test!(
    gsub_context2_lookupflag_1,
    "tests\\fonts\\aot\\gsub_context2_lookupflag_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}Z\u{15}[\\\u{16}\u{0}",
    &["0", "60", "90", "61", "91", "92", "62", "0"],
    false,
    false
);
shaping_test!(
    gsub_context2_lookupflag_2,
    "tests\\fonts\\aot\\gsub_context2_lookupflag_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}Z\u{15}[\\\u{16}\u{0}",
    &["0", "20", "90", "61", "91", "92", "0"],
    false,
    false
);
shaping_test!(
    gsub_context2_multiple_subrules_1,
    "tests\\fonts\\aot\\gsub_context2_multiple_subrules_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}\u{14}\u{15}\u{0}",
    &["0", "60", "21", "22", "0", "20", "61", "0"],
    false,
    false
);
shaping_test!(
    gsub_context2_multiple_subrules_2,
    "tests\\fonts\\aot\\gsub_context2_multiple_subrules_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}\u{14}\u{15}\u{0}",
    &["0", "20", "61", "22", "0", "20", "61", "0"],
    false,
    false
);
shaping_test!(
    gsub_context2_next_glyph_1,
    "tests\\fonts\\aot\\gsub_context2_next_glyph_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &["0", "60", "20", "60", "20", "20", "0"],
    false,
    false
);
shaping_test!(
    gsub_context2_simple_1,
    "tests\\fonts\\aot\\gsub_context2_simple_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}",
    &["0", "60", "61", "62", "0"],
    false,
    false
);
shaping_test!(
    gsub_context2_simple_2,
    "tests\\fonts\\aot\\gsub_context2_simple_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{0}\u{14}\u{15}\u{0}",
    &["0", "20", "0", "20", "21", "0"],
    false,
    false
);
shaping_test!(
    gsub_context2_simple_3,
    "tests\\fonts\\aot\\gsub_context2_simple_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &["0", "20", "60", "20", "20", "20", "0"],
    false,
    false
);
shaping_test!(
    gsub_context2_successive_1,
    "tests\\fonts\\aot\\gsub_context2_successive_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "61", "63", "0"],
    false,
    false
);
shaping_test!(
    gsub_context3_boundary_1,
    "tests\\fonts\\aot\\gsub_context3_boundary_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &["0", "20", "20", "20", "20", "20", "0"],
    false,
    false
);
shaping_test!(
    gsub_context3_boundary_2,
    "tests\\fonts\\aot\\gsub_context3_boundary_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &["0", "60", "60", "60", "60", "60", "0"],
    false,
    false
);
shaping_test!(
    gsub_context3_lookupflag_1,
    "tests\\fonts\\aot\\gsub_context3_lookupflag_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}Z\u{15}[\\\u{16}\u{0}",
    &["0", "60", "90", "61", "91", "92", "62", "0"],
    false,
    false
);
shaping_test!(
    gsub_context3_lookupflag_2,
    "tests\\fonts\\aot\\gsub_context3_lookupflag_f2.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}Z\u{15}[\\\u{16}\u{0}",
    &["0", "20", "90", "61", "91", "92", "0"],
    false,
    false
);
shaping_test!(
    gsub_context3_next_glyph_1,
    "tests\\fonts\\aot\\gsub_context3_next_glyph_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{14}\u{14}\u{14}\u{14}\u{0}",
    &["0", "60", "20", "60", "20", "20", "0"],
    false,
    false
);
shaping_test!(
    gsub_context3_simple_1,
    "tests\\fonts\\aot\\gsub_context3_simple_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{0}",
    &["0", "60", "61", "62", "0"],
    false,
    false
);
shaping_test!(
    gsub_context3_simple_2,
    "tests\\fonts\\aot\\gsub_context3_simple_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{0}\u{14}\u{15}\u{0}\u{14}\u{15}\u{16}\u{0}",
    &["0", "20", "0", "20", "21", "0", "60", "61", "62", "0"],
    false,
    false
);
shaping_test!(
    gsub_context3_successive_1,
    "tests\\fonts\\aot\\gsub_context3_successive_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{0}\u{14}\u{15}\u{16}\u{17}\u{0}",
    &["0", "20", "61", "63", "0"],
    false,
    false
);
shaping_test!(
    lookupflag_ignore_attach_1,
    "tests\\fonts\\aot\\lookupflag_ignore_attach_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\n\u{b}\r\u{1a}\n",
    &["10", "15", "10"],
    false,
    false
);
shaping_test!(
    lookupflag_ignore_attach_2,
    "tests\\fonts\\aot\\lookupflag_ignore_attach_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\n\u{b}\u{15}\r\u{16}\u{17}\u{1d}\u{1a}\n",
    &["10", "15", "21", "22", "23", "29", "10"],
    false,
    false
);
shaping_test!(
    lookupflag_ignore_attach_3,
    "tests\\fonts\\aot\\lookupflag_ignore_attach_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\n\u{b}\u{15}\r\u{16}\u{1b}\u{1a}\n",
    &["10", "11", "21", "13", "22", "27", "26", "10"],
    false,
    false
);
shaping_test!(
    lookupflag_ignore_attach_4,
    "tests\\fonts\\aot\\lookupflag_ignore_attach_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\n\u{b}\u{1b}\r\u{16}\u{17}\u{1a}\n",
    &["10", "11", "27", "13", "22", "23", "26", "10"],
    false,
    false
);
shaping_test!(
    lookupflag_ignore_attach_5,
    "tests\\fonts\\aot\\lookupflag_ignore_attach_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\n\u{b}\u{1b}\r\u{e}\u{17}\u{1a}\n",
    &["10", "11", "27", "13", "14", "23", "26", "10"],
    false,
    false
);
shaping_test!(
    lookupflag_ignore_base_1,
    "tests\\fonts\\aot\\lookupflag_ignore_base_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{13}\u{14}\u{15}",
    &["17", "23", "21"],
    false,
    false
);
shaping_test!(
    lookupflag_ignore_base_2,
    "tests\\fonts\\aot\\lookupflag_ignore_base_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{18}\u{18}\u{13}\u{19}\u{14}\u{15}",
    &["17", "23", "24", "24", "25", "21"],
    false,
    false
);
shaping_test!(
    lookupflag_ignore_combination_1,
    "tests\\fonts\\aot\\lookupflag_ignore_combination_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{1a}\u{13}\u{14}\u{15}",
    &["17", "23", "26", "21"],
    false,
    false
);
shaping_test!(
    lookupflag_ignore_combination_2,
    "tests\\fonts\\aot\\lookupflag_ignore_combination_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{1a}\u{13}\u{18}\u{1e}\u{1f}\u{14}\u{15}",
    &["17", "23", "26", "24", "30", "31", "21"],
    false,
    false
);
shaping_test!(
    lookupflag_ignore_combination_3,
    "tests\\fonts\\aot\\lookupflag_ignore_combination_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{1a}\u{13}\u{18}\u{1e} \u{14}\u{15}",
    &["17", "18", "26", "19", "24", "30", "32", "20", "21"],
    false,
    false
);
shaping_test!(
    lookupflag_ignore_ligatures_1,
    "tests\\fonts\\aot\\lookupflag_ignore_ligatures_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{1a}\u{1b}\u{13}\u{1b}\u{14}\u{15}",
    &["17", "23", "26", "27", "27", "21"],
    false,
    false
);
shaping_test!(
    lookupflag_ignore_ligatures_2,
    "tests\\fonts\\aot\\lookupflag_ignore_ligatures_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{1a}\u{18}\u{13}\u{1b}\u{14}\u{15}",
    &["17", "18", "26", "24", "19", "27", "20", "21"],
    false,
    false
);
shaping_test!(
    lookupflag_ignore_ligatures_3,
    "tests\\fonts\\aot\\lookupflag_ignore_ligatures_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{1a}*\u{13}\u{1b}\u{14}\u{15}",
    &["17", "18", "26", "42", "19", "27", "20", "21"],
    false,
    false
);
shaping_test!(
    lookupflag_ignore_marks_1,
    "tests\\fonts\\aot\\lookupflag_ignore_marks_f1.otf",
    75,
    &[("no-clusters", 1)],
    &[("features", 0.0)],
    "\u{11}\u{12}\u{1c}\u{1d}\u{13}\u{1d}\u{14}\u{15}",
    &["17", "23", "28", "29", "29", "21"],
    false,
    false
);
