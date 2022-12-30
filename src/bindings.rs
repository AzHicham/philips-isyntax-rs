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
        pub type Facade;
        pub type Image;

        fn new_(url: &str) -> Result<UniquePtr<PhilipsSlide>>;

        // SDK properties
        fn containers(self: &PhilipsSlide) -> &CxxVector<CxxString>;
        fn sdkVersion(self: &PhilipsSlide) -> &CxxString;
        fn containerVersion(self: &PhilipsSlide, container: &CxxString) -> Result<&CxxString>;
        fn compressors(self: &PhilipsSlide) -> &CxxVector<CxxString>;
        fn pixelTransforms(self: &PhilipsSlide) -> &CxxVector<CxxString>;
        fn colorspaceTransforms(self: &PhilipsSlide) -> &CxxVector<CxxString>;
        fn qualityPresets(self: &PhilipsSlide) -> &CxxVector<CxxString>;
        fn supportedFilters(self: &PhilipsSlide) -> &CxxVector<CxxString>;

        fn facade(self: &PhilipsSlide, input: &CxxString) -> Result<UniquePtr<Facade>>;

        // File properties
        fn numImages(self: &PhilipsSlide) -> Result<usize>;
        fn iSyntaxFileVersion(self: &PhilipsSlide) -> Result<&CxxString>;
        fn id(self: &PhilipsSlide) -> Result<&CxxString>;
        fn barcode(self: &PhilipsSlide) -> Result<&CxxString>;
        fn scannerCalibrationStatus(self: &PhilipsSlide) -> Result<&CxxString>;
        fn softwareVersions(self: &PhilipsSlide) -> Result<&CxxVector<CxxString>>;
        fn derivationDescription(self: &PhilipsSlide) -> Result<&CxxString>;
        fn acquisitionDateTime(self: &PhilipsSlide) -> Result<&CxxString>;
        fn manufacturer(self: &PhilipsSlide) -> Result<&CxxString>;
        fn modelName(self: &PhilipsSlide) -> Result<&CxxString>;
        fn deviceSerialNumber(self: &PhilipsSlide) -> Result<&CxxString>;
        fn scannerRackNumber(self: &PhilipsSlide) -> Result<u16>;
        fn scannerSlotNumber(self: &PhilipsSlide) -> Result<u16>;
        fn scannerOperatorId(self: &PhilipsSlide) -> Result<&CxxString>;
        fn scannerRackPriority(self: &PhilipsSlide) -> Result<u16>;
        fn dateOfLastCalibration(self: &PhilipsSlide) -> Result<&CxxVector<CxxString>>;
        fn timeOfLastCalibration(self: &PhilipsSlide) -> Result<&CxxVector<CxxString>>;
        fn isPhilips(self: &PhilipsSlide) -> Result<bool>;
        fn isHamamatsu(self: &PhilipsSlide) -> Result<bool>;
        fn isUFS(self: &PhilipsSlide) -> Result<bool>;
        fn isUFSb(self: &PhilipsSlide) -> Result<bool>;
        fn isUVS(self: &PhilipsSlide) -> Result<bool>;

        // File properties
        fn open(self: &Facade, url: &str) -> Result<()>;
        fn numImages(self: &Facade) -> Result<usize>;
        fn iSyntaxFileVersion(self: &Facade) -> Result<&CxxString>;
        fn id(self: &Facade) -> Result<&CxxString>;
        fn barcode(self: &Facade) -> Result<&CxxString>;
        fn scannerCalibrationStatus(self: &Facade) -> Result<&CxxString>;
        fn softwareVersions(self: &Facade) -> Result<&CxxVector<CxxString>>;
        fn derivationDescription(self: &Facade) -> Result<&CxxString>;
        fn acquisitionDateTime(self: &Facade) -> Result<&CxxString>;
        fn manufacturer(self: &Facade) -> Result<&CxxString>;
        fn modelName(self: &Facade) -> Result<&CxxString>;
        fn deviceSerialNumber(self: &Facade) -> Result<&CxxString>;
        fn scannerRackNumber(self: &Facade) -> Result<u16>;
        fn scannerSlotNumber(self: &Facade) -> Result<u16>;
        fn scannerOperatorId(self: &Facade) -> Result<&CxxString>;
        fn scannerRackPriority(self: &Facade) -> Result<u16>;
        fn dateOfLastCalibration(self: &Facade) -> Result<&CxxVector<CxxString>>;
        fn timeOfLastCalibration(self: &Facade) -> Result<&CxxVector<CxxString>>;
        fn isPhilips(self: &Facade) -> Result<bool>;
        fn isHamamatsu(self: &Facade) -> Result<bool>;
        fn isUFS(self: &Facade) -> Result<bool>;
        fn isUFSb(self: &Facade) -> Result<bool>;
        fn isUVS(self: &Facade) -> Result<bool>;
        fn sub_image(self: &Facade, image_type: &CxxString) -> Result<UniquePtr<Image>>;

        // Sub Image properties
        fn pixelTransform(self: &PhilipsSlide, sub_image: &CxxString) -> Result<&CxxString>;
        fn qualityPreset(self: &PhilipsSlide, sub_image: &CxxString) -> Result<&CxxString>;
        fn quality(self: &PhilipsSlide, sub_image: &CxxString) -> Result<usize>;
        fn compressor(self: &PhilipsSlide, sub_image: &CxxString) -> Result<&CxxString>;
        fn colorspaceTransform(self: &PhilipsSlide, sub_image: &CxxString) -> Result<&CxxString>;
        fn numTiles(self: &PhilipsSlide, sub_image: &CxxString) -> Result<usize>;
        fn iccProfile(self: &PhilipsSlide, sub_image: &CxxString) -> Result<&CxxString>;
        fn iccMatrix(self: &PhilipsSlide, sub_image: &CxxString) -> Result<[f64; 9]>;
        fn imageData(self: &PhilipsSlide, sub_image: &CxxString) -> Result<&CxxVector<u8>>;
        fn lossyImageCompression(self: &PhilipsSlide, sub_image: &CxxString) -> Result<&CxxString>;
        fn lossyImageCompressionRatio(self: &PhilipsSlide, sub_image: &CxxString) -> Result<f64>;
        fn colorLinearity(self: &PhilipsSlide, sub_image: &CxxString) -> Result<&CxxString>;

        // Sub Image properties
        fn pixelTransform(self: &Image) -> Result<&CxxString>;
        fn qualityPreset(self: &Image) -> Result<&CxxString>;
        fn quality(self: &Image) -> Result<usize>;
        fn compressor(self: &Image) -> Result<&CxxString>;
        fn colorspaceTransform(self: &Image) -> Result<&CxxString>;
        fn numTiles(self: &Image) -> Result<usize>;
        fn iccProfile(self: &Image) -> Result<&CxxString>;
        fn iccMatrix(self: &Image) -> Result<[f64; 9]>;
        fn imageData(self: &Image) -> Result<&CxxVector<u8>>;
        fn lossyImageCompression(self: &Image) -> Result<&CxxString>;
        fn lossyImageCompressionRatio(self: &Image) -> Result<f64>;
        fn colorLinearity(self: &Image) -> Result<&CxxString>;

        // View Properties
        fn dimensionRanges(
            self: &PhilipsSlide,
            sub_image: &CxxString,
            level: u32,
        ) -> Result<DimensionsRange>;
        fn dimensionNames<'a>(
            self: &PhilipsSlide,
            sub_image: &CxxString,
        ) -> &'a CxxVector<CxxString>;
        fn dimensionUnits<'a>(
            self: &PhilipsSlide,
            sub_image: &CxxString,
        ) -> &'a CxxVector<CxxString>;
        fn dimensionTypes<'a>(
            self: &PhilipsSlide,
            sub_image: &CxxString,
        ) -> &'a CxxVector<CxxString>;
        fn scale<'a>(self: &PhilipsSlide, sub_image: &CxxString) -> &'a CxxVector<f64>;
        fn origin<'a>(self: &PhilipsSlide, sub_image: &CxxString) -> &'a CxxVector<f64>;
        fn envelopesAsRectangles(
            self: &PhilipsSlide,
            sub_image: &CxxString,
            level: u32,
        ) -> Result<Vec<Rectangle>>;
        fn bitsAllocated(self: &PhilipsSlide, sub_image: &CxxString) -> u16;
        fn bitsStored(self: &PhilipsSlide, sub_image: &CxxString) -> u16;
        fn highBit(self: &PhilipsSlide, sub_image: &CxxString) -> u16;
        fn pixelRepresentation(self: &PhilipsSlide, sub_image: &CxxString) -> Result<u16>;
        fn planarConfiguration(self: &PhilipsSlide, sub_image: &CxxString) -> Result<u16>;
        fn samplesPerPixel(self: &PhilipsSlide, sub_image: &CxxString) -> Result<u16>;
        fn numDerivedLevels(self: &PhilipsSlide, sub_image: &CxxString) -> u32;

        // read WSI tile
        pub(crate) fn read_region(
            self: &PhilipsSlide,
            request: &RegionRequest,
            buffer: &mut Vec<u8>,
            image_size: &mut Size,
        ) -> Result<()>;

    }
}

unsafe impl Send for ffi::PhilipsSlide {}
