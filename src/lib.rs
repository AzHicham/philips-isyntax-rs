mod bindings;
pub mod errors;
#[cfg(feature = "native-sdk")]
mod facade;
#[cfg(feature = "native-sdk")]
mod pixel_engine;
#[cfg(not(feature = "native-sdk"))]
mod stub;
#[cfg(feature = "native-sdk")]
mod sub_image;
#[cfg(feature = "image")]
mod utils;
#[cfg(feature = "native-sdk")]
mod view;

pub type Size = bindings::ffi::Size;
pub type Rectangle = bindings::ffi::Rectangle;
pub type RegionRequest = bindings::ffi::RegionRequest;
pub type DimensionsRange = bindings::ffi::DimensionsRange;
pub type ViewOptions = bindings::ffi::ViewOptions;

/// The corresponding result type used by the crate.
pub type Result<T, E = errors::PhilipsSlideError> = std::result::Result<T, E>;

#[cfg(feature = "native-sdk")]
use cxx::UniquePtr;
use std::marker::PhantomData;

#[cfg(feature = "native-sdk")]
type EngineInner = UniquePtr<bindings::ffi::PhilipsEngine>;
#[cfg(not(feature = "native-sdk"))]
type EngineInner = bindings::ffi::PhilipsEngine;

#[cfg(feature = "native-sdk")]
type FacadeInner = UniquePtr<bindings::ffi::Facade>;
#[cfg(not(feature = "native-sdk"))]
type FacadeInner = bindings::ffi::Facade;

#[cfg(feature = "native-sdk")]
type ImageInner = UniquePtr<bindings::ffi::Image>;
#[cfg(not(feature = "native-sdk"))]
type ImageInner = bindings::ffi::Image;

#[cfg(feature = "native-sdk")]
type ViewInner = UniquePtr<bindings::ffi::ImageView>;
#[cfg(not(feature = "native-sdk"))]
type ViewInner = bindings::ffi::ImageView;

#[cfg_attr(not(feature = "native-sdk"), allow(dead_code))]
pub struct PhilipsEngine {
    inner: EngineInner,
}

#[cfg_attr(not(feature = "native-sdk"), allow(dead_code))]
pub struct Facade<'a> {
    inner: FacadeInner,
    _lifetime: PhantomData<&'a ()>, // Note: Represent PixelEngine Lifetime
}

#[cfg_attr(not(feature = "native-sdk"), allow(dead_code))]
pub struct Image<'a> {
    inner: ImageInner,
    _lifetime: PhantomData<&'a ()>, // Note: Represent Facade Lifetime
}

#[cfg_attr(not(feature = "native-sdk"), allow(dead_code))]
pub struct View<'a> {
    inner: ViewInner,
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
