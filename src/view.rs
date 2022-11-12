#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("philips-sys/cpp/view.hpp");

        pub type View;
        pub type SourceView;
        pub type DisplayView;
        pub type UserView;

        pub(crate) fn addChainedView(self: Pin<&mut View>) -> Pin<&mut UserView>;

        pub(crate) fn dimensionNames(self: &View) -> &CxxVector<CxxString>;
        pub(crate) fn dimensionUnits(self: &View) -> &CxxVector<CxxString>;
        pub(crate) fn dimensionTypes(self: &View) -> &CxxVector<CxxString>;
        pub(crate) fn scale(self: &View) -> &CxxVector<f64>;
        pub(crate) fn origin(self: &View) -> &CxxVector<f64>;
        pub(crate) fn bitsAllocated(self: &View) -> u16;
        pub(crate) fn bitsStored(self: &View) -> u16;
        pub(crate) fn highBit(self: &View) -> u16;
        pub(crate) fn pixelRepresentation(self: &View) -> u16;
        pub(crate) fn planarConfiguration(self: &View) -> u16;
        pub(crate) fn samplesPerPixel(self: &View) -> u16;
        pub(crate) fn id(self: &View) -> usize;
        pub(crate) fn numDerivedLevels(self: &View) -> usize;

    }
}
