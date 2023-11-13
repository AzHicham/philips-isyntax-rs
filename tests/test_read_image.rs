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
