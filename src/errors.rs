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
    /// Native Philips SDK support was not compiled into this build.
    #[error("Philips SDK support is not compiled; enable the native-sdk feature")]
    SdkUnavailable,
    /// Computed RGB buffer size overflowed usize.
    #[error(
        "RGB buffer size overflow for {width}x{height} image with {bytes_per_pixel} bytes per pixel"
    )]
    BufferSizeOverflow {
        width: u32,
        height: u32,
        bytes_per_pixel: usize,
    },
    /// Native decoder wrote a different number of bytes than the Rust side expected.
    #[error("unexpected RGB buffer size: expected {expected} bytes, got {actual} bytes")]
    UnexpectedBufferSize { expected: usize, actual: usize },
    /// Requested region coordinates are invalid for the requested level.
    #[error("invalid region: ({start_x}, {start_y}) to ({end_x}, {end_y})")]
    InvalidRegion {
        start_x: u32,
        end_x: u32,
        start_y: u32,
        end_y: u32,
    },
    /// Image dimensions must be non-zero.
    #[error("invalid image size: {width}x{height}")]
    InvalidSize { width: u32, height: u32 },
    /// Thumbnail generation would require decoding an excessively large intermediate image.
    #[error("thumbnail intermediate image is too large: {bytes} bytes exceeds limit {limit} bytes")]
    ThumbnailTooLarge { bytes: usize, limit: usize },
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
