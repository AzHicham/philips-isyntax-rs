mod fixture;

#[cfg(feature = "image")]
use {
    fixture::{sample, sample_i2syntax},
    philips_isyntax_rs::{ContainerName, ImageType, PhilipsEngine},
    rstest::rstest,
    std::path::Path,
};

#[rstest]
#[case(sample())]
#[cfg(feature = "image")]
fn test_sub_image_slide(#[case] filename: &Path) {
    let engine = PhilipsEngine::new();
    let facade = engine
        .facade(filename, &ContainerName::CachingFicom)
        .unwrap();
    let image = facade.image(&ImageType::WSI).unwrap();

    assert_eq!(image.pixel_transform().unwrap(), "legall53");
    assert_eq!(image.quality_preset().unwrap(), "Q1");
    assert_eq!(image.quality().unwrap(), 18446744073709551615);
    assert_eq!(image.compressor().unwrap(), "hulsken");
    assert_eq!(image.colorspace_transform().unwrap(), "RGB2YCoCg");
    assert_eq!(image.num_tiles().unwrap(), 212892);
    assert!(image.icc_profile().is_ok()); // too long to display
    assert_eq!(
        image.icc_matrix().unwrap(),
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
    assert_eq!(image.lossy_image_compression().unwrap(), "01");
    assert_eq!(image.lossy_image_compression_ratio().unwrap(), 7.5);
    assert_eq!(image.color_linearity().unwrap(), "sRGB");
    // image_data not available for slide image
    assert_eq!(image.image_data().unwrap().len(), 0);
}

#[rstest]
#[case(sample())]
#[cfg(feature = "image")]
fn test_sub_image_macro(#[case] filename: &Path) {
    let engine = PhilipsEngine::new();
    let facade = engine
        .facade(filename, &ContainerName::CachingFicom)
        .unwrap();
    let image = facade.image(&ImageType::MacroImage).unwrap();

    // Some function are only available with ImageType::WSI
    assert!(image.pixel_transform().is_err());
    assert!(image.quality_preset().is_err());
    assert!(image.quality().is_err());
    assert!(image.compressor().is_err());
    assert!(image.colorspace_transform().is_err());
    assert!(image.num_tiles().is_err());
    assert!(image.color_linearity().is_err());
    assert!(image.icc_profile().is_ok()); // too long to display
    assert_eq!(
        image.icc_matrix().unwrap(),
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
    assert_eq!(image.lossy_image_compression().unwrap(), "01");
    assert_eq!(image.lossy_image_compression_ratio().unwrap(), 26.0);
    assert_eq!(image.image_data().unwrap().len(), 75580);

    let macro_image = image.get_image().unwrap();
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
    let engine = PhilipsEngine::new();
    let facade = engine
        .facade(filename, &ContainerName::CachingFicom)
        .unwrap();
    let image = facade.image(&ImageType::LabelImage).unwrap();

    // Some function are only available with ImageType::slide
    assert!(image.pixel_transform().is_err());
    assert!(image.quality_preset().is_err());
    assert!(image.quality().is_err());
    assert!(image.compressor().is_err());
    assert!(image.colorspace_transform().is_err());
    assert!(image.num_tiles().is_err());
    assert!(image.color_linearity().is_err());
    assert!(image.icc_matrix().is_err());
    assert!(image.icc_profile().is_ok()); // too long to display
    assert_eq!(image.lossy_image_compression().unwrap(), "01");
    assert_eq!(image.lossy_image_compression_ratio().unwrap(), 26.0);
    assert_eq!(image.image_data().unwrap().len(), 52734);

    let label_image = image.get_image().unwrap();
    image::save_buffer(
        Path::new("label_image.jpeg"),
        label_image.as_bytes(),
        label_image.width(),
        label_image.height(),
        label_image.color(),
    )
    .unwrap();
}

#[rstest]
#[case(sample_i2syntax())]
#[cfg(feature = "image")]
fn test_i2syntax_sub_image_slide(#[case] filename: &Path) {
    let engine = PhilipsEngine::new();
    let facade = engine
        .facade(filename, &ContainerName::CachingFicom)
        .unwrap();
    let image = facade.image(&ImageType::WSI).unwrap();

    assert_eq!(image.pixel_transform().unwrap(), "legall53");
    assert_eq!(image.quality_preset().unwrap(), "Q2");
    assert_eq!(image.quality().unwrap(), 18446744073709551615);
    assert_eq!(image.compressor().unwrap(), "hulsken2");
    assert_eq!(image.colorspace_transform().unwrap(), "RGB2YCoCg");
    assert_eq!(image.num_tiles().unwrap(), 2097168);
    assert!(image.icc_profile().is_ok()); // too long to display
    assert_eq!(
        image.icc_matrix().unwrap(),
        [
            1.7378870276206972,
            -0.3340537894621137,
            0.02305579175279321,
            -0.28660139078112445,
            1.4693326858187172,
            -0.5411745946267302,
            -0.3561981843724411,
            -0.04204419797506245,
            1.6160681688222087
        ]
    );
    assert_eq!(image.lossy_image_compression().unwrap(), "01");
    assert_eq!(image.lossy_image_compression_ratio().unwrap(), 15.0);
    assert_eq!(image.color_linearity().unwrap(), "linear");
    // image_data not available for slide image
    assert_eq!(image.image_data().unwrap().len(), 0);
}

#[rstest]
#[case(sample_i2syntax())]
#[cfg(feature = "image")]
fn test_i2syntax_sub_image_macro(#[case] filename: &Path) {
    let engine = PhilipsEngine::new();
    let facade = engine
        .facade(filename, &ContainerName::CachingFicom)
        .unwrap();
    let image = facade.image(&ImageType::MacroImage).unwrap();

    // Some function are only available with ImageType::WSI
    assert!(image.pixel_transform().is_err());
    assert!(image.quality_preset().is_err());
    assert!(image.quality().is_err());
    assert!(image.compressor().is_err());
    assert!(image.colorspace_transform().is_err());
    assert!(image.num_tiles().is_err());
    assert!(image.color_linearity().is_err());
    assert!(image.icc_profile().is_ok()); // too long to display
    assert!(image.icc_matrix().is_err());
    assert_eq!(image.lossy_image_compression().unwrap(), "");
    assert!(image.lossy_image_compression_ratio().is_err());
    assert_eq!(image.image_data().unwrap().len(), 1209706);

    let macro_image = image.get_image().unwrap();
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
#[case(sample_i2syntax())]
#[cfg(feature = "image")]
fn test_i2syntax_sub_image_label(#[case] filename: &Path) {
    let engine = PhilipsEngine::new();
    let facade = engine
        .facade(filename, &ContainerName::CachingFicom)
        .unwrap();
    let image = facade.image(&ImageType::LabelImage).unwrap();

    // Some function are only available with ImageType::slide
    assert!(image.pixel_transform().is_err());
    assert!(image.quality_preset().is_err());
    assert!(image.quality().is_err());
    assert!(image.compressor().is_err());
    assert!(image.colorspace_transform().is_err());
    assert!(image.num_tiles().is_err());
    assert!(image.color_linearity().is_err());
    assert!(image.icc_matrix().is_err());
    assert!(image.icc_profile().is_ok()); // too long to display
    assert_eq!(image.lossy_image_compression().unwrap(), "");
    assert!(image.lossy_image_compression_ratio().is_err());
    assert_eq!(image.image_data().unwrap().len(), 470591);

    let label_image = image.get_image().unwrap();
    image::save_buffer(
        Path::new("label_image.jpeg"),
        label_image.as_bytes(),
        label_image.width(),
        label_image.height(),
        label_image.color(),
    )
    .unwrap();
}
