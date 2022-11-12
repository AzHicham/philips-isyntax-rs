#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("philips-sys/cpp/subimage.hpp");

        pub type SubImage;
        type SourceView = crate::view::ffi::SourceView;
        type DisplayView = crate::view::ffi::DisplayView;
        type UserView = crate::view::ffi::UserView;

        pub(crate) fn hasDisplayView(self: &SubImage) -> bool;
        pub(crate) fn sourceView(self: Pin<&mut SubImage>) -> Pin<&mut SourceView>;
        pub(crate) fn displayView(self: Pin<&mut SubImage>) -> Pin<&mut DisplayView>;
        pub(crate) fn addView(self: Pin<&mut SubImage>) -> Pin<&mut UserView>;

        pub(crate) fn imageType(self: &SubImage) -> &CxxString;
        pub(crate) fn pixelTransform(self: &SubImage) -> &CxxString;
        pub(crate) fn qualityPreset(self: &SubImage) -> &CxxString;
        pub(crate) fn quality(self: &SubImage) -> usize;
        pub(crate) fn compressor(self: &SubImage) -> &CxxString;
        pub(crate) fn colorspaceTransform(self: &SubImage) -> &CxxString;
        pub(crate) fn numTiles(self: &SubImage) -> usize;
        pub(crate) fn iccProfile(self: &SubImage) -> &CxxString;
        pub(crate) fn imageData(self: &SubImage) -> &CxxVector<u8>;
        pub(crate) fn lossyImageCompression(self: &SubImage) -> &CxxString;
        pub(crate) fn lossyImageCompressionRatio(self: &SubImage) -> f64;
        pub(crate) fn colorLinearity(self: &SubImage) -> &CxxString;
    }
}
