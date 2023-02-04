use philips_isyntax_rs::PhilipsEngine;
use rstest::rstest;

#[rstest]
fn test_pixel_engine_version() {
    let slide = PhilipsEngine::new();
    assert_eq!(slide.sdk_version().unwrap(), "5.1.0".to_string());
}

#[rstest]
fn test_containers() {
    let slide = PhilipsEngine::new();
    assert_eq!(
        slide.containers().collect::<Vec<_>>(),
        vec!["ficom", "dicom", "caching-ficom", "s3", "legacy"]
    )
}

#[rstest]
fn test_container_version() {
    let slide = PhilipsEngine::new();
    assert_eq!(slide.container_version("ficom").unwrap(), "100.5");
}

#[rstest]
#[should_panic(expected = "CoreError(\"Invalid factory\")")]
fn test_error_container_version() {
    let slide = PhilipsEngine::new();
    slide.container_version("unknown").unwrap();
}

#[rstest]
fn test_compressors() {
    let slide = PhilipsEngine::new();
    assert_eq!(
        slide.compressors().collect::<Vec<_>>(),
        vec!["hulsken2", "hulsken", "none", "jpeg"]
    )
}

#[rstest]
fn test_pixel_transforms() {
    let slide = PhilipsEngine::new();
    assert_eq!(
        slide.pixel_transforms().collect::<Vec<_>>(),
        vec!["legall53", "pyramid", "passthrough"]
    )
}

#[rstest]
fn test_colorspace_transforms() {
    let slide = PhilipsEngine::new();
    assert_eq!(
        slide.colorspace_transforms().collect::<Vec<_>>(),
        vec!["none", "RGB2YCoCg", "RGB10Packed2RGB", "RGB10Packed2YCoCg"]
    )
}

#[rstest]
fn test_quality_presets() {
    let slide = PhilipsEngine::new();
    assert_eq!(
        slide.quality_presets().collect::<Vec<_>>(),
        vec!["Q0", "Q1", "Q2"]
    )
}

#[rstest]
fn test_supported_filters() {
    let slide = PhilipsEngine::new();
    assert_eq!(
        slide.supported_filters().collect::<Vec<_>>(),
        vec!["3x3Matrix16", "Sharpness8", "Linear16ToSRGB8"]
    )
}
