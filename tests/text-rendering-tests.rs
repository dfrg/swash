mod shaping_impl;
use shaping_impl::shape;

#[test]
fn avar_1_0() {
    assert_eq!(
        shape("fonts/TestAVAR.ttf", 1000, &[("TEST", 100.0)], "⨁"),
        "[gid1]"
    )
}

#[test]
fn avar_1_1() {
    assert_eq!(
        shape("fonts/TestAVAR.ttf", 1000, &[("TEST", 150.0)], "⨁"),
        "[gid1]"
    )
}

#[test]
fn avar_1_2() {
    assert_eq!(
        shape("fonts/TestAVAR.ttf", 1000, &[("TEST", 200.0)], "⨁"),
        "[gid1]"
    )
}

#[test]
fn avar_1_3() {
    assert_eq!(
        shape("fonts/TestAVAR.ttf", 1000, &[("TEST", 250.0)], "⨁"),
        "[gid1]"
    )
}

#[test]
fn avar_1_4() {
    assert_eq!(
        shape("fonts/TestAVAR.ttf", 1000, &[("TEST", 300.0)], "⨁"),
        "[gid1]"
    )
}

#[test]
fn avar_1_5() {
    assert_eq!(
        shape("fonts/TestAVAR.ttf", 1000, &[("TEST", 350.0)], "⨁"),
        "[gid1]"
    )
}

#[test]
fn avar_1_6() {
    assert_eq!(
        shape("fonts/TestAVAR.ttf", 1000, &[("TEST", 400.0)], "⨁"),
        "[gid1]"
    )
}

#[test]
fn avar_1_7() {
    assert_eq!(
        shape("fonts/TestAVAR.ttf", 1000, &[("TEST", 450.0)], "⨁"),
        "[gid1]"
    )
}

#[test]
fn avar_1_8() {
    assert_eq!(
        shape("fonts/TestAVAR.ttf", 1000, &[("TEST", 500.0)], "⨁"),
        "[gid1]"
    )
}

#[test]
fn avar_1_9() {
    assert_eq!(
        shape("fonts/TestAVAR.ttf", 1000, &[("TEST", 550.0)], "⨁"),
        "[gid1]"
    )
}

#[test]
fn avar_1_10() {
    assert_eq!(
        shape("fonts/TestAVAR.ttf", 1000, &[("TEST", 600.0)], "⨁"),
        "[gid1]"
    )
}

#[test]
fn avar_1_11() {
    assert_eq!(
        shape("fonts/TestAVAR.ttf", 1000, &[("TEST", 650.0)], "⨁"),
        "[gid1]"
    )
}

#[test]
fn avar_1_12() {
    assert_eq!(
        shape("fonts/TestAVAR.ttf", 1000, &[("TEST", 700.0)], "⨁"),
        "[gid1]"
    )
}

#[test]
fn avar_1_13() {
    assert_eq!(
        shape("fonts/TestAVAR.ttf", 1000, &[("TEST", 750.0)], "⨁"),
        "[gid1]"
    )
}

#[test]
fn avar_1_14() {
    assert_eq!(
        shape("fonts/TestAVAR.ttf", 1000, &[("TEST", 800.0)], "⨁"),
        "[gid1]"
    )
}

#[test]
fn avar_1_15() {
    assert_eq!(
        shape("fonts/TestAVAR.ttf", 1000, &[("TEST", 850.0)], "⨁"),
        "[gid1]"
    )
}

#[test]
fn avar_1_16() {
    assert_eq!(
        shape("fonts/TestAVAR.ttf", 1000, &[("TEST", 900.0)], "⨁"),
        "[gid1]"
    )
}

#[test]
fn cff_1_0() {
    assert_eq!(shape("fonts/FDArrayTest257.otf", 1000, &[], "A"), "[gid66]")
}

#[test]
fn cff_1_1() {
    assert_eq!(shape("fonts/FDArrayTest257.otf", 1000, &[], "ℝ"), "[gid30]")
}

#[test]
fn cff_1_2() {
    assert_eq!(
        shape("fonts/FDArrayTest257.otf", 1000, &[], "⓪"),
        "[gid235]"
    )
}

#[test]
fn cff_1_3() {
    assert_eq!(shape("fonts/FDArrayTest257.otf", 1000, &[], "①"), "[gid97]")
}

#[test]
fn cff_1_4() {
    assert_eq!(shape("fonts/FDArrayTest257.otf", 1000, &[], "②"), "[gid98]")
}

#[test]
fn cff_1_5() {
    assert_eq!(
        shape("fonts/FDArrayTest257.otf", 1000, &[], "仿"),
        "[gid256]"
    )
}

#[test]
fn cff_1_6() {
    assert_eq!(
        shape("fonts/FDArrayTest257.otf", 1000, &[], "Ａ"),
        "[gid34]"
    )
}

#[test]
fn cff_1_7() {
    assert_eq!(shape("fonts/FDArrayTest257.otf", 1000, &[], "𐄳"), "[gid52]")
}

#[test]
fn cff_1_8() {
    assert_eq!(
        shape("fonts/FDArrayTest257.otf", 1000, &[], "𝓐"),
        "[gid209]"
    )
}

#[test]
fn cff_1_9() {
    assert_eq!(
        shape("fonts/FDArrayTest257.otf", 1000, &[], "🌺"),
        "[gid59]"
    )
}

#[test]
fn cff_1_10() {
    assert_eq!(
        shape("fonts/FDArrayTest257.otf", 1000, &[], "🌻"),
        "[gid60]"
    )
}

#[test]
fn cff_1_11() {
    assert_eq!(
        shape("fonts/FDArrayTest257.otf", 1000, &[], "💧"),
        "[gid168]"
    )
}

#[test]
fn cff_1_12() {
    assert_eq!(
        shape("fonts/FDArrayTest257.otf", 1000, &[], "🥝"),
        "[gid94]"
    )
}

#[test]
fn cff_2_0() {
    assert_eq!(
        shape("fonts/FDArrayTest65535.otf", 1000, &[], "A"),
        "[gid66]"
    )
}

#[test]
fn cff_2_1() {
    assert_eq!(
        shape("fonts/FDArrayTest65535.otf", 1000, &[], "ℝ"),
        "[gid8478]"
    )
}

#[test]
fn cff_2_2() {
    assert_eq!(
        shape("fonts/FDArrayTest65535.otf", 1000, &[], "⓪"),
        "[gid9451]"
    )
}

#[test]
fn cff_2_3() {
    assert_eq!(
        shape("fonts/FDArrayTest65535.otf", 1000, &[], "①"),
        "[gid9313]"
    )
}

#[test]
fn cff_2_4() {
    assert_eq!(
        shape("fonts/FDArrayTest65535.otf", 1000, &[], "②"),
        "[gid9314]"
    )
}

#[test]
fn cff_2_5() {
    assert_eq!(
        shape("fonts/FDArrayTest65535.otf", 1000, &[], "仿"),
        "[gid20224]"
    )
}

#[test]
fn cff_2_6() {
    assert_eq!(
        shape("fonts/FDArrayTest65535.otf", 1000, &[], "Ａ"),
        "[gid65314]"
    )
}

#[test]
fn cff_2_7() {
    assert_eq!(
        shape("fonts/FDArrayTest65535.otf", 1000, &[], "𐄳"),
        "[gid308]"
    )
}

#[test]
fn cff_2_8() {
    assert_eq!(
        shape("fonts/FDArrayTest65535.otf", 1000, &[], "𝓐"),
        "[gid54481]"
    )
}

#[test]
fn cff_2_9() {
    assert_eq!(
        shape("fonts/FDArrayTest65535.otf", 1000, &[], "🌺"),
        "[gid62267]"
    )
}

#[test]
fn cff_2_10() {
    assert_eq!(
        shape("fonts/FDArrayTest65535.otf", 1000, &[], "🌻"),
        "[gid62268]"
    )
}

#[test]
fn cff_2_11() {
    assert_eq!(
        shape("fonts/FDArrayTest65535.otf", 1000, &[], "💧"),
        "[gid62632]"
    )
}

#[test]
fn cff_2_12() {
    assert_eq!(
        shape("fonts/FDArrayTest65535.otf", 1000, &[], "🥝"),
        "[gid63838]"
    )
}

#[test]
fn cff2_1_0() {
    assert_eq!(
        shape(
            "fonts/AdobeVFPrototype-Subset.otf",
            1000,
            &[("wght", 100.0)],
            "$"
        ),
        "[dollar]"
    )
}

#[test]
fn cff2_1_1() {
    assert_eq!(
        shape(
            "fonts/AdobeVFPrototype-Subset.otf",
            1000,
            &[("wght", 200.0)],
            "$"
        ),
        "[dollar]"
    )
}

#[test]
fn cff2_1_2() {
    assert_eq!(
        shape(
            "fonts/AdobeVFPrototype-Subset.otf",
            1000,
            &[("wght", 300.0)],
            "$"
        ),
        "[dollar]"
    )
}

#[test]
fn cff2_1_3() {
    assert_eq!(
        shape(
            "fonts/AdobeVFPrototype-Subset.otf",
            1000,
            &[("wght", 400.0)],
            "$"
        ),
        "[dollar]"
    )
}

#[test]
fn cff2_1_4() {
    assert_eq!(
        shape(
            "fonts/AdobeVFPrototype-Subset.otf",
            1000,
            &[("wght", 500.0)],
            "$"
        ),
        "[dollar]"
    )
}

#[test]
fn cff2_1_5() {
    assert_eq!(
        shape(
            "fonts/AdobeVFPrototype-Subset.otf",
            1000,
            &[("wght", 600.0)],
            "$"
        ),
        "[dollar]"
    )
}

#[test]
fn cff2_1_6() {
    assert_eq!(
        shape(
            "fonts/AdobeVFPrototype-Subset.otf",
            1000,
            &[("wght", 700.0)],
            "$"
        ),
        "[dollar]"
    )
}

#[test]
fn cff2_1_7() {
    assert_eq!(
        shape(
            "fonts/AdobeVFPrototype-Subset.otf",
            1000,
            &[("wght", 800.0)],
            "$"
        ),
        "[dollar.nostroke]"
    )
}

#[test]
fn cff2_1_8() {
    assert_eq!(
        shape(
            "fonts/AdobeVFPrototype-Subset.otf",
            1000,
            &[("wght", 900.0)],
            "$"
        ),
        "[dollar.nostroke]"
    )
}

#[test]
fn cvar_1_0() {
    assert_eq!(
        shape(
            "fonts/TestCVARGVARTwo.ttf",
            1000,
            &[("wght", 28.0), ("wdth", 100.0), ("opsz", 72.0)],
            "hon"
        ),
        "[uni0068|uni006F@595,0|uni006E@1126,0]"
    )
}

#[test]
fn cvar_1_1() {
    assert_eq!(
        shape(
            "fonts/TestCVARGVARTwo.ttf",
            1000,
            &[("wght", 94.0), ("wdth", 100.0), ("opsz", 72.0)],
            "hon"
        ),
        "[uni0068|uni006F@635,0|uni006E@1212,0]"
    )
}

#[test]
fn cvar_1_2() {
    assert_eq!(
        shape(
            "fonts/TestCVARGVARTwo.ttf",
            1000,
            &[("wght", 194.0), ("wdth", 100.0), ("opsz", 72.0)],
            "hon"
        ),
        "[uni0068|uni006F@691,0|uni006E@1331,0]"
    )
}

#[test]
fn cvar_2_0() {
    assert_eq!(
        shape(
            "fonts/TestCVARGVAROne.ttf",
            1000,
            &[("wght", 28.0), ("wdth", 100.0), ("opsz", 72.0)],
            "hon"
        ),
        "[uni0068|uni006F@595,0|uni006E@1126,0]"
    )
}

#[test]
fn cvar_2_1() {
    assert_eq!(
        shape(
            "fonts/TestCVARGVAROne.ttf",
            1000,
            &[("wght", 94.0), ("wdth", 100.0), ("opsz", 72.0)],
            "hon"
        ),
        "[uni0068|uni006F@635,0|uni006E@1212,0]"
    )
}

#[test]
fn cvar_2_2() {
    assert_eq!(
        shape(
            "fonts/TestCVARGVAROne.ttf",
            1000,
            &[("wght", 194.0), ("wdth", 100.0), ("opsz", 72.0)],
            "hon"
        ),
        "[uni0068|uni006F@691,0|uni006E@1331,0]"
    )
}

#[test]
fn glyf_1_0() {
    assert_eq!(
        shape("fonts/TestGLYFOne.ttf", 1000, &[], "ģ"),
        "[gcommaabove]"
    )
}

#[test]
fn gpos_1_0() {
    assert_eq!(
        shape("fonts/TestGPOSOne.ttf", 1000, &[], "ĄJ"),
        "[Aogonek|J@732,0]"
    )
}

#[test]
fn gpos_1_1() {
    assert_eq!(
        shape("fonts/TestGPOSOne.ttf", 1000, &[], "Ąg"),
        "[Aogonek|g@692,0]"
    )
}

#[test]
fn gpos_1_2() {
    assert_eq!(
        shape("fonts/TestGPOSOne.ttf", 1000, &[], "Ąģ"),
        "[Aogonek|gcommaabove@692,0]"
    )
}

#[test]
fn gpos_1_3() {
    assert_eq!(
        shape("fonts/TestGPOSOne.ttf", 1000, &[], "Ąj"),
        "[Aogonek|j@752,0]"
    )
}

#[test]
fn gpos_1_4() {
    assert_eq!(
        shape("fonts/TestGPOSOne.ttf", 1000, &[], "Ąȷ"),
        "[Aogonek|dotlessj@752,0]"
    )
}

