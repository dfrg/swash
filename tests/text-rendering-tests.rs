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
fn cmap_1_0() {
    assert_eq!(
        shape("fonts/TestCMAP14.otf", 1000, &[], "芦"),
        "[uni82A6_uE0100]"
    )
}

#[test]
fn cmap_1_1() {
    assert_eq!(
        shape("fonts/TestCMAP14.otf", 1000, &[], "芦󠄀"),
        "[uni82A6_uE0100]"
    )
}

#[test]
fn cmap_1_2() {
    assert_eq!(
        shape("fonts/TestCMAP14.otf", 1000, &[], "芦󠄁"),
        "[uni82A6_uE0101]"
    )
}

#[test]
fn cmap_1_3() {
    assert_eq!(
        shape("fonts/TestCMAP14.otf", 1000, &[], "芦󠄂"),
        "[uni82A6_uE0100]"
    )
}

#[test]
fn cmap_2_0() {
    assert_eq!(shape("fonts/TestCMAP14.otf", 1000, &[], "≩"), "[uni2269]")
}

#[test]
fn cmap_2_1() {
    assert_eq!(
        shape("fonts/TestCMAP14.otf", 1000, &[], "≩︀"),
        "[uni2269FE00]"
    )
}

#[test]
fn cmap_3_0() {
    assert_eq!(
        shape("fonts/TestCMAPMacTurkish.ttf", 1000, &[], "“"),
        "[gid200]"
    )
}

#[test]
fn cmap_3_1() {
    assert_eq!(
        shape("fonts/TestCMAPMacTurkish.ttf", 1000, &[], "A"),
        "[gid34]"
    )
}

#[test]
fn cmap_3_2() {
    assert_eq!(
        shape("fonts/TestCMAPMacTurkish.ttf", 1000, &[], "B"),
        "[gid35]"
    )
}

#[test]
fn cmap_3_3() {
    assert_eq!(
        shape("fonts/TestCMAPMacTurkish.ttf", 1000, &[], "Ç"),
        "[gid126]"
    )
}

#[test]
fn cmap_3_4() {
    assert_eq!(
        shape("fonts/TestCMAPMacTurkish.ttf", 1000, &[], "Ğ"),
        "[gid176]"
    )
}

#[test]
fn cmap_3_5() {
    assert_eq!(
        shape("fonts/TestCMAPMacTurkish.ttf", 1000, &[], "I"),
        "[gid42]"
    )
}

#[test]
fn cmap_3_6() {
    assert_eq!(
        shape("fonts/TestCMAPMacTurkish.ttf", 1000, &[], "İ"),
        "[gid178]"
    )
}

#[test]
fn cmap_3_7() {
    assert_eq!(
        shape("fonts/TestCMAPMacTurkish.ttf", 1000, &[], "Ö"),
        "[gid140]"
    )
}

#[test]
fn cmap_3_8() {
    assert_eq!(
        shape("fonts/TestCMAPMacTurkish.ttf", 1000, &[], "Ş"),
        "[gid181]"
    )
}

#[test]
fn cmap_3_9() {
    assert_eq!(
        shape("fonts/TestCMAPMacTurkish.ttf", 1000, &[], "Ü"),
        "[gid145]"
    )
}

#[test]
fn cmap_3_10() {
    assert_eq!(
        shape("fonts/TestCMAPMacTurkish.ttf", 1000, &[], "”"),
        "[gid201]"
    )
}

#[test]
fn cmap_3_11() {
    assert_eq!(
        shape("fonts/TestCMAPMacTurkish.ttf", 1000, &[], "a"),
        "[gid66]"
    )
}

#[test]
fn cmap_3_12() {
    assert_eq!(
        shape("fonts/TestCMAPMacTurkish.ttf", 1000, &[], "b"),
        "[gid67]"
    )
}

#[test]
fn cmap_3_13() {
    assert_eq!(
        shape("fonts/TestCMAPMacTurkish.ttf", 1000, &[], "ç"),
        "[gid154]"
    )
}

#[test]
fn cmap_3_14() {
    assert_eq!(
        shape("fonts/TestCMAPMacTurkish.ttf", 1000, &[], "ğ"),
        "[gid177]"
    )
}

#[test]
fn cmap_3_15() {
    assert_eq!(
        shape("fonts/TestCMAPMacTurkish.ttf", 1000, &[], "ı"),
        "[gid222]"
    )
}

#[test]
fn cmap_3_16() {
    assert_eq!(
        shape("fonts/TestCMAPMacTurkish.ttf", 1000, &[], "i"),
        "[gid74]"
    )
}

#[test]
fn cmap_3_17() {
    assert_eq!(
        shape("fonts/TestCMAPMacTurkish.ttf", 1000, &[], "ö"),
        "[gid168]"
    )
}

#[test]
fn cmap_3_18() {
    assert_eq!(
        shape("fonts/TestCMAPMacTurkish.ttf", 1000, &[], "ş"),
        "[gid182]"
    )
}

#[test]
fn cmap_3_19() {
    assert_eq!(
        shape("fonts/TestCMAPMacTurkish.ttf", 1000, &[], "ü"),
        "[gid174]"
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

// #[test]
// fn morx_24_0() {
//     assert_eq!(
//         shape("fonts/TestMORXTwentyfour.ttf", 0, &[], "ABCDE"),
//         "*"
//     )
// }

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

#[test]
fn shlana_1_0() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩫ"),
        "[uni1A20|uni1A6B@762,0]"
    )
}

#[test]
fn shlana_1_1() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨣᩴ"),
        "[uni1A23|uni1A74@592,0]"
    )
}

#[test]
fn shlana_1_2() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨧᩢ"),
        "[uni1A27|uni1A62@592,0]"
    )
}

#[test]
fn shlana_1_3() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨲ᩠ᩅᩫᩡ"),
        "[uni1A32|uni1A601A45@762,0|uni1A6B@762,0|uni1A61@933,0]"
    )
}

#[test]
fn shlana_1_4() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨷ᩠ᩅᩫ"),
        "[uni1A37|uni1A601A45@592,0|uni1A6B@592,0]"
    )
}

#[test]
fn shlana_1_5() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠ᩠ᩅ"),
        "[uni1A20|uni1A601A45@762,0]"
    )
}

#[test]
fn shlana_1_6() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨡᩬᩴ"),
        "[uni1A21|uni1A6C@592,-98|uni1A74@592,0]"
    )
}

