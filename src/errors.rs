//! This module contains errors defined in this library
//!

use cxx::Exception;
use std::str::Utf8Error;
use thiserror::Error;

/// Enum defining all possible error when manipulating OpenSlide struct
#[derive(Error, Debug)]
pub enum PhilipsSlideError {
    /// CxxString to &str conversion error
    #[error(transparent)]
    StringConversionError(#[from] Utf8Error),
    /// PhilipsSlide lib error
    #[error(transparent)]
    CoreError(#[from] Exception),
    /// NullPtr Error
    #[error("Null pointer error")]
    NullPtrError,
    /// Error while creating Image from vector
    #[cfg(feature = "image")]
    #[error(transparent)]
    ImageError(#[from] ImageError),
}

#[cfg(feature = "image")]
#[derive(Error, Debug)]
pub enum ImageError {
    /// Error while creating Image from vector
    #[error(transparent)]
    Image(#[from] image::ImageError),
    /// PhilipsSlide lib error
    #[error("{0}")]
    Other(String),
}
