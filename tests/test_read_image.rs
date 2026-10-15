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

#[rstest]
#[case(sample())]
#[case(sample_i2syntax())]
fn test_read_region_outside_envelopes_is_white(#[case] filename: &Path) {
    let engine = PhilipsEngine::new();
    let facade = engine
        .facade(filename, &ContainerName::CachingFicom)
        .unwrap();
    let image = facade.image(&ImageType::WSI).unwrap();
    let view = image.view().unwrap();

    // A strip of the valid range just past the right or bottom edge of every
    // data envelope, if the slide has one.
    let range = view.dimension_ranges(0).unwrap();
    let envelopes = view.envelopes_as_rectangles(0).unwrap();
    let right = envelopes.iter().map(|e| e.end_x).max().unwrap();
    let bottom = envelopes.iter().map(|e| e.end_y).max().unwrap();
    let roi = if right + 10 <= range.end_x {
        Rectangle {
            start_x: right + 1,
            end_x: right + 10,
            start_y: range.start_y,
            end_y: range.start_y + 9,
        }
    } else if bottom + 10 <= range.end_y {
        Rectangle {
            start_x: range.start_x,
            end_x: range.start_x + 9,
            start_y: bottom + 1,
            end_y: bottom + 10,
        }
    } else {
        eprintln!("{filename:?}: the data envelopes cover the whole valid range");
        return;
    };

    let (buffer, size) = view
        .read_region(&engine, &RegionRequest { roi, level: 0 })
        .unwrap();
    assert_eq!((size.w, size.h), (10, 10));
    assert!(buffer.iter().all(|&v| v == 255));
}
