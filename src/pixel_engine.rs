//! This module contains all functions related to Philips ISyntax SDK
//! Results of theses functions should only depend on the SDK and not ISyntax file
//!

use crate::{bindings::ffi, Facade, ImageType, PhilipsEngine, Result};
use cxx::let_cxx_string;

impl PhilipsEngine {
    /// Create a new instance of PhilipsSlide
    /// May fail if the SDK cannot read the file
    pub fn new() -> Self {
        PhilipsEngine { inner: ffi::new_() }
    }

    /// Create a new instance of Facade
    /// A Facade is a reference to a Philips Engine internal object
    /// You can create multiple Facade for handling multiple files
    /// WARNING: multiple Facade handler created with the same id will points
    /// to the same reference in Philips Engine internal.
    pub fn facade(&self, id: &str) -> Result<Facade> {
        let_cxx_string!(id = id);
        Ok(Facade {
            inner: self.inner.facade(&id)?,
            _lifetime: Default::default(),
        })
    }

    /// Returns the SDK PixelEngine version
    pub fn sdk_version(&self) -> Result<String> {
        Ok(self.inner.sdkVersion().to_str()?.to_string())
    }

    /// Returns all containers supported by the SDK PixelEngine
    pub fn containers(&self) -> impl Iterator<Item = &str> {
        self.inner
            .containers()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    /// Returns the version of a container
    pub fn container_version(&self, container: &str) -> Result<&str> {
        let_cxx_string!(container = container);
        Ok(self.inner.containerVersion(&container)?.to_str()?)
    }

    /// Returns all compressors supported by the SDK PixelEngine
    pub fn compressors(&self) -> impl Iterator<Item = &str> {
        self.inner
            .compressors()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    /// Returns all pixel_transforms supported by the SDK PixelEngine
    pub fn pixel_transforms(&self) -> impl Iterator<Item = &str> {
        self.inner
            .pixelTransforms()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    /// Returns all colorspace_transforms supported by the SDK PixelEngine
    pub fn colorspace_transforms(&self) -> impl Iterator<Item = &str> {
        self.inner
            .colorspaceTransforms()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    /// Returns all quality_presets supported by the SDK PixelEngine
    pub fn quality_presets(&self) -> impl Iterator<Item = &str> {
        self.inner
            .qualityPresets()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    /// Returns all supported_filters supported by the SDK PixelEngine
    /// filters can be added to the pipeline using addFilter
    pub fn supported_filters(&self) -> impl Iterator<Item = &str> {
        self.inner
            .supportedFilters()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
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
