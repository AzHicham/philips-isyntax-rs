//! This module contains all functions related to Philips Views
//!

use crate::{DimensionsRange, PhilipsEngine, Rectangle, RegionRequest, Result, Size, View};

#[cfg(feature = "image")]
use {crate::errors::ImageError, image::RgbImage};

//#[cfg(feature = "image")]
//use {crate::errors::PhilipsSlideError, image::RgbImage};

impl<'a> View<'a> {
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
    /// This function reads and decompresses a region of a whole slide image into an Vec<u8>
    pub fn read_region(
        &self,
        engine: &PhilipsEngine,
        request: &RegionRequest,
    ) -> Result<(Vec<u8>, Size), cxx::Exception> {
        let mut buffer = Vec::<u8>::new();
        let mut image_size = Size { w: 0, h: 0 };

        self.inner
            .read_region(&engine.inner, request, &mut buffer, &mut image_size)?;
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
    pub fn read_image(&self, engine: &PhilipsEngine, request: &RegionRequest) -> Result<RgbImage> {
        let (buffer, size) = self.read_region(engine, request)?;
        let image = RgbImage::from_vec(size.w, size.h, buffer).ok_or_else(|| {
            ImageError::Other("Error while creating RgbImage from buffer".to_string())
        })?;
        Ok(image)
    }
}
