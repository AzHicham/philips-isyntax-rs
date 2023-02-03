//! This module contains all functions related to Philips ISyntaxFacade
//!

use crate::{Facade, Image, ImageType, Result};
use cxx::let_cxx_string;
use std::path::Path;

impl<'a> Facade<'a> {
    /// Open an ISyntax file
    pub fn open<P: AsRef<Path>>(&self, filename: P) -> Result<()> {
        let filename = filename.as_ref().display().to_string();
        Ok(self.inner.open(&filename)?)
    }

    /// Returns numbers of images in ISyntax file
    /// Should always return 3 images eg WSI, Macro, Label/ILE
    pub fn num_images(&self) -> Result<usize> {
        Ok(self.inner.numImages()?)
    }

    /// Returns the version of isyntax file
    pub fn isyntax_file_version(&self) -> Result<&str> {
        Ok(self.inner.iSyntaxFileVersion()?.to_str()?)
    }

    pub fn id(&self) -> Result<&str> {
        Ok(self.inner.id()?.to_str()?)
    }

    /// Returns the barcode in the Label/ILE image
    pub fn barcode(&self) -> Result<&str> {
        Ok(self.inner.barcode()?.to_str()?)
    }

    /// Returns the calibration status of the scanner used to create the image file
    pub fn scanner_calibration_status(&self) -> Result<&str> {
        Ok(self.inner.scannerCalibrationStatus()?.to_str()?)
    }

    /// Returns the software versions used to create the image file
    pub fn software_versions(&self) -> Result<impl Iterator<Item = &str>> {
        Ok(self
            .inner
            .softwareVersions()?
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok()))
    }

    /// Returns the derivation desription
    /// Example: "PHILIPS UFS V1.6.6063 | Quality=1 | DWT=1 | Compressor=16"
    pub fn derivation_description(&self) -> Result<&str> {
        Ok(self.inner.derivationDescription()?.to_str()?)
    }

    /// Returns the acquisition DateTime of the image file
    pub fn acquisition_date_time(&self) -> Result<&str> {
        Ok(self.inner.acquisitionDateTime()?.to_str()?)
    }

    /// Returns the scanner manufacturer used to create the image file
    pub fn manufacturer(&self) -> Result<&str> {
        Ok(self.inner.manufacturer()?.to_str()?)
    }

    /// Returns the scanner model used to create the image file
    pub fn model_name(&self) -> Result<&str> {
        Ok(self.inner.modelName()?.to_str()?)
    }

    /// Returns the scanner serial number used to create the image file
    pub fn device_serial_number(&self) -> Result<&str> {
        Ok(self.inner.deviceSerialNumber()?.to_str()?)
    }

    /// Returns the scanner rack number used to create the image file
    pub fn scanner_rack_number(&self) -> Result<u16> {
        Ok(self.inner.scannerRackNumber()?)
    }

    /// Returns the scanner slot number used to create the image file
    pub fn scanner_slot_number(&self) -> Result<u16> {
        Ok(self.inner.scannerSlotNumber()?)
    }

    pub fn scanner_operator_id(&self) -> Result<&str> {
        Ok(self.inner.scannerOperatorId()?.to_str()?)
    }

    pub fn scanner_rack_priority(&self) -> Result<u16> {
        Ok(self.inner.scannerRackPriority()?)
    }

    /// Returns the last calibration date of the scanner used to create the image file
    pub fn date_of_last_calibration(&self) -> Result<impl Iterator<Item = &str>> {
        Ok(self
            .inner
            .dateOfLastCalibration()?
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok()))
    }

    /// Returns the last calibration time of the scanner used to create the image file
    pub fn time_of_last_calibration(&self) -> Result<impl Iterator<Item = &str>> {
        Ok(self
            .inner
            .timeOfLastCalibration()?
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok()))
    }

    /// Returns true if the distributor of the image file is Philips
    pub fn is_philips(&self) -> Result<bool> {
        Ok(self.inner.isPhilips()?)
    }

    /// Returns true if the distributor of the image file is Hamamatsu
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

    pub fn image(&self, image_type: ImageType) -> Result<Image> {
        let_cxx_string!(image_type = image_type);
        Ok(Image {
            inner: self.inner.image(&image_type)?,
            _lifetime: Default::default(),
        })
    }
}
