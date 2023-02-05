//! This module contains all functions related to Philips ISyntaxFacade
//!

use crate::{ContainerName, Facade, Image, ImageType, Result};
use cxx::let_cxx_string;
use std::path::Path;

impl<'a> Drop for Facade<'a> {
    fn drop(&mut self) {
        if let Err(_err) = self.close() {
            // todo! log?
        };
    }
}

/// A facade is a reference to a Philips Engine internal object
/// The facade allow file manipulation & file information retrieval
/// NOTE: Philips Engine and all internal objects are not thread safe
impl<'a> Facade<'a> {
    /// Open an ISyntax file through a facade
    pub(crate) fn open<P: AsRef<Path>>(
        &self,
        filename: P,
        container: &ContainerName,
    ) -> Result<()> {
        let filename = filename.as_ref().display().to_string();
        Ok(self.inner.open(&filename, container.as_str())?)
    }

    // close close hold by the facade
    // WARNING: Do not call this function if the facade was not "opened" or already closed
    // this will cause a SIGSEGV
    fn close(&self) -> Result<()> {
        Ok(self.inner.close()?)
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

    /// Return the id of the facade
    /// See also PhilipsEngine::facade
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

    /// Returns the scanner operator id used to create the image file
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

    /// Returns true if the file was created by Philips Ultra Fast Scanner
    pub fn is_ufs(&self) -> Result<bool> {
        Ok(self.inner.isUFS()?)
    }

    pub fn is_ufsb(&self) -> Result<bool> {
        Ok(self.inner.isUFSb()?)
    }

    pub fn is_uvs(&self) -> Result<bool> {
        Ok(self.inner.isUVS()?)
    }

    /// Create a new instance of Image
    /// An Image is a reference to a Philips Engine internal object
    /// You can create multiple Image handler for every possible ImageType
    /// WARNING: multiple Image handler created with the same image_type will points
    /// to the same reference in Philips Engine internal.
    pub fn image(&self, image_type: &ImageType) -> Result<Image> {
        let_cxx_string!(image_type = image_type);
        Ok(Image {
            inner: self.inner.image(&image_type)?,
            _lifetime: Default::default(),
        })
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
