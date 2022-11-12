#[cxx::bridge]
pub mod ffi {

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Rectangle {
        pub xmin: u32,
        pub xmax: u32,
        pub ymin: u32,
        pub ymax: u32,
    }

    unsafe extern "C++" {
        include!("philips-sys/cpp/dataenvelopes.hpp");

        pub type DataEnvelopes;

        pub(crate) fn asRectangles(data_envelopes: &DataEnvelopes) -> Vec<Rectangle>;
    }
}
