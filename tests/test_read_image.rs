mod fixture;

use fixture::{sample, sample_i2syntax};
use std::path::Path;

use philips_isyntax_rs::{ContainerName, ImageType, PhilipsEngine, Rectangle, RegionRequest};
use rstest::rstest;

#[rstest]
#[case(sample())]
#[case(sample_i2syntax())]
fn test_read_region_wsi(#[case] filename: &Path) {
    let engine = PhilipsEngine::new();
    let facade = engine
        .facade(filename, &ContainerName::CachingFicom)
        .unwrap();
    let image = facade.image(&ImageType::WSI).unwrap();
    let view = image.view().unwrap();

    let req = RegionRequest {
        roi: Rectangle {
            start_x: 0,
            start_y: 0,
            end_x: 199,
            end_y: 99,
        },
        level: 0,
    };

    let (buffer, size) = view.read_region(&engine, &req).unwrap();
    assert_eq!(size.w, 200);
    assert_eq!(size.h, 100);
    assert_eq!(buffer.len(), 60000);
}

#[rstest]
#[case(sample())]
#[case(sample_i2syntax())]
#[cfg(feature = "image")]
fn test_read_image_wsi(#[case] filename: &Path) {
    let engine = PhilipsEngine::new();
    let facade = engine
        .facade(filename, &ContainerName::CachingFicom)
        .unwrap();
    let image = facade.image(&ImageType::WSI).unwrap();
    let view = image.view().unwrap();

    let req = RegionRequest {
        roi: Rectangle {
            start_x: 0,
            start_y: 0,
            end_x: 99,
            end_y: 199,
        },
        level: 0,
    };

    let image = view.read_image(&engine, &req).unwrap();
    assert_eq!(image.width(), 100);
    assert_eq!(image.height(), 200);
    assert_eq!(image.len(), 60000);
}

/// Outside the data of a slide (past the valid range of a level, or outside its data envelopes), pixels
/// are filled with white, at every level.
#[rstest]
#[case(sample(), 0)]
#[case(sample(), 2)]
#[case(sample_i2syntax(), 0)]
#[case(sample_i2syntax(), 2)]
fn test_read_region_past_the_valid_range_is_white(#[case] filename: &Path, #[case] level: u32) {
    let engine = PhilipsEngine::new();
    let facade = engine
        .facade(filename, &ContainerName::CachingFicom)
        .unwrap();
    let image = facade.image(&ImageType::WSI).unwrap();
    let view = image.view().unwrap();

    // 10 x 10 pixels of the level, right of its valid range. Coordinates are level 0 pixels.
    let range = view.dimension_ranges(level).unwrap();
    let roi = Rectangle {
        start_x: range.end_x + range.step_x,
        end_x: range.end_x + 10 * range.step_x,
        start_y: range.start_y,
        end_y: range.start_y + 9 * range.step_y,
    };

    let (buffer, size) = view
        .read_region(&engine, &RegionRequest { roi, level })
        .unwrap();
    assert_eq!((size.w, size.h), (10, 10));
    assert!(buffer.iter().all(|&v| v == 255));
}