#[test]
fn gpos_1_5() {
    assert_eq!(
        shape("fonts/TestGPOSOne.ttf", 1000, &[], "Qȷ"),
        "[Q|dotlessj@734,0]"
    )
}

#[test]
fn gpos_1_6() {
    assert_eq!(
        shape("fonts/TestGPOSOne.ttf", 1000, &[], "ąj"),
        "[aogonek|j@588,0]"
    )
}

#[test]
fn gpos_1_7() {
    assert_eq!(
        shape("fonts/TestGPOSOne.ttf", 1000, &[], "ąȷ"),
        "[aogonek|dotlessj@588,0]"
    )
}

#[test]
fn gpos_1_8() {
    assert_eq!(
        shape("fonts/TestGPOSOne.ttf", 1000, &[], "gȷ"),
        "[g|dotlessj@563,0]"
    )
}

#[test]
fn gpos_1_9() {
    assert_eq!(
        shape("fonts/TestGPOSOne.ttf", 1000, &[], "ģȷ"),
        "[gcommaabove|dotlessj@563,0]"
    )
}

#[test]
fn gpos_1_10() {
    assert_eq!(
        shape("fonts/TestGPOSOne.ttf", 1000, &[], "ıȷ"),
        "[dotlessi|dotlessj@334,0]"
    )
}

#[test]
fn gpos_1_11() {
    assert_eq!(
        shape("fonts/TestGPOSOne.ttf", 1000, &[], "ųȷ"),
        "[uogonek|dotlessj@656,0]"
    )
}

#[test]
fn gpos_1_12() {
    assert_eq!(
        shape("fonts/TestGPOSOne.ttf", 1000, &[], "vȷ"),
        "[v|dotlessj@587,0]"
    )
}

#[test]
fn gpos_1_13() {
    assert_eq!(
        shape("fonts/TestGPOSOne.ttf", 1000, &[], "Va"),
        "[V|a@594,0]"
    )
}

#[test]
fn gpos_1_14() {
    assert_eq!(
        shape("fonts/TestGPOSOne.ttf", 1000, &[], "Vá"),
        "[V|aacute@594,0]"
    )
}

#[test]
fn gpos_1_15() {
    assert_eq!(
        shape("fonts/TestGPOSOne.ttf", 1000, &[], "Vą"),
        "[V|aogonek@594,0]"
    )
}

#[test]
fn gpos_1_16() {
    assert_eq!(
        shape("fonts/TestGPOSOne.ttf", 1000, &[], "Vf"),
        "[V|f@634,0]"
    )
}

#[test]
fn gpos_1_17() {
    assert_eq!(
        shape("fonts/TestGPOSOne.ttf", 1000, &[], "Vﬂ"),
        "[V|fl@634,0]"
    )
}

#[test]
fn gpos_1_18() {
    assert_eq!(
        shape("fonts/TestGPOSOne.ttf", 1000, &[], "V."),
        "[V|period@504,0]"
    )
}

#[test]
fn gpos_2_0() {
    assert_eq!(shape("fonts/TestGPOSTwo.otf", 1000, &[], "◯"), "[uni25EF]")
}

#[test]
fn gpos_2_1() {
    assert_eq!(shape("fonts/TestGPOSTwo.otf", 1000, &[], "☼"), "[sun]")
}

#[test]
fn gpos_2_2() {
    assert_eq!(
        shape("fonts/TestGPOSTwo.otf", 1000, &[], "◯☼"),
        "[uni25EF|sun]"
    )
}

#[test]
fn gpos_3_0() {
    assert_eq!(
        shape("fonts/TestShapeEthi.ttf", 1000, &[], "ለ"),
        "[uni1208]"
    )
}

#[test]
fn gpos_3_1() {
    assert_eq!(
        shape("fonts/TestShapeEthi.ttf", 1000, &[], "ለ፞"),
        "[uni1208|uni135E@303,0]"
    )
}

#[test]
fn gpos_3_2() {
    assert_eq!(
        shape("fonts/TestShapeEthi.ttf", 1000, &[], "ለ፟"),
        "[uni1208|uni135F@303,0]"
    )
}

#[test]
fn gpos_3_3() {
    assert_eq!(
        shape("fonts/TestShapeEthi.ttf", 1000, &[], "ለ፝"),
        "[uni1208|uni135D@303,0]"
    )
}

#[test]
fn gpos_4_0() {
    assert_eq!(
        shape("fonts/TestGPOSThree.ttf", 1000, &[], "ǘ"),
        "[u|uni0308@529,-31|acutecomb@537,138]"
    )
}

#[test]
fn gpos_4_1() {
    assert_eq!(
        shape("fonts/TestGPOSThree.ttf", 1000, &[], "ǖ"),
        "[u|uni0308@529,-31|uni0304@526,138]"
    )
}

#[test]
fn gpos_4_2() {
    assert_eq!(
        shape("fonts/TestGPOSThree.ttf", 1000, &[], "ü̈"),
        "[u|uni0308@529,-31|uni0308@529,138]"
    )
}

#[test]
fn gpos_4_3() {
    assert_eq!(
        shape("fonts/TestGPOSThree.ttf", 1000, &[], "ü̈̈"),
        "[u|uni0308@529,-31|uni0308@529,138|uni0308@529,307]"
    )
}

#[test]
fn gpos_5_0() {
    assert_eq!(
        shape("fonts/TestGPOSFour.ttf", 1000, &[("wght", 100.0)], "شْ"),
        "[uni0652@663,144|uni0634]"
    )
}

#[test]
fn gpos_5_1() {
    assert_eq!(
        shape("fonts/TestGPOSFour.ttf", 1000, &[("wght", 300.0)], "شْ"),
        "[uni0652@680,165|uni0634]"
    )
}

#[test]
fn gpos_5_2() {
    assert_eq!(
        shape("fonts/TestGPOSFour.ttf", 1000, &[("wght", 600.0)], "شْ"),
        "[uni0652@730,246|uni0634]"
    )
}

#[test]
fn gpos_5_3() {
    assert_eq!(
        shape("fonts/TestGPOSFour.ttf", 1000, &[("wght", 700.0)], "شْ"),
        "[uni0652@750,282|uni0634]"
    )
}

#[test]
fn gpos_5_4() {
    assert_eq!(
        shape("fonts/TestGPOSFour.ttf", 1000, &[("wght", 900.0)], "شْ"),
        "[uni0652@784,351|uni0634]"
    )
}

#[test]
fn gsub_1_0() {
    assert_eq!(
        shape("fonts/TestGSUBOne.otf", 1000, &[], "a a"),
        "[a.alt|space@500,0|a@1000,0]"
    )
}

#[test]
fn gsub_2_0() {
    assert_eq!(
        shape("fonts/TestShapeEthi.ttf", 1000, &[], "፳"),
        "[uni1373]"
    )
}

#[test]
fn gsub_2_1() {
    assert_eq!(
        shape("fonts/TestShapeEthi.ttf", 1000, &[], "፫"),
        "[uni136B]"
    )
}

#[test]
fn gsub_2_2() {
    assert_eq!(
        shape("fonts/TestShapeEthi.ttf", 1000, &[], "፵"),
        "[uni1375]"
    )
}

#[test]
fn gsub_2_3() {
    assert_eq!(
        shape("fonts/TestShapeEthi.ttf", 1000, &[], "፭"),
        "[uni136D]"
    )
}

#[test]
fn gsub_2_4() {
    assert_eq!(
        shape("fonts/TestShapeEthi.ttf", 1000, &[], "፳፫"),
        "[uni1373.init|uni136B.fina@621,0]"
    )
}

#[test]
fn gsub_2_5() {
    assert_eq!(
        shape("fonts/TestShapeEthi.ttf", 1000, &[], "፵፭"),
        "[uni1375.init|uni136D.fina@662,0]"
    )
}

#[test]
fn gsub_2_6() {
    assert_eq!(
        shape("fonts/TestShapeEthi.ttf", 1000, &[], "፻"),
        "[uni137B]"
    )
}

#[test]
fn gsub_2_7() {
    assert_eq!(
        shape("fonts/TestShapeEthi.ttf", 1000, &[], "፳፫፻"),
        "[uni1373.init|uni136B.medi@621,0|uni137B.fina@1102,0]"
    )
}

#[test]
fn gsub_2_8() {
    assert_eq!(
        shape("fonts/TestShapeEthi.ttf", 1000, &[], "፳፫፻፳፫"),
        "[uni1373.init|uni136B.medi@621,0|uni137B.medi@1102,0|uni1373.medi@1489,0|uni136B.fina@2110,0]"
    )
}

#[test]
fn gsub_2_9() {
    assert_eq!(
        shape("fonts/TestShapeEthi.ttf", 1000, &[], "፳፫፻፵፭"),
        "[uni1373.init|uni136B.medi@621,0|uni137B.medi@1102,0|uni1375.medi@1489,0|uni136D.fina@2157,0]"
    )
}

#[test]
fn gsub_2_10() {
    assert_eq!(
        shape("fonts/TestShapeEthi.ttf", 1000, &[], "፵፭፻፳፫"),
        "[uni1375.init|uni136D.medi@662,0|uni137B.medi@1203,0|uni1373.medi@1590,0|uni136B.fina@2211,0]"
    )
}

#[test]
fn gsub_3_0() {
    assert_eq!(shape("fonts/TestGSUBThree.ttf", 0, &[], "lol"), "*")
}

