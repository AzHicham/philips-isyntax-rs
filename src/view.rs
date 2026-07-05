//! This module contains all functions related to Philips Views
//!

#[cfg(feature = "image")]
use crate::utils::{
    get_best_level_for_dimensions, preserve_aspect_ratio, resize_rgb_image, validate_non_zero_size,
};
use crate::{
    DimensionsRange, PhilipsEngine, Rectangle, RegionRequest, Result, Size, View,
    errors::PhilipsSlideError,
};

#[cfg(feature = "image")]
use {crate::errors::ImageError, image::RgbImage};

const RGB_BYTES_PER_PIXEL: usize = 3;
#[cfg(feature = "image")]
const MAX_THUMBNAIL_INTERMEDIATE_BYTES: usize = 512 * 1024 * 1024;

fn rgb_buffer_len(size: &Size) -> Result<usize> {
    let pixels = (size.w as usize).checked_mul(size.h as usize).ok_or(
        PhilipsSlideError::BufferSizeOverflow {
            width: size.w,
            height: size.h,
            bytes_per_pixel: RGB_BYTES_PER_PIXEL,
        },
    )?;
    pixels
        .checked_mul(RGB_BYTES_PER_PIXEL)
        .ok_or(PhilipsSlideError::BufferSizeOverflow {
            width: size.w,
            height: size.h,
            bytes_per_pixel: RGB_BYTES_PER_PIXEL,
        })
}

fn invalid_region(roi: &Rectangle) -> PhilipsSlideError {
    PhilipsSlideError::InvalidRegion {
        start_x: roi.start_x,
        end_x: roi.end_x,
        start_y: roi.start_y,
        end_y: roi.end_y,
    }
}

fn region_output_size(request: &RegionRequest, dimension_range: &DimensionsRange) -> Result<Size> {
    if dimension_range.step_x == 0 {
        return Err(crate::errors::DimensionsRangeToSizeError::NullStepX.into());
    }
    if dimension_range.step_y == 0 {
        return Err(crate::errors::DimensionsRangeToSizeError::NullStepY.into());
    }

    let width = request
        .roi
        .end_x
        .checked_sub(request.roi.start_x)
        .ok_or_else(|| invalid_region(&request.roi))?
        / dimension_range.step_x;
    let height = request
        .roi
        .end_y
        .checked_sub(request.roi.start_y)
        .ok_or_else(|| invalid_region(&request.roi))?
        / dimension_range.step_y;

    Ok(Size {
        w: width
            .checked_add(1)
            .ok_or_else(|| invalid_region(&request.roi))?,
        h: height
            .checked_add(1)
            .ok_or_else(|| invalid_region(&request.roi))?,
    })
}

