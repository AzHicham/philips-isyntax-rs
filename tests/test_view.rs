mod fixture;

use fixture::sample;
use std::path::Path;

use philips_isyntax_rs::{ContainerName, DimensionsRange, ImageType, PhilipsEngine, Rectangle};
use rstest::rstest;

#[rstest]
#[case(sample())]
fn test_view_wsi(#[case] filename: &Path) {
    let engine = PhilipsEngine::new();
    let facade = engine.facade("facade_name2").unwrap();
    facade.open(filename, &ContainerName::CachingFicom).unwrap();
    let image = facade.image(&ImageType::WSI).unwrap();
    let view = image.view().unwrap();

    assert_eq!(
        view.dimension_ranges(0).unwrap(),
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
        view.dimension_ranges(1).unwrap(),
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
        view.dimension_ranges(9).unwrap(),
        DimensionsRange {
            start_x: 1024,
            step_x: 512,
            end_x: 157696,
            start_y: 1024,
            step_y: 512,
            end_y: 90112
        }
    );
    assert!(view.dimension_ranges(10).is_err());
    assert_eq!(
        view.dimension_names().collect::<Vec<_>>(),
        vec!["x", "y", "component"]
    );
    assert_eq!(
        view.dimension_units().collect::<Vec<_>>(),
        vec!["MicroMeter", "MicroMeter", ""]
    );
    assert_eq!(
        view.dimension_types().collect::<Vec<_>>(),
        vec!["spatial", "spatial", "colour component"]
    );

    assert_eq!(view.scale(), [0.25, 0.25, 1.0]);
    assert_eq!(view.origin(), [1125.5, 1212.0]);
    assert_eq!(view.bits_allocated(), 8);
    assert_eq!(view.bits_stored(), 8);
    assert_eq!(view.high_bit(), 7);
    assert_eq!(view.pixel_representation().unwrap(), 0);
    assert_eq!(view.planar_configuration().unwrap(), 0);
    assert_eq!(view.samples_per_pixel().unwrap(), 4);
    assert_eq!(view.num_derived_levels(), 9);
}

#[rstest]
#[case(sample())]
fn test_view_macro(#[case] filename: &Path) {
    let engine = PhilipsEngine::new();
    let facade = engine.facade("facade_name2").unwrap();
    facade.open(filename, &ContainerName::CachingFicom).unwrap();
    let image = facade.image(&ImageType::MacroImage).unwrap();
    let view = image.view().unwrap();

    assert_eq!(
        view.dimension_ranges(0).unwrap(),
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
    assert!(view.dimension_ranges(1).is_err());
    assert_eq!(view.dimension_names().collect::<Vec<_>>(), vec!["x", "y"]);
    assert_eq!(
        view.dimension_units().collect::<Vec<_>>(),
        vec!["MicroMeter", "MicroMeter"]
    );
    assert_eq!(
        view.dimension_types().collect::<Vec<_>>(),
        vec!["spatial", "spatial"]
    );

    assert_eq!(view.scale(), [32.0, 32.0]);
    assert_eq!(view.origin(), [0.0, 0.0]);
    assert_eq!(view.bits_allocated(), 8);
    assert_eq!(view.bits_stored(), 8);
    assert_eq!(view.high_bit(), 7);
    assert_eq!(view.num_derived_levels(), 0);
    // Not available for macro image
    assert!(view.pixel_representation().is_err());
    assert!(view.planar_configuration().is_err());
    assert!(view.samples_per_pixel().is_err());
}

#[rstest]
#[case(sample())]
fn test_view_label(#[case] filename: &Path) {
    let engine = PhilipsEngine::new();
    let facade = engine.facade("facade_name2").unwrap();
    facade.open(filename, &ContainerName::CachingFicom).unwrap();
    let image = facade.image(&ImageType::LabelImage).unwrap();
    let view = image.view().unwrap();

    assert_eq!(
        view.dimension_ranges(0).unwrap(),
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
    assert!(view.dimension_ranges(1).is_err());
    assert_eq!(view.dimension_names().collect::<Vec<_>>(), vec!["x", "y"]);
    assert_eq!(
        view.dimension_units().collect::<Vec<_>>(),
        vec!["MicroMeter", "MicroMeter"]
    );
    assert_eq!(
        view.dimension_types().collect::<Vec<_>>(),
        vec!["spatial", "spatial"]
    );
    assert_eq!(view.scale(), [32.0, 32.0]);
    assert_eq!(view.origin(), [57472.0, 0.0]);
    assert_eq!(view.bits_allocated(), 8);
    assert_eq!(view.bits_stored(), 8);
    assert_eq!(view.high_bit(), 7);
    assert_eq!(view.num_derived_levels(), 0);
    // Not available for label image
    assert!(view.pixel_representation().is_err());
    assert!(view.planar_configuration().is_err());
    assert!(view.samples_per_pixel().is_err());
}

#[rstest]
#[case(sample())]
fn test_envelopes(#[case] filename: &Path) {
    let engine = PhilipsEngine::new();
    let facade = engine.facade("facade_name2").unwrap();
    facade.open(filename, &ContainerName::CachingFicom).unwrap();
    let image = facade.image(&ImageType::WSI).unwrap();
    let view = image.view().unwrap();

    let envelopes_range_0 = view.envelopes_as_rectangles(0).unwrap();
    let envelopes_range_9 = view.envelopes_as_rectangles(9).unwrap();

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