#[test]
fn gvar_1_0() {
    assert_eq!(
        shape("fonts/TestGVAROne.ttf", 1000, &[("wght", 300.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_1_1() {
    assert_eq!(
        shape("fonts/TestGVAROne.ttf", 1000, &[("wght", 350.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_1_2() {
    assert_eq!(
        shape("fonts/TestGVAROne.ttf", 1000, &[("wght", 400.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_1_3() {
    assert_eq!(
        shape("fonts/TestGVAROne.ttf", 1000, &[("wght", 450.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_1_4() {
    assert_eq!(
        shape("fonts/TestGVAROne.ttf", 1000, &[("wght", 500.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_1_5() {
    assert_eq!(
        shape("fonts/TestGVAROne.ttf", 1000, &[("wght", 550.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_1_6() {
    assert_eq!(
        shape("fonts/TestGVAROne.ttf", 1000, &[("wght", 600.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_1_7() {
    assert_eq!(
        shape("fonts/TestGVAROne.ttf", 1000, &[("wght", 650.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_1_8() {
    assert_eq!(
        shape("fonts/TestGVAROne.ttf", 1000, &[("wght", 700.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_2_0() {
    assert_eq!(
        shape("fonts/TestGVARTwo.ttf", 1000, &[("wght", 300.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_2_1() {
    assert_eq!(
        shape("fonts/TestGVARTwo.ttf", 1000, &[("wght", 350.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_2_2() {
    assert_eq!(
        shape("fonts/TestGVARTwo.ttf", 1000, &[("wght", 400.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_2_3() {
    assert_eq!(
        shape("fonts/TestGVARTwo.ttf", 1000, &[("wght", 450.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_2_4() {
    assert_eq!(
        shape("fonts/TestGVARTwo.ttf", 1000, &[("wght", 500.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_2_5() {
    assert_eq!(
        shape("fonts/TestGVARTwo.ttf", 1000, &[("wght", 550.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_2_6() {
    assert_eq!(
        shape("fonts/TestGVARTwo.ttf", 1000, &[("wght", 600.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_2_7() {
    assert_eq!(
        shape("fonts/TestGVARTwo.ttf", 1000, &[("wght", 650.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_2_8() {
    assert_eq!(
        shape("fonts/TestGVARTwo.ttf", 1000, &[("wght", 700.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_3_0() {
    assert_eq!(
        shape("fonts/TestGVARThree.ttf", 1000, &[("wght", 300.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_3_1() {
    assert_eq!(
        shape("fonts/TestGVARThree.ttf", 1000, &[("wght", 350.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_3_2() {
    assert_eq!(
        shape("fonts/TestGVARThree.ttf", 1000, &[("wght", 400.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_3_3() {
    assert_eq!(
        shape("fonts/TestGVARThree.ttf", 1000, &[("wght", 450.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_3_4() {
    assert_eq!(
        shape("fonts/TestGVARThree.ttf", 1000, &[("wght", 500.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_3_5() {
    assert_eq!(
        shape("fonts/TestGVARThree.ttf", 1000, &[("wght", 550.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_3_6() {
    assert_eq!(
        shape("fonts/TestGVARThree.ttf", 1000, &[("wght", 600.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_3_7() {
    assert_eq!(
        shape("fonts/TestGVARThree.ttf", 1000, &[("wght", 650.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_3_8() {
    assert_eq!(
        shape("fonts/TestGVARThree.ttf", 1000, &[("wght", 700.0)], "彌"),
        "[gid2]"
    )
}

#[test]
fn gvar_4_0() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", -1.0), ("T1", 0.0)], "🦎"),
        "[gid5]"
    )
}

#[test]
fn gvar_4_1() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", -0.8), ("T1", 0.1)], "🦎"),
        "[gid5]"
    )
}

#[test]
fn gvar_4_2() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", -0.6), ("T1", 0.2)], "🦎"),
        "[gid5]"
    )
}

#[test]
fn gvar_4_3() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", -0.4), ("T1", 0.3)], "🦎"),
        "[gid5]"
    )
}

#[test]
fn gvar_4_4() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", -0.2), ("T1", 0.4)], "🦎"),
        "[gid5]"
    )
}

#[test]
fn gvar_4_5() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", 0.0), ("T1", 0.5)], "🦎"),
        "[gid5]"
    )
}

#[test]
fn gvar_4_6() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", 0.2), ("T1", 0.6)], "🦎"),
        "[gid5]"
    )
}

#[test]
fn gvar_4_7() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", 0.4), ("T1", 0.7)], "🦎"),
        "[gid5]"
    )
}

#[test]
fn gvar_4_8() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", 0.6), ("T1", 0.8)], "🦎"),
        "[gid5]"
    )
}

#[test]
fn gvar_4_9() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", 0.8), ("T1", 0.9)], "🦎"),
        "[gid5]"
    )
}

#[test]
fn gvar_4_10() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", 1.0), ("T1", 1.0)], "🦎"),
        "[gid5]"
    )
}

#[test]
fn gvar_5_0() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", -1.0)], "🌝"),
        "[gid15]"
    )
}

#[test]
fn gvar_5_1() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", -0.8)], "🌝"),
        "[gid15]"
    )
}

#[test]
fn gvar_5_2() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", -0.6)], "🌝"),
        "[gid15]"
    )
}

#[test]
fn gvar_5_3() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", -0.4)], "🌝"),
        "[gid15]"
    )
}

#[test]
fn gvar_5_4() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", -0.2)], "🌝"),
        "[gid15]"
    )
}

#[test]
fn gvar_5_5() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", 0.0)], "🌝"),
        "[gid15]"
    )
}

#[test]
fn gvar_5_6() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", 0.2)], "🌝"),
        "[gid15]"
    )
}

#[test]
fn gvar_5_7() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", 0.4)], "🌝"),
        "[gid15]"
    )
}

#[test]
fn gvar_5_8() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", 0.6)], "🌝"),
        "[gid15]"
    )
}

#[test]
fn gvar_5_9() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", 0.8)], "🌝"),
        "[gid15]"
    )
}

#[test]
fn gvar_5_10() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("M1", 1.0)], "🌝"),
        "[gid15]"
    )
}

#[test]
fn gvar_6_0() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("T1", 0.0)], "🐢"),
        "[gid12]"
    )
}

#[test]
fn gvar_6_1() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("T1", 0.1)], "🐢"),
        "[gid12]"
    )
}

#[test]
fn gvar_6_2() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("T1", 0.2)], "🐢"),
        "[gid12]"
    )
}

#[test]
fn gvar_6_3() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("T1", 0.3)], "🐢"),
        "[gid12]"
    )
}

#[test]
fn gvar_6_4() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("T1", 0.4)], "🐢"),
        "[gid12]"
    )
}

#[test]
fn gvar_6_5() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("T1", 0.5)], "🐢"),
        "[gid12]"
    )
}

#[test]
fn gvar_6_6() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("T1", 0.6)], "🐢"),
        "[gid12]"
    )
}

#[test]
fn gvar_6_7() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("T1", 0.7)], "🐢"),
        "[gid12]"
    )
}

#[test]
fn gvar_6_8() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("T1", 0.8)], "🐢"),
        "[gid12]"
    )
}

#[test]
fn gvar_6_9() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("T1", 0.9)], "🐢"),
        "[gid12]"
    )
}

#[test]
fn gvar_6_10() {
    assert_eq!(
        shape("fonts/Zycon.ttf", 1000, &[("T1", 1.0)], "🐢"),
        "[gid12]"
    )
}

#[test]
fn gvar_7_0() {
    assert_eq!(
        shape("fonts/TestGVARFour.ttf", 1000, &[("wght", 150.0)], "OIO"),
        "[uni004F|uni0049@706,0|uni004F@1072,0]"
    )
}

#[test]
fn gvar_7_1() {
    assert_eq!(
        shape("fonts/TestGVARFour.ttf", 1000, &[("wght", 200.0)], "OIO"),
        "[uni004F|uni0049@707,0|uni004F@1074,0]"
    )
}

#[test]
fn gvar_7_2() {
    assert_eq!(
        shape("fonts/TestGVARFour.ttf", 1000, &[("wght", 250.0)], "OIO"),
        "[uni004F|uni0049@707,0|uni004F@1075,0]"
    )
}

#[test]
fn gvar_7_3() {
    assert_eq!(
        shape("fonts/TestGVARFour.ttf", 1000, &[("wght", 300.0)], "OIO"),
        "[uni004F|uni0049@707,0|uni004F@1076,0]"
    )
}

#[test]
fn gvar_7_4() {
    assert_eq!(
        shape("fonts/TestGVARFour.ttf", 1000, &[("wght", 350.0)], "OIO"),
        "[uni004F|uni0049@707,0|uni004F@1077,0]"
    )
}

#[test]
fn gvar_7_5() {
    assert_eq!(
        shape("fonts/TestGVARFour.ttf", 1000, &[("wght", 400.0)], "OIO"),
        "[uni004F|uni0049@707,0|uni004F@1078,0]"
    )
}

#[test]
fn gvar_7_6() {
    assert_eq!(
        shape("fonts/TestGVARFour.ttf", 1000, &[("wght", 450.0)], "OIO"),
        "[uni004F|uni0049@706,0|uni004F@1079,0]"
    )
}

#[test]
fn gvar_8_0() {
    assert_eq!(
        shape("fonts/TestGVAREight.ttf", 1000, &[("HV", 0.0)], "H"),
        "[H]"
    )
}

#[test]
fn gvar_8_1() {
    assert_eq!(
        shape("fonts/TestGVAREight.ttf", 1000, &[("HV", -0.2)], "H"),
        "[H]"
    )
}

#[test]
fn gvar_8_2() {
    assert_eq!(
        shape("fonts/TestGVAREight.ttf", 1000, &[("HV", -0.4)], "H"),
        "[H]"
    )
}

#[test]
fn gvar_8_3() {
    assert_eq!(
        shape("fonts/TestGVAREight.ttf", 1000, &[("HV", -0.6)], "H"),
        "[H]"
    )
}

#[test]
fn gvar_8_4() {
    assert_eq!(
        shape("fonts/TestGVAREight.ttf", 1000, &[("HV", -0.8)], "H"),
        "[H]"
    )
}

#[test]
fn gvar_8_5() {
    assert_eq!(
        shape("fonts/TestGVAREight.ttf", 1000, &[("HV", -1.0)], "H"),
        "[H]"
    )
}

#[test]
fn gvar_9_0() {
    assert_eq!(
        shape("fonts/TestGVARNine.ttf", 1000, &[("TEST", -1.0)], "A"),
        "[A]"
    )
}

#[test]
fn gvar_9_1() {
    assert_eq!(
        shape("fonts/TestGVARNine.ttf", 1000, &[("TEST", -0.5)], "A"),
        "[A]"
    )
}

#[test]
fn gvar_9_2() {
    assert_eq!(
        shape("fonts/TestGVARNine.ttf", 1000, &[("TEST", 0.0)], "A"),
        "[A]"
    )
}

#[test]
fn gvar_9_3() {
    assert_eq!(
        shape("fonts/TestGVARNine.ttf", 1000, &[("TEST", 0.5)], "A"),
        "[A]"
    )
}

#[test]
fn gvar_9_4() {
    assert_eq!(
        shape("fonts/TestGVARNine.ttf", 1000, &[("TEST", 0.6)], "A"),
        "[A]"
    )
}

#[test]
fn gvar_9_5() {
    assert_eq!(
        shape("fonts/TestGVARNine.ttf", 1000, &[("TEST", 0.7)], "A"),
        "[A]"
    )
}

#[test]
fn gvar_9_6() {
    assert_eq!(
        shape("fonts/TestGVARNine.ttf", 1000, &[("TEST", 0.8)], "A"),
        "[A]"
    )
}

#[test]
fn gvar_9_7() {
    assert_eq!(
        shape("fonts/TestGVARNine.ttf", 1000, &[("TEST", 0.9)], "A"),
        "[A]"
    )
}

#[test]
fn gvar_9_8() {
    assert_eq!(
        shape("fonts/TestGVARNine.ttf", 1000, &[("TEST", 0.944444)], "A"),
        "[A]"
    )
}

#[test]
fn gvar_9_9() {
    assert_eq!(
        shape("fonts/TestGVARNine.ttf", 1000, &[("TEST", 1.0)], "A"),
        "[A]"
    )
}

#[test]
fn hvar_1_0() {
    assert_eq!(
        shape("fonts/TestHVAROne.otf", 1000, &[("wght", 0.0)], "ABC"),
        "[A|B@520,0|C@1094,0]"
    )
}

#[test]
fn hvar_1_1() {
    assert_eq!(
        shape("fonts/TestHVAROne.otf", 1000, &[("wght", 200.0)], "ABC"),
        "[A|B@533,0|C@1115,0]"
    )
}

#[test]
fn hvar_1_2() {
    assert_eq!(
        shape("fonts/TestHVAROne.otf", 1000, &[("wght", 400.0)], "ABC"),
        "[A|B@546,0|C@1135,0]"
    )
}

#[test]
fn hvar_1_3() {
    assert_eq!(
        shape("fonts/TestHVAROne.otf", 1000, &[("wght", 600.0)], "ABC"),
        "[A|B@558,0|C@1155,0]"
    )
}

#[test]
fn hvar_1_4() {
    assert_eq!(
        shape("fonts/TestHVAROne.otf", 1000, &[("wght", 800.0)], "ABC"),
        "[A|B@571,0|C@1175,0]"
    )
}

#[test]
fn hvar_1_5() {
    assert_eq!(
        shape("fonts/TestHVAROne.otf", 1000, &[("wght", 1000.0)], "ABC"),
        "[A|B@584,0|C@1196,0]"
    )
}

#[test]
fn hvar_2_0() {
    assert_eq!(
        shape("fonts/TestHVARTwo.ttf", 1000, &[("wght", 0.0)], "AB"),
        "[uni0041|uni0042@450,0]"
    )
}

#[test]
fn hvar_2_1() {
    assert_eq!(
        shape("fonts/TestHVARTwo.ttf", 1000, &[("wght", 200.0)], "AB"),
        "[uni0041|uni0042@515,0]"
    )
}

#[test]
fn hvar_2_2() {
    assert_eq!(
        shape("fonts/TestHVARTwo.ttf", 1000, &[("wght", 400.0)], "AB"),
        "[uni0041|uni0042@584,0]"
    )
}

#[test]
fn hvar_2_3() {
    assert_eq!(
        shape("fonts/TestHVARTwo.ttf", 1000, &[("wght", 600.0)], "AB"),
        "[uni0041|uni0042@673,0]"
    )
}

#[test]
fn hvar_2_4() {
    assert_eq!(
        shape("fonts/TestHVARTwo.ttf", 1000, &[("wght", 800.0)], "AB"),
        "[uni0041|uni0042@761,0]"
    )
}

#[test]
fn hvar_2_5() {
    assert_eq!(
        shape("fonts/TestHVARTwo.ttf", 1000, &[("wght", 1000.0)], "AB"),
        "[uni0041|uni0042@850,0]"
    )
}

#[test]
fn kern_1_0() {
    assert_eq!(
        shape("fonts/TestKERNOne.otf", 1000, &[], "ıTuTuTı"),
        "[dotlessi|T|u@400,0|T@600,0|u@1000,0|T@1200,0|dotlessi@1600,0]"
    )
}

#[test]
fn kern_2_0() {
    assert_eq!(
        shape("fonts/TestKERNOne.otf", 1000, &[], "uııTııTııu"),
        "[u|dotlessi@400,0|dotlessi@1100,0|T@1100,0|dotlessi@1500,0|dotlessi@2200,0|T@2200,0|dotlessi@2600,0|dotlessi@3300,0|u@3500,0]"
    )
}

#[test]
fn morx_1_0() {
    assert_eq!(
        shape("fonts/TestMORXOne.ttf", 1000, &[], "ABC"),
        "[A.alt|B@1000,0|C.alt@2000,0]"
    )
}

#[test]
fn morx_10_0() {
    assert_eq!(
        shape("fonts/TestMORXTen.ttf", 1000, &[], "ABABAB"),
        "[A|B@638,0|A@1288,0|B@1926,0|B@2576,0|A@3226,0]"
    )
}

#[test]
fn morx_11_0() {
    assert_eq!(
        shape("fonts/TestMORXEleven.ttf", 1000, &[], "BABBAABX"),
        "[B|A@650,0|B@1288,0|B@1938,0|A@2588,0|X@3226,0|A@3812,0|B@4450,0]"
    )
}

#[test]
fn morx_12_0() {
    assert_eq!(
        shape("fonts/TestMORXTwelve.ttf", 1000, &[], "XABCX1"),
        "[X|C@598,0|A@1230,0|B@1868,0|X@2518,0|one@3116,0]"
    )
}

#[test]
fn morx_12_1() {
    assert_eq!(
        shape("fonts/TestMORXTwelve.ttf", 1000, &[], "XABCX2"),
        "[X|C@598,0|A@1230,0|B@1868,0|X@2518,0|two@3116,0]"
    )
}

#[test]
fn morx_12_2() {
    assert_eq!(
        shape("fonts/TestMORXTwelve.ttf", 1000, &[], "XABCX3"),
        "[X|B@598,0|C@1248,0|A@1880,0|X@2518,0|three@3116,0]"
    )
}

#[test]
fn morx_13_0() {
    assert_eq!(
        shape("fonts/TestMORXThirteen.ttf", 1000, &[], "ABCDE"),
        "[B|C@626,0|D@1222,0|E@1896,0|A@2452,0]"
    )
}

#[test]
fn morx_14_0() {
    assert_eq!(
        shape("fonts/TestMORXFourteen.ttf", 1000, &[], "ABCDE"),
        "[B|C@626,0|D@1222,0|E@1896,0|A@2452,0]"
    )
}

#[test]
fn morx_14_1() {
    assert_eq!(
        shape("fonts/TestMORXFourteen.ttf", 0, &[], "ABBBCCCDDDBCDCE"),
        "*"
    )
}

#[test]
fn morx_16_0() {
    assert_eq!(
        shape("fonts/TestMORXSixteen.ttf", 1000, &[], "ABCDE"),
        "[B|C@626,0|D@1222,0|E@1896,0|A@2452,0]"
    )
}

#[test]
fn morx_17_0() {
    assert_eq!(
        shape("fonts/TestMORXSeventeen.ttf", 1000, &[], "AB"),
        "[B|A@626,0]"
    )
}

#[test]
fn morx_18_0() {
    assert_eq!(
        shape("fonts/TestMORXEighteen.ttf", 1000, &[], "ABCDE"),
        "[A|B.alt@639,0|C@1639,0|D.alt1@2235,0|E@3235,0]"
    )
}

#[test]
fn morx_18_1() {
    assert_eq!(
        shape("fonts/TestMORXEighteen.ttf", 1000, &[], "ABBBDE"),
        "[A|B@639,0|B@1265,0|B.alt@1891,0|D.alt1@2891,0|E@3891,0]"
    )
}

#[test]
fn morx_18_2() {
    assert_eq!(
        shape("fonts/TestMORXEighteen.ttf", 1000, &[], "ABDE"),
        "[A|B.alt@639,0|D.alt1@1639,0|E@2639,0]"
    )
}

#[test]
fn morx_18_3() {
    assert_eq!(
        shape("fonts/TestMORXEighteen.ttf", 1000, &[], "ABE"),
        "[A|B@639,0|E@1265,0]"
    )
}

#[test]
fn morx_19_0() {
    assert_eq!(
        shape("fonts/TestMORXEighteen.ttf", 1000, &[], "ACDE"),
        "[A.alt|C@1000,0|D.alt1@1596,0|E@2596,0]"
    )
}

#[test]
fn morx_19_1() {
    assert_eq!(
        shape("fonts/TestMORXEighteen.ttf", 1000, &[], "D"),
        "[D.alt]"
    )
}

#[test]
fn morx_2_0() {
    assert_eq!(
        shape("fonts/TestMORXTwo.ttf", 1000, &[], "OOOABXYZCDOOO⓿"),
        "[O|O@418,0|O@836,0|A@1254,0|B@2084,0|X@2914,0|Y@3744,0|Z@4574,0|C@5404,0|D@6234,0|O@7064,0|O@7482,0|O@7900,0|zero@8318,0]"
    )
}

#[test]
fn morx_2_1() {
    assert_eq!(
        shape("fonts/TestMORXTwo.ttf", 1000, &[], "OOOABXYZCDOOO➊"),
        "[O|O@418,0|O@836,0|B@1254,0|X@2084,0|Y@2914,0|Z@3744,0|C@4574,0|D@5404,0|A@6234,0|O@7064,0|O@7482,0|O@7900,0|one@8318,0]"
    )
}

#[test]
fn morx_2_2() {
    assert_eq!(
        shape("fonts/TestMORXTwo.ttf", 1000, &[], "OOOABXYZCDOOO➋"),
        "[O|O@418,0|O@836,0|D@1254,0|A@2084,0|B@2914,0|X@3744,0|Y@4574,0|Z@5404,0|C@6234,0|O@7064,0|O@7482,0|O@7900,0|two@8318,0]"
    )
}

#[test]
fn morx_2_3() {
    assert_eq!(
        shape("fonts/TestMORXTwo.ttf", 1000, &[], "OOOABXYZCDOOO3"),
        "[O|O@418,0|O@836,0|D@1254,0|B@2084,0|X@2914,0|Y@3744,0|Z@4574,0|C@5404,0|A@6234,0|O@7064,0|O@7482,0|O@7900,0|three@8318,0]"
    )
}

#[test]
fn morx_2_4() {
    assert_eq!(
        shape("fonts/TestMORXTwo.ttf", 1000, &[], "OOOABXYZCDOOO4"),
        "[O|O@418,0|O@836,0|X@1254,0|Y@2084,0|Z@2914,0|C@3744,0|D@4574,0|A@5404,0|B@6234,0|O@7064,0|O@7482,0|O@7900,0|four@8318,0]"
    )
}

#[test]
fn morx_2_5() {
    assert_eq!(
        shape("fonts/TestMORXTwo.ttf", 1000, &[], "OOOABXYZCDOOO5"),
        "[O|O@418,0|O@836,0|X@1254,0|Y@2084,0|Z@2914,0|C@3744,0|D@4574,0|B@5404,0|A@6234,0|O@7064,0|O@7482,0|O@7900,0|five@8318,0]"
    )
}

#[test]
fn morx_2_6() {
    assert_eq!(
        shape("fonts/TestMORXTwo.ttf", 1000, &[], "OOOABXYZCDOOO6"),
        "[O|O@418,0|O@836,0|C@1254,0|D@2084,0|A@2914,0|B@3744,0|X@4574,0|Y@5404,0|Z@6234,0|O@7064,0|O@7482,0|O@7900,0|six@8318,0]"
    )
}

#[test]
fn morx_2_7() {
    assert_eq!(
        shape("fonts/TestMORXTwo.ttf", 1000, &[], "OOOABXYZCDOOO7"),
        "[O|O@418,0|O@836,0|D@1254,0|C@2084,0|A@2914,0|B@3744,0|X@4574,0|Y@5404,0|Z@6234,0|O@7064,0|O@7482,0|O@7900,0|seven@8318,0]"
    )
}

#[test]
fn morx_2_8() {
    assert_eq!(
        shape("fonts/TestMORXTwo.ttf", 1000, &[], "OOOABXYZCDOOO8"),
        "[O|O@418,0|O@836,0|C@1254,0|D@2084,0|B@2914,0|X@3744,0|Y@4574,0|Z@5404,0|A@6234,0|O@7064,0|O@7482,0|O@7900,0|eight@8318,0]"
    )
}

#[test]
fn morx_2_9() {
    assert_eq!(
        shape("fonts/TestMORXTwo.ttf", 1000, &[], "OOOABXYZCDOOO9"),
        "[O|O@418,0|O@836,0|D@1254,0|C@2084,0|B@2914,0|X@3744,0|Y@4574,0|Z@5404,0|A@6234,0|O@7064,0|O@7482,0|O@7900,0|nine@8318,0]"
    )
}

#[test]
fn morx_2_10() {
    assert_eq!(
        shape("fonts/TestMORXTwo.ttf", 1000, &[], "OOOABXYZCDOOO➓"),
        "[O|O@418,0|O@836,0|D@1254,0|X@2084,0|Y@2914,0|Z@3744,0|C@4574,0|A@5404,0|B@6234,0|O@7064,0|O@7482,0|O@7900,0|one_zero@8318,0]"
    )
}

#[test]
fn morx_2_11() {
    assert_eq!(
        shape("fonts/TestMORXTwo.ttf", 1000, &[], "OOOABXYZCDOOO⓫"),
        "[O|O@418,0|O@836,0|D@1254,0|X@2084,0|Y@2914,0|Z@3744,0|C@4574,0|B@5404,0|A@6234,0|O@7064,0|O@7482,0|O@7900,0|one_one@8318,0]"
    )
}

#[test]
fn morx_2_12() {
    assert_eq!(
        shape("fonts/TestMORXTwo.ttf", 1000, &[], "OOOABXYZCDOOO⓬"),
        "[O|O@418,0|O@836,0|C@1254,0|D@2084,0|X@2914,0|Y@3744,0|Z@4574,0|A@5404,0|B@6234,0|O@7064,0|O@7482,0|O@7900,0|one_two@8318,0]"
    )
}

#[test]
fn morx_2_13() {
    assert_eq!(
        shape("fonts/TestMORXTwo.ttf", 1000, &[], "OOOABXYZCDOOO⓭"),
        "[O|O@418,0|O@836,0|C@1254,0|D@2084,0|X@2914,0|Y@3744,0|Z@4574,0|B@5404,0|A@6234,0|O@7064,0|O@7482,0|O@7900,0|one_three@8318,0]"
    )
}

#[test]
fn morx_2_14() {
    assert_eq!(
        shape("fonts/TestMORXTwo.ttf", 1000, &[], "OOOABXYZCDOOO⓮"),
        "[O|O@418,0|O@836,0|D@1254,0|C@2084,0|X@2914,0|Y@3744,0|Z@4574,0|A@5404,0|B@6234,0|O@7064,0|O@7482,0|O@7900,0|one_four@8318,0]"
    )
}

#[test]
fn morx_2_15() {
    assert_eq!(
        shape("fonts/TestMORXTwo.ttf", 1000, &[], "OOOABXYZCDOOO⓯"),
        "[O|O@418,0|O@836,0|D@1254,0|C@2084,0|X@2914,0|Y@3744,0|Z@4574,0|B@5404,0|A@6234,0|O@7064,0|O@7482,0|O@7900,0|one_five@8318,0]"
    )
}

#[test]
fn morx_20_0() {
    assert_eq!(
        shape("fonts/TestMORXTwenty.ttf", 1000, &[], "ABCDE"),
        "[A|B@639,0|C.alt@1265,0|D@2265,0|E.alt1@2939,0]"
    )
}

#[test]
fn morx_20_1() {
    assert_eq!(
        shape("fonts/TestMORXTwenty.ttf", 1000, &[], "ABC"),
        "[A|B@639,0|C.alt@1265,0]"
    )
}

#[test]
fn morx_20_2() {
    assert_eq!(
        shape("fonts/TestMORXTwenty.ttf", 1000, &[], "ABE"),
        "[A|B.alt@639,0|E.alt1@1639,0]"
    )
}

#[test]
fn morx_20_3() {
    assert_eq!(
        shape("fonts/TestMORXTwenty.ttf", 1000, &[], "AE"),
        "[A.alt|E.alt1@1000,0]"
    )
}

#[test]
fn morx_20_4() {
    assert_eq!(
        shape("fonts/TestMORXTwenty.ttf", 1000, &[], "EE"),
        "[E|E@556,0]"
    )
}

#[test]
fn morx_20_5() {
    assert_eq!(shape("fonts/TestMORXTwenty.ttf", 1000, &[], "A"), "[A.alt]")
}

#[test]
fn morx_20_6() {
    assert_eq!(shape("fonts/TestMORXTwenty.ttf", 1000, &[], "E"), "[E]")
}

#[test]
fn morx_21_0() {
    assert_eq!(
        shape("fonts/TestMORXTwentyone.ttf", 1000, &[], "ABCDE"),
        "[A|B.alt@639,0|C@1639,0|D@2235,0|E@2909,0]"
    )
}

#[test]
fn morx_22_0() {
    assert_eq!(shape("fonts/TestMORXTwentytwo.ttf", 1000, &[], "A"), "[C]")
}

#[test]
fn morx_23_0() {
    assert_eq!(
        shape("fonts/TestMORXTwentythree.ttf", 1000, &[], "ABCDE"),
        "[E|E@556,0|E@1112,0|E@1668,0|E@2224,0]"
    )
}

#[test]
fn morx_24_0() {
    assert_eq!(
        shape("fonts/TestMORXTwentyfour.ttf", 0, &[], "ABCDE"),
        "*"
    )
}

#[test]
fn morx_25_0() {
    assert_eq!(
        shape("fonts/TestMORXTwentyfive.ttf", 1000, &[], "ABCDE"),
        "[A.alt|B.alt@1000,0|C.alt@2000,0|D.alt@3000,0|E.alt@4000,0]"
    )
}

#[test]
fn morx_25_1() {
    assert_eq!(
        shape("fonts/TestMORXTwentyfive.ttf", 1000, &[], "EBCDA"),
        "[E|B@556,0|C@1182,0|D@1778,0|A@2452,0]"
    )
}

#[test]
fn morx_25_2() {
    assert_eq!(
        shape("fonts/TestMORXTwentyfive.ttf", 1000, &[], "CBABC"),
        "[C|B@596,0|A.alt@1222,0|B.alt@2222,0|C.alt@3222,0]"
    )
}

#[test]
fn morx_25_3() {
    assert_eq!(
        shape("fonts/TestMORXTwentyfive.ttf", 1000, &[], "ABC"),
        "[A.alt|B.alt@1000,0|C.alt@2000,0]"
    )
}

#[test]
fn morx_25_4() {
    assert_eq!(
        shape("fonts/TestMORXTwentyfive.ttf", 1000, &[], "CBA"),
        "[C|B@596,0|A@1222,0]"
    )
}

#[test]
fn morx_25_5() {
    assert_eq!(
        shape("fonts/TestMORXTwentyfive.ttf", 1000, &[], "AB"),
        "[A.alt|B.alt@1000,0]"
    )
}

#[test]
fn morx_25_6() {
    assert_eq!(
        shape("fonts/TestMORXTwentyfive.ttf", 1000, &[], "BA"),
        "[B|A@626,0]"
    )
}

#[test]
fn morx_25_7() {
    assert_eq!(shape("fonts/TestMORXTwentyfive.ttf", 1000, &[], "A"), "[A]")
}

#[test]
fn morx_25_8() {
    assert_eq!(shape("fonts/TestMORXTwentyfive.ttf", 1000, &[], "B"), "[B]")
}

#[test]
fn morx_26_0() {
    assert_eq!(
        shape("fonts/TestMORXTwentysix.ttf", 1000, &[], "AB"),
        "[A|B@639,0]"
    )
}

#[test]
fn morx_26_1() {
    assert_eq!(
        shape("fonts/TestMORXTwentysix.ttf", 1000, &[], "B"),
        "[B.alt]"
    )
}

#[test]
fn morx_27_0() {
    assert_eq!(
        shape("fonts/TestMORXTwentyseven.ttf", 1000, &[], "AEB"),
        "[A_E_B]"
    )
}

#[test]
fn morx_27_1() {
    assert_eq!(
        shape("fonts/TestMORXTwentyseven.ttf", 1000, &[], "AEC"),
        "[A_E_C]"
    )
}

#[test]
fn morx_27_2() {
    assert_eq!(
        shape("fonts/TestMORXTwentyseven.ttf", 1000, &[], "AED"),
        "[A_E_D]"
    )
}

#[test]
fn morx_28_0() {
    assert_eq!(
        shape("fonts/TestMORXTwentyeight.ttf", 1000, &[], "AED"),
        "[A_E_D]"
    )
}

#[test]
fn morx_28_1() {
    assert_eq!(
        shape("fonts/TestMORXTwentyeight.ttf", 1000, &[], "AxED"),
        "[A_E_D|x@1394,0]"
    )
}

#[test]
fn morx_28_2() {
    assert_eq!(
        shape("fonts/TestMORXTwentyeight.ttf", 1000, &[], "AEyD"),
        "[A_E_D|y@1394,0]"
    )
}

#[test]
fn morx_28_3() {
    assert_eq!(
        shape("fonts/TestMORXTwentyeight.ttf", 1000, &[], "AxEyD"),
        "[A_E_D|x@1394,0|y@1923,0]"
    )
}

#[test]
fn morx_28_4() {
    assert_eq!(
        shape("fonts/TestMORXTwentyeight.ttf", 1000, &[], "AxxxEyyyD"),
        "[A_E_D|x@1394,0|x@1923,0|x@2452,0|y@2981,0|y@3491,0|y@4001,0]"
    )
}

#[test]
fn morx_29_0() {
    assert_eq!(
        shape("fonts/TestMORXTwentynine.ttf", 1000, &[], "PQRMMXXMMYYAZZ"),
        "[P|Q@333,0|R@699,0|M@1050,0|M@1880,0|X@2710,0|X@3074,0|M@3438,0|I@4268,0|N@5098,0|S@5928,0|M@6758,0|Y@7588,0|Y@7920,0|A@8252,0|Z@9082,0|Z@9404,0]"
    )
}

#[test]
fn morx_29_1() {
    assert_eq!(
        shape("fonts/TestMORXTwentynine.ttf", 1000, &[], "PQRMMXXMMYYBZZ"),
        "[P|Q@333,0|R@699,0|M@1050,0|M@1880,0|X@2710,0|X@3074,0|M@3438,0|M@4268,0|I@5098,0|N@5928,0|S@6758,0|Y@7588,0|Y@7920,0|B@8252,0|Z@9082,0|Z@9404,0]"
    )
}

#[test]
fn morx_29_2() {
    assert_eq!(
        shape("fonts/TestMORXTwentynine.ttf", 1000, &[], "PQRMMXXMMYYCZZ"),
        "[P|Q@333,0|R@699,0|M@1050,0|M@1880,0|X@2710,0|X@3074,0|M@3438,0|M@4268,0|Y@5098,0|Y@5430,0|I@5762,0|N@6592,0|S@7422,0|C@8252,0|Z@9082,0|Z@9404,0]"
    )
}

#[test]
fn morx_29_3() {
    assert_eq!(
        shape("fonts/TestMORXTwentynine.ttf", 1000, &[], "PQRMMXXMMYYDZZ"),
        "[P|Q@333,0|R@699,0|M@1050,0|M@1880,0|X@2710,0|X@3074,0|M@3438,0|M@4268,0|Y@5098,0|Y@5430,0|D@5762,0|I@6592,0|N@7422,0|S@8252,0|Z@9082,0|Z@9404,0]"
    )
}

#[test]
fn morx_3_0() {
    assert_eq!(
        shape("fonts/TestMORXThree.ttf", 1000, &[], "ABXCD0"),
        "[A|B@363,0|X@722,0|C@1086,0|D@1402,0|zero@1793,0]"
    )
}

#[test]
fn morx_3_1() {
    assert_eq!(
        shape("fonts/TestMORXThree.ttf", 1000, &[], "ABXCD1"),
        "[A|B@363,0|X@722,0|C@1086,0|D@1402,0|one@1793,0]"
    )
}

#[test]
fn morx_3_2() {
    assert_eq!(
        shape("fonts/TestMORXThree.ttf", 1000, &[], "ABXCD2"),
        "[A|B@363,0|X@722,0|C@1086,0|D@1402,0|two@1793,0]"
    )
}

#[test]
fn morx_3_3() {
    assert_eq!(
        shape("fonts/TestMORXThree.ttf", 1000, &[], "ABXCD3"),
        "[A|B@363,0|X@722,0|C@1086,0|D@1402,0|three@1793,0]"
    )
}

#[test]
fn morx_3_4() {
    assert_eq!(
        shape("fonts/TestMORXThree.ttf", 1000, &[], "ABXCD4"),
        "[A|B@363,0|X@722,0|C@1086,0|D@1402,0|four@1793,0]"
    )
}

#[test]
fn morx_3_5() {
    assert_eq!(
        shape("fonts/TestMORXThree.ttf", 1000, &[], "ABXCD5"),
        "[A|B@363,0|X@722,0|C@1086,0|D@1402,0|five@1793,0]"
    )
}

#[test]
fn morx_3_6() {
    assert_eq!(
        shape("fonts/TestMORXThree.ttf", 1000, &[], "ABXCD6"),
        "[A|B@363,0|X@722,0|C@1086,0|D@1402,0|six@1793,0]"
    )
}

#[test]
fn morx_3_7() {
    assert_eq!(
        shape("fonts/TestMORXThree.ttf", 1000, &[], "ABXCD7"),
        "[A|B@363,0|X@722,0|C@1086,0|D@1402,0|seven@1793,0]"
    )
}

#[test]
fn morx_3_8() {
    assert_eq!(
        shape("fonts/TestMORXThree.ttf", 1000, &[], "ABXCD8"),
        "[A|B@363,0|X@722,0|C@1086,0|D@1402,0|eight@1793,0]"
    )
}

#[test]
fn morx_3_9() {
    assert_eq!(
        shape("fonts/TestMORXThree.ttf", 1000, &[], "ABXCD9"),
        "[A|B@363,0|X@722,0|C@1086,0|D@1402,0|nine@1793,0]"
    )
}

#[test]
fn morx_3_10() {
    assert_eq!(
        shape("fonts/TestMORXThree.ttf", 1000, &[], "ABXCD➓"),
        "[A|B@363,0|X@722,0|C@1086,0|D@1402,0|one_zero@1793,0]"
    )
}

#[test]
fn morx_3_11() {
    assert_eq!(
        shape("fonts/TestMORXThree.ttf", 1000, &[], "ABXCD⓫"),
        "[A|B@363,0|X@722,0|C@1086,0|D@1402,0|one_one@1793,0]"
    )
}

#[test]
fn morx_3_12() {
    assert_eq!(
        shape("fonts/TestMORXThree.ttf", 1000, &[], "ABXCD⓬"),
        "[A|B@363,0|X@722,0|C@1086,0|D@1402,0|one_two@1793,0]"
    )
}

#[test]
fn morx_3_13() {
    assert_eq!(
        shape("fonts/TestMORXThree.ttf", 1000, &[], "ABXCD⓭"),
        "[A|B@363,0|X@722,0|C@1086,0|D@1402,0|one_three@1793,0]"
    )
}

#[test]
fn morx_3_14() {
    assert_eq!(
        shape("fonts/TestMORXThree.ttf", 1000, &[], "ABXCD⓮"),
        "[A|B@363,0|X@722,0|C@1086,0|D@1402,0|one_four@1793,0]"
    )
}

#[test]
fn morx_3_15() {
    assert_eq!(
        shape("fonts/TestMORXThree.ttf", 1000, &[], "ABXCD⓯"),
        "[A|B@363,0|X@722,0|C@1086,0|D@1402,0|one_five@1793,0]"
    )
}

#[test]
fn morx_30_0() {
    assert_eq!(
        shape("fonts/TestMORXTwentynine.ttf", 1000, &[], "PQRMMXXXAYYAZZ"),
        "[P|Q@333,0|R@699,0|M@1050,0|I@1880,0|N@2710,0|S@3540,0|I@4370,0|N@5200,0|S@6030,0|M@6860,0|X@7690,0|X@8054,0|X@8418,0|A@8782,0|Y@9612,0|Y@9944,0|A@10276,0|Z@11106,0|Z@11428,0]"
    )
}

#[test]
fn morx_30_1() {
    assert_eq!(
        shape("fonts/TestMORXTwentynine.ttf", 1000, &[], "PQRMMXXXAYYBZZ"),
        "[P|Q@333,0|R@699,0|M@1050,0|I@1880,0|I@2710,0|N@3540,0|S@4370,0|N@5200,0|S@6030,0|M@6860,0|X@7690,0|X@8054,0|X@8418,0|A@8782,0|Y@9612,0|Y@9944,0|B@10276,0|Z@11106,0|Z@11428,0]"
    )
}

#[test]
fn morx_30_2() {
    assert_eq!(
        shape("fonts/TestMORXTwentynine.ttf", 1000, &[], "PQRMMXXXBYYAZZ"),
        "[P|Q@333,0|R@699,0|M@1050,0|I@1880,0|N@2710,0|S@3540,0|M@4370,0|I@5200,0|N@6030,0|S@6860,0|X@7690,0|X@8054,0|X@8418,0|B@8782,0|Y@9612,0|Y@9944,0|A@10276,0|Z@11106,0|Z@11428,0]"
    )
}

#[test]
fn morx_30_3() {
    assert_eq!(
        shape("fonts/TestMORXTwentynine.ttf", 1000, &[], "PQRMMXXXBYYBZZ"),
        "[P|Q@333,0|R@699,0|M@1050,0|M@1880,0|I@2710,0|N@3540,0|S@4370,0|I@5200,0|N@6030,0|S@6860,0|X@7690,0|X@8054,0|X@8418,0|B@8782,0|Y@9612,0|Y@9944,0|B@10276,0|Z@11106,0|Z@11428,0]"
    )
}

#[test]
fn morx_31_0() {
    assert_eq!(
        shape("fonts/TestMORXThirtyone.ttf", 1000, &[], "XXAYYAZZ"),
        "[I|N@830,0|I@1660,0|N@2490,0|S@3320,0|S@4150,0|X@4980,0|X@5344,0|A@5708,0|Y@6538,0|Y@6870,0|A@7202,0|Z@8032,0|Z@8354,0]"
    )
}

#[test]
fn morx_31_1() {
    assert_eq!(
        shape("fonts/TestMORXThirtyone.ttf", 1000, &[], "XXAYYBYY"),
        "[I|N@830,0|S@1660,0|I@2490,0|N@3320,0|S@4150,0|X@4980,0|X@5344,0|A@5708,0|Y@6538,0|Y@6870,0|B@7202,0|Y@8032,0|Y@8364,0]"
    )
}

#[test]
fn morx_31_2() {
    assert_eq!(
        shape("fonts/TestMORXThirtyone.ttf", 1000, &[], "XXBYYAZZ"),
        "[X|I@364,0|I@1194,0|N@2024,0|S@2854,0|N@3684,0|S@4514,0|X@5344,0|B@5708,0|Y@6538,0|Y@6870,0|A@7202,0|Z@8032,0|Z@8354,0]"
    )
}

#[test]
fn morx_31_3() {
    assert_eq!(
        shape("fonts/TestMORXThirtyone.ttf", 1000, &[], "XXBYYBZZ"),
        "[X|I@364,0|N@1194,0|I@2024,0|N@2854,0|S@3684,0|S@4514,0|X@5344,0|B@5708,0|Y@6538,0|Y@6870,0|B@7202,0|Z@8032,0|Z@8354,0]"
    )
}

#[test]
fn morx_31_4() {
    assert_eq!(
        shape("fonts/TestMORXThirtyone.ttf", 1000, &[], "MPQRAXYZA"),
        "[I|N@830,0|S@1660,0|M@2490,0|I@3320,0|N@4150,0|S@4980,0|P@5810,0|Q@6143,0|R@6509,0|A@6860,0|X@7690,0|Y@8054,0|Z@8386,0|A@8708,0]"
    )
}

#[test]
fn morx_31_5() {
    assert_eq!(
        shape("fonts/TestMORXThirtyone.ttf", 1000, &[], "MPQRAXYZB"),
        "[I|N@830,0|S@1660,0|M@2490,0|P@3320,0|I@3653,0|N@4483,0|S@5313,0|Q@6143,0|R@6509,0|A@6860,0|X@7690,0|Y@8054,0|Z@8386,0|B@8708,0]"
    )
}

#[test]
fn morx_31_6() {
    assert_eq!(
        shape("fonts/TestMORXThirtyone.ttf", 1000, &[], "MPQRBXYZA"),
        "[M|I@830,0|N@1660,0|S@2490,0|I@3320,0|N@4150,0|S@4980,0|P@5810,0|Q@6143,0|R@6509,0|B@6860,0|X@7690,0|Y@8054,0|Z@8386,0|A@8708,0]"
    )
}

#[test]
fn morx_31_7() {
    assert_eq!(
        shape("fonts/TestMORXThirtyone.ttf", 1000, &[], "MPQRBXYZB"),
        "[M|I@830,0|N@1660,0|S@2490,0|P@3320,0|I@3653,0|N@4483,0|S@5313,0|Q@6143,0|R@6509,0|B@6860,0|X@7690,0|Y@8054,0|Z@8386,0|B@8708,0]"
    )
}

#[test]
fn morx_32_0() {
    assert_eq!(
        shape("fonts/TestMORXThirtytwo.ttf", 1000, &[], "A"),
        "[I|N@830,0|S@1660,0|A@2490,0]"
    )
}

#[test]
fn morx_32_1() {
    assert_eq!(
        shape("fonts/TestMORXThirtytwo.ttf", 1000, &[], "XAY"),
        "[I|N@830,0|S@1660,0|X@2490,0|A@2854,0|Y@3684,0]"
    )
}

#[test]
fn morx_32_2() {
    assert_eq!(
        shape("fonts/TestMORXThirtytwo.ttf", 1000, &[], "B"),
        "[B|I@830,0|N@1660,0|S@2490,0]"
    )
}

#[test]
fn morx_32_3() {
    assert_eq!(
        shape("fonts/TestMORXThirtytwo.ttf", 1000, &[], "XBY"),
        "[X|I@364,0|N@1194,0|S@2024,0|B@2854,0|Y@3684,0]"
    )
}

#[test]
fn morx_33_0() {
    assert_eq!(
        shape("fonts/TestMORXThirtythree.ttf", 1000, &[], "ha"),
        "[h|a@618,0|h@1179,0|a@1797,0]"
    )
}

#[test]
fn morx_33_1() {
    assert_eq!(
        shape("fonts/TestMORXThirtythree.ttf", 1000, &[], "haha"),
        "[h|a@618,0|h@1179,0|a@1797,0|h@2358,0|a@2976,0|h@3537,0|a@4155,0]"
    )
}

#[test]
fn morx_33_2() {
    assert_eq!(
        shape("fonts/TestMORXThirtythree.ttf", 1000, &[], "ah"),
        "[a|h@561,0]"
    )
}

#[test]
fn morx_34_0() {
    assert_eq!(shape("fonts/TestMORXThirtyfour.ttf", 0, &[], "ha"), "*")
}

#[test]
fn morx_35_0() {
    assert_eq!(
        shape("fonts/TestMORXThirtyfive.ttf", 1000, &[], "A"),
        "[A|B@639,0|C@1265,0|E@1861,0]"
    )
}

#[test]
fn morx_35_1() {
    assert_eq!(
        shape("fonts/TestMORXThirtyfive.ttf", 1000, &[], "XAY"),
        "[X|A@586,0|B@1225,0|C@1851,0|E@2447,0|Y@3003,0]"
    )
}

#[test]
fn morx_36_0() {
    assert_eq!(shape("fonts/TestMORXThirtysix.ttf", 0, &[], "A"), "*")
}

#[test]
fn morx_37_0() {
    assert_eq!(
        shape("fonts/TestMORXThirtyseven.ttf", 1000, &[], "AB"),
        "[A.alt|B.alt@1000,0]"
    )
}

#[test]
fn morx_37_1() {
    assert_eq!(
        shape("fonts/TestMORXThirtyseven.ttf", 1000, &[], "BA"),
        "[B|A@650,0]"
    )
}

#[test]
fn morx_37_2() {
    assert_eq!(
        shape("fonts/TestMORXThirtyseven.ttf", 1000, &[], "אב"),
        "[uni05D1|uni05D0@542,0]"
    )
}

#[test]
fn morx_37_3() {
    assert_eq!(
        shape("fonts/TestMORXThirtyseven.ttf", 1000, &[], "בא"),
        "[uni05D0.alt|uni05D1.alt@1000,0]"
    )
}

#[test]
fn morx_38_0() {
    assert_eq!(
        shape("fonts/TestMORXThirtyeight.ttf", 1000, &[], "AB"),
        "[A.alt|B.alt@1000,0]"
    )
}

#[test]
fn morx_38_1() {
    assert_eq!(
        shape("fonts/TestMORXThirtyeight.ttf", 1000, &[], "BA"),
        "[B|A@650,0]"
    )
}

#[test]
fn morx_38_2() {
    assert_eq!(
        shape("fonts/TestMORXThirtyeight.ttf", 1000, &[], "אב"),
        "[uni05D1.alt|uni05D0.alt@1000,0]"
    )
}

#[test]
fn morx_38_3() {
    assert_eq!(
        shape("fonts/TestMORXThirtyeight.ttf", 1000, &[], "בא"),
        "[uni05D0|uni05D1@606,0]"
    )
}

#[test]
fn morx_39_0() {
    assert_eq!(
        shape("fonts/TestMORXThirtynine.ttf", 1000, &[], "AB"),
        "[A|B@639,0]"
    )
}

#[test]
fn morx_39_1() {
    assert_eq!(
        shape("fonts/TestMORXThirtynine.ttf", 1000, &[], "BA"),
        "[B.alt|A.alt@1000,0]"
    )
}

#[test]
fn morx_39_2() {
    assert_eq!(
        shape("fonts/TestMORXThirtynine.ttf", 1000, &[], "אב"),
        "[uni05D1.alt|uni05D0.alt@1000,0]"
    )
}

#[test]
fn morx_39_3() {
    assert_eq!(
        shape("fonts/TestMORXThirtynine.ttf", 1000, &[], "בא"),
        "[uni05D0|uni05D1@606,0]"
    )
}

#[test]
fn morx_4_0() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRAXYZ1"),
        "[P|Q@333,0|R@699,0|A@1050,0|X@1880,0|Y@2244,0|Z@2576,0|one@2898,0]"
    )
}

#[test]
fn morx_4_1() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRAXYZ2"),
        "[P|Q@333,0|R@699,0|A@1050,0|X@1880,0|Y@2244,0|Z@2576,0|two@2898,0]"
    )
}

#[test]
fn morx_4_2() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRADXYZ3"),
        "[P|Q@333,0|R@699,0|D@1050,0|A@1880,0|X@2710,0|Y@3074,0|Z@3406,0|three@3728,0]"
    )
}

#[test]
fn morx_4_3() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABXYZ4"),
        "[P|Q@333,0|R@699,0|A@1050,0|B@1880,0|X@2710,0|Y@3074,0|Z@3406,0|four@3728,0]"
    )
}

#[test]
fn morx_4_4() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABXYZ5"),
        "[P|Q@333,0|R@699,0|B@1050,0|A@1880,0|X@2710,0|Y@3074,0|Z@3406,0|five@3728,0]"
    )
}

#[test]
fn morx_4_5() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABXYZ6"),
        "[P|Q@333,0|R@699,0|A@1050,0|B@1880,0|X@2710,0|Y@3074,0|Z@3406,0|six@3728,0]"
    )
}

#[test]
fn morx_4_6() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABXYZ7"),
        "[P|Q@333,0|R@699,0|B@1050,0|A@1880,0|X@2710,0|Y@3074,0|Z@3406,0|seven@3728,0]"
    )
}

#[test]
fn morx_4_7() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRACDXYZ8"),
        "[P|Q@333,0|R@699,0|C@1050,0|D@1880,0|A@2710,0|X@3540,0|Y@3904,0|Z@4236,0|eight@4558,0]"
    )
}

#[test]
fn morx_4_8() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRACDXYZ9"),
        "[P|Q@333,0|R@699,0|D@1050,0|C@1880,0|A@2710,0|X@3540,0|Y@3904,0|Z@4236,0|nine@4558,0]"
    )
}

#[test]
fn morx_4_9() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABDXYZ➓"),
        "[P|Q@333,0|R@699,0|D@1050,0|A@1880,0|B@2710,0|X@3540,0|Y@3904,0|Z@4236,0|one_zero@4558,0]"
    )
}

#[test]
fn morx_4_10() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABDXYZ⓫"),
        "[P|Q@333,0|R@699,0|D@1050,0|B@1880,0|A@2710,0|X@3540,0|Y@3904,0|Z@4236,0|one_one@4558,0]"
    )
}

#[test]
fn morx_4_11() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABCDXYZ⓬"),
        "[P|Q@333,0|R@699,0|C@1050,0|D@1880,0|A@2710,0|B@3540,0|X@4370,0|Y@4734,0|Z@5066,0|one_two@5388,0]"
    )
}

#[test]
fn morx_4_12() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABCDXYZ⓭"),
        "[P|Q@333,0|R@699,0|C@1050,0|D@1880,0|B@2710,0|A@3540,0|X@4370,0|Y@4734,0|Z@5066,0|one_three@5388,0]"
    )
}

#[test]
fn morx_4_13() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABCDXYZ⓮"),
        "[P|Q@333,0|R@699,0|D@1050,0|C@1880,0|A@2710,0|B@3540,0|X@4370,0|Y@4734,0|Z@5066,0|one_four@5388,0]"
    )
}

#[test]
fn morx_4_14() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABCDXYZ⓯"),
        "[P|Q@333,0|R@699,0|D@1050,0|C@1880,0|B@2710,0|A@3540,0|X@4370,0|Y@4734,0|Z@5066,0|one_five@5388,0]"
    )
}

#[test]
fn morx_40_0() {
    assert_eq!(
        shape("fonts/TestMORXForty.ttf", 1000, &[], "AB"),
        "[A|B@639,0]"
    )
}

#[test]
fn morx_40_1() {
    assert_eq!(
        shape("fonts/TestMORXForty.ttf", 1000, &[], "BA"),
        "[B.alt|A.alt@1000,0]"
    )
}

#[test]
fn morx_40_2() {
    assert_eq!(
        shape("fonts/TestMORXForty.ttf", 1000, &[], "אב"),
        "[uni05D1|uni05D0@542,0]"
    )
}

#[test]
fn morx_40_3() {
    assert_eq!(
        shape("fonts/TestMORXForty.ttf", 1000, &[], "בא"),
        "[uni05D0.alt|uni05D1.alt@1000,0]"
    )
}

#[test]
fn morx_41_0() {
    assert_eq!(
        shape("fonts/TestMORXFourtyone.ttf", 1000, &[], "ac"),
        "[a_c]"
    )
}

#[test]
fn morx_41_1() {
    assert_eq!(
        shape("fonts/TestMORXFourtyone.ttf", 1000, &[], "bc"),
        "[b_c]"
    )
}

#[test]
fn morx_41_2() {
    assert_eq!(shape("fonts/TestMORXFourtyone.ttf", 0, &[], "cc"), "*")
}

#[test]
fn morx_41_3() {
    assert_eq!(shape("fonts/TestMORXFourtyone.ttf", 0, &[], "abcc"), "*")
}

#[test]
fn morx_5_0() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRAXYZ3"),
        "[P|Q@333,0|R@699,0|A@1050,0|X@1880,0|Y@2244,0|Z@2576,0|three@2898,0]"
    )
}

#[test]
fn morx_5_1() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRAXYZ4"),
        "[P|Q@333,0|R@699,0|A@1050,0|X@1880,0|Y@2244,0|Z@2576,0|four@2898,0]"
    )
}

#[test]
fn morx_5_2() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRAXYZ5"),
        "[P|Q@333,0|R@699,0|A@1050,0|X@1880,0|Y@2244,0|Z@2576,0|five@2898,0]"
    )
}

#[test]
fn morx_5_3() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRAXYZ6"),
        "[P|Q@333,0|R@699,0|A@1050,0|X@1880,0|Y@2244,0|Z@2576,0|six@2898,0]"
    )
}

#[test]
fn morx_5_4() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRAXYZ7"),
        "[P|Q@333,0|R@699,0|A@1050,0|X@1880,0|Y@2244,0|Z@2576,0|seven@2898,0]"
    )
}

#[test]
fn morx_5_5() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRAXYZ8"),
        "[P|Q@333,0|R@699,0|A@1050,0|X@1880,0|Y@2244,0|Z@2576,0|eight@2898,0]"
    )
}

#[test]
fn morx_5_6() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABXYZ8"),
        "[P|Q@333,0|R@699,0|A@1050,0|B@1880,0|X@2710,0|Y@3074,0|Z@3406,0|eight@3728,0]"
    )
}

