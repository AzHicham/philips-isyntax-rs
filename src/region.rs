#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("philips-sys/cpp/view.hpp");

        type Region = crate::view::ffi::Region;

        fn ready(self: &Region) -> bool;
        fn range(self: &Region) -> &CxxVector<usize>;
        fn id(self: &Region) -> usize;
        fn fill_buffer(region: SharedPtr<Region>, buffer: &mut Vec<u8>);
    }
}

use crate::{errors::PhilipsSlideError, Region, Result};

impl Region {
    pub fn ready(&self) -> Result<bool> {
        if let Some(region) = self.0.as_ref() {
            Ok(region.ready())
        } else {
            Err(PhilipsSlideError::NullPtrError)
        }
    }
    pub fn id(&self) -> Result<usize> {
        if let Some(region) = self.0.as_ref() {
            Ok(region.id())
        } else {
            Err(PhilipsSlideError::NullPtrError)
        }
    }

    pub fn range(&self) -> Result<&[usize]> {
        if let Some(region) = self.0.as_ref() {
            Ok(region.range().as_slice())
        } else {
            Err(PhilipsSlideError::NullPtrError)
        }
    }
}
