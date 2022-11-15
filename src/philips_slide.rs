#[cxx::bridge]
pub(crate) mod ffi {

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Size {
        pub w: u32,
        pub h: u32,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RegionRequest {
        pub start_x: u32,
        pub start_y: u32,
        pub end_x: u32,
        pub end_y: u32,
        pub level: u32,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct DimensionsRange {
        pub start_x: u32,
        pub step_x: u32,
        pub end_x: u32,
        pub start_y: u32,
        pub step_y: u32,
        pub end_y: u32,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Rectangle {
        pub x_min: u32,
        pub x_max: u32,
        pub y_min: u32,
        pub y_max: u32,
    }

    unsafe extern "C++" {
        include!("philips-isyntax-rs/cpp/philipsslide.hpp");

        pub type PhilipsSlide;

        fn new_(url: &CxxString) -> Result<UniquePtr<PhilipsSlide>>;

        // SDK properties
        fn containers(&self) -> &CxxVector<CxxString>;
        fn sdkVersion(&self) -> &CxxString;
        fn containerVersion(&self, container: &CxxString) -> Result<&CxxString>;
        fn compressors(&self) -> &CxxVector<CxxString>;
        fn pixelTransforms(&self) -> &CxxVector<CxxString>;
        fn colorspaceTransforms(&self) -> &CxxVector<CxxString>;
        fn qualityPresets(&self) -> &CxxVector<CxxString>;
        fn supportedFilters(&self) -> &CxxVector<CxxString>;

        // File properties
        fn numImages(&self) -> Result<usize>;
        fn iSyntaxFileVersion(&self) -> Result<&CxxString>;
        fn id(&self) -> Result<&CxxString>;
        fn barcode(&self) -> Result<&CxxString>;
        fn scannerCalibrationStatus(&self) -> Result<&CxxString>;
        fn softwareVersions(&self) -> Result<&CxxVector<CxxString>>;
        fn derivationDescription(&self) -> Result<&CxxString>;
        fn acquisitionDateTime(&self) -> Result<&CxxString>;
        fn manufacturer(&self) -> Result<&CxxString>;
        fn modelName(&self) -> Result<&CxxString>;
        fn deviceSerialNumber(&self) -> Result<&CxxString>;
        fn scannerRackNumber(&self) -> Result<u16>;
        fn scannerSlotNumber(&self) -> Result<u16>;
        fn scannerOperatorId(&self) -> Result<&CxxString>;
        fn scannerRackPriority(&self) -> Result<u16>;
        fn dateOfLastCalibration(&self) -> Result<&CxxVector<CxxString>>;
        fn timeOfLastCalibration(&self) -> Result<&CxxVector<CxxString>>;
        fn isPhilips(&self) -> Result<bool>;
        fn isHamamatsu(&self) -> Result<bool>;
        fn isUFS(&self) -> Result<bool>;
        fn isUFSb(&self) -> Result<bool>;
        fn isUVS(&self) -> Result<bool>;

        // Sub Image properties
        fn pixelTransform(&self, sub_image: &CxxString) -> Result<&CxxString>;
        fn qualityPreset(&self, sub_image: &CxxString) -> Result<&CxxString>;
        fn quality(&self, sub_image: &CxxString) -> Result<usize>;
        fn compressor(&self, sub_image: &CxxString) -> Result<&CxxString>;
        fn colorspaceTransform(&self, sub_image: &CxxString) -> Result<&CxxString>;
        fn numTiles(&self, sub_image: &CxxString) -> Result<usize>;
        fn iccProfile(&self, sub_image: &CxxString) -> Result<&CxxString>;
        fn iccMatrix(&self, sub_image: &CxxString) -> Result<[f64; 9]>;
        fn imageData(&self, sub_image: &CxxString) -> Result<&CxxVector<u8>>;
        fn lossyImageCompression(&self, sub_image: &CxxString) -> Result<&CxxString>;
        fn lossyImageCompressionRatio(&self, sub_image: &CxxString) -> Result<f64>;
        fn colorLinearity(&self, sub_image: &CxxString) -> Result<&CxxString>;

        // View Properties
        fn dimensionRanges(&self, sub_image: &CxxString, level: u32) -> Result<DimensionsRange>;
        fn dimensionNames<'a>(&self, sub_image: &CxxString) -> &'a CxxVector<CxxString>;
        fn dimensionUnits<'a>(&self, sub_image: &CxxString) -> &'a CxxVector<CxxString>;
        fn dimensionTypes<'a>(&self, sub_image: &CxxString) -> &'a CxxVector<CxxString>;
        fn scale<'a>(&self, sub_image: &CxxString) -> &'a CxxVector<f64>;
        fn origin<'a>(&self, sub_image: &CxxString) -> &'a CxxVector<f64>;
        fn envelopesAsRectangles(
            &self,
            sub_image: &CxxString,
            level: u32,
        ) -> Result<Vec<Rectangle>>;
        fn bitsAllocated(&self, sub_image: &CxxString) -> u16;
        fn bitsStored(&self, sub_image: &CxxString) -> u16;
        fn highBit(&self, sub_image: &CxxString) -> u16;
        fn pixelRepresentation(&self, sub_image: &CxxString) -> Result<u16>;
        fn planarConfiguration(&self, sub_image: &CxxString) -> Result<u16>;
        fn samplesPerPixel(&self, sub_image: &CxxString) -> Result<u16>;
        fn numDerivedLevels(&self, sub_image: &CxxString) -> usize;

        // read WSI tile
        pub(crate) fn read_region(
            &self,
            request: &RegionRequest,
            buffer: &mut Vec<u8>,
            image_size: &mut Size,
        ) -> Result<()>;

    }
}

use crate::philips_slide::ffi::Rectangle;
use crate::{
    errors::PhilipsSlideError,
    philips_slide::ffi::{DimensionsRange, Size},
    ImageType, PhilipsSlide, RegionRequest, Result,
};
use cxx::let_cxx_string;
use image::RgbImage;

impl PhilipsSlide {
    pub fn new(filename: &str) -> Result<Self> {
        let_cxx_string!(filename = filename);
        Ok(PhilipsSlide {
            inner: ffi::new_(&filename)?,
        })
    }
    // SDK info
    pub fn sdk_version(&self) -> Result<String> {
        Ok(self.inner.sdkVersion().to_str()?.to_string())
    }

    pub fn containers(&self) -> impl Iterator<Item = &str> {
        self.inner
            .containers()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    pub fn container_version(&self, container: &str) -> Result<&str> {
        let_cxx_string!(container = container);
        Ok(self.inner.containerVersion(&container)?.to_str()?)
    }

    pub fn compressors(&self) -> impl Iterator<Item = &str> {
        self.inner
            .compressors()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    pub fn pixel_transforms(&self) -> impl Iterator<Item = &str> {
        self.inner
            .pixelTransforms()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    pub fn colorspace_transforms(&self) -> impl Iterator<Item = &str> {
        self.inner
            .colorspaceTransforms()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    pub fn quality_presets(&self) -> impl Iterator<Item = &str> {
        self.inner
            .qualityPresets()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    pub fn supported_filters(&self) -> impl Iterator<Item = &str> {
        self.inner
            .supportedFilters()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    // Facade info
    pub fn num_images(&self) -> Result<usize> {
        Ok(self.inner.numImages()?)
    }

    pub fn isyntax_file_version(&self) -> Result<&str> {
        Ok(self.inner.iSyntaxFileVersion()?.to_str()?)
    }

    pub fn id(&self) -> Result<&str> {
        Ok(self.inner.id()?.to_str()?)
    }

    pub fn barcode(&self) -> Result<&str> {
        Ok(self.inner.barcode()?.to_str()?)
    }

    pub fn scanner_calibration_status(&self) -> Result<&str> {
        Ok(self.inner.scannerCalibrationStatus()?.to_str()?)
    }

    pub fn software_versions(&self) -> Result<impl Iterator<Item = &str>> {
        Ok(self
            .inner
            .softwareVersions()?
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok()))
    }

    pub fn derivation_description(&self) -> Result<&str> {
        Ok(self.inner.derivationDescription()?.to_str()?)
    }

    pub fn acquisition_date_time(&self) -> Result<&str> {
        Ok(self.inner.acquisitionDateTime()?.to_str()?)
    }

    pub fn manufacturer(&self) -> Result<&str> {
        Ok(self.inner.manufacturer()?.to_str()?)
    }

    pub fn model_name(&self) -> Result<&str> {
        Ok(self.inner.modelName()?.to_str()?)
    }

    pub fn device_serial_number(&self) -> Result<&str> {
        Ok(self.inner.deviceSerialNumber()?.to_str()?)
    }

    pub fn scanner_rack_number(&self) -> Result<u16> {
        Ok(self.inner.scannerRackNumber()?)
    }

    pub fn scanner_slot_number(&self) -> Result<u16> {
        Ok(self.inner.scannerSlotNumber()?)
    }

    pub fn scanner_operator_id(&self) -> Result<&str> {
        Ok(self.inner.scannerOperatorId()?.to_str()?)
    }

    pub fn scanner_rack_priority(&self) -> Result<u16> {
        Ok(self.inner.scannerRackPriority()?)
    }

    pub fn date_of_last_calibration(&self) -> Result<impl Iterator<Item = &str>> {
        Ok(self
            .inner
            .dateOfLastCalibration()?
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok()))
    }

    pub fn time_of_last_calibration(&self) -> Result<impl Iterator<Item = &str>> {
        Ok(self
            .inner
            .timeOfLastCalibration()?
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok()))
    }

    pub fn is_philips(&self) -> Result<bool> {
        Ok(self.inner.isPhilips()?)
    }

    pub fn is_hamamatsu(&self) -> Result<bool> {
        Ok(self.inner.isHamamatsu()?)
    }

    pub fn is_ufs(&self) -> Result<bool> {
        Ok(self.inner.isUFS()?)
    }

    pub fn is_ufsb(&self) -> Result<bool> {
        Ok(self.inner.isUFSb()?)
    }

    pub fn is_uvs(&self) -> Result<bool> {
        Ok(self.inner.isUVS()?)
    }

    // Sub Image info
    pub fn pixel_transform(&self, image_type: ImageType) -> Result<&str> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.pixelTransform(&image_type)?.to_str()?)
    }

    pub fn quality_preset(&self, image_type: ImageType) -> Result<&str> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.qualityPreset(&image_type)?.to_str()?)
    }

    pub fn quality(&self, image_type: ImageType) -> Result<usize> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.quality(&image_type)?)
    }