#[test]
fn morx_5_7() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRAXYZ9"),
        "[P|Q@333,0|R@699,0|A@1050,0|X@1880,0|Y@2244,0|Z@2576,0|nine@2898,0]"
    )
}

#[test]
fn morx_5_8() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABXYZ9"),
        "[P|Q@333,0|R@699,0|A@1050,0|B@1880,0|X@2710,0|Y@3074,0|Z@3406,0|nine@3728,0]"
    )
}

#[test]
fn morx_5_9() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRAXYZ➓"),
        "[P|Q@333,0|R@699,0|A@1050,0|X@1880,0|Y@2244,0|Z@2576,0|one_zero@2898,0]"
    )
}

#[test]
fn morx_5_10() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABXYZ➓"),
        "[P|Q@333,0|R@699,0|A@1050,0|B@1880,0|X@2710,0|Y@3074,0|Z@3406,0|one_zero@3728,0]"
    )
}

#[test]
fn morx_5_11() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRAXYZ⓫"),
        "[P|Q@333,0|R@699,0|A@1050,0|X@1880,0|Y@2244,0|Z@2576,0|one_one@2898,0]"
    )
}

#[test]
fn morx_5_12() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABXYZ⓫"),
        "[P|Q@333,0|R@699,0|A@1050,0|B@1880,0|X@2710,0|Y@3074,0|Z@3406,0|one_one@3728,0]"
    )
}

