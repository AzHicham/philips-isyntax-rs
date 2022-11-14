#[cxx::bridge]
pub(crate) mod ffi {

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Rectangle {
        pub x_min: u32,
        pub x_max: u32,
        pub y_min: u32,
        pub y_max: u32,
    }

    unsafe extern "C++" {
        include!("philips-isyntax-rs/cpp/dataenvelopes.hpp");

        pub type DataEnvelopes;

        fn as_rectangles(data_envelopes: &DataEnvelopes) -> Vec<Rectangle>;
    }
}

use crate::DataEnvelopes;

impl<'a> DataEnvelopes<'a> {
    pub fn as_rectangles(&self) -> Vec<ffi::Rectangle> {
        ffi::as_rectangles(self.0)
    }
}