    pub fn compressor(&self, image_type: ImageType) -> Result<&str> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.compressor(&image_type)?.to_str()?)
    }

    pub fn colorspace_transform(&self, image_type: ImageType) -> Result<&str> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.colorspaceTransform(&image_type)?.to_str()?)
    }

    pub fn num_tiles(&self, image_type: ImageType) -> Result<usize> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.numTiles(&image_type)?)
    }

    pub fn icc_profile(&self, image_type: ImageType) -> Result<&str> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.iccProfile(&image_type)?.to_str()?)
    }

    pub fn icc_matrix(&self, image_type: ImageType) -> Result<[f64; 9]> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.iccMatrix(&image_type)?)
    }

    pub fn image_data(&self, image_type: ImageType) -> Result<&[u8]> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.imageData(&image_type)?.as_slice())
    }

    pub fn lossy_image_compression(&self, image_type: ImageType) -> Result<&str> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.lossyImageCompression(&image_type)?.to_str()?)
    }

    pub fn lossy_image_compression_ratio(&self, image_type: ImageType) -> Result<f64> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.lossyImageCompressionRatio(&image_type)?)
    }

    pub fn color_linearity(&self, image_type: ImageType) -> Result<&str> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.colorLinearity(&image_type)?.to_str()?)
    }

    // VIEW
    pub fn dimension_ranges(&self, image_type: ImageType, level: u32) -> Result<DimensionsRange> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.dimensionRanges(&image_type, level)?)
    }

    pub fn dimension_names(&self, image_type: ImageType) -> impl Iterator<Item = &str> {
        let_cxx_string!(image_type = image_type);
        self.inner
            .dimensionNames(&image_type)
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    pub fn dimension_units(&self, image_type: ImageType) -> impl Iterator<Item = &str> {
        let_cxx_string!(image_type = image_type);
        self.inner
            .dimensionUnits(&image_type)
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    pub fn dimension_types(&self, image_type: ImageType) -> impl Iterator<Item = &str> {
        let_cxx_string!(image_type = image_type);
        self.inner
            .dimensionTypes(&image_type)
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    pub fn scale(&self, image_type: ImageType) -> &[f64] {
        let_cxx_string!(image_type = image_type);
        self.inner.scale(&image_type).as_slice()
    }

    pub fn origin(&self, image_type: ImageType) -> &[f64] {
        let_cxx_string!(image_type = image_type);
        self.inner.origin(&image_type).as_slice()
    }

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

    pub fn num_derived_levels(&self, image_type: ImageType) -> usize {
        let_cxx_string!(image_type = image_type);
        self.inner.numDerivedLevels(&image_type)
    }

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

    #[cfg(feature = "image")]
    pub fn read_image(&self, request: &RegionRequest) -> Result<RgbImage> {
        let (buffer, size) = self.read_region(request)?;
        let image = RgbImage::from_vec(size.w, size.h, buffer).ok_or_else(|| {
            PhilipsSlideError::ImageError("Error while creating RgbImage from buffer".to_string())
        })?;
        Ok(image)
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
