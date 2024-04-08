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

pub struct PhilipsEngine {
    inner: UniquePtr<bindings::ffi::PhilipsEngine>,
}

pub struct Facade<'a> {
    inner: UniquePtr<bindings::ffi::Facade>,
    _lifetime: PhantomData<&'a ()>, // Note: Represent PixelEngine Lifetime
}

pub struct Image<'a> {
    inner: UniquePtr<bindings::ffi::Image>,
    _lifetime: PhantomData<&'a ()>, // Note: Represent Facade Lifetime
}

pub struct View<'a> {
    inner: UniquePtr<bindings::ffi::ImageView>,
    _lifetime: PhantomData<&'a ()>, // Note: Represent Image Lifetime
}

#[derive(Debug, Clone)]
pub enum ImageType {
    WSI,
    MacroImage,
    LabelImage,
}

#[derive(Debug, Clone)]
pub enum ContainerName {
    Default,
    Ficom,
    Dicom,
    CachingFicom,
    S3,
    Legacy,
}