#[test]
fn shlana_1_7() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩬ"),
        "[uni1A20|uni1A6C.wide@933,0]"
    )
}

#[test]
fn shlana_1_8() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨦᩡ"),
        "[uni1A26|uni1A61@592,0]"
    )
}

#[test]
fn shlana_1_9() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩣ"),
        "[uni1A20|uni1A63@933,0]"
    )
}

#[test]
fn shlana_1_10() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨴᩤ"),
        "[uni1A34|uni1A64@592,0]"
    )
}

#[test]
fn shlana_1_11() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩌᩣᩴ"),
        "[uni1A4C|uni1A74@762,0|uni1A63@933,0]"
    )
}

#[test]
fn shlana_1_12() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨣᩤᩴ"),
        "[uni1A23|uni1A74@592,0|uni1A64@592,0]"
    )
}

#[test]
fn shlana_1_13() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨳᩥ"),
        "[uni1A33|uni1A65@592,0]"
    )
}

#[test]
fn shlana_1_14() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨺᩦ "),
        "[uni1A3A|uni1A66@738,0|space@592,0]"
    )
}

#[test]
fn shlana_1_15() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨩᩧ"),
        "[uni1A29|uni1A67@592,0]"
    )
}

#[test]
fn shlana_1_16() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨾᩨ"),
        "[uni1A3E.v2|uni1A68@592,0]"
    )
}

#[test]
fn shlana_1_17() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨵᩩ"),
        "[uni1A35|uni1A69@592,0]"
    )
}

#[test]
fn shlana_1_18() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨦᩪ"),
        "[uni1A26|uni1A6A@592,0]"
    )
}

#[test]
fn shlana_1_19() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨲᩮᩡ"),
        "[uni1A6E|uni1A32@592,0|uni1A61@1524,0]"
    )
}

#[test]
fn shlana_1_20() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨽᩮ"),
        "[uni1A6E|uni1A3D@592,0]"
    )
}

#[test]
fn shlana_1_21() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨤᩯᩡ"),
        "[uni1A6F|uni1A24@1096,0|uni1A61@1688,0]"
    )
}

#[test]
fn shlana_1_22() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨧᩯ"),
        "[uni1A6F|uni1A27@1096,0]"
    )
}

#[test]
fn shlana_1_23() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨸᩮᩬᩥᩡ"),
        "[uni1A6E|uni1A38@592,0|uni1A6C@1184,0|uni1A65@1098,0|uni1A61@1184,0]"
    )
}

#[test]
fn shlana_1_24() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨶᩮᩬᩥ"),
        "[uni1A6E|uni1A36@592,0|uni1A6C@1184,0|uni1A65@1184,0]"
    )
}

#[test]
fn shlana_1_25() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩮᩬᩨᩡ"),
        "[uni1A6E|uni1A20@592,0|uni1A6C.wide@1524,0|uni1A68@1354,0|uni1A61@1524,0]"
    )
}

#[test]
fn shlana_1_26() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩮᩬᩨ"),
        "[uni1A6E|uni1A20@592,0|uni1A6C.wide@1524,0|uni1A68@1354,0]"
    )
}

#[test]
fn shlana_1_27() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩁᩮᩢᩣ"),
        "[uni1A6E|uni1A41@592,0|uni1A62@1184,0|uni1A63@1184,0]"
    )
}

#[test]
fn shlana_1_28() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨾᩳ"),
        "[uni1A3E.v2|uni1A73@592,0]"
    )
}

#[test]
fn shlana_1_29() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩮᩣ"),
        "[uni1A6E|uni1A20@592,0|uni1A63@1524,0]"
    )
}

#[test]
fn shlana_1_30() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨹ᩠ᨿᩮᩡ"),
        "[uni1A6E|uni1A39@592,0|uni1A601A3F@1324,0|uni1A61@1551,0]"
    )
}

#[test]
fn shlana_1_31() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨻ᩠ᨿᩮ"),
        "[uni1A6E|uni1A3B@592,0|uni1A601A3F@1524,0]"
    )
}

#[test]
fn shlana_1_32() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠ᩠ᨿ"),
        "[uni1A20|uni1A601A3F@933,0]"
    )
}

#[test]
fn shlana_1_33() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨾᩮᩬᩥᩋᩡ"),
        "[uni1A6E|uni1A3E.v2@592,0|uni1A6C@1184,0|uni1A65@1184,0|uni1A4B@1324,0|uni1A61@1916,0]"
    )
}

#[test]
fn shlana_1_34() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩖᩮᩬᩥᩋ"),
        "[uni1A6E|uni1A20@592,0|uni1A56.wide@1524,0|uni1A6C.wide@1524,-547|uni1A65@1524,0|uni1A4B@1524,0]"
    )
}

#[test]
fn shlana_1_35() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈᩰᩡ"),
        "[uni1A70|uni1A48@592,0|uni1A61@1184,0]"
    )
}

#[test]
fn shlana_1_36() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨾᩰ"),
        "[uni1A70|uni1A3E.v2@592,0]"
    )
}

#[test]
fn shlana_1_37() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨪᩰᩬᩡ"),
        "[uni1A70|uni1A2A@592,0|uni1A6C@1184,0|uni1A61@1184,0]"
    )
}

#[test]
fn shlana_1_38() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨩᩢ᩠ᨿ"),
        "[uni1A29|uni1A62@592,0|uni1A601A3F@592,0]"
    )
}

#[test]
fn shlana_1_39() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨶᩲ"),
        "[uni1A72|uni1A36@592,0]"
    )
}

#[test]
fn shlana_1_40() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨢᩱ"),
        "[uni1A71|uni1A22@592,0]"
    )
}

#[test]
fn shlana_1_41() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨴᩱ᩠ᨿ"),
        "[uni1A71|uni1A34@592,0|uni1A601A3F@1184,0]"
    )
}

#[test]
fn shlana_1_42() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩮᩬᩨᩡ"),
        "[uni1A6E|uni1A20@592,0|uni1A6C.wide@1524,0|uni1A68@1354,0|uni1A61@1524,0]"
    )
}

#[test]
fn shlana_1_43() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩮᩬᩨ"),
        "[uni1A6E|uni1A20@592,0|uni1A6C.wide@1524,0|uni1A68@1354,0]"
    )
}

