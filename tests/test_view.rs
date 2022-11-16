mod fixture;

use fixture::sample;
use std::path::Path;

use philips_isyntax_rs::{DimensionsRange, ImageType, PhilipsSlide, Rectangle};
use rstest::rstest;

#[rstest]
#[case(sample())]
fn test_view_wsi(#[case] filename: &Path) {
    let slide = PhilipsSlide::new(filename.to_str().unwrap()).unwrap();

    assert_eq!(
        slide.dimension_ranges(ImageType::WSI, 0).unwrap(),
        DimensionsRange {
            start_x: 0,
            step_x: 1,
            end_x: 158725,
            start_y: 0,
            step_y: 1,
            end_y: 91141
        }
    );
    assert_eq!(
        slide.dimension_ranges(ImageType::WSI, 1).unwrap(),
        DimensionsRange {
            start_x: 2,
            step_x: 2,
            end_x: 158722,
            start_y: 2,
            step_y: 2,
            end_y: 91138
        }
    );
    assert_eq!(
        slide.dimension_ranges(ImageType::WSI, 9).unwrap(),
        DimensionsRange {
            start_x: 1024,
            step_x: 512,
            end_x: 157696,
            start_y: 1024,
            step_y: 512,
            end_y: 90112
        }
    );
    assert!(slide.dimension_ranges(ImageType::WSI, 10).is_err());
    assert_eq!(
        slide.dimension_names(ImageType::WSI).collect::<Vec<_>>(),
        vec!["x", "y", "component"]
    );
    assert_eq!(
        slide.dimension_units(ImageType::WSI).collect::<Vec<_>>(),
        vec!["MicroMeter", "MicroMeter", ""]
    );
    assert_eq!(
        slide.dimension_types(ImageType::WSI).collect::<Vec<_>>(),
        vec!["spatial", "spatial", "colour component"]
    );

    assert_eq!(slide.scale(ImageType::WSI), [0.25, 0.25, 1.0]);
    assert_eq!(slide.origin(ImageType::WSI), [1125.5, 1212.0]);
    assert_eq!(slide.bits_allocated(ImageType::WSI), 8);
    assert_eq!(slide.bits_stored(ImageType::WSI), 8);
    assert_eq!(slide.high_bit(ImageType::WSI), 7);
    assert_eq!(slide.pixel_representation(ImageType::WSI).unwrap(), 0);
    assert_eq!(slide.planar_configuration(ImageType::WSI).unwrap(), 0);
    assert_eq!(slide.samples_per_pixel(ImageType::WSI).unwrap(), 4);
    assert_eq!(slide.num_derived_levels(ImageType::WSI), 9);
}

#[rstest]
#[case(sample())]
fn test_view_macro(#[case] filename: &Path) {
    let slide = PhilipsSlide::new(filename.to_str().unwrap()).unwrap();

    assert_eq!(
        slide.dimension_ranges(ImageType::MacroImage, 0).unwrap(),
        DimensionsRange {
            start_x: 0,
            step_x: 1,
            end_x: 1796,
            start_y: 0,
            step_y: 1,
            end_y: 821
        }
    );
    // level > 0 not available for macro & label images
    assert!(slide.dimension_ranges(ImageType::MacroImage, 1).is_err());
    assert_eq!(
        slide
            .dimension_names(ImageType::MacroImage)
            .collect::<Vec<_>>(),
        vec!["x", "y"]
    );
    assert_eq!(
        slide
            .dimension_units(ImageType::MacroImage)
            .collect::<Vec<_>>(),
        vec!["MicroMeter", "MicroMeter"]
    );
    assert_eq!(
        slide
            .dimension_types(ImageType::MacroImage)
            .collect::<Vec<_>>(),
        vec!["spatial", "spatial"]
    );

    assert_eq!(slide.scale(ImageType::MacroImage), [32.0, 32.0]);
    assert_eq!(slide.origin(ImageType::MacroImage), [0.0, 0.0]);
    assert_eq!(slide.bits_allocated(ImageType::MacroImage), 8);
    assert_eq!(slide.bits_stored(ImageType::MacroImage), 8);
    assert_eq!(slide.high_bit(ImageType::MacroImage), 7);
    assert_eq!(slide.num_derived_levels(ImageType::MacroImage), 0);
    // Not available for macro image
    assert!(slide.pixel_representation(ImageType::MacroImage).is_err());
    assert!(slide.planar_configuration(ImageType::MacroImage).is_err());
    assert!(slide.samples_per_pixel(ImageType::MacroImage).is_err());
}

#[rstest]
#[case(sample())]
fn test_view_label(#[case] filename: &Path) {
    let slide = PhilipsSlide::new(filename.to_str().unwrap()).unwrap();

    assert_eq!(
        slide.dimension_ranges(ImageType::LabelImage, 0).unwrap(),
        DimensionsRange {
            start_x: 0,
            step_x: 1,
            end_x: 681,
            start_y: 0,
            step_y: 1,
            end_y: 821
        }
    );
    // level > 0 not available for macro & label images
    assert!(slide.dimension_ranges(ImageType::LabelImage, 1).is_err());
    assert_eq!(
        slide
            .dimension_names(ImageType::LabelImage)
            .collect::<Vec<_>>(),
        vec!["x", "y"]
    );
    assert_eq!(
        slide
            .dimension_units(ImageType::LabelImage)
            .collect::<Vec<_>>(),
        vec!["MicroMeter", "MicroMeter"]
    );
    assert_eq!(
        slide
            .dimension_types(ImageType::LabelImage)
            .collect::<Vec<_>>(),
        vec!["spatial", "spatial"]
    );
    assert_eq!(slide.scale(ImageType::LabelImage), [32.0, 32.0]);
    assert_eq!(slide.origin(ImageType::LabelImage), [57472.0, 0.0]);
    assert_eq!(slide.bits_allocated(ImageType::LabelImage), 8);
    assert_eq!(slide.bits_stored(ImageType::LabelImage), 8);
    assert_eq!(slide.high_bit(ImageType::LabelImage), 7);
    assert_eq!(slide.num_derived_levels(ImageType::LabelImage), 0);
    // Not available for label image
    assert!(slide.pixel_representation(ImageType::LabelImage).is_err());
    assert!(slide.planar_configuration(ImageType::LabelImage).is_err());
    assert!(slide.samples_per_pixel(ImageType::LabelImage).is_err());
}

#[rstest]
#[case(sample())]
fn test_envelopes(#[case] filename: &Path) {
    let slide = PhilipsSlide::new(filename.to_str().unwrap()).unwrap();

    let envelopes_range_0 = slide.envelopes_as_rectangles(ImageType::WSI, 0).unwrap();
    let envelopes_range_9 = slide.envelopes_as_rectangles(ImageType::WSI, 9).unwrap();

    assert_eq!(envelopes_range_0.len(), 5);
    assert_eq!(
        envelopes_range_0[0],
        Rectangle {
            start_x: 0,
            end_x: 45570,
            start_y: 42499,
            end_y: 69122
        }
    );
    assert_eq!(
        envelopes_range_0[4],
        Rectangle {
            start_x: 112131,
            end_x: 158725,
            start_y: 41475,
            end_y: 62978
        }
    );
    assert_eq!(
        envelopes_range_9[0],
        Rectangle {
            start_x: 1024,
            end_x: 44544,
            start_y: 44032,
            end_y: 68096
        }
    );
    assert_eq!(
        envelopes_range_9[4],
        Rectangle {
            start_x: 113664,
            end_x: 157696,
            start_y: 43008,
            end_y: 61952
        }
    );
}
