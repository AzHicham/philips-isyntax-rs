#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("philips-sys/cpp/region.hpp");

        pub type Region;
        pub type RegionWrapper;

        pub(crate) fn ready(self: &Region) -> bool;
        pub(crate) fn range(self: &Region) -> &CxxVector<usize>;
        pub(crate) fn id(self: &Region) -> usize;
        pub(crate) fn draw(self: Pin<&mut Region>, target: usize);
    }
}
