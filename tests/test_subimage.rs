mod fixture;

use fixture::sample;
use std::path::Path;

use philips_isyntax_rs::{ImageType, PhilipsSlide};
use rstest::rstest;

#[rstest]
#[case(sample())]
#[cfg(feature = "image")]
fn test_sub_image_slide(#[case] filename: &Path) {
    let slide = PhilipsSlide::new(filename.to_str().unwrap()).unwrap();

    assert_eq!(slide.pixel_transform(ImageType::WSI).unwrap(), "legall53");
    assert_eq!(slide.quality_preset(ImageType::WSI).unwrap(), "Q1");
    assert_eq!(slide.quality(ImageType::WSI).unwrap(), 18446744073709551615);
    assert_eq!(slide.compressor(ImageType::WSI).unwrap(), "hulsken");
    assert_eq!(
        slide.colorspace_transform(ImageType::WSI).unwrap(),
        "RGB2YCoCg"
    );
    assert_eq!(slide.num_tiles(ImageType::WSI).unwrap(), 212892);
    assert_eq!(
        slide.icc_matrix(ImageType::WSI).unwrap(),
        [
            2.347703959974617,
            -0.09146761300483147,
            0.07763151671045589,
            -0.9462112425510685,
            1.5714784234328798,
            -0.04482334970478791,
            0.12227660652625261,
            -0.012414532359697458,
            1.5236921332915387
        ]
    );
    assert_eq!(slide.lossy_image_compression(ImageType::WSI).unwrap(), "01");
    assert_eq!(
        slide.lossy_image_compression_ratio(ImageType::WSI).unwrap(),
        7.5
    );
    assert_eq!(slide.color_linearity(ImageType::WSI).unwrap(), "sRGB");
    // image_data not available for slide image
    assert_eq!(slide.image_data(ImageType::WSI).unwrap().len(), 0);
}

#[rstest]
#[case(sample())]
#[cfg(feature = "image")]
fn test_sub_image_macro(#[case] filename: &Path) {
    let slide = PhilipsSlide::new(filename.to_str().unwrap()).unwrap();

    // Some function are only available with ImageType::WSI
    assert!(slide.pixel_transform(ImageType::MacroImage).is_err());
    assert!(slide.quality_preset(ImageType::MacroImage).is_err());
    assert!(slide.quality(ImageType::MacroImage).is_err());
    assert!(slide.compressor(ImageType::MacroImage).is_err());
    assert!(slide.colorspace_transform(ImageType::MacroImage).is_err());
    assert!(slide.num_tiles(ImageType::MacroImage).is_err());
    assert!(slide.color_linearity(ImageType::MacroImage).is_err());
    assert_eq!(
        slide.icc_matrix(ImageType::MacroImage).unwrap(),
        [
            4.2469514940794975,
            -0.5188781721132593,
            -0.11345170545981326,
            -2.977016025985196,
            2.9391279998572375,
            -0.2851057435833021,
            0.4864946822867946,
            -0.9486413384636889,
            1.881488484919372
        ]
    );
    assert_eq!(
        slide
            .lossy_image_compression(ImageType::MacroImage)
            .unwrap(),
        "01"
    );
    assert_eq!(
        slide
            .lossy_image_compression_ratio(ImageType::MacroImage)
            .unwrap(),
        26.0
    );
    assert_eq!(
        slide.image_data(ImageType::MacroImage).unwrap().len(),
        75580
    );

    let macro_image = slide.get_image(ImageType::MacroImage).unwrap();
    image::save_buffer(
        Path::new("macro_image.jpeg"),
        macro_image.as_bytes(),
        macro_image.width(),
        macro_image.height(),
        macro_image.color(),
    )
    .unwrap();
}

#[rstest]
#[case(sample())]
#[cfg(feature = "image")]
fn test_sub_image_label(#[case] filename: &Path) {
    let slide = PhilipsSlide::new(filename.to_str().unwrap()).unwrap();

    // Some function are only available with ImageType::slide
    assert!(slide.pixel_transform(ImageType::LabelImage).is_err());
    assert!(slide.quality_preset(ImageType::LabelImage).is_err());
    assert!(slide.quality(ImageType::LabelImage).is_err());
    assert!(slide.compressor(ImageType::LabelImage).is_err());
    assert!(slide.colorspace_transform(ImageType::LabelImage).is_err());
    assert!(slide.num_tiles(ImageType::LabelImage).is_err());
    assert!(slide.color_linearity(ImageType::LabelImage).is_err());
    assert!(slide.icc_matrix(ImageType::LabelImage).is_err());
    assert_eq!(
        slide
            .lossy_image_compression(ImageType::LabelImage)
            .unwrap(),
        "01"
    );
    assert_eq!(
        slide
            .lossy_image_compression_ratio(ImageType::LabelImage)
            .unwrap(),
        26.0
    );
    assert_eq!(
        slide.image_data(ImageType::LabelImage).unwrap().len(),
        52734
    );

    let label_image = slide.get_image(ImageType::LabelImage).unwrap();
    image::save_buffer(
        Path::new("label_image.jpeg"),
        label_image.as_bytes(),
        label_image.width(),
        label_image.height(),
        label_image.color(),
    )
    .unwrap();
}
