mod fixture;

use fixture::sample;
use std::path::Path;

use philips_isyntax_rs::{PhilipsSlide, Rectangle, RegionRequest};
use rstest::rstest;

#[rstest]
#[case(sample())]
fn test_read_region_wsi(#[case] filename: &Path) {
    let slide = PhilipsSlide::new(filename.to_str().unwrap()).unwrap();

    let req = RegionRequest {
        roi: Rectangle {
            start_x: 0,
            start_y: 0,
            end_x: 199,
            end_y: 99,
        },
        level: 0,
    };

    let (buffer, size) = slide.read_region(&req).unwrap();
    assert_eq!(size.w, 200);
    assert_eq!(size.h, 100);
    assert_eq!(buffer.len(), 60000);
}

#[rstest]
#[case(sample())]
fn test_read_image_wsi(#[case] filename: &Path) {
    let slide = PhilipsSlide::new(filename.to_str().unwrap()).unwrap();

    let req = RegionRequest {
        roi: Rectangle {
            start_x: 0,
            start_y: 0,
            end_x: 99,
            end_y: 199,
        },
        level: 0,
    };

    let image = slide.read_image(&req).unwrap();
    assert_eq!(image.width(), 100);
    assert_eq!(image.height(), 200);
    assert_eq!(image.len(), 60000);
}
