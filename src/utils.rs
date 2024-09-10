use crate::{Result, Size};
use std::cmp;

#[cfg(feature = "image")]
use fast_image_resize as fr;
#[cfg(feature = "image")]
use {crate::errors::ImageError, image::RgbImage};

// Get the appropriate level for the given dimensions: i.e. the level with at least one
// dimensions (i.e along one axis) greater than the dimension requested
#[cfg(feature = "image")]
pub fn get_best_level_for_dimensions(
    dimension: &Size,
    dimension_level_0: &Size,
    level_count: u32,
) -> u32 {
    let downsample = f64::max(
        f64::from(dimension_level_0.w) / f64::from(dimension.w),
        f64::from(dimension_level_0.h) / f64::from(dimension.h),
    );
    let level_dowsamples: Vec<f64> = (0..level_count)
        .map(|level| 2_u32.pow(level) as f64)
        .collect();
    if downsample < 1.0 {
        return 0;
    }
    for i in 1..level_count {
        if downsample < level_dowsamples[i as usize] {
            return i - 1;
        }
    }
    level_count - 1
}

#[cfg(feature = "image")]
pub(crate) fn resize_rgb_image(image: RgbImage, new_size: &Size) -> Result<RgbImage> {
    let src_image = fr::images::Image::from_vec_u8(
        image.width(),
        image.height(),
        image.into_raw(),
        fr::PixelType::U8x3,
    )
    .map_err(|err| ImageError::Other(err.to_string()))?;

    let mut dst_image = fr::images::Image::new(new_size.w, new_size.h, fr::PixelType::U8x3);
    let mut resizer = fr::Resizer::new();
    let option = fr::ResizeOptions {
        algorithm: fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3),
        cropping: fr::SrcCropping::None,
        mul_div_alpha: false,
    };
    resizer
        .resize(&src_image, &mut dst_image, &option)
        .map_err(|err| ImageError::Other(err.to_string()))?;
    let image = RgbImage::from_vec(new_size.w, new_size.h, dst_image.into_vec()).unwrap(); // safe because dst_image buffer is big enough

    Ok(image)
}

#[cfg(feature = "image")]
pub(crate) fn preserve_aspect_ratio(size: &Size, dimension: &Size) -> Size {
    // Code adapted from https://pillow.readthedocs.io/en/latest/_modules/PIL/Image.html#Image.thumbnail
    fn round_aspect<F: FnMut(f32) -> f32>(number: f32, mut key: F) -> u32 {
        cmp::max(
            cmp::min_by_key(number.floor() as u32, number.ceil() as u32, |n| {
                key(*n as f32).round() as u32
            }),
            1,
        )
    }
    let w = size.w as f32;
    let h = size.h as f32;
    let aspect: f32 = dimension.w as f32 / dimension.h as f32;
    if { w / h } >= aspect {
        Size::new(
            round_aspect(h * aspect, |n| (aspect - n / h).abs()),
            h as u32,
        )
    } else {
        Size::new(
            w as u32,
            round_aspect(w / aspect, |n| {
                if n == 0. {
                    0.
                } else {
                    (aspect - w / n).abs()
                }
            }),
        )
    }
}

#[cfg(test)]
#[cfg(feature = "image")]
mod tests {
    use super::*;
    use rstest::rstest;

    // Note: the dimensions for each levels are:
    // {
    //     0: Size { w: 158726, h: 90627},
    //     1: Size { w: 79361, h: 45313 },
    //     2: Size { w: 39678, h: 22655 },
    //     3: Size { w: 19837, h: 11327 },
    //     4: Size { w: 9917, h: 5663 },
    //     5: Size { w: 4957, h: 2831 },
    //     6: Size { w: 2477, h: 1415 },
    //     7: Size { w: 1237, h: 707 },
    //     8: Size { w: 617, h: 353 },
    //     9: Size { w: 307, h: 175 },
    // }
    #[rstest]
    #[case(Size::new(500, 500), 8)]
    #[case(Size::new(100, 100), 9)]
    #[case(Size::new(800, 800), 7)]
    #[case(Size::new(100000, 100000), 0)]
    fn test_get_best_level_for_dimensions(#[case] size: Size, #[case] expected_level: u32) {
        let dimension_level_0 = Size::new(158726, 90627);
        let level_count = 10;
        assert_eq!(
            get_best_level_for_dimensions(&size, &dimension_level_0, level_count),
            expected_level
        );
    }

    #[test]
    fn test_preserve_aspect_ratio() {
        assert_eq!(
            preserve_aspect_ratio(&Size { w: 100, h: 100 }, &Size { w: 50, h: 50 }),
            Size { w: 100, h: 100 }
        );
        assert_eq!(
            preserve_aspect_ratio(&Size { w: 100, h: 100 }, &Size { w: 25, h: 50 }),
            Size { w: 50, h: 100 }
        );
        assert_eq!(
            // Edge case
            preserve_aspect_ratio(&Size { w: 1, h: 1 }, &Size { w: 25, h: 50 }),
            Size { w: 1, h: 1 }
        );
        assert_eq!(
            // Edge case
            preserve_aspect_ratio(&Size { w: 100, h: 200 }, &Size { w: 1, h: 1 }),
            Size { w: 100, h: 100 }
        );
        assert_eq!(
            // Edge case
            preserve_aspect_ratio(&Size { w: 0, h: 5 }, &Size { w: 1, h: 10 }),
            Size { w: 0, h: 1 }
        );
        assert_eq!(
            // Not round ratio
            preserve_aspect_ratio(&Size { w: 33, h: 100 }, &Size { w: 12, h: 13 }),
            Size { w: 33, h: 35 }
        );
        assert_eq!(
            // Not round ratio
            preserve_aspect_ratio(&Size { w: 33, h: 15 }, &Size { w: 12, h: 13 }),
            Size { w: 13, h: 15 }
        );
    }
}
