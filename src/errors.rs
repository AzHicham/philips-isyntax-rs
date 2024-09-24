//! This module contains errors defined in this library
//!

use cxx::Exception;
use std::str::Utf8Error;
use thiserror::Error;

/// Enum defining all possible error when manipulating Philips struct
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
    #[error(transparent)]
    DimensionsRangeToSizeError(#[from] DimensionsRangeToSizeError),
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

#[derive(Error, Debug)]
pub enum DimensionsRangeToSizeError {
    #[error("Step X is null")]
    NullStepX,
    #[error("Step Y is null")]
    NullStepY,
    #[error("End X is smaller than Start X")]
    NegativeWidth,
    #[error("End Y is smaller than Start Y")]
    NegativeHeight,
}