#[test]
fn shlana_1_44() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩰᩢ"),
        "[uni1A70|uni1A20@592,0|uni1A62@1354,0]"
    )
}

#[test]
fn shlana_1_45() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈᩘ"),
        "[uni1A48|uni1A58@592,0]"
    )
}

#[test]
fn shlana_1_46() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨴᩢ᩠ᨦ"),
        "[uni1A34|uni1A62@592,0|uni1A601A26@592,0]"
    )
}

#[test]
fn shlana_1_47() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩌᩥᩴ "),
        "[uni1A4C|uni1A65@762,0|uni1A74@1211,0|space@933,0]"
    )
}

#[test]
fn shlana_1_48() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩥ᩠ᨦ"),
        "[uni1A20|uni1A65@762,0|uni1A601A26@762,0]"
    )
}

#[test]
fn shlana_1_49() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩢ᩠ᨾ"),
        "[uni1A20|uni1A62@762,0|uni1A601A3E@762,0]"
    )
}

#[test]
fn shlana_1_50() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩢᨾ"),
        "[uni1A20|uni1A62@762,0|uni1A3E.v2@933,0]"
    )
}

#[test]
fn shlana_1_51() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨯᩭ"),
        "[uni1A2F|uni1A6D@592,0]"
    )
}

#[test]
fn shlana_10_0() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩓᩯ"),
        "[uni1A6F|uni1A53@1096,0]"
    )
}

#[test]
fn shlana_10_1() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩐᩣ"),
        "[uni1A50|uni1A63@592,0]"
    )
}

#[test]
fn shlana_10_2() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩐ᩵ᩣ"),
        "[uni1A50|uni1A75@738,0|uni1A63@592,0]"
    )
}

#[test]
fn shlana_10_3() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨯᩪᩕᩣ"),
        "[uni1A55|uni1A2F@227,0|uni1A6A@818,0|uni1A63@818,0]"
    )
}

#[test]
fn shlana_10_4() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨯᩮᩬᩥᩁᨹᩫᩖᨣᩩᨱ᩺"),
        "[uni1A6E|uni1A2F@592,0|uni1A6C@1184,-188|uni1A65@1184,0|uni1A41@1184,0|uni1A39@1775,0|uni1A6B@2367,0|uni1A56@2367,0|uni1A23@2508,0|uni1A69@3100,0|uni1A31@3100,0|uni1A7A@3861,0]"
    )
}

#[test]
fn shlana_10_5() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨻᩣᨷᩰᩖ"),
        "[uni1A3B|uni1A63@933,0|uni1A70@1524,0|uni1A37@2116,0|uni1A56@2708,0]"
    )
}

#[test]
fn shlana_10_6() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨶ᩶ᩭ"),
        "[uni1A36|uni1A76@592,0|uni1A6D@592,0]"
    )
}

#[test]
fn shlana_10_7() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨹᩥᩖᩩ᩵᩻"),
        "[uni1A39|uni1A65@592,0|uni1A56@592,0|uni1A69@592,-547|uni1A75@592,357|uni1A78@864,357]"
    )
}

#[test]
fn shlana_10_8() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩉᩖ᩠ᩅᨦ"),
        "[uni1A49|uni1A56.wide@933,0|uni1A601A45@762,-547|uni1A26@933,0]"
    )
}

#[test]
fn shlana_10_9() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩉᩖ᩠ᩅᩣ"),
        "[uni1A49|uni1A56.wide@933,0|uni1A601A45@762,-547|uni1A63@933,0]"
    )
}

#[test]
fn shlana_10_10() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨻᩕ᩠ᨿᩮᩡ"),
        "[uni1A6E|uni1A55@592,0|uni1A3B@818,0|uni1A601A3F@1751,0|uni1A61@1978,0]"
    )
}

#[test]
fn shlana_10_11() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩩ᩶ᩣ᩠ᨶ᩠ᨦ"),
        "[uni1A20|uni1A69@762,0|uni1A76@762,0|uni1A63@933,0|uni1A601A36@1524,0|uni1A601A26@1524,-367]"
    )
}

#[test]
fn shlana_10_12() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩋᩢ᩠ᨭᩛ"),
        "[uni1A4B|uni1A62@592,0|uni1A601A2D1A5B@592,0]"
    )
}

#[test]
fn shlana_10_13() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩆᩢᨠ᩠ᨯᩥ᩺"),
        "[uni1A46|uni1A62@592,0|uni1A20@681,0|uni1A601A2F@1442,0|uni1A65@1442,0|uni1A7A@1442,357]"
    )
}

#[test]
fn shlana_10_14() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩆᩢᨠ᩠ᨯᩥ᩼"),
        "[uni1A46|uni1A62@592,0|uni1A20@681,0|uni1A601A2F@1442,0|uni1A65@1442,0|uni1A7C@1442,357]"
    )
}

#[test]
fn shlana_10_15() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨾᩉᩣᩉᩥᨦ᩠ᨣᩩ᩺"),
        "[uni1A3E.v2|uni1A49@732,0|uni1A63@1665,0|uni1A49@2257,0|uni1A65@3019,0|uni1A26@3189,0|uni1A601A23@3781,0|uni1A69@3781,-547|uni1A7A@3781,0]"
    )
}

#[test]
fn shlana_10_16() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨾᩉᩣᩉᩥᨦ᩠ᨣᩩ᩼"),
        "[uni1A3E.v2|uni1A49@732,0|uni1A63@1665,0|uni1A49@2257,0|uni1A65@3019,0|uni1A26@3189,0|uni1A601A23@3781,0|uni1A69@3781,-547|uni1A7C@3781,0]"
    )
}

#[test]
fn shlana_10_17() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩆᩣᩈ᩠ᨲᩕ᩺"),
        "[uni1A46|uni1A63@681,0|uni1A55@1272,0|uni1A48@1499,0|uni1A601A32@2091,0|uni1A7A@2091,0]"
    )
}

#[test]
fn shlana_10_18() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈᩣᩈ᩠ᨲᩕ᩼"),
        "[uni1A48|uni1A63@592,0|uni1A55@1184,0|uni1A48@1410,0|uni1A601A32@2002,0|uni1A7C@2002,0]"
    )
}

#[test]
fn shlana_10_19() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩁᩪ᩠ᨷ"),
        "[uni1A41|uni1A601A37@592,0|uni1A6A@725,-367]"
    )
}

