//! This module contains the bindings to the Philips Open Pathology C++ library
//!

use crate::errors::DimensionsRangeToSizeError;

#[cfg(feature = "native-sdk")]
#[path = "native_bridge.rs"]
mod native_bridge;

#[cfg(feature = "native-sdk")]
pub(crate) use native_bridge::ffi;

#[cfg(not(feature = "native-sdk"))]
pub(crate) mod ffi {
    /// Simple struct Size with width and height for an image/tile
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Size {
        pub w: u32,
        pub h: u32,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RegionRequest {
        pub roi: Rectangle,
        pub level: u32,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct DimensionsRange {
        pub start_x: u32,
        pub step_x: u32,
        pub end_x: u32,
        pub start_y: u32,
        pub step_y: u32,
        pub end_y: u32,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Rectangle {
        pub start_x: u32,
        pub end_x: u32,
        pub start_y: u32,
        pub end_y: u32,
    }

    /// Options used when constructing a view and rendering regions from it.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ViewOptions {
        pub apply_color_correction: bool,
        pub background_r: u8,
        pub background_g: u8,
        pub background_b: u8,
    }

    pub struct PhilipsEngine;
    pub struct Facade;
    pub struct Image;
    pub struct ImageView;

    pub fn new_() -> PhilipsEngine {
        PhilipsEngine
    }
}

impl ffi::Size {
    pub fn new(w: u32, h: u32) -> Self {
        Self { w, h }
    }
}

impl Default for ffi::ViewOptions {
    fn default() -> Self {
        Self {
            apply_color_correction: true,
            background_r: 254,
            background_g: 254,
            background_b: 254,
        }
    }
}

impl ffi::ViewOptions {
    pub fn new(
        apply_color_correction: bool,
        background_r: u8,
        background_g: u8,
        background_b: u8,
    ) -> Self {
        Self {
            apply_color_correction,
            background_r,
            background_g,
            background_b,
        }
    }
}
impl TryFrom<&ffi::DimensionsRange> for ffi::Size {
    type Error = DimensionsRangeToSizeError;

    fn try_from(value: &ffi::DimensionsRange) -> Result<Self, Self::Error> {
        if value.step_x == 0 {
            return Err(DimensionsRangeToSizeError::NullStepX);
        }
        if value.step_y == 0 {
            return Err(DimensionsRangeToSizeError::NullStepY);
        }
        if let Some(width) = value.end_x.checked_sub(value.start_x) {
            if let Some(height) = value.end_y.checked_sub(value.start_y) {
                Ok(Self {
                    w: width / value.step_x,
                    h: height / value.step_y,
                })
            } else {
                Err(DimensionsRangeToSizeError::NegativeHeight)
            }
        } else {
            Err(DimensionsRangeToSizeError::NegativeWidth)
        }
    }
}
