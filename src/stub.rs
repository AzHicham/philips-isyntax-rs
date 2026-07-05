//! SDK-less stubs used when the `native-sdk` feature is disabled.

use crate::{
    ContainerName, DimensionsRange, Facade, Image, ImageType, PhilipsEngine, Rectangle,
    RegionRequest, Result, Size, View, bindings::ffi, errors::PhilipsSlideError,
};
use std::{iter, path::Path};

fn unavailable<T>() -> Result<T> {
    Err(PhilipsSlideError::SdkUnavailable)
}

impl PhilipsEngine {
    /// Create a stub engine handle.
    pub fn new() -> Self {
        Self { inner: ffi::new_() }
    }

    /// Create a new instance of Facade.
    ///
    /// This requires the `native-sdk` feature and returns
    /// [`PhilipsSlideError::SdkUnavailable`] in SDK-less builds.
    pub fn facade<P: AsRef<Path>>(
        &self,
        _filename: P,
        _container: &ContainerName,
    ) -> Result<Facade<'_>> {
        unavailable()
    }

    /// Create a new instance of Facade with a cache file.
    ///
    /// This requires the `native-sdk` feature and returns
    /// [`PhilipsSlideError::SdkUnavailable`] in SDK-less builds.
    pub fn facade_with_cache_file<P: AsRef<Path>, R: AsRef<Path>>(
        &self,
        _filename: P,
        _container: &ContainerName,
        _cache_filename: R,
    ) -> Result<Facade<'_>> {
        unavailable()
    }

    /// Returns the SDK PixelEngine version.
    pub fn sdk_version(&self) -> Result<String> {
        unavailable()
    }

    /// Returns all containers supported by the SDK PixelEngine.
    pub fn containers(&self) -> impl Iterator<Item = &str> {
        iter::empty()
    }

    /// Returns the version of a container.
    pub fn container_version(&self, _container: &str) -> Result<&str> {
        unavailable()
    }

    /// Returns all compressors supported by the SDK PixelEngine.
    pub fn compressors(&self) -> impl Iterator<Item = &str> {
        iter::empty()
    }

    /// Returns all pixel_transforms supported by the SDK PixelEngine.
    pub fn pixel_transforms(&self) -> impl Iterator<Item = &str> {
        iter::empty()
    }

    /// Returns all colorspace_transforms supported by the SDK PixelEngine.
    pub fn colorspace_transforms(&self) -> impl Iterator<Item = &str> {
        iter::empty()
    }

    /// Returns all quality_presets supported by the SDK PixelEngine.
    pub fn quality_presets(&self) -> impl Iterator<Item = &str> {
        iter::empty()
    }

    /// Returns all supported_filters supported by the SDK PixelEngine.
    pub fn supported_filters(&self) -> impl Iterator<Item = &str> {
        iter::empty()
    }
}

impl Default for PhilipsEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageType {
    pub fn as_str(&self) -> &str {
        match &self {
            Self::WSI => "WSI",
            Self::MacroImage => "MACROIMAGE",
            Self::LabelImage => "LABELIMAGE",
        }
    }
}