#[test]
fn shlana_10_20() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨻᩦ᩠᩵ᨶᩬ᩶ᨦ"),
        "[uni1A3B|uni1A66@762,0|uni1A75@762,357|uni1A601A36@762,0|uni1A6C@762,-367|uni1A76@1087,357|uni1A26@933,0]"
    )
}

#[test]
fn shlana_10_21() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨸᩢᩣ"),
        "[uni1A38|uni1A62@506,0|uni1A63@592,0]"
    )
}

#[test]
fn shlana_10_22() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩃᩪᩢ"),
        "[uni1A43|uni1A6A@762,0|uni1A62@762,0]"
    )
}

#[test]
fn shlana_10_23() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨯᩬᩢ"),
        "[uni1A2F|uni1A6C@592,-188|uni1A62@592,0]"
    )
}

#[test]
fn shlana_10_24() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨯᩬᩢᩡ"),
        "[uni1A2F|uni1A6C@592,-188|uni1A62@592,0|uni1A61@592,0]"
    )
}

#[test]
fn shlana_10_25() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨻ᩠ᩅᩢᩡ"),
        "[uni1A3B|uni1A601A45@762,0|uni1A62@762,0|uni1A61@933,0]"
    )
}

#[test]
fn shlana_10_26() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨲᩯ᩠ᨶᩬᩴ᩵"),
        "[uni1A6F|uni1A32@1096,0|uni1A601A36@1857,0|uni1A6C@1857,-367|uni1A74@1857,0|uni1A75@1857,357]"
    )
}

#[test]
fn shlana_10_27() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨲᩬᩴ᩵͏ᩯ᩠ᨶ"),
        "[uni1A6F|uni1A32@1096,0|uni1A6C.wide@2028,0|uni1A74@2028,0|uni1A75@2028,357|uni1A601A36@1857,-367]"
    )
}

#[test]
fn shlana_10_28() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨲᩬᩴ᩵ᩯ᩠ᨶ"),
        "[uni1A6F|uni1A32@1096,0|uni1A6C.wide@2028,0|uni1A74@2028,0|uni1A75@2028,357|uni1A601A36@1857,-367]"
    )
}

#[test]
fn shlana_10_29() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈ᩠ᨶᩫ᩻"),
        "[uni1A48|uni1A601A36@592,0|uni1A6B@592,0|uni1A78@592,357]"
    )
}

#[test]
fn shlana_10_30() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᨾᩛᩦ"),
        "[uni1A20|uni1A3E1A5B@933,0|uni1A66@1549,0]"
    )
}

#[test]
fn shlana_10_31() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨶᩥᨻᩛᩣ᩠ᨶ"),
        "[uni1A36|uni1A65@592,0|uni1A3B1A5B@592,0|uni1A63@1524,0|uni1A601A36@2116,0]"
    )
}

#[test]
fn shlana_10_32() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨵᨾᩜᩥᨠ"),
        "[uni1A35|uni1A3E.v2@592,0|uni1A5C@1184,0|uni1A65@1184,0|uni1A20@1324,0]"
    )
}

#[test]
fn shlana_10_33() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈᨵᩩ᩠ᨷ"),
        "[uni1A48|uni1A35@592,0|uni1A601A37@1184,0|uni1A69@1317,-367]"
    )
}

#[test]
fn shlana_10_34() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩅᩥᩈᩮ᩠ᩈ"),
        "[uni1A45|uni1A65@592,0|uni1A6E@592,0|uni1A48@1184,0|uni1A601A48@1775,0]"
    )
}

#[test]
fn shlana_10_35() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨢ᩶ᩣ"),
        "[uni1A22|uni1A76@738,0|uni1A63@592,0]"
    )
}

#[test]
fn shlana_10_36() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈᩣᩈᨶᩣ"),
        "[uni1A48|uni1A63@592,0|uni1A48@1184,0|uni1A361A63@1775,0]"
    )
}

#[test]
fn shlana_10_37() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈᩣᩈ᩠ᨶ᩻ᩣ"),
        "[uni1A48|uni1A63@592,0|uni1A48@1184,0|uni1A601A36@1775,0|uni1A78@1775,0|uni1A63@1775,0]"
    )
}

#[test]
fn shlana_10_38() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈ᩠ᨶ᩻ᩮᩢ᩶ᩣ"),
        "[uni1A6E|uni1A48@592,0|uni1A601A36@1184,0|uni1A78@1184,0|uni1A621A76@1599,0|uni1A63@1184,0]"
    )
}

#[test]
fn shlana_10_39() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈ᩠ᨶ᩻ᩮᩢᩢ᩶ᩣ"),
        "[uni1A6E|uni1A48@592,0|uni1A601A36@1184,0|uni1A78@1184,0|uni1A621A621A76@1599,0|uni1A63@1184,0]"
    )
}

#[test]
fn shlana_10_40() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨲᩦ͏ᩣ᩠ᨿ"),
        "[uni1A32|uni1A66@762,0|uni1A63@933,0|uni1A601A3F@1524,0]"
    )
}

#[test]
fn shlana_10_41() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩉ᩠ᨾ᩵ᩣᩴ᩻"),
        "[uni1A49|uni1A601A3E@762,0|uni1A74@762,0|uni1A75@762,357|uni1A63@933,0|uni1A78@1524,0]"
    )
}

#[test]
fn shlana_10_42() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩃᩮᩞ"),
        "[uni1A6E|uni1A43@592,0|uni1A5E@1354,0]"
    )
}

#[test]
fn shlana_10_43() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩋᨶᩣᨳᨷᩥᨱ᩠ᨯᩥᨠᩈᩞ"),
        "[uni1A4B|uni1A361A63@592,0|uni1A33@1184,0|uni1A37@1775,0|uni1A65@2367,0|uni1A31@2367,0|uni1A601A2F@3129,0|uni1A65@3129,0|uni1A20@3300,0|uni1A48@4232,0|uni1A5E@4824,0]"
    )
}

#[test]
fn shlana_10_44() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨯᩣᩴ᩻"),
        "[uni1A2F|uni1A74@592,0|uni1A63@592,0|uni1A78@1184,0]"
    )
}