#[test]
fn morx_5_13() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRAXYZ⓬"),
        "[P|Q@333,0|R@699,0|A@1050,0|X@1880,0|Y@2244,0|Z@2576,0|one_two@2898,0]"
    )
}

#[test]
fn morx_5_14() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABXYZ⓬"),
        "[P|Q@333,0|R@699,0|A@1050,0|B@1880,0|X@2710,0|Y@3074,0|Z@3406,0|one_two@3728,0]"
    )
}

#[test]
fn morx_5_15() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABCXYZ⓬"),
        "[P|Q@333,0|R@699,0|A@1050,0|B@1880,0|C@2710,0|X@3540,0|Y@3904,0|Z@4236,0|one_two@4558,0]"
    )
}

#[test]
fn morx_5_16() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRAXYZ⓭"),
        "[P|Q@333,0|R@699,0|A@1050,0|X@1880,0|Y@2244,0|Z@2576,0|one_three@2898,0]"
    )
}

#[test]
fn morx_5_17() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABXYZ⓭"),
        "[P|Q@333,0|R@699,0|A@1050,0|B@1880,0|X@2710,0|Y@3074,0|Z@3406,0|one_three@3728,0]"
    )
}

#[test]
fn morx_5_18() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABCXYZ⓭"),
        "[P|Q@333,0|R@699,0|A@1050,0|B@1880,0|C@2710,0|X@3540,0|Y@3904,0|Z@4236,0|one_three@4558,0]"
    )
}

