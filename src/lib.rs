mod bindings;
pub mod errors;
mod facade;
mod pixel_engine;
mod sub_image;
mod view;

pub type Size = bindings::ffi::Size;
pub type Rectangle = bindings::ffi::Rectangle;
pub type RegionRequest = bindings::ffi::RegionRequest;
pub type DimensionsRange = bindings::ffi::DimensionsRange;

/// The corresponding result type used by the crate.
pub type Result<T, E = errors::PhilipsSlideError> = std::result::Result<T, E>;

use cxx::UniquePtr;
use std::marker::PhantomData;

pub struct PhilipsSlide {
    inner: UniquePtr<bindings::ffi::PhilipsSlide>,
}

pub struct Facade<'a> {
    inner: UniquePtr<bindings::ffi::Facade>,
    _lifetime: PhantomData<&'a ()>, // Note: Represent PixelEngine Lifetime
}

pub struct Image<'a> {
    inner: UniquePtr<bindings::ffi::Image>,
    _lifetime: PhantomData<&'a ()>, // Note: Represent Facade Lifetime
}

pub enum ImageType {
    WSI,
    MacroImage,
    LabelImage,
}