#[test]
fn shlana_10_45() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩉ᩠ᨾᩴ᩵ᩣ᩻"),
        "[uni1A49|uni1A601A3E@762,0|uni1A74@762,0|uni1A75@762,357|uni1A63@933,0|uni1A78@1524,0]"
    )
}

#[test]
fn shlana_10_46() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨯᩴᩣ᩻"),
        "[uni1A2F|uni1A74@592,0|uni1A63@592,0|uni1A78@1184,0]"
    )
}

#[test]
fn shlana_2_0() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "᪓᩠ᨴ"),
        "[uni1A93|uni1A601A34@592,0]"
    )
}

#[test]
fn shlana_2_1() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨲ᩵ᩣ᩠ᨦ᩻"),
        "[uni1A32|uni1A75@762,0|uni1A63@933,0|uni1A601A26@1524,0|uni1A78@1524,0]"
    )
}

#[test]
fn shlana_2_2() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨳ᩠ᨶ᩻ᩫᩁ"),
        "[uni1A33|uni1A601A36@592,0|uni1A78@592,0|uni1A6B@954,0|uni1A41@592,0]"
    )
}

#[test]
fn shlana_2_3() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨡᩢ᩶᩻ᩬᨦ"),
        "[uni1A21|uni1A621A76@592,0|uni1A78@1007,0|uni1A6C@592,-98|uni1A26@592,0]"
    )
}

#[test]
fn shlana_2_4() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "᪭ᩣ"),
        "[uni1AAD|uni1A63@592,0]"
    )
}

#[test]
fn shlana_2_5() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩉ᩠ᨶᩦ"),
        "[uni1A49|uni1A601A36@762,0|uni1A66@762,0]"
    )
}

#[test]
fn shlana_2_6() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨤ᩠ᩅᩯ᩶ᩁ"),
        "[uni1A6F|uni1A24@1096,0|uni1A601A45@1688,0|uni1A76@1688,0|uni1A41@1688,0]"
    )
}

#[test]
fn shlana_2_7() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩉ᩠ᩅᩫ"),
        "[uni1A49|uni1A601A45@762,0|uni1A6B@762,0]"
    )
}

#[test]
fn shlana_2_8() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨯᩢ᩵ᨦ᩠ᨶᩦ᩶"),
        "[uni1A2F|uni1A621A75@592,0|uni1A26@592,0|uni1A601A36@1184,0|uni1A66@1184,0|uni1A76@1184,357]"
    )
}

#[test]
fn shlana_2_9() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩉᩥ᩠ᨶ"),
        "[uni1A49|uni1A65@762,0|uni1A601A36@762,0]"
    )
}

#[test]
fn shlana_2_10() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨲᩣ᩠ᨾ"),
        "[uni1A32|uni1A63@933,0|uni1A601A3E@1524,0]"
    )
}

#[test]
fn shlana_2_11() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨻ᩠ᨿᩣ᩠ᨵᩥ"),
        "[uni1A3B|uni1A601A3F@933,0|uni1A63@1159,0|uni1A601A35@1751,0|uni1A65@1751,0]"
    )
}

#[test]
fn shlana_2_12() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨸ᩠ᩃ᩠ᨿ᩵ᩁ"),
        "[uni1A38|uni1A601A43@592,0|uni1A601A3F@818,0|uni1A75@1049,0|uni1A41@1045,0]"
    )
}

#[test]
fn shlana_2_13() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨾᩯ᩠᩶ᨶ᩠ᩅ᩵ᩣ"),
        "[uni1A6F|uni1A3E.v2@1096,0|uni1A76@1688,0|uni1A601A36@1688,0|uni1A601A45@1688,-367|uni1A75@2013,0|uni1A63@1828,0]"
    )
}

#[test]
fn shlana_2_14() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈ᩠ᩅᩯ᩵"),
        "[uni1A6F|uni1A48@1096,0|uni1A601A45@1688,0|uni1A75@1688,0]"
    )
}

#[test]
fn shlana_2_15() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈ᩵ᩯ᩠ᩅ"),
        "[uni1A6F|uni1A48@1096,0|uni1A75@1688,0|uni1A601A45@1688,0]"
    )
}

#[test]
fn shlana_2_16() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨿᩪ"),
        "[uni1A3F|uni1A6A@762,0]"
    )
}

#[test]
fn shlana_2_17() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨾᩦ"),
        "[uni1A3E.v2|uni1A66@592,0]"
    )
}

#[test]
fn shlana_2_18() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩉ᩠ᨾᩪ"),
        "[uni1A49|uni1A601A3E@762,0|uni1A6A@762,-367]"
    )
}

#[test]
fn shlana_2_19() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩉ᩠ᨾᩦ"),
        "[uni1A49|uni1A601A3E@762,0|uni1A66@762,0]"
    )
}

#[test]
fn shlana_2_20() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨹ᩠ᩅᩫ"),
        "[uni1A39|uni1A601A45@592,0|uni1A6B@592,0]"
    )
}

#[test]
fn shlana_2_21() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩉ᩠ᩃᩬᩴ᩵"),
        "[uni1A49|uni1A601A43@933,0|uni1A6C@1060,-547|uni1A74@1164,0|uni1A75@1164,357]"
    )
}

#[test]
fn shlana_2_22() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨾᩣ"),
        "[uni1A3E.v2|uni1A63@732,0]"
    )
}

#[test]
fn shlana_2_23() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩉᩱ᩵"),
        "[uni1A71|uni1A49@592,0|uni1A75@1354,0]"
    )
}

#[test]
fn shlana_2_24() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩅ᩠ᨿᨦ"),
        "[uni1A45|uni1A601A3F@592,0|uni1A26@818,0]"
    )
}

#[test]
fn shlana_2_25() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩉᩣ᩠ᨾ"),
        "[uni1A49|uni1A63@933,0|uni1A601A3E@1524,0]"
    )
}

#[test]
fn shlana_2_26() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨯᩣᩴ"),
        "[uni1A2F|uni1A74@592,0|uni1A63@592,0]"
    )
}

#[test]
fn shlana_2_27() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩉ᩠ᨾᩣ"),
        "[uni1A49|uni1A601A3E@762,0|uni1A63@933,0]"
    )
}

#[test]
fn shlana_2_28() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩕᩣ᩠ᨸ"),
        "[uni1A55|uni1A20@227,0|uni1A63@1159,0|uni1A601A37@1751,0]"
    )
}

