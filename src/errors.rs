//! This module contains errors defined in this library
//!

use cxx::Exception;
use std::str::Utf8Error;
use thiserror::Error;

/// Enum defining all possible error when manipulating OpenSlide struct
#[derive(Error, Debug, Clone)]
pub enum PhilipsSlideError {
    /// CxxSring to &str conversion error
    #[error(transparent)]
    StringConversionError(#[from] Utf8Error),
    /// PhilipsSlide lib error
    #[error("{0}")]
    CoreError(String),
    /// NullPtr Error
    #[error("Null pointer error")]
    NullPtrError,
    /// Error while creating Image from vector
    #[error("{0}")]
    ImageError(String),
}

impl From<Exception> for PhilipsSlideError {
    fn from(error: Exception) -> Self {
        PhilipsSlideError::CoreError(error.to_string())
    }
}

#[cfg(feature = "image")]
impl From<image::ImageError> for PhilipsSlideError {
    fn from(error: image::ImageError) -> Self {
        PhilipsSlideError::ImageError(error.to_string())
    }
}
