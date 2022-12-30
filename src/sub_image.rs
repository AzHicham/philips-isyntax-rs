//! This module contains all functions related to Philips SubImages
//!

use crate::{Image, ImageType, PhilipsSlide, Result};
use cxx::let_cxx_string;

#[cfg(feature = "image")]
use {
    crate::errors::PhilipsSlideError,
    image::{
        codecs::jpeg::JpegDecoder, ColorType, DynamicImage, ImageDecoder, RgbImage, RgbaImage,
    },
    std::io::Cursor,
};

impl PhilipsSlide {
    /// Returns the pixel transform used for the SubImage
    pub fn pixel_transform(&self, image_type: ImageType) -> Result<&str> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.pixelTransform(&image_type)?.to_str()?)
    }

    /// Returns the quality preset used for the SubImage
    pub fn quality_preset(&self, image_type: ImageType) -> Result<&str> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.qualityPreset(&image_type)?.to_str()?)
    }

    /// Returns the quality used for the SubImage
    pub fn quality(&self, image_type: ImageType) -> Result<usize> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.quality(&image_type)?)
    }

    /// Returns the compressor used for the SubImage
    pub fn compressor(&self, image_type: ImageType) -> Result<&str> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.compressor(&image_type)?.to_str()?)
    }

    /// Returns the colorspace transform used for the SubImage
    pub fn colorspace_transform(&self, image_type: ImageType) -> Result<&str> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.colorspaceTransform(&image_type)?.to_str()?)
    }

    /// Returns the number of tiles in the SubImage
    /// Only applicable to the WSI SubImage,
    /// Returns an error for the Macro and Label/ILE SubImage
    pub fn num_tiles(&self, image_type: ImageType) -> Result<usize> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.numTiles(&image_type)?)
    }

    pub fn icc_profile(&self, image_type: ImageType) -> Result<&str> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.iccProfile(&image_type)?.to_str()?)
    }

    /// ICC (International Color Consortium) matrix (3x3) re-
    // presenting an affine transformation for the correction
    // calculated from the ICC profile.
    pub fn icc_matrix(&self, image_type: ImageType) -> Result<[f64; 9]> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.iccMatrix(&image_type)?)
    }

    /// Returns image data encoder as JPEG
    /// Only applicable to Macro and Label/ILE SubImage
    pub fn image_data(&self, image_type: ImageType) -> Result<&[u8]> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.imageData(&image_type)?.as_slice())
    }

    /// Returns image data as a DynamicImage
    /// Only applicable to Macro and Label/ILE SubImage
    #[cfg(feature = "image")]
    pub fn get_image(&self, image_type: ImageType) -> Result<DynamicImage> {
        let buffer = self.image_data(image_type)?;
        PhilipsSlide::decode_jpeg(buffer)
    }

    /// Indicates whether the image is compressed with or without loss.
    pub fn lossy_image_compression(&self, image_type: ImageType) -> Result<&str> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.lossyImageCompression(&image_type)?.to_str()?)
    }

    /// Returns the compression ratio of the SubImage
    pub fn lossy_image_compression_ratio(&self, image_type: ImageType) -> Result<f64> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.lossyImageCompressionRatio(&image_type)?)
    }

    pub fn color_linearity(&self, image_type: ImageType) -> Result<&str> {
        let_cxx_string!(image_type = image_type);
        Ok(self.inner.colorLinearity(&image_type)?.to_str()?)
    }

    #[cfg(feature = "image")]
    fn decode_jpeg(buffer: &[u8]) -> Result<DynamicImage> {
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
}

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
        PhilipsSlide::decode_jpeg(buffer)
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
    fn decode_jpeg(buffer: &[u8]) -> Result<DynamicImage> {
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
}
