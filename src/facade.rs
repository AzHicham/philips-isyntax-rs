#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("philips-isyntax-rs/cpp/facade.hpp");

        pub type Facade;
        type SubImage = crate::subimage::ffi::SubImage;

        fn open(
            facade: Pin<&mut Facade>,
            url: &CxxString,
            container_name: &CxxString,
            cache_name: &CxxString,
        ) -> Result<()>;
        fn close(self: Pin<&mut Facade>) -> usize;
        fn abort(self: Pin<&mut Facade>);
        fn remainingPixelsToEncode(self: &Facade) -> Result<usize>;

        fn numImages(self: &Facade) -> Result<usize>;
        fn sub_image<'a, 'b>(
            facade: Pin<&mut Facade>,
            image_type: &'b CxxString,
        ) -> Result<Pin<&'a mut SubImage>>;

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
    }
}

use crate::{Facade, ImageType, Result, SubImage};
use cxx::let_cxx_string;

impl<'a> Facade<'a> {
    pub fn open(&mut self, url: &str, container_name: &str, cache_name: &str) -> Result<()> {
        let_cxx_string!(url = url);
        let_cxx_string!(container_name = container_name);
        let_cxx_string!(cache_name = cache_name);
        Ok(ffi::open(
            self.0.as_mut(),
            &url,
            &container_name,
            &cache_name,
        )?)
    }

    pub fn close(&mut self) -> usize {
        self.0.as_mut().close()
    }

    pub fn abort(&mut self) {
        self.0.as_mut().abort()
    }

    pub fn remaining_pixels_to_encode(&self) -> Result<usize> {
        Ok(self.0.remainingPixelsToEncode()?)
    }

    pub fn num_images(&self) -> Result<usize> {
        Ok(self.0.numImages()?)
    }

    pub fn sub_image(&mut self, image_type: ImageType) -> Result<SubImage> {
        let image_type = image_type.as_str();
        let_cxx_string!(image_type = image_type);
        Ok(SubImage(ffi::sub_image(self.0.as_mut(), &image_type)?))
    }

    pub fn isyntax_file_version(&self) -> Result<&str> {
        Ok(self.0.iSyntaxFileVersion()?.to_str()?)
    }

    pub fn id(&self) -> Result<&str> {
        Ok(self.0.id()?.to_str()?)
    }

    pub fn barcode(&self) -> Result<&str> {
        Ok(self.0.barcode()?.to_str()?)
    }

    pub fn scanner_calibration_status(&self) -> Result<&str> {
        Ok(self.0.scannerCalibrationStatus()?.to_str()?)
    }

    pub fn software_versions(&self) -> Result<impl Iterator<Item = &str>> {
        Ok(self
            .0
            .softwareVersions()?
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok()))
    }

    pub fn derivation_description(&self) -> Result<&str> {
        Ok(self.0.derivationDescription()?.to_str()?)
    }

    pub fn acquisition_date_time(&self) -> Result<&str> {
        Ok(self.0.acquisitionDateTime()?.to_str()?)
    }

    pub fn manufacturer(&self) -> Result<&str> {
        Ok(self.0.manufacturer()?.to_str()?)
    }

    pub fn model_name(&self) -> Result<&str> {
        Ok(self.0.modelName()?.to_str()?)
    }

    pub fn device_serial_number(&self) -> Result<&str> {
        Ok(self.0.deviceSerialNumber()?.to_str()?)
    }

    pub fn scanner_rack_number(&self) -> Result<u16> {
        Ok(self.0.scannerRackNumber()?)
    }

    pub fn scanner_slot_number(&self) -> Result<u16> {
        Ok(self.0.scannerSlotNumber()?)
    }

    pub fn scanner_operator_id(&self) -> Result<&str> {
        Ok(self.0.scannerOperatorId()?.to_str()?)
    }

    pub fn scanner_rack_priority(&self) -> Result<u16> {
        Ok(self.0.scannerRackPriority()?)
    }

    pub fn date_of_last_calibration(&self) -> Result<impl Iterator<Item = &str>> {
        Ok(self
            .0
            .dateOfLastCalibration()?
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok()))
    }

    pub fn time_of_last_calibration(&self) -> Result<impl Iterator<Item = &str>> {
        Ok(self
            .0
            .timeOfLastCalibration()?
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok()))
    }

    pub fn is_philips(&self) -> Result<bool> {
        Ok(self.0.isPhilips()?)
    }

    pub fn is_hamamatsu(&self) -> Result<bool> {
        Ok(self.0.isHamamatsu()?)
    }

    pub fn is_ufs(&self) -> Result<bool> {
        Ok(self.0.isUFS()?)
    }
    pub fn is_ufsb(&self) -> Result<bool> {
        Ok(self.0.isUFSb()?)
    }

    pub fn is_uvs(&self) -> Result<bool> {
        Ok(self.0.isUVS()?)
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
