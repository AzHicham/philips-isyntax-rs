use philips_isyntax_rs::PhilipsSlide;
use rstest::rstest;
use std::path::Path;
mod fixture;
use fixture::sample;

#[rstest]
#[case(sample())]
fn test_pixel_engine_version(#[case] filename: &Path) {
    let slide = PhilipsSlide::new(filename.to_str().unwrap()).unwrap();
    assert_eq!(slide.sdk_version().unwrap(), "5.1.0".to_string());
}

#[rstest]
#[case(sample())]
fn test_containers(#[case] filename: &Path) {
    let slide = PhilipsSlide::new(filename.to_str().unwrap()).unwrap();
    assert_eq!(
        slide.containers().collect::<Vec<_>>(),
        vec!["ficom", "dicom", "caching-ficom", "s3", "legacy"]
    )
}

#[rstest]
#[case(sample())]
fn test_container_version(#[case] filename: &Path) {
    let slide = PhilipsSlide::new(filename.to_str().unwrap()).unwrap();
    assert_eq!(slide.container_version("ficom").unwrap(), "100.5");
}

#[rstest]
#[case(sample())]
#[should_panic(expected = "CoreError(\"Invalid factory\")")]
fn test_error_container_version(#[case] filename: &Path) {
    let slide = PhilipsSlide::new(filename.to_str().unwrap()).unwrap();
    slide.container_version("unknown").unwrap();
}

#[rstest]
#[case(sample())]
fn test_compressors(#[case] filename: &Path) {
    let slide = PhilipsSlide::new(filename.to_str().unwrap()).unwrap();
    assert_eq!(
        slide.compressors().collect::<Vec<_>>(),
        vec!["hulsken2", "hulsken", "none", "jpeg"]
    )
}

#[rstest]
#[case(sample())]
fn test_pixel_transforms(#[case] filename: &Path) {
    let slide = PhilipsSlide::new(filename.to_str().unwrap()).unwrap();
    assert_eq!(
        slide.pixel_transforms().collect::<Vec<_>>(),
        vec!["legall53", "pyramid", "passthrough"]
    )
}

#[rstest]
#[case(sample())]
fn test_colorspace_transforms(#[case] filename: &Path) {
    let slide = PhilipsSlide::new(filename.to_str().unwrap()).unwrap();
    assert_eq!(
        slide.colorspace_transforms().collect::<Vec<_>>(),
        vec!["none", "RGB2YCoCg", "RGB10Packed2RGB", "RGB10Packed2YCoCg"]
    )
}

#[rstest]
#[case(sample())]
fn test_quality_presets(#[case] filename: &Path) {
    let slide = PhilipsSlide::new(filename.to_str().unwrap()).unwrap();
    assert_eq!(
        slide.quality_presets().collect::<Vec<_>>(),
        vec!["Q0", "Q1", "Q2"]
    )
}

#[rstest]
#[case(sample())]
fn test_supported_filters(#[case] filename: &Path) {
    let slide = PhilipsSlide::new(filename.to_str().unwrap()).unwrap();
    assert_eq!(
        slide.supported_filters().collect::<Vec<_>>(),
        vec!["3x3Matrix16", "Sharpness8", "Linear16ToSRGB8"]
    )
}
