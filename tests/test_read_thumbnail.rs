mod fixture;

use fixture::{sample, sample_i2syntax};
use std::path::Path;

use philips_isyntax_rs::{ContainerName, ImageType, PhilipsEngine, Size};
use rstest::rstest;

#[cfg(feature = "image")]
#[rstest]
fn test_thumbnail(
    #[values(sample(), sample_i2syntax())] filename: &Path,
    #[values(
        Size { w: 254, h: 254 },
        Size { w: 10, h: 100 },
        Size { w: 1000, h: 1000 },
        Size { w: 200, h: 10 }
    )]
    size: Size,
) {
    let engine = PhilipsEngine::new();
    let facade = engine
        .facade(filename, &ContainerName::CachingFicom)
        .unwrap();
    let image = facade.image(&ImageType::WSI).unwrap();
    let view = image.view().unwrap();

    let thumbnail = view.read_thumbnail(&engine, &size).unwrap();
    thumbnail.save(format!("thumbnail{0}.jpg", size.w)).unwrap();

    // Make sure one of the dimensions is equal to the requested one
    // and the other one is smaller than the requested one
    assert!(
        (thumbnail.width() == size.w && thumbnail.height() <= size.h)
            || (thumbnail.width() <= size.w && thumbnail.height() == size.h)
    );
}