#[test]
fn shlana_2_29() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨻᩕ᩵ᩣᩴ"),
        "[uni1A55|uni1A3B@227,0|uni1A74@988,0|uni1A75@988,357|uni1A63@1159,0]"
    )
}

#[test]
fn shlana_2_30() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩕᩬᨦ"),
        "[uni1A55|uni1A20@227,0|uni1A6C.wide@1159,0|uni1A26@1159,0]"
    )
}

#[test]
fn shlana_2_31() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈᩕᩫᨾ᩠ᨱ᩺"),
        "[uni1A55|uni1A48@227,0|uni1A6B@818,0|uni1A3E.v2@818,0|uni1A601A31.narrow@1410,0|uni1A7A@1410,0]"
    )
}

#[test]
fn shlana_2_32() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈᩕ᩠ᩅᩫᨾ"),
        "[uni1A55|uni1A48@227,0|uni1A601A45@818,0|uni1A6B@818,0|uni1A3E.v2@818,0]"
    )
}

#[test]
fn shlana_2_33() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩉ᩠ᨾ᩵ᩣᩴ"),
        "[uni1A49|uni1A601A3E@762,0|uni1A74@762,0|uni1A75@762,357|uni1A63@933,0]"
    )
}

#[test]
fn shlana_2_34() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩉ᩠ᨾᩮᩬᩨᨦ"),
        "[uni1A6E|uni1A49@592,0|uni1A601A3E@1524,0|uni1A6C@1524,-367|uni1A68@1524,0|uni1A26@1524,0]"
    )
}

#[test]
fn shlana_2_35() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩉ᩠ᨿᩮᩬᩨᨦ"),
        "[uni1A6E|uni1A49@592,0|uni1A601A3F@1524,0|uni1A6C@1729,-367|uni1A68@1755,0|uni1A26@1751,0]"
    )
}

#[test]
fn shlana_2_36() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩉ᩠ᩃᩣ᩠ᨿ"),
        "[uni1A49|uni1A601A43@933,0|uni1A63@1159,0|uni1A601A3F@1751,0]"
    )
}

#[test]
fn shlana_3_0() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩬᩢᩃ᩠ᨼ᩺"),
        "[uni1A20|uni1A6C.wide@933,0|uni1A62@762,0|uni1A43@933,0|antler@933,0|uni1A601A3C.wide@1865,0|uni1A7A@1964,0]"
    )
}

#[test]
fn shlana_3_1() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩬᩢᩃ᩠᩺ᨼ"),
        "[uni1A20|uni1A6C.wide@933,0|uni1A62@762,0|uni1A43@933,0|antler@933,0|uni1A7A@1964,0|uni1A601A3C.wide@1865,0]"
    )
}

#[test]
fn shlana_3_2() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩬᩢᩃ᩠᩺ᨼ"),
        "[uni1A20|uni1A6C.wide@933,0|uni1A62@762,0|uni1A43@933,0|antler@933,0|uni1A7A@1964,0|uni1A601A3C.wide@1865,0]"
    )
}

#[test]
fn shlana_3_3() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩕᩣ᩠ᨼ"),
        "[uni1A55|uni1A20@227,0|uni1A63@1159,0|antler@1159,0|uni1A601A3C@1751,0]"
    )
}

#[test]
fn shlana_3_4() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨴᩬᨼ᩠ᨼᩦ᩵"),
        "[uni1A34|uni1A6C@592,0|uni1A3C@592,0|antler@771,0|uni1A601A3C.wide@1524,0|uni1A66@1558,0|uni1A75@1558,357]"
    )
}

#[test]
fn shlana_3_5() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᨽᩚ"),
        "[uni1A20|uni1A3D@933,0|uni1A5A@1865,0]"
    )
}

#[test]
fn shlana_3_6() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈᨱᩛᩣ᩠ᨶ"),
        "[uni1A48|uni1A311A5B@592,0|uni1A63@1524,0|uni1A601A36@2116,0]"
    )
}

#[test]
fn shlana_3_7() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩁᨭᩛᨷᩣ᩠ᩃ"),
        "[uni1A41|uni1A2D1A5B@592,0|uni1A37@1184,0|uni1A63@1775,0|uni1A601A43@2367,0]"
    )
}

#[test]
fn shlana_3_8() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩁᩢᨭᩛᨷᩣ᩠ᩃ"),
        "[uni1A41|uni1A62@592,0|uni1A2D1A5B@592,0|uni1A37@1184,0|uni1A63@1775,0|uni1A601A43@2367,0]"
    )
}

#[test]
fn shlana_3_9() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈᨻᩛ"),
        "[uni1A48|uni1A3B1A5B@592,0]"
    )
}

#[test]
fn shlana_3_10() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩋᨾᩛ"),
        "[uni1A4B|uni1A3E1A5B@592,0]"
    )
}

#[test]
fn shlana_3_11() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩁᩣᨩᨽᩢ᩠ᨮ"),
        "[uni1A41|uni1A63@592,0|uni1A29@1184,0|uni1A3D@1775,0|uni1A62@2537,0|uni1A601A2E@2537,0]"
    )
}

#[test]
fn shlana_3_12() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨷᩢᨱ᩠ᨻᨷᩩᩁᩩᩈ"),
        "[uni1A37|uni1A62@592,0|uni1A31@592,0|uni1A601A3B.wide@1524,0|uni1A37@1524,0|uni1A69@2116,0|uni1A41@2116,0|uni1A69@2708,0|uni1A48@2708,0]"
    )
}

#[test]
fn shlana_4_0() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨴᩘ᩠ᩃᩣ᩠ᨿ"),
        "[uni1A34|uni1A58@592,0|uni1A601A43@592,0|uni1A63@818,0|uni1A601A3F@1410,0]"
    )
}

#[test]
fn shlana_4_1() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈᩘᨥᩮᩣ"),
        "[uni1A48|uni1A6E@592,0|uni1A25@1184,0|uni1A58@1945,0|uni1A63@2117,0]"
    )
}

#[test]
fn shlana_4_2() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩁᩘᩈᩦ"),
        "[uni1A41|uni1A58@592,0|uni1A48@592,0|uni1A66@1184,0]"
    )
}

#[test]
fn shlana_5_0() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨶᩣᩴ"),
        "[uni1A361A63|uni1A74@592,0]"
    )
}

#[test]
fn shlana_5_1() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨾᨶᩮᩣ"),
        "[uni1A3E.v2|uni1A6E@732,0|uni1A361A63@1324,0]"
    )
}