impl View<'_> {
    /// Returns the dimension ranges of the SubImage for a certain level
    /// For Macro and Label/ILE image this function return a result only for level 0
    pub fn dimension_ranges(&self, level: u32) -> Result<DimensionsRange> {
        Ok(self.inner.dimensionRanges(level)?)
    }

    /// Returns the dimension names of the SubImage
    /// Example : ["x", "y"]
    pub fn dimension_names(&self) -> impl Iterator<Item = &str> {
        self.inner
            .dimensionNames()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    /// Returns the dimension units of the SubImage
    /// Example : ["MicroMeter", "MicroMeter"]
    pub fn dimension_units(&self) -> impl Iterator<Item = &str> {
        self.inner
            .dimensionUnits()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    /// Returns the dimension types of the SubImage
    /// Example : ["spatial", "spatial"]
    pub fn dimension_types(&self) -> impl Iterator<Item = &str> {
        self.inner
            .dimensionTypes()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    /// Returns the scale factor, i.e. the resolution of various dimensions with
    /// with reference to the 0 level.
    pub fn scale(&self) -> &[f64] {
        self.inner.scale().as_slice()
    }

    /// Returns the origin of the Label/ILE SubImage.
    pub fn origin(&self) -> &[f64] {
        self.inner.origin().as_slice()
    }

    /// Returns envelopes coordinates as Rectangles
    pub fn envelopes_as_rectangles(&self, level: u32) -> Result<Vec<Rectangle>> {
        Ok(self.inner.envelopesAsRects(level)?)
    }

    /// Returns the number of bit allocated per sub-pixel
    pub fn bits_allocated(&self) -> u16 {
        self.inner.bitsAllocated()
    }

    /// Returns the number of bit really used per sub-pixel
    pub fn bits_stored(&self) -> u16 {
        self.inner.bitsStored()
    }

    /// Returns the highest bit
    pub fn high_bit(&self) -> u16 {
        self.inner.highBit()
    }

    pub fn pixel_representation(&self) -> Result<u16> {
        Ok(self.inner.pixelRepresentation()?)
    }

    pub fn planar_configuration(&self) -> Result<u16> {
        Ok(self.inner.planarConfiguration()?)
    }

    /// Returns the number of sub pixel per pixel, 3 for RGB and 4 for RGBA
    pub fn samples_per_pixel(&self) -> Result<u16> {
        Ok(self.inner.samplesPerPixel()?)
    }

    /// Returns the number of level available for a SubImage
    pub fn num_derived_levels(&self) -> u32 {
        self.inner.numDerivedLevels()
    }

    /// Read a tile from a WSI SubImage.
    ///
    /// This function reads and decompresses a region of a whole slide image into an `Vec<u8>`
    pub fn read_region(
        &self,
        engine: &PhilipsEngine,
        request: &RegionRequest,
    ) -> Result<(Vec<u8>, Size)> {
        let mut buffer = Vec::<u8>::new();
        let image_size = self.read_region_into(engine, request, &mut buffer)?;
        Ok((buffer, image_size))
    }

    /// Read a tile from a WSI SubImage into a caller-provided buffer.
    ///
    /// The buffer is resized and reused, which avoids repeated allocations for tile sweeps.
    pub fn read_region_into(
        &self,
        engine: &PhilipsEngine,
        request: &RegionRequest,
        buffer: &mut Vec<u8>,
    ) -> Result<Size> {
        let dimension_range = self.dimension_ranges(request.level)?;
        let mut image_size = region_output_size(request, &dimension_range)?;
        let expected_size = rgb_buffer_len(&image_size)?;

        if buffer.len() != expected_size {
            buffer.resize(expected_size, 0);
        }
        let bytes_written =
            self.inner
                .read_region(&engine.inner, request, &mut buffer, &mut image_size)?;
        let actual_expected_size = rgb_buffer_len(&image_size)?;

        if bytes_written != actual_expected_size || buffer.len() != actual_expected_size {
            return Err(PhilipsSlideError::UnexpectedBufferSize {
                expected: actual_expected_size,
                actual: bytes_written,
            });
        }

        Ok(image_size)
    }

    /// Read a tile from a WSI SubImage.
    ///
    /// This function reads and decompresses a region of a whole slide image into an RgbImage
    #[cfg(feature = "image")]
    pub fn read_image(&self, engine: &PhilipsEngine, request: &RegionRequest) -> Result<RgbImage> {
        let (buffer, size) = self.read_region(engine, request)?;
        let image = RgbImage::from_vec(size.w, size.h, buffer).ok_or_else(|| {
            ImageError::Other("Error while creating RgbImage from buffer".to_string())
        })?;
        Ok(image)
    }

    /// Read a thumbnail from a WSI SubImage.
    ///
    /// This function reads and decompresses a thumbnail of a whole slide image into an RgbImage
    #[cfg(feature = "image")]
    pub fn read_thumbnail(&self, engine: &PhilipsEngine, size: &Size) -> Result<RgbImage> {
        validate_non_zero_size(size)?;
        let level_count = self.num_derived_levels() + 1;
        let dimension_level_0 = Size::try_from(&self.dimension_ranges(0)?)?;
        let best_level = get_best_level_for_dimensions(size, &dimension_level_0, level_count)?;
        let dimensions_range = self.dimension_ranges(best_level)?;
        let intermediate_size = Size::try_from(&dimensions_range)?;
        let intermediate_bytes = rgb_buffer_len(&intermediate_size)?;
        if intermediate_bytes > MAX_THUMBNAIL_INTERMEDIATE_BYTES {
            return Err(PhilipsSlideError::ThumbnailTooLarge {
                bytes: intermediate_bytes,
                limit: MAX_THUMBNAIL_INTERMEDIATE_BYTES,
            });
        }
        let region_request = RegionRequest {
            roi: Rectangle {
                start_x: dimensions_range.start_x,
                end_x: dimensions_range.end_x,
                start_y: dimensions_range.start_y,
                end_y: dimensions_range.end_y,
            },
            level: best_level,
        };
        let image = self.read_image(engine, &region_request)?;
        let final_size = preserve_aspect_ratio(size, &intermediate_size);
        let image = resize_rgb_image(image, &final_size)?;
        Ok(image)
    }

    // Get the appropriate level for the given dimensions: i.e. the level with at least one
    // dimensions greater than the dimension requested along one axis
    pub fn get_best_level_for_dimensions(
        &self,
        dimension: &Size,
        dimension_level_0: &Size,
        level_count: u32,
    ) -> Result<u32> {
        if dimension.w == 0 || dimension.h == 0 {
            return Err(PhilipsSlideError::InvalidSize {
                width: dimension.w,
                height: dimension.h,
            });
        }
        if dimension_level_0.w == 0 || dimension_level_0.h == 0 || level_count == 0 {
            return Err(PhilipsSlideError::InvalidSize {
                width: dimension_level_0.w,
                height: dimension_level_0.h,
            });
        }

        let downsample = f64::max(
            f64::from(dimension_level_0.w) / f64::from(dimension.w),
            f64::from(dimension_level_0.h) / f64::from(dimension.h),
        );
        Ok((0..level_count)
            .map(|level| (1_u64 << level.min(63)) as f64)
            .enumerate()
            .rfind(|(_, ds)| ds <= &downsample)
            .map_or(0, |(index, _)| index) as u32)
    }
}
