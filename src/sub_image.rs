//! This module contains all functions related to Philips SubImages
//!

use crate::{Image, Result, View};

#[cfg(feature = "image")]
use {
    crate::errors::PhilipsSlideError,
    image::{
        codecs::jpeg::JpegDecoder, ColorType, DynamicImage, ImageDecoder, RgbImage, RgbaImage,
    },
    std::io::Cursor,
};

impl<'a> Image<'a> {
    /// Returns the pixel transform used for the SubImage
    pub fn pixel_transform(&self) -> Result<&str> {
        Ok(self.inner.pixelTransform()?.to_str()?)
    }

    /// Returns the quality preset used for the SubImage
    pub fn quality_preset(&self) -> Result<&str> {
        Ok(self.inner.qualityPreset()?.to_str()?)
    }

    /// Returns the quality used for the SubImage
    pub fn quality(&self) -> Result<usize> {
        Ok(self.inner.quality()?)
    }

    /// Returns the compressor used for the SubImage
    pub fn compressor(&self) -> Result<&str> {
        Ok(self.inner.compressor()?.to_str()?)
    }

    /// Returns the colorspace transform used for the SubImage
    pub fn colorspace_transform(&self) -> Result<&str> {
        Ok(self.inner.colorspaceTransform()?.to_str()?)
    }

    /// Returns the number of tiles in the SubImage
    /// Only applicable to the WSI SubImage,
    /// Returns an error for the Macro and Label/ILE SubImage
    pub fn num_tiles(&self) -> Result<usize> {
        Ok(self.inner.numTiles()?)
    }

    pub fn icc_profile(&self) -> Result<&str> {
        Ok(self.inner.iccProfile()?.to_str()?)
    }

    /// ICC (International Color Consortium) matrix (3x3) re-
    // presenting an affine transformation for the correction
    // calculated from the ICC profile.
    pub fn icc_matrix(&self) -> Result<[f64; 9]> {
        Ok(self.inner.iccMatrix()?)
    }

    /// Returns image data encoder as JPEG
    /// Only applicable to Macro and Label/ILE SubImage
    pub fn image_data(&self) -> Result<&[u8]> {
        Ok(self.inner.imageData()?.as_slice())
    }

    /// Returns image data as a DynamicImage
    /// Only applicable to Macro and Label/ILE SubImage
    #[cfg(feature = "image")]
    pub fn get_image(&self) -> Result<DynamicImage> {
        let buffer = self.image_data()?;
        Image::decode_jpeg(buffer)
    }

    /// Indicates whether the image is compressed with or without loss.
    pub fn lossy_image_compression(&self) -> Result<&str> {
        Ok(self.inner.lossyImageCompression()?.to_str()?)
    }

    /// Returns the compression ratio of the SubImage
    pub fn lossy_image_compression_ratio(&self) -> Result<f64> {
        Ok(self.inner.lossyImageCompressionRatio()?)
    }

    pub fn color_linearity(&self) -> Result<&str> {
        Ok(self.inner.colorLinearity()?.to_str()?)
    }

    #[cfg(feature = "image")]
    pub fn decode_jpeg(buffer: &[u8]) -> Result<DynamicImage> {
        let cursor = Cursor::new(buffer);
        let decoder = JpegDecoder::new(cursor)?;
        let mut image_buffer = vec![0_u8; decoder.total_bytes() as usize];
        let (w, h) = decoder.dimensions();
        let color_type = decoder.color_type();
        decoder.read_image(&mut image_buffer)?;

        if color_type == ColorType::Rgb8 {
            Ok(DynamicImage::ImageRgb8(
                RgbImage::from_vec(w, h, image_buffer).ok_or_else(|| {
                    PhilipsSlideError::ImageError(
                        "Error while creating RgbImage from buffer".to_string(),
                    )
                })?,
            ))
        } else if color_type == ColorType::Rgba8 {
            Ok(DynamicImage::ImageRgba8(
                RgbaImage::from_vec(w, h, image_buffer).ok_or_else(|| {
                    PhilipsSlideError::ImageError(
                        "Error while creating RgbImage from buffer".to_string(),
                    )
                })?,
            ))
        } else {
            Err(PhilipsSlideError::ImageError(
                "Error while creating RgbImage from buffer".to_string(),
            ))
        }
    }

    pub fn view(&self) -> Result<View> {
        Ok(View {
            inner: self.inner.view()?,
            _lifetime: Default::default(),
        })
    }
}