#[test]
fn morx_5_19() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRAXYZ⓮"),
        "[P|Q@333,0|R@699,0|A@1050,0|X@1880,0|Y@2244,0|Z@2576,0|one_four@2898,0]"
    )
}

#[test]
fn morx_5_20() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABXYZ⓮"),
        "[P|Q@333,0|R@699,0|A@1050,0|B@1880,0|X@2710,0|Y@3074,0|Z@3406,0|one_four@3728,0]"
    )
}

#[test]
fn morx_5_21() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABCXYZ⓮"),
        "[P|Q@333,0|R@699,0|A@1050,0|B@1880,0|C@2710,0|X@3540,0|Y@3904,0|Z@4236,0|one_four@4558,0]"
    )
}

#[test]
fn morx_5_22() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRAXYZ⓯"),
        "[P|Q@333,0|R@699,0|A@1050,0|X@1880,0|Y@2244,0|Z@2576,0|one_five@2898,0]"
    )
}

#[test]
fn morx_5_23() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABXYZ⓯"),
        "[P|Q@333,0|R@699,0|A@1050,0|B@1880,0|X@2710,0|Y@3074,0|Z@3406,0|one_five@3728,0]"
    )
}

#[test]
fn morx_5_24() {
    assert_eq!(
        shape("fonts/TestMORXFour.ttf", 1000, &[], "PQRABCXYZ⓯"),
        "[P|Q@333,0|R@699,0|A@1050,0|B@1880,0|C@2710,0|X@3540,0|Y@3904,0|Z@4236,0|one_five@4558,0]"
    )
}

