extern crate core;

use cxx::UniquePtr;

pub mod errors;
mod philips_slide;

pub type Rectangle = philips_slide::ffi::Rectangle;
pub type RegionRequest = philips_slide::ffi::RegionRequest;
pub type DimensionsRange = philips_slide::ffi::DimensionsRange;

/// The corresponding result type used by the crate.
pub type Result<T, E = errors::PhilipsSlideError> = std::result::Result<T, E>;

pub struct PhilipsSlide {
    inner: UniquePtr<philips_slide::ffi::PhilipsSlide>,
}

pub enum ImageType {
    WSI,
    MacroImage,
    LabelImage,
}
