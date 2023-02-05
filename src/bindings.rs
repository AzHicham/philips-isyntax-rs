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

    extern "Rust" {
        fn println(str: String);
    }

    unsafe extern "C++" {
        include!("philips-isyntax-rs/cpp/philipsslide.hpp");

        pub type PhilipsEngine;
        pub type Facade;
        pub type Image;
        pub type ImageView;

        // Pixel Engine
        fn new_() -> UniquePtr<PhilipsEngine>;
        fn containers(self: &PhilipsEngine) -> &CxxVector<CxxString>;
        fn sdkVersion(self: &PhilipsEngine) -> &CxxString;
        fn containerVersion(self: &PhilipsEngine, container: &CxxString) -> Result<&CxxString>;
        fn compressors(self: &PhilipsEngine) -> &CxxVector<CxxString>;
        fn pixelTransforms(self: &PhilipsEngine) -> &CxxVector<CxxString>;
        fn colorspaceTransforms(self: &PhilipsEngine) -> &CxxVector<CxxString>;
        fn qualityPresets(self: &PhilipsEngine) -> &CxxVector<CxxString>;
        fn supportedFilters(self: &PhilipsEngine) -> &CxxVector<CxxString>;
        fn facade(self: &PhilipsEngine, input: &CxxString) -> Result<UniquePtr<Facade>>;

        // Facade properties
        fn open(self: &Facade, url: &str, container: &str) -> Result<()>;
        fn close(self: &Facade) -> Result<()>;
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
        fn image(self: &Facade, image_type: &CxxString) -> Result<UniquePtr<Image>>;

        // Image properties
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
        fn view(self: &Image) -> Result<UniquePtr<ImageView>>;

        // View properties
        fn dimensionRanges(self: &ImageView, level: u32) -> Result<DimensionsRange>;
        fn dimensionNames<'a>(self: &ImageView) -> &'a CxxVector<CxxString>;
        fn dimensionUnits<'a>(self: &ImageView) -> &'a CxxVector<CxxString>;
        fn dimensionTypes<'a>(self: &ImageView) -> &'a CxxVector<CxxString>;
        fn scale<'a>(self: &ImageView) -> &'a CxxVector<f64>;
        fn origin<'a>(self: &ImageView) -> &'a CxxVector<f64>;
        fn bitsAllocated(self: &ImageView) -> u16;
        fn bitsStored(self: &ImageView) -> u16;
        fn highBit(self: &ImageView) -> u16;
        fn pixelRepresentation(self: &ImageView) -> Result<u16>;
        fn planarConfiguration(self: &ImageView) -> Result<u16>;
        fn samplesPerPixel(self: &ImageView) -> Result<u16>;
        fn numDerivedLevels(self: &ImageView) -> u32;
        fn envelopesAsRects(self: &ImageView, level: u32) -> Result<Vec<Rectangle>>;
        fn read_region(
            self: &ImageView,
            engine: &UniquePtr<PhilipsEngine>,
            request: &RegionRequest,
            buffer: &mut Vec<u8>,
            image_size: &mut Size,
        ) -> Result<()>;
    }
}

fn println(str: String) {
    println!("{str}");
}

unsafe impl Send for ffi::PhilipsEngine {}
unsafe impl Send for ffi::Facade {}
unsafe impl Send for ffi::Image {}
unsafe impl Send for ffi::ImageView {}
