#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("philips-sys/cpp/pixelengine.hpp");

        pub type PixelEngine;
        pub type RenderContext;
        pub type RenderBackend;
        pub type RenderBackendInstance;
        type Facade = crate::facade::ffi::Facade;
        type Region = crate::region::ffi::Region;
        type RegionWrapper = crate::region::ffi::RegionWrapper;

        pub(crate) fn make_pixel_engine(
            render_context: &UniquePtr<RenderContext>,
            render_backend: &UniquePtr<RenderBackend>,
        ) -> UniquePtr<PixelEngine>;
        pub(crate) fn make_render_context() -> UniquePtr<RenderContext>;
        pub(crate) fn make_render_backend() -> UniquePtr<RenderBackend>;

        pub(crate) fn pe_version() -> UniquePtr<CxxString>;

        pub(crate) fn facade<'a, 'b>(
            pixel_engine: Pin<&'a mut PixelEngine>,
            name: &'b CxxString,
        ) -> Pin<&'a mut Facade>;

        pub(crate) fn containers(self: &PixelEngine) -> &CxxVector<CxxString>;
        pub(crate) fn containerVersion<'a, 'b>(
            self: &PixelEngine,
            container: &'a CxxString,
        ) -> Result<&'b CxxString>;
        pub(crate) fn compressors(self: &PixelEngine) -> &CxxVector<CxxString>;
        pub(crate) fn pixelTransforms(self: &PixelEngine) -> &CxxVector<CxxString>;
        /* pub(crate) fn blockSizes<'a>(
                    self: &PixelEngine,
                    container: &'a CxxString,
                ) -> UniquePtr<CxxVector<CxxString>>;
        */
        pub(crate) fn colorspaceTransforms(self: &PixelEngine) -> &CxxVector<CxxString>;
        pub(crate) fn qualityPresets(self: &PixelEngine) -> &CxxVector<CxxString>;
        pub(crate) fn supportedFilters(self: &PixelEngine) -> &CxxVector<CxxString>;

        pub(crate) fn waitAll(
            pixel_engine: Pin<&mut PixelEngine>,
            regions: &CxxVector<RegionWrapper>,
        );

        pub(crate) fn clearRenderTarget(
            self: Pin<&mut PixelEngine>,
            color: &CxxVector<usize>,
            target: usize,
        );
        pub(crate) fn clearRenderCache(self: Pin<&mut PixelEngine>);
        pub(crate) fn clearRenderBuffers(self: Pin<&mut PixelEngine>);

        pub(crate) fn renderBackendInstance(self: &PixelEngine)
            -> SharedPtr<RenderBackendInstance>;
    }
}

#[cfg(test)]
mod tests {
    use crate::pixelengine::ffi;
    use core::ops::DerefMut;
    use cxx::let_cxx_string;
    use std::pin::Pin;

    #[test]
    fn it_works() {
        let render_context = ffi::make_render_context();
        let render_backend = ffi::make_render_backend();
        let mut pixel_engine = ffi::make_pixel_engine(&render_context, &render_backend);
        assert_eq!(ffi::pe_version().to_str().unwrap(), "5.1.0");
        let containers = pixel_engine.containers();
        assert_eq!(
            containers
                .iter()
                .map(|cxx_str| cxx_str.to_str().unwrap())
                .collect::<Vec<&str>>(),
            vec!["ficom", "dicom", "caching-ficom", "s3", "legacy"]
        );
        let_cxx_string!(container = "ficom");
        assert_eq!(
            pixel_engine
                .containerVersion(&container)
                .unwrap()
                .to_str()
                .unwrap(),
            "100.5"
        );
        let facade_name = "in";
        let_cxx_string!(facade_name = facade_name);
        let facade = ffi::facade(pixel_engine.pin_mut(), &facade_name);
    }
}