impl AsRef<[u8]> for ImageType {
    fn as_ref(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl Facade<'_> {
    /// Returns numbers of images in ISyntax file.
    pub fn num_images(&self) -> Result<usize> {
        unavailable()
    }

    /// Returns the version of isyntax file.
    pub fn isyntax_file_version(&self) -> Result<&str> {
        unavailable()
    }

    /// Return the id of the facade.
    pub fn id(&self) -> Result<&str> {
        unavailable()
    }

    /// Returns the barcode in the Label/ILE image.
    pub fn barcode(&self) -> Result<&str> {
        unavailable()
    }

    /// Returns the calibration status of the scanner used to create the image file.
    pub fn scanner_calibration_status(&self) -> Result<&str> {
        unavailable()
    }

    /// Returns the software versions used to create the image file.
    pub fn software_versions(&self) -> Result<impl Iterator<Item = &str>> {
        Ok(iter::empty())
    }

    /// Returns the derivation description.
    pub fn derivation_description(&self) -> Result<&str> {
        unavailable()
    }

    /// Returns the acquisition DateTime of the image file.
    pub fn acquisition_date_time(&self) -> Result<&str> {
        unavailable()
    }

    /// Returns the scanner manufacturer used to create the image file.
    pub fn manufacturer(&self) -> Result<&str> {
        unavailable()
    }

    /// Returns the scanner model used to create the image file.
    pub fn model_name(&self) -> Result<&str> {
        unavailable()
    }

    /// Returns the scanner serial number used to create the image file.
    pub fn device_serial_number(&self) -> Result<&str> {
        unavailable()
    }

    /// Returns the scanner rack number used to create the image file.
    pub fn scanner_rack_number(&self) -> Result<u16> {
        unavailable()
    }

    /// Returns the scanner slot number used to create the image file.
    pub fn scanner_slot_number(&self) -> Result<u16> {
        unavailable()
    }

    /// Returns the scanner operator id used to create the image file.
    pub fn scanner_operator_id(&self) -> Result<&str> {
        unavailable()
    }

    pub fn scanner_rack_priority(&self) -> Result<u16> {
        unavailable()
    }

    /// Returns the last calibration date of the scanner used to create the image file.
    pub fn date_of_last_calibration(&self) -> Result<impl Iterator<Item = &str>> {
        Ok(iter::empty())
    }

    /// Returns the last calibration time of the scanner used to create the image file.
    pub fn time_of_last_calibration(&self) -> Result<impl Iterator<Item = &str>> {
        Ok(iter::empty())
    }

    /// Returns true if the distributor of the image file is Philips.
    pub fn is_philips(&self) -> Result<bool> {
        unavailable()
    }

    /// Returns true if the distributor of the image file is Hamamatsu.
    pub fn is_hamamatsu(&self) -> Result<bool> {
        unavailable()
    }

    /// Returns true if the file was created by Philips Ultra Fast Scanner.
    pub fn is_ufs(&self) -> Result<bool> {
        unavailable()
    }

    pub fn is_ufsb(&self) -> Result<bool> {
        unavailable()
    }

    pub fn is_uvs(&self) -> Result<bool> {
        unavailable()
    }

    /// Create a new instance of Image.
    pub fn image(&self, _image_type: &ImageType) -> Result<Image<'_>> {
        unavailable()
    }
}

impl ContainerName {
    pub fn as_str(&self) -> &str {
        match &self {
            Self::Default => "",
            Self::Ficom => "ficom",
            Self::Dicom => "dicom",
            Self::CachingFicom => "caching-ficom",
            Self::S3 => "s3",
            Self::Legacy => "legacy",
        }
    }
}

impl Image<'_> {
    /// Returns the pixel transform used for this Image.
    pub fn pixel_transform(&self) -> Result<&str> {
        unavailable()
    }

    /// Returns the quality preset used for this Image.
    pub fn quality_preset(&self) -> Result<&str> {
        unavailable()
    }

    /// Returns the quality used for this Image.
    pub fn quality(&self) -> Result<usize> {
        unavailable()
    }

    /// Returns the compressor used for this Image.
    pub fn compressor(&self) -> Result<&str> {
        unavailable()
    }

    /// Returns the colorspace transform used for this Image.
    pub fn colorspace_transform(&self) -> Result<&str> {
        unavailable()
    }

    /// Returns the number of tiles in this Image.
    pub fn num_tiles(&self) -> Result<usize> {
        unavailable()
    }

    /// Returns the ICC Profile of this Image.
    pub fn icc_profile(&self) -> Result<&str> {
        unavailable()
    }

    /// ICC matrix (3x3) representing color correction from the ICC profile.
    pub fn icc_matrix(&self) -> Result<[f64; 9]> {
        unavailable()
    }

    /// Returns image data encoded as JPEG.
    pub fn image_data(&self) -> Result<&[u8]> {
        unavailable()
    }

    /// Returns image data as a DynamicImage.
    #[cfg(feature = "image")]
    pub fn get_image(&self) -> Result<image::DynamicImage> {
        unavailable()
    }

    /// Indicates whether the image is compressed with or without loss.
    pub fn lossy_image_compression(&self) -> Result<&str> {
        unavailable()
    }

    /// Returns the compression ratio of the SubImage.
    pub fn lossy_image_compression_ratio(&self) -> Result<f64> {
        unavailable()
    }

    pub fn color_linearity(&self) -> Result<&str> {
        unavailable()
    }

    /// Create a new instance of View.
    pub fn view(&self) -> Result<View<'_>> {
        unavailable()
    }
}