#[test]
fn morx_6_0() {
    assert_eq!(
        shape("fonts/TestMORXTwo.ttf", 1000, &[], "OOOABCDEFGOOO3141"),
        "[O|O@418,0|O@836,0|E@1254,0|F@2084,0|A@2914,0|G@3744,0|B@4574,0|C@5404,0|D@6234,0|O@7064,0|O@7482,0|O@7900,0|three@8318,0|one@9168,0|four@10018,0|one@10868,0]"
    )
}

#[test]
fn morx_7_0() {
    assert_eq!(
        shape("fonts/TestMORXTwo.ttf", 1000, &[], "OBCD1"),
        "[B|C@830,0|D@1660,0|O@2490,0|one@2908,0]"
    )
}

#[test]
fn morx_8_0() {
    assert_eq!(
        shape("fonts/TestMORXEight.ttf", 1000, &[], "0ABC"),
        "[zero|A@914,0|B@1552,0|C@2202,0]"
    )
}

#[test]
fn morx_8_1() {
    assert_eq!(
        shape("fonts/TestMORXEight.ttf", 1000, &[], "1ABC"),
        "[one|B@914,0|C@1564,0|A@2196,0]"
    )
}

#[test]
fn morx_8_2() {
    assert_eq!(
        shape("fonts/TestMORXEight.ttf", 1000, &[], "2ABC"),
        "[two|C@914,0|A@1546,0|B@2184,0]"
    )
}

#[test]
fn morx_9_0() {
    assert_eq!(
        shape("fonts/TestMORXNine.ttf", 1000, &[], "ABXAB"),
        "[B|A@650,0|X@1288,0|A@1874,0|B@2512,0]"
    )
}

#[test]
fn sfnt_1_0() {
    assert_eq!(shape("fonts/TestSFNTOne.otf", 1000, &[], "A"), "[A]")
}

#[test]
fn sfnt_1_1() {
    assert_eq!(shape("fonts/TestSFNTOne.otf", 1000, &[], "B"), "[B]")
}

#[test]
fn sfnt_2_0() {
    assert_eq!(shape("fonts/TestSFNTTwo.ttf", 1000, &[], "A"), "[A]")
}

#[test]
fn sfnt_2_1() {
    assert_eq!(shape("fonts/TestSFNTTwo.ttf", 1000, &[], "B"), "[B]")
}

#[test]
fn sharan_1_0() {
    assert_eq!(
        shape("fonts/TestShapeAran.ttf", 1000, &[], "لسان"),
        "[OneDotEnclNS@398,-1|NoonxSep|AlefFin@861,0|SeenMed.inT2outT1@1125,0|sp0@1664,0|LamIni.outT2@1664,223]"
    )
}

#[test]
fn sharan_1_1() {
    assert_eq!(
        shape("fonts/TestShapeAran.ttf", 1000, &[], "یونیکوڈ"),
        "[TahSmallNS@118,-213|DalSep|WawFin.cut@300,0|KafMed.outT3@573,206|TwoDotsBelowNS@1115,220|BehxMed.inT2outT1@903,304|OneDotAboveNS@1271,-71|sp1@1170,0|BehxIni.outT2@1170,449|WawFin.inD2@1387,0|TwoDotsBelowNS@1867,1|sp0@1758,0|BehxIni.outD2WQ@1758,323]"
    )
}

#[test]
fn sharan_1_2() {
    assert_eq!(
        shape("fonts/TestShapeAran.ttf", 1000, &[], "فونٹ"),
        "[TahSmallNS@595,-331|BehxFin.soft|OneDotAboveNS@1163,-182|sp0@1184,0|BehxIni.outT2B@1184,300|WawFin.inD2alt@1340,0|OneDotAboveNS@1784,108|sp0@1599,0|FehxIni.outD2WQ@1599,237]"
    )
}

#[test]
fn sharan_1_3() {
    assert_eq!(
        shape("fonts/TestShapeAran.ttf", 1000, &[], "ٹائپ فیس"),
        "[SeenFin|TwoDotsBelowNS@1216,269|BehxMed.inT1outT2SeenWide@1041,455|OneDotAboveNS@1454,224|sp0@1271,0|FehxIni@1271,490|space@1584,0|ThreeDotsDownBelowNS@2290,-159|BehxFin.soft@1715,0|HamzaAboveNS@2878,-201|sp0@2899,0|BehxIni.outT2B@2899,300|AlefFin.narrow@3056,0|TahSmallNS@3442,-420|sp0@3295,0|BehxIni.A@3295,0]"
    )
}

#[test]
fn sharan_1_4() {
    assert_eq!(
        shape("fonts/TestShapeAran.ttf", 1000, &[], "فن خطاطی"),
        "[YehxFin|sp0@521,0|TahIni.outD2@521,380|AlefFin@1119,0|TahMed.inD1outT1@1382,0|OneDotAboveNS@2081,-47|sp0@1451,0|HahIni.outD1@1451,36|space@2326,0|OneDotEnclNS@2855,-2|NoonxFin@2458,0|OneDotAboveNS@3361,188|sp0@3208,0|FehxIni.outT2N@3208,336]"
    )
}

#[test]
fn sharan_1_5() {
    assert_eq!(
        shape("fonts/TestShapeAran.ttf", 1000, &[], "نستعلیق"),
        "[TwoDotsAboveNS@519,-199|QafxFin.cut|TwoDotsBelowNS@977,141|BehxMed.inT2outD2WQ@692,272|LamMed.outT2@1023,434|AinMed.inT3outT1@1301,507|TwoDotsAboveNS@1785,209|BehxMed.inT2outT3@1563,603|SeenMed.inT2outT2@1865,735|OneDotAboveNS@2574,670|sp0@2434,0|BehxIni.outT2tall@2434,952]"
    )
}

#[test]
fn shbali_1_0() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓᬸᬀ"),
        "[gid23|gid60@1113,0|gid4@1064,0]"
    )
}

#[test]
fn shbali_1_1() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬕ᭄ᬖᬂ"),
        "[gid25|gid132@1092,0|gid6@942,0]"
    )
}

#[test]
fn shbali_1_2() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬘᬻ"),
        "[gid28|gid62@796,0|gid57@794,0]"
    )
}

#[test]
fn shbali_1_3() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬙᭀ"),
        "[gid66|gid29@483,0|gid57@1536,0]"
    )
}

#[test]
fn shbali_1_4() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬚᬿ"),
        "[gid67|gid30@483,0]"
    )
}

#[test]
fn shbali_1_5() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬔᬶ"),
        "[gid24|gid58@828,0]"
    )
}

#[test]
fn shbali_1_6() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓ᭄ᬓᬁ"),
        "[gid23|gid129@1111,0|gid5@1064,0]"
    )
}

#[test]
fn shbali_1_7() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓ᭄ᬛᬁ"),
        "[gid23|gid137@1111,0|gid5@1379,181]"
    )
}

#[test]
fn shbali_1_8() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓ᭄ᬦᬃ"),
        "[gid23|gid148@1111,0|gid7@991,0]"
    )
}

#[test]
fn shbali_1_9() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓ᭄ᬓᬸ"),
        "[gid23|gid129@1111,0|gid60@1111,-488]"
    )
}

#[test]
fn shbali_1_10() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓ᭄ᬓᬼ"),
        "[gid23|gid129@1111,0|gid70@1128,0|gid170@1113,0]"
    )
}

#[test]
fn shbali_1_11() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓ᭄ᬓᬽ"),
        "[gid23|gid129@1111,0|gid70@1128,0|gid170@1113,0|gid57@1111,0]"
    )
}

#[test]
fn shbali_1_12() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓᬾ"),
        "[gid66|gid23@483,0]"
    )
}

#[test]
fn shbali_1_13() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓᬶᬾ"),
        "[gid23|gid58@1064,0|gid66@1111,0|gid128@1594,0]"
    )
}

#[test]
fn shbali_1_14() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓᬸᬾ"),
        "[gid23|gid60@1113,0|gid66@1111,0|gid128@1594,0]"
    )
}

#[test]
fn shbali_1_15() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓ᭄ᬕᬾ"),
        "[gid66|gid23@483,0|gid131@1594,0]"
    )
}

#[test]
fn shbali_1_16() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓᭀ"),
        "[gid66|gid23@483,0|gid57@1594,0]"
    )
}

#[test]
fn shbali_1_17() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓᬾ"),
        "[gid66|gid23@483,0]"
    )
}

#[test]
fn shbali_1_18() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓᬾᬶ"),
        "[gid66|gid23@483,0|gid58@1548,0]"
    )
}

#[test]
fn shbali_1_19() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓᬾᬸ"),
        "[gid66|gid23@483,0|gid60@1597,0]"
    )
}

#[test]
fn shbali_1_20() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓ᭄ᬕᬾ"),
        "[gid66|gid23@483,0|gid131@1594,0]"
    )
}

#[test]
fn shbali_1_21() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓᭀ"),
        "[gid66|gid23@483,0|gid57@1594,0]"
    )
}

#[test]
fn shbali_2_0() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓ᭄ᬧᬾ"),
        "[gid66|gid23@483,0|gid149@1594,0]"
    )
}

#[test]
fn shbali_2_1() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓ᭄ᬨᬿ"),
        "[gid67|gid23@483,0|gid150@1594,0]"
    )
}

#[test]
fn shbali_2_2() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓ᭄ᬱᬾ"),
        "[gid66|gid23@483,0|gid159@1594,0]"
    )
}

#[test]
fn shbali_2_3() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓ᭄ᬲᬾ"),
        "[gid66|gid23@483,0|gid60@1597,0|gid149@1594,0]"
    )
}

#[test]
fn shbali_2_4() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓ᭄ᭊᬾ"),
        "[gid66|gid23@483,0|gid60@1597,0|gid165@1594,0]"
    )
}

#[test]
fn shbali_2_5() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬛ᭄ᬓ"),
        "[gid181|gid129@1064,-195]"
    )
}

#[test]
fn shbali_2_6() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬛ᭄ᬓᬾ"),
        "[gid66|gid181@483,0|gid129@1548,-195]"
    )
}

#[test]
fn shbali_2_7() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬛ᭄ᬓᬸᬀ"),
        "[gid181|gid129@1064,-195|gid60@1064,-684|gid4@855,0]"
    )
}

#[test]
fn shbali_2_8() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓ᭄ᬓᬸ"),
        "[gid23|gid129@1111,0|gid60@1111,-488]"
    )
}

#[test]
fn shbali_2_9() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓ᭄ᬛᬹ"),
        "[gid23|gid137@1111,0|gid61@1261,-488]"
    )
}

