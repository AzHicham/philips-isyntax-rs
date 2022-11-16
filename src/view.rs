//! This module contains all functions related to Philips Views
//!

use crate::{
    errors::PhilipsSlideError, DimensionsRange, ImageType, PhilipsSlide, Rectangle, RegionRequest,
    Result, Size,
};
use cxx::let_cxx_string;

#[cfg(feature = "image")]
use image::RgbImage;

impl PhilipsSlide {
    /// Returns the dimension ranges of the SubImage for a certain level
    /// For Macro and Label/ILE image this function return a result only for level 0
    pub fn dimension_ranges(&self, image_type: ImageType, level: u32) -> Result<DimensionsRange> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.dimensionRanges(&image_type, level)?)
    }

    /// Returns the dimension names of the SubImage
    /// Example : ["x", "y"]
    pub fn dimension_names(&self, image_type: ImageType) -> impl Iterator<Item = &str> {
        let_cxx_string!(image_type = image_type);
        self.inner
            .dimensionNames(&image_type)
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    /// Returns the dimension units of the SubImage
    /// Example : ["MicroMeter", "MicroMeter"]
    pub fn dimension_units(&self, image_type: ImageType) -> impl Iterator<Item = &str> {
        let_cxx_string!(image_type = image_type);
        self.inner
            .dimensionUnits(&image_type)
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    /// Returns the dimension types of the SubImage
    /// Example : ["spatial", "spatial"]
    pub fn dimension_types(&self, image_type: ImageType) -> impl Iterator<Item = &str> {
        let_cxx_string!(image_type = image_type);
        self.inner
            .dimensionTypes(&image_type)
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    /// Returns the scale factor, i.e. the resolution of various dimensions with
    /// with reference to the 0 level.
    pub fn scale(&self, image_type: ImageType) -> &[f64] {
        let_cxx_string!(image_type = image_type);
        self.inner.scale(&image_type).as_slice()
    }

    /// Returns the origin of the Label/ILE SubImage.
    pub fn origin(&self, image_type: ImageType) -> &[f64] {
        let_cxx_string!(image_type = image_type);
        self.inner.origin(&image_type).as_slice()
    }

    /// Returns envelopes coordinates as Rectangles
    pub fn envelopes_as_rectangles(
        &self,
        image_type: ImageType,
        level: u32,
    ) -> Result<Vec<Rectangle>> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.envelopesAsRectangles(&image_type, level)?)
    }

    pub fn bits_allocated(&self, image_type: ImageType) -> u16 {
        let_cxx_string!(image_type = image_type);
        self.inner.bitsAllocated(&image_type)
    }

    pub fn bits_stored(&self, image_type: ImageType) -> u16 {
        let_cxx_string!(image_type = image_type);
        self.inner.bitsStored(&image_type)
    }

    pub fn high_bit(&self, image_type: ImageType) -> u16 {
        let_cxx_string!(image_type = image_type);
        self.inner.highBit(&image_type)
    }

    pub fn pixel_representation(&self, image_type: ImageType) -> Result<u16> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.pixelRepresentation(&image_type)?)
    }

    pub fn planar_configuration(&self, image_type: ImageType) -> Result<u16> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.planarConfiguration(&image_type)?)
    }

    pub fn samples_per_pixel(&self, image_type: ImageType) -> Result<u16> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.samplesPerPixel(&image_type)?)
    }

    /// Returns the number of level available for a SubImage
    pub fn num_derived_levels(&self, image_type: ImageType) -> usize {
        let_cxx_string!(image_type = image_type);
        self.inner.numDerivedLevels(&image_type)
    }

    /// Read a tile from a WSI SubImage.
    ///
    /// This function reads and decompresses a region of a whole slide image into an Vec<u8>
    pub fn read_region(&self, request: &RegionRequest) -> Result<(Vec<u8>, Size), cxx::Exception> {
        let mut buffer = Vec::<u8>::new();
        let mut image_size = Size { w: 0, h: 0 };

        self.inner
            .read_region(request, &mut buffer, &mut image_size)?;
        let size = (image_size.w * image_size.h * 3) as usize; // RGB Image

        unsafe {
            buffer.set_len(size);
        }

        Ok((buffer, image_size))
    }

    /// Read a tile from a WSI SubImage.
    ///
    /// This function reads and decompresses a region of a whole slide image into an RgbImage
    #[cfg(feature = "image")]
    pub fn read_image(&self, request: &RegionRequest) -> Result<RgbImage> {
        let (buffer, size) = self.read_region(request)?;
        let image = RgbImage::from_vec(size.w, size.h, buffer).ok_or_else(|| {
            PhilipsSlideError::ImageError("Error while creating RgbImage from buffer".to_string())
        })?;
        Ok(image)
    }
}
