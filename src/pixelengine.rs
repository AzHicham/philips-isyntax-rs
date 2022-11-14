#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("philips-isyntax-rs/cpp/pixelengine.hpp");

        pub type PixelEngine;
        pub type RenderContext;
        pub type RenderBackend;
        pub type RenderBackendInstance;
        type Facade = crate::facade::ffi::Facade;

        fn make_pixel_engine(
            render_context: &UniquePtr<RenderContext>,
            render_backend: &UniquePtr<RenderBackend>,
        ) -> UniquePtr<PixelEngine>;
        fn make_render_context() -> UniquePtr<RenderContext>;
        fn make_render_backend() -> UniquePtr<RenderBackend>;
        fn version() -> UniquePtr<CxxString>;
        fn facade<'a, 'b>(
            pixel_engine: Pin<&'a mut PixelEngine>,
            name: &'b CxxString,
        ) -> Pin<&'a mut Facade>;
        fn containers(self: &PixelEngine) -> &CxxVector<CxxString>;
        fn containerVersion<'a, 'b>(
            self: &PixelEngine,
            container: &'a CxxString,
        ) -> Result<&'b CxxString>;
        fn compressors(self: &PixelEngine) -> &CxxVector<CxxString>;
        fn pixelTransforms(self: &PixelEngine) -> &CxxVector<CxxString>;
        fn colorspaceTransforms(self: &PixelEngine) -> &CxxVector<CxxString>;
        fn qualityPresets(self: &PixelEngine) -> &CxxVector<CxxString>;
        fn supportedFilters(self: &PixelEngine) -> &CxxVector<CxxString>;
        fn clearRenderCache(self: Pin<&mut PixelEngine>);
        fn clearRenderBuffers(self: Pin<&mut PixelEngine>);
    }
}

use crate::{Facade, PixelEngine, Result};
use cxx::let_cxx_string;

impl PixelEngine {
    pub fn new() -> Self {
        let _render_context = ffi::make_render_context();
        let _render_backend = ffi::make_render_backend();
        PixelEngine {
            pe: ffi::make_pixel_engine(&_render_context, &_render_backend),
            _render_context,
            _render_backend,
        }
    }

    pub fn version() -> Result<String> {
        let version = ffi::version();
        Ok(version.to_str()?.to_string())
    }

    pub fn facade(&mut self, name: &str) -> Facade {
        let_cxx_string!(name = name);
        Facade(ffi::facade(self.pe.pin_mut(), &name))
    }

    pub fn containers(&self) -> impl Iterator<Item = &str> {
        self.pe
            .containers()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    pub fn container_version(&self, container: &str) -> Result<&str> {
        let_cxx_string!(container = container);
        Ok(self.pe.containerVersion(&container)?.to_str()?)
    }

    pub fn compressors(&self) -> impl Iterator<Item = &str> {
        self.pe
            .compressors()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    pub fn pixel_transforms(&self) -> impl Iterator<Item = &str> {
        self.pe
            .pixelTransforms()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    pub fn colorspace_transforms(&self) -> impl Iterator<Item = &str> {
        self.pe
            .colorspaceTransforms()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    pub fn quality_presets(&self) -> impl Iterator<Item = &str> {
        self.pe
            .qualityPresets()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    pub fn supported_filters(&self) -> impl Iterator<Item = &str> {
        self.pe
            .supportedFilters()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    pub fn clear_render_cache(&mut self) {
        self.pe.pin_mut().clearRenderCache()
    }

    pub fn clear_render_buffers(&mut self) {
        self.pe.pin_mut().clearRenderBuffers()
    }
}