impl View<'_> {
    /// Returns the dimension ranges of the SubImage for a certain level.
    pub fn dimension_ranges(&self, _level: u32) -> Result<DimensionsRange> {
        unavailable()
    }

    /// Returns the dimension names of the SubImage.
    pub fn dimension_names(&self) -> impl Iterator<Item = &str> {
        iter::empty()
    }

    /// Returns the dimension units of the SubImage.
    pub fn dimension_units(&self) -> impl Iterator<Item = &str> {
        iter::empty()
    }

    /// Returns the dimension types of the SubImage.
    pub fn dimension_types(&self) -> impl Iterator<Item = &str> {
        iter::empty()
    }

    /// Returns the scale factor.
    pub fn scale(&self) -> &[f64] {
        &[]
    }

    /// Returns the origin of the Label/ILE SubImage.
    pub fn origin(&self) -> &[f64] {
        &[]
    }

    /// Returns envelopes coordinates as Rectangles.
    pub fn envelopes_as_rectangles(&self, _level: u32) -> Result<Vec<Rectangle>> {
        unavailable()
    }

    /// Returns the number of bit allocated per sub-pixel.
    pub fn bits_allocated(&self) -> u16 {
        0
    }

    /// Returns the number of bit really used per sub-pixel.
    pub fn bits_stored(&self) -> u16 {
        0
    }

    /// Returns the highest bit.
    pub fn high_bit(&self) -> u16 {
        0
    }

    pub fn pixel_representation(&self) -> Result<u16> {
        unavailable()
    }

    pub fn planar_configuration(&self) -> Result<u16> {
        unavailable()
    }

    /// Returns the number of sub pixel per pixel.
    pub fn samples_per_pixel(&self) -> Result<u16> {
        unavailable()
    }

    /// Returns the number of level available for a SubImage.
    pub fn num_derived_levels(&self) -> u32 {
        0
    }

    /// Read a tile from a WSI SubImage.
    pub fn read_region(
        &self,
        _engine: &PhilipsEngine,
        _request: &RegionRequest,
    ) -> Result<(Vec<u8>, Size)> {
        unavailable()
    }

    /// Read a tile from a WSI SubImage into a caller-provided buffer.
    pub fn read_region_into(
        &self,
        _engine: &PhilipsEngine,
        _request: &RegionRequest,
        _buffer: &mut Vec<u8>,
    ) -> Result<Size> {
        unavailable()
    }

    /// Read a tile from a WSI SubImage into an RgbImage.
    #[cfg(feature = "image")]
    pub fn read_image(
        &self,
        _engine: &PhilipsEngine,
        _request: &RegionRequest,
    ) -> Result<image::RgbImage> {
        unavailable()
    }

    /// Read a thumbnail from a WSI SubImage.
    #[cfg(feature = "image")]
    pub fn read_thumbnail(&self, _engine: &PhilipsEngine, _size: &Size) -> Result<image::RgbImage> {
        unavailable()
    }

    /// Get the appropriate level for the given dimensions.
    pub fn get_best_level_for_dimensions(
        &self,
        _dimension: &Size,
        _dimension_level_0: &Size,
        _level_count: u32,
    ) -> u32 {
        0
    }
}
