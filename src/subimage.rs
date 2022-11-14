#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("philips-isyntax-rs/cpp/subimage.hpp");

        pub type SubImage;
        type SourceView = crate::view::ffi::SourceView;
        type DisplayView = crate::view::ffi::DisplayView;
        type UserView = crate::view::ffi::UserView;

        fn hasDisplayView(self: &SubImage) -> bool;
        fn sourceView(self: Pin<&mut SubImage>) -> Pin<&mut SourceView>;
        fn displayView(self: Pin<&mut SubImage>) -> Pin<&mut DisplayView>;
        fn addView(self: Pin<&mut SubImage>) -> Pin<&mut UserView>;

        fn imageType(self: &SubImage) -> &CxxString;
        fn pixelTransform(self: &SubImage) -> &CxxString;
        fn qualityPreset(self: &SubImage) -> &CxxString;
        fn quality(self: &SubImage) -> usize;
        fn compressor(self: &SubImage) -> &CxxString;
        fn colorspaceTransform(self: &SubImage) -> &CxxString;
        fn numTiles(self: &SubImage) -> usize;
        fn iccProfile(self: &SubImage) -> &CxxString;
        fn iccMatrix(self: &SubImage) -> [f64; 9];
        fn imageData(self: &SubImage) -> &CxxVector<u8>;
        fn lossyImageCompression(self: &SubImage) -> &CxxString;
        fn lossyImageCompressionRatio(self: &SubImage) -> f64;
        fn colorLinearity(self: &SubImage) -> &CxxString;
    }
}

use crate::{DisplayView, Result, SourceView, SubImage, UserView};

impl<'a> SubImage<'a> {
    pub fn source_view(&mut self) -> SourceView {
        SourceView(self.0.as_mut().sourceView())
    }
    pub fn display_view(&mut self) -> DisplayView {
        DisplayView(self.0.as_mut().displayView())
    }
    pub fn add_view(&mut self) -> UserView {
        UserView(self.0.as_mut().addView())
    }
    pub fn has_display_view(&self) -> bool {
        self.0.hasDisplayView()
    }
    pub fn image_type(&self) -> Result<&str> {
        Ok(self.0.imageType().to_str()?)
    }
    pub fn pixel_transform(&self) -> Result<&str> {
        Ok(self.0.pixelTransform().to_str()?)
    }
    pub fn quality_preset(&self) -> Result<&str> {
        Ok(self.0.qualityPreset().to_str()?)
    }
    pub fn quality(&self) -> usize {
        self.0.quality()
    }
    pub fn compressor(&self) -> Result<&str> {
        Ok(self.0.compressor().to_str()?)
    }
    pub fn colorspace_transform(&self) -> Result<&str> {
        Ok(self.0.colorspaceTransform().to_str()?)
    }
    pub fn num_tiles(&self) -> usize {
        self.0.numTiles()
    }
    pub fn icc_profile(&self) -> Result<&str> {
        Ok(self.0.iccProfile().to_str()?)
    }
    pub fn icc_matrix(&self) -> [f64; 9] {
        self.0.iccMatrix()
    }
    pub fn image_data(&self) -> &[u8] {
        self.0.imageData().as_slice()
    }
    pub fn lossy_image_compression(&self) -> Result<&str> {
        Ok(self.0.lossyImageCompression().to_str()?)
    }
    pub fn lossy_image_compression_ratio(&self) -> f64 {
        self.0.lossyImageCompressionRatio()
    }
    pub fn color_linearity(&self) -> Result<&str> {
        Ok(self.0.colorLinearity().to_str()?)
    }
}
