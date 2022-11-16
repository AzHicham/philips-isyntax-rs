//! This module contains the bindings to the Philips Open Pathology C++ library
//!

#[cxx::bridge]
pub(crate) mod ffi {
    /// Simple struct Size with width and height for an image/tile
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Size {
        pub w: u32,
        pub h: u32,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RegionRequest {
        pub roi: Rectangle,
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
        pub start_x: u32,
        pub end_x: u32,
        pub start_y: u32,
        pub end_y: u32,
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