#[test]
fn shlana_5_2() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨶᩮᩢᩣ"),
        "[uni1A6E|uni1A361A63@592,0|uni1A62@1184,0]"
    )
}

#[test]
fn shlana_5_3() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨶᩣ᩠ᨿ"),
        "[uni1A361A63|uni1A601A3F@592,0]"
    )
}

#[test]
fn shlana_5_4() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨶ᩵ᩣ᩠ᨶ"),
        "[uni1A361A63|uni1A75@592,0|uni1A601A36@592,0]"
    )
}

#[test]
fn shlana_5_5() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩍᨶ᩠ᨴᩣ"),
        "[uni1A4D|uni1A361A63@933,0|uni1A601A34@1524,0]"
    )
}

#[test]
fn shlana_5_6() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩋᩫᨶ᩠ᨲᩕᩣ᩠ᨿ"),
        "[uni1A4B|uni1A6B@592,0|uni1A55@592,0|uni1A361A63@818,0|uni1A601A32@1410,0|uni1A601A3F@1410,0]"
    )
}

#[test]
fn shlana_5_7() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨶ᩶ᩣᩴ"),
        "[uni1A361A63|uni1A74@592,0|uni1A76@592,357]"
    )
}

#[test]
fn shlana_5_8() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨶ᩠ᩅᩣ᩠ᨷ"),
        "[uni1A361A63|uni1A601A45@592,0|uni1A601A37@592,0]"
    )
}

#[test]
fn shlana_5_9() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨴᩤᩴᨶ᩠ᩅ‌ᩣ᩠ᨿ"),
        "[uni1A34|uni1A74@592,0|uni1A64@592,0|uni1A36@818,0|uni1A601A45@1410,0|uni1A63@1410,0|uni1A601A3F@2002,0]"
    )
}

#[test]
fn shlana_5_10() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨲ᩵ᩣᩴᨶ᩠ᩅᩣ᩠ᨿ"),
        "[uni1A32|uni1A74@762,0|uni1A75@762,357|uni1A63@933,0|uni1A361A63@1524,0|uni1A601A45@2116,0|uni1A601A3F@2116,0]"
    )
}

#[test]
fn shlana_5_11() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨶ‌ᩣ"),
        "[uni1A36|uni1A63@592,0]"
    )
}

#[test]
fn shlana_5_12() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩉ᩠ᨶ᩶ᩣ"),
        "[uni1A49|uni1A601A36@762,0|uni1A76@762,0|uni1A63@933,0]"
    )
}

#[test]
fn shlana_6_0() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩯᩢ᩠ᩈ"),
        "[uni1A6F|uni1A20@1096,0|uni1A62@1857,0|uni1A601A48@2028,0]"
    )
}

#[test]
fn shlana_6_1() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨴᩕᩯ᩠ᨠᨴᩮᩬᩥᩁ᩺"),
        "[uni1A6F|uni1A55@1096,0|uni1A34@1322,0|uni1A601A20@1914,0|uni1A6E@1914,0|uni1A34@2506,0|uni1A6C@3098,0|uni1A65@3098,0|uni1A41@3098,0|uni1A7A@3689,0]"
    )
}

#[test]
fn shlana_6_2() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨶᩰᩫ᩠᩶ᨲ"),
        "[uni1A70|uni1A36@592,0|uni1A6B@1184,0|uni1A76@1184,357|uni1A601A32@1184,0]"
    )
}

#[test]
fn shlana_6_3() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨷᩕᩰᨴᩦ᩠ᨶ"),
        "[uni1A70|uni1A55@592,0|uni1A37@818,0|uni1A34@1410,0|uni1A66@2002,0|uni1A601A36@2002,0]"
    )
}

#[test]
fn shlana_6_4() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨼᩥᩅ᩠ᩈ᩺"),
        "[uni1A3C|uni1A65@859,0|uni1A45@933,0|uni1A601A48@1524,0|uni1A7A@1755,0]"
    )
}

#[test]
fn shlana_6_5() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈᨲᩯᨾ᩠ᨷ᩺"),
        "[uni1A48|uni1A6F@592,0|uni1A32@1688,0|uni1A3E.v2@2620,0|uni1A601A37@3353,0|uni1A7A@3584,0]"
    )
}

#[test]
fn shlana_6_6() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈᩮᩥᩁ᩠᩺ᨷ"),
        "[uni1A6E|uni1A48@592,0|uni1A65@1184,0|uni1A41@1184,0|uni1A7A@1775,0|uni1A601A37@1775,0]"
    )
}

#[test]
fn shlana_7_0() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨴᩢ᩵ᩗᩣ"),
        "[uni1A34|uni1A621A75@592,0|uni1A57@592,0|uni1A63@818,0]"
    )
}

#[test]
fn shlana_7_1() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨡᨶ᩠ᨵᩣ"),
        "[uni1A21|uni1A361A63@592,0|uni1A601A35@1184,0]"
    )
}

#[test]
fn shlana_7_2() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨣ᩠᩶ᨯᩦ "),
        "[uni1A23|uni1A76@592,0|uni1A601A2F@592,0|uni1A66@1095,0|space_nb@592,0]"
    )
}

#[test]
fn shlana_7_3() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨷ᩠᩶ᨾᩣ"),
        "[uni1A37|uni1A76@592,0|uni1A601A3E@592,0|uni1A63@592,0]"
    )
}

#[test]
fn shlana_7_4() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨷ᩠᩶ᨾᩣ"),
        "[uni1A37|uni1A76@592,0|uni1A601A3E@592,0|uni1A63@592,0]"
    )
}

#[test]
fn shlana_7_5() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨷ᩠᩶ᨯᩣ᩠ᨿ"),
        "[uni1A37|uni1A76@592,0|uni1A601A2F@592,0|uni1A63@592,0|uni1A601A3F@1184,0]"
    )
}

#[test]
fn shlana_7_6() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨧᩢ᩠ᩅᩤ"),
        "[uni1A27|uni1A62@592,0|uni1A601A45@592,0|uni1A64@592,0]"
    )
}

#[test]
fn shlana_7_7() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈᩮ᩠ᩓ᩠ᩅ"),
        "[uni1A6E|uni1A48@592,0|uni1A601A53@1184,0|uni1A601A45@1311,-547]"
    )
}

