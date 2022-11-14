extern crate core;

mod dataenvelopes;
mod errors;
mod facade;
mod pixelengine;
mod region;
mod subimage;
mod view;

/// The corresponding result type used by the crate.
pub type Result<T, E = errors::PhilipsSlideError> = std::result::Result<T, E>;

use cxx::{SharedPtr, UniquePtr};
use std::pin::Pin;

pub struct PixelEngine {
    pe: UniquePtr<pixelengine::ffi::PixelEngine>,
    render_context: UniquePtr<pixelengine::ffi::RenderContext>,
    render_backend: UniquePtr<pixelengine::ffi::RenderBackend>,
}

pub struct DataEnvelopes<'a>(&'a dataenvelopes::ffi::DataEnvelopes);

pub struct Facade<'a>(Pin<&'a mut facade::ffi::Facade>);

pub struct SubImage<'a>(Pin<&'a mut subimage::ffi::SubImage>);

pub struct View<'a>(Pin<&'a mut view::ffi::View>);

pub struct SourceView<'a>(Pin<&'a mut view::ffi::SourceView>);

pub struct DisplayView<'a>(Pin<&'a mut view::ffi::DisplayView>);

pub struct UserView<'a>(Pin<&'a mut view::ffi::UserView>);

pub struct Region(SharedPtr<view::ffi::Region>);

pub enum ImageType {
    WSI,
    MacroImage,
    LabelImage,
}