#[test]
fn shbali_2_10() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓ᭄ᬱᬺ"),
        "[gid23|gid159@1111,0|gid62@1753,0]"
    )
}

#[test]
fn shbali_2_11() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "ᬓ᭄ᭅᬸ"),
        "[gid23|gid162@1111,0|gid60@1111,-488]"
    )
}

#[test]
fn shbali_3_0() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "᭦᭫"),
        "[gid102|gid107@560,-10]"
    )
}

#[test]
fn shbali_3_1() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "᭦᭬"),
        "[gid102|gid108@573,49]"
    )
}

#[test]
fn shbali_3_2() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "᭦᭭"),
        "[gid102|gid109@652,-10]"
    )
}

#[test]
fn shbali_3_3() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "᭦᭮"),
        "[gid102|gid110@652,-98]"
    )
}

#[test]
fn shbali_3_4() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "᭦᭯"),
        "[gid102|gid111@667,-10]"
    )
}

#[test]
fn shbali_3_5() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "᭦᭰"),
        "[gid102|gid112@667,-10]"
    )
}

#[test]
fn shbali_3_6() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "᭦᭱"),
        "[gid102|gid113@667,-10]"
    )
}

#[test]
fn shbali_3_7() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "᭦᭲"),
        "[gid102|gid114@667,-10]"
    )
}

#[test]
fn shbali_3_8() {
    assert_eq!(
        shape("fonts/NotoSansBalinese-Regular.ttf", 1000, &[], "᭦᭳"),
        "[gid102|gid115@599,-10]"
    )
}

#[test]
fn shknda_1_0() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಲ್ಲಿ"),
        "[knLI|knLAc2@757,0]"
    )
}

#[test]
fn shknda_1_1() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಟ್ಸ್"),
        "[knTT|knSAc2@1021,0]"
    )
}

#[test]
fn shknda_1_2() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಳಿ"),
        "[knLLI]"
    )
}

#[test]
fn shknda_1_3() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಡಿ"),
        "[knDDI]"
    )
}

#[test]
fn shknda_1_4() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಮೆ"),
        "[knME]"
    )
}

#[test]
fn shknda_1_5() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ರಿ"),
        "[knRI]"
    )
}

#[test]
fn shknda_1_6() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಖ್ಯೆ"),
        "[knKHE|knYAc2@846,0]"
    )
}

#[test]
fn shknda_1_7() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಫ್ರಿ"),
        "[knPHI|knRAc2@735,0]"
    )
}

#[test]
fn shknda_1_8() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ನೆ"),
        "[knNE]"
    )
}

#[test]
fn shknda_1_9() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಗಿ"),
        "[knGI]"
    )
}

#[test]
fn shknda_1_10() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಷ್ಟಿ"),
        "[knSSI|knTTAc2@746,0]"
    )
}

#[test]
fn shknda_1_11() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಯಿಂ"),
        "[knYI|knAnusvara@1252,0]"
    )
}

#[test]
fn shknda_1_12() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಚೀ"),
        "[knCI|knLengthmark@766,0]"
    )
}

#[test]
fn shknda_1_13() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ನಿ"),
        "[knNI]"
    )
}

#[test]
fn shknda_1_14() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಗ್ಲಿ"),
        "[knGI|knLAc2@621,0]"
    )
}

#[test]
fn shknda_1_15() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಷಿ"),
        "[knSSI]"
    )
}

#[test]
fn shknda_1_16() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಗೆ"),
        "[knGE]"
    )
}

#[test]
fn shknda_1_17() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ದ್ವಿ"),
        "[knDI|knVAc2@740,0]"
    )
}

#[test]
fn shknda_1_18() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ತೀ"),
        "[knTI|knLengthmark@613,0]"
    )
}

#[test]
fn shknda_1_19() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಮಿ"),
        "[knMI]"
    )
}

#[test]
fn shknda_1_20() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಲಿ"),
        "[knLI]"
    )
}

#[test]
fn shknda_1_21() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಗಿ"),
        "[knGI]"
    )
}

#[test]
fn shknda_1_22() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ನ್"),
        "[knN]"
    )
}

#[test]
fn shknda_1_23() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಬಿ"),
        "[knBI]"
    )
}

#[test]
fn shknda_1_24() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಲಿ"),
        "[knLI]"
    )
}

#[test]
fn shknda_1_25() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ನ್ನಿಂ"),
        "[knNI|knNAc2@678,0|knAnusvara@755,0]"
    )
}

#[test]
fn shknda_1_26() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಲ್ಲಿ"),
        "[knLI|knLAc2@757,0]"
    )
}

#[test]
fn shknda_1_27() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಧಿ"),
        "[knDHI]"
    )
}

#[test]
fn shknda_1_28() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಪೌ"),
        "[knPA.base|knmAU@739,0]"
    )
}

#[test]
fn shknda_1_29() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ವಿಂ"),
        "[knVI|knAnusvara@749,0]"
    )
}

#[test]
fn shknda_1_30() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಡಿ"),
        "[knDDI]"
    )
}

#[test]
fn shknda_1_31() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಟಿ"),
        "[knTTI]"
    )
}

#[test]
fn shknda_1_32() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ನಿ"),
        "[knNI]"
    )
}

#[test]
fn shknda_1_33() {
    assert_eq!(
        shape("fonts/NotoSerifKannada-Regular.ttf", 1000, &[], "ಧಿ"),
        "[knDHI]"
    )
}

#[test]
fn shknda_2_0() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ನ್ನಾ"),
        "[gid150|gid57@711,0|gid116@1160,0]"
    )
}

#[test]
fn shknda_2_1() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ನ್ನಾ"),
        "[gid150|gid57@711,0|gid116@1160,0]"
    )
}

#[test]
fn shknda_2_2() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ತ್ತಾ"),
        "[gid146|gid57@623,0|gid112@1071,0]"
    )
}

#[test]
fn shknda_2_3() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಟ್ಟಾ"),
        "[gid141|gid57@815,0|gid107@1264,0]"
    )
}

#[test]
fn shknda_2_4() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಡೋಂಗಿ"),
        "[gid249|gid61@768,0|gid71@1513,0|gid4@1925,0|gid207@2475,0]"
    )
}

#[test]
fn shknda_2_5() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಜಿ಼ೕಬೆನ್"),
        "[gid211|gid55@652,0|gid71@776,0|gid259@1188,0|gid186@1994,0]"
    )
}

#[test]
fn shknda_2_6() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಜಾ಼ಕಿರ್"),
        "[gid139|gid57@776,0|gid55@652,0|gid205@1225,0|gid193@1799,0]"
    )
}

#[test]
fn shknda_2_7() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಇನ್ಫ್ಲೆಕ್ಷನಲ್"),
        "[gid8|gid256@711,0|gid118@1422,0|gid335@1591,0|gid282@1978,0|gid39@2552,0|gid195@3263,0]"
    )
}

#[test]
fn shknda_2_8() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಇನ್ಫ್ಲೆಕ್ಷನ್"),
        "[gid8|gid256@711,0|gid118@1422,0|gid335@1591,0|gid282@1978,0|gid186@2552,0]"
    )
}

#[test]
fn shknda_2_9() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ದಟ್ಸ್"),
        "[gid37|gid177@765,0|gid130@1814,0]"
    )
}

#[test]
fn shknda_2_10() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಎಕ್ಸ್"),
        "[gid14|gid167@787,0|gid130@1596,0]"
    )
}

#[test]
fn shknda_2_11() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಮಾರ್ಚ್"),
        "[gid155|gid57@1156,0|gid172@1605,0|gid94@2718,0]"
    )
}

#[test]
fn shknda_2_12() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಟೆಕ್ಸ್ಟ್"),
        "[gid247|gid167@815,0|gid130@1624,0|gid317@1792,0]"
    )
}

#[test]
fn shknda_2_13() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಬುಕ್ಸ್"),
        "[gid42|gid60@801,0|gid167@1165,0|gid130@1974,0]"
    )
}

#[test]
fn shknda_2_14() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಸಾಫ್ಟ್"),
        "[gid163|gid57@709,0|gid188@1158,0|gid107@2184,0]"
    )
}

#[test]
fn shknda_2_15() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಜಸ್ಟ್"),
        "[gid27|gid200@776,0|gid107@1720,0]"
    )
}

#[test]
fn shknda_3_0() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಕೋಂ"),
        "[gid239|gid61@574,0|gid71@1319,0|gid4@1731,0]"
    )
}

#[test]
fn shknda_3_1() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಖೋಂ"),
        "[gid240|gid61@865,0|gid71@1610,0|gid4@2022,0]"
    )
}

#[test]
fn shknda_3_2() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಗೋಂ"),
        "[gid241|gid61@648,0|gid71@1393,0|gid4@1805,0]"
    )
}

#[test]
fn shknda_3_3() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಘೋಂ"),
        "[gid242|gid279@997,0|gid71@1742,0|gid4@2153,0]"
    )
}

#[test]
fn shknda_3_4() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಙೋಂ"),
        "[gid24|gid67@737,0|gid71@1718,0|gid4@2130,0]"
    )
}

#[test]
fn shknda_3_5() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಚೋಂ"),
        "[gid243|gid61@795,0|gid71@1540,0|gid4@1952,0]"
    )
}

#[test]
fn shknda_3_6() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಛೋಂ"),
        "[gid244|gid61@843,0|gid71@1588,0|gid4@2000,0]"
    )
}

#[test]
fn shknda_3_7() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಜೋಂ"),
        "[gid245|gid61@776,0|gid71@1522,0|gid4@1933,0]"
    )
}

#[test]
fn shknda_3_8() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಝೋಂ"),
        "[gid246|gid61@1379,0|gid71@2124,0|gid4@2536,0]"
    )
}

#[test]
fn shknda_3_9() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಞೋಂ"),
        "[gid29|gid67@968,0|gid71@1949,0|gid4@2360,0]"
    )
}

#[test]
fn shknda_3_10() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಟೋಂ"),
        "[gid247|gid61@815,0|gid71@1560,0|gid4@1972,0]"
    )
}

#[test]
fn shknda_3_11() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಠೋಂ"),
        "[gid248|gid61@651,0|gid71@1397,0|gid4@1808,0]"
    )
}

#[test]
fn shknda_3_12() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಡೋಂ"),
        "[gid249|gid61@768,0|gid71@1513,0|gid4@1925,0]"
    )
}

#[test]
fn shknda_3_13() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಢೋಂ"),
        "[gid250|gid61@768,0|gid71@1513,0|gid4@1925,0]"
    )
}

#[test]
fn shknda_3_14() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಣೋಂ"),
        "[gid251|gid61@867,0|gid71@1612,0|gid4@2023,0]"
    )
}

#[test]
fn shknda_3_15() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ತೋಂ"),
        "[gid252|gid61@623,0|gid71@1368,0|gid4@1779,0]"
    )
}

#[test]
fn shknda_3_16() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಥೋಂ"),
        "[gid253|gid61@765,0|gid71@1510,0|gid4@1921,0]"
    )
}

#[test]
fn shknda_3_17() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ದೋಂ"),
        "[gid254|gid61@765,0|gid71@1510,0|gid4@1921,0]"
    )
}

#[test]
fn shknda_3_18() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಧೋಂ "),
        "[gid255|gid61@765,0|gid71@1510,0|gid4@1921,0|gid3@2472,0]"
    )
}

#[test]
fn shknda_3_19() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ನೋಂ"),
        "[gid256|gid61@711,0|gid71@1456,0|gid4@1868,0]"
    )
}

#[test]
fn shknda_3_20() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಪೋಂ"),
        "[gid257|gid275@792,0|gid71@1434,0|gid4@1846,0]"
    )
}

#[test]
fn shknda_3_21() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಫೋಂ"),
        "[gid258|gid277@792,0|gid71@1434,0|gid4@1846,0]"
    )
}

#[test]
fn shknda_3_22() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಬೋಂ"),
        "[gid259|gid61@806,0|gid71@1551,0|gid4@1963,0]"
    )
}

#[test]
fn shknda_3_23() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಭೋಂ"),
        "[gid260|gid61@806,0|gid71@1551,0|gid4@1963,0]"
    )
}

#[test]
fn shknda_3_24() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಮೋಂ"),
        "[gid280|gid71@1539,0|gid4@1951,0]"
    )
}

#[test]
fn shknda_3_25() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಯೋಂ"),
        "[gid281|gid71@1712,0|gid4@2124,0]"
    )
}

#[test]
fn shknda_3_26() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ರೋಂ"),
        "[gid263|gid61@651,0|gid71@1397,0|gid4@1808,0]"
    )
}

#[test]
fn shknda_3_27() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಱೋಂ"),
        "[gid47|gid67@831,0|gid71@1812,0|gid4@2223,0]"
    )
}

#[test]
fn shknda_3_28() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಲೋಂ"),
        "[gid264|gid61@769,0|gid71@1514,0|gid4@1925,0]"
    )
}

#[test]
fn shknda_3_29() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ವೋಂ"),
        "[gid266|gid275@794,0|gid71@1437,0|gid4@1848,0]"
    )
}

#[test]
fn shknda_3_30() {
    assert_eq!(
        shape("fonts/NotoSansKannada-Regular.ttf", 1000, &[], "ಆ್ಯಕ್ಷಿಸ್‌"),
        "[gid7|gid122@838,0|gid285@1098,0|gid200@1672,0|gid3@2694,0]"
    )
}