#[test]
fn shlana_7_8() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨴᩯ᩠ᨶᩳ"),
        "[uni1A6F|uni1A34@1096,0|uni1A601A36@1688,0|uni1A73@1688,0]"
    )
}

#[test]
fn shlana_7_9() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩓ᩠ᨾᩣ"),
        "[uni1A53|uni1A601A3E@762,0|uni1A63@933,0]"
    )
}

#[test]
fn shlana_7_10() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨠᩮ᩠ᩈᩣ"),
        "[uni1A6E|uni1A20@592,0|uni1A601A48@1524,0|uni1A63@1751,0]"
    )
}

#[test]
fn shlana_7_11() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨻᩱ᩠ᨾᩣ"),
        "[uni1A71|uni1A3B@592,0|uni1A601A3E@1354,0|uni1A63@1524,0]"
    )
}

#[test]
fn shlana_7_12() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈᩮ᩠ᩅ᩶ᩤ"),
        "[uni1A6E|uni1A48@592,0|uni1A601A45@1184,0|uni1A76@1184,0|uni1A64@1184,0]"
    )
}

#[test]
fn shlana_7_13() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩅᩮ᩠ᩃᩣ"),
        "[uni1A6E|uni1A45@592,0|uni1A601A43@1184,0|uni1A63@1410,0]"
    )
}

#[test]
fn shlana_7_14() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨵᩤ᩠ᨲᩩ"),
        "[uni1A35|uni1A64@592,0|uni1A601A32@863,0|uni1A69@863,-547]"
    )
}

#[test]
fn shlana_7_15() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨩ᩠ᩓ"),
        "[uni1A29|uni1A601A53@592,0]"
    )
}

#[test]
fn shlana_7_16() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨻᩭ᩠ᩅ᩻ᩣ"),
        "[uni1A3B|uni1A6D@933,0|uni1A601A45@1137,-547|uni1A78@1204,195|uni1A63@1159,0]"
    )
}

#[test]
fn shlana_7_17() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈᩫ᩠ᨦᩣ᩠ᨶ"),
        "[uni1A48|uni1A6B@592,0|uni1A601A26@592,0|uni1A63@592,0|uni1A601A36@1184,0]"
    )
}

#[test]
fn shlana_8_0() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨻᩬᩳ᩵"),
        "[uni1A3B|uni1A6C.wide@933,0|uni1A73@762,0|uni1A75@762,447]"
    )
}

#[test]
fn shlana_8_1() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩈᨷ᩷ᩣ᩠ᨿ"),
        "[uni1A48|uni1A37@592,0|uni1A77@1184,0|uni1A63@1184,0|uni1A601A3F@1775,0]"
    )
}

#[test]
fn shlana_8_2() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩅ᩠ᨿᩙ"),
        "[uni1A45|uni1A601A3F@592,0|uni1A59@823,0]"
    )
}

#[test]
fn shlana_8_3() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨣᩪ᩺"),
        "[uni1A23|uni1A6A@592,0|uni1A7A@592,0]"
    )
}

#[test]
fn shlana_8_4() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "᪁᪂ ᨻᩢ᩠ᨶ᩻ᩣ"),
        "[uni1A81|uni1A82@592,0|space_nb@1184,0|uni1A3B@1501,0|uni1A62@2263,0|uni1A601A36@2263,0|uni1A78@2263,357|uni1A63@2434,0]"
    )
}

#[test]
fn shlana_8_5() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨻᩱ᩻ᩣ᩠ᨿ"),
        "[uni1A71|uni1A3B@592,0|uni1A78@1354,0|uni1A63@1524,0|uni1A601A3F@2116,0]"
    )
}

#[test]
fn shlana_8_6() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨩ᩠ᨿᩙᨲᩩᩴ"),
        "[uni1A29|uni1A601A3F@592,0|uni1A59@823,0|uni1A32@818,0|uni1A69@1580,0|uni1A74@1580,0]"
    )
}

#[test]
fn shlana_8_7() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩅᨲᩛᩩ"),
        "[uni1A45|uni1A321A5B@592,0|uni1A69@1184,-734]"
    )
}

#[test]
fn shlana_8_8() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩅᨲ᩠ᨳᩩ"),
        "[uni1A45|uni1A32@592,0|uni1A601A33@1354,0|uni1A69@1354,-547]"
    )
}

#[test]
fn shlana_8_9() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨯ᩠ᨿᩴ"),
        "[uni1A2F|uni1A601A3F@592,0|uni1A74@823,0]"
    )
}

#[test]
fn shlana_8_10() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩉ᩠ᨶᩦᩢ᩶"),
        "[uni1A49|uni1A601A36@762,0|uni1A66@762,0|uni1A621A76@1265,0]"
    )
}

#[test]
fn shlana_8_11() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩁᩮᩂ᩠ᨠ"),
        "[uni1A6E|uni1A41@592,0|uni1A42@1184,0|uni1A601A20@1775,-547]"
    )
}

#[test]
fn shlana_8_12() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᩁ᩠ᨿ᩺"),
        "[uni1A41|uni1A601A3F@592,0|uni1A7A@823,0]"
    )
}

#[test]
fn shlana_9_0() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨣᩴᨷᩴ᩵"),
        "[uni1A23|uni1A74@592,0|uni1A37@592,0|uni1A74@1184,0|uni1A75@1184,357]"
    )
}

#[test]
fn shlana_9_1() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨣᩴᨷᩴ"),
        "[uni1A23|uni1A74@592,0|uni1A37@592,0|uni1A74@1184,0]"
    )
}

#[test]
fn shlana_9_2() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨣᩝᩴ᩵"),
        "[uni1A23|uni1A5D@592,0|uni1A74@592,0|uni1A75@592,357]"
    )
}

#[test]
fn shlana_9_3() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨣᩴᩝ᩵"),
        "[uni1A23|uni1A74@592,0|uni1A5D@592,0|uni1A75@592,357]"
    )
}

#[test]
fn shlana_9_4() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨣᩝᩴ"),
        "[uni1A23|uni1A5D@592,0|uni1A74@592,0]"
    )
}

#[test]
fn shlana_9_5() {
    assert_eq!(
        shape("fonts/TestShapeLana.ttf", 1000, &[], "ᨣᩴᩝ"),
        "[uni1A23|uni1A74@592,0|uni1A5D@592,0]"
    )
}
