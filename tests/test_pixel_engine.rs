use philips_isyntax_rs::PixelEngine;
use rstest::rstest;

#[rstest]
fn test_pixel_engine_version() {
    assert_eq!(PixelEngine::version().unwrap(), "5.1.0".to_string());
}

#[rstest]
fn test_containers() {
    let pe = PixelEngine::new();
    assert_eq!(
        pe.containers().collect::<Vec<_>>(),
        vec!["ficom", "dicom", "caching-ficom", "s3", "legacy"]
    )
}

#[rstest]
fn test_container_version() {
    let pe = PixelEngine::new();
    assert_eq!(pe.container_version("ficom").unwrap(), "100.5");
}

#[rstest]
#[should_panic(expected = "CoreError(\"Invalid factory\")")]
fn test_error_container_version() {
    let pe = PixelEngine::new();
    pe.container_version("unknown").unwrap();
}

#[rstest]
fn test_compressors() {
    let pe = PixelEngine::new();
    assert_eq!(
        pe.compressors().collect::<Vec<_>>(),
        vec!["hulsken2", "hulsken", "none", "jpeg"]
    )
}

#[rstest]
fn test_pixel_transforms() {
    let pe = PixelEngine::new();
    assert_eq!(
        pe.pixel_transforms().collect::<Vec<_>>(),
        vec!["legall53", "pyramid", "passthrough"]
    )
}

#[rstest]
fn test_colorspace_transforms() {
    let pe = PixelEngine::new();
    assert_eq!(
        pe.colorspace_transforms().collect::<Vec<_>>(),
        vec!["none", "RGB2YCoCg", "RGB10Packed2RGB", "RGB10Packed2YCoCg"]
    )
}

#[rstest]
fn test_quality_presets() {
    let pe = PixelEngine::new();
    assert_eq!(
        pe.quality_presets().collect::<Vec<_>>(),
        vec!["Q0", "Q1", "Q2"]
    )
}

#[rstest]
fn test_supported_filters() {
    let pe = PixelEngine::new();
    assert_eq!(
        pe.supported_filters().collect::<Vec<_>>(),
        vec!["3x3Matrix16", "Sharpness8", "Linear16ToSRGB8"]
    )
}

#[rstest]
fn test_make_facade() {
    let mut pe = PixelEngine::new();
    pe.facade("in");
}
