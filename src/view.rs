#[cxx::bridge]
pub(crate) mod ffi {

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct DimensionsRange {
        pub start_x: u32,
        pub step_x: u32,
        pub end_x: u32,
        pub start_y: u32,
        pub step_y: u32,
        pub end_y: u32,
    }

    unsafe extern "C++" {
        include!("philips-sys/cpp/view.hpp");

        pub type View;
        pub type SourceView;
        pub type DisplayView;
        pub type UserView;
        pub type Region;
        type Rectangle = crate::dataenvelopes::ffi::Rectangle;

        fn source_view_as_view(source_view: Pin<&mut SourceView>) -> Pin<&mut View>;
        fn display_view_as_view(source_view: Pin<&mut DisplayView>) -> Pin<&mut View>;
        fn user_view_as_view(source_view: Pin<&mut UserView>) -> Pin<&mut View>;

        fn addChainedView(self: Pin<&mut View>) -> Pin<&mut UserView>;

        fn request_region(
            view: Pin<&mut View>,
            ranges: &Rectangle,
            async_: bool,
            bg_color: &[usize; 3],
        ) -> SharedPtr<Region>;

        fn dimension_ranges(view: &View, level: u32) -> DimensionsRange;
        fn dimensionNames(self: &View) -> &CxxVector<CxxString>;
        fn dimensionUnits(self: &View) -> &CxxVector<CxxString>;
        fn dimensionTypes(self: &View) -> &CxxVector<CxxString>;
        fn scale(self: &View) -> &CxxVector<f64>;
        fn origin(self: &View) -> &CxxVector<f64>;
        fn bitsAllocated(self: &View) -> u16;
        fn bitsStored(self: &View) -> u16;
        fn highBit(self: &View) -> u16;
        fn pixelRepresentation(self: &View) -> u16;
        fn planarConfiguration(self: &View) -> u16;
        fn samplesPerPixel(self: &View) -> u16;
        fn id(self: &View) -> usize;
        fn numDerivedLevels(self: &View) -> usize;

        fn loadDefaultParameters(self: Pin<&mut SourceView>);
        // !todo:  Understand & Fix truncationLevel
        fn truncation(view: Pin<&mut SourceView>, enabled: bool, rounding: bool);

        fn sharpness(self: &DisplayView) -> f64;
        fn contrastClipLimit(self: &DisplayView) -> f64;
        fn colorCorrectionGamma(self: &DisplayView) -> f64;
        fn colorCorrectionBlackPoint(self: &DisplayView) -> f64;
        fn colorCorrectionWhitePoint(self: &DisplayView) -> f64;
        fn colorGain(self: &DisplayView) -> f64;
    }
}

use crate::{DisplayView, SourceView, UserView, View};

impl<'a> View<'a> {
    pub fn dimension_names(&self) -> impl Iterator<Item = &str> {
        self.0
            .dimensionNames()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    pub fn dimension_units(&self) -> impl Iterator<Item = &str> {
        self.0
            .dimensionUnits()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    pub fn dimension_types(&self) -> impl Iterator<Item = &str> {
        self.0
            .dimensionTypes()
            .iter()
            .filter_map(|cxx_str| cxx_str.to_str().ok())
    }

    pub fn scale(&self) -> &[f64] {
        self.0.scale().as_slice()
    }

    pub fn origin(&self) -> &[f64] {
        self.0.origin().as_slice()
    }

    pub fn bits_allocated(&self) -> u16 {
        self.0.bitsAllocated()
    }

    pub fn bits_stored(&self) -> u16 {
        self.0.bitsStored()
    }

    pub fn high_bit(&self) -> u16 {
        self.0.highBit()
    }

    pub fn pixel_representation(&self) -> u16 {
        self.0.pixelRepresentation()
    }

    pub fn planar_configuration(&self) -> u16 {
        self.0.planarConfiguration()
    }

    pub fn samples_per_pixel(&self) -> u16 {
        self.0.samplesPerPixel()
    }

    pub fn id(&self) -> usize {
        self.0.id()
    }

    pub fn num_derived_levels(&self) -> usize {
        self.0.numDerivedLevels()
    }
}

impl<'a> SourceView<'a> {
    pub fn load_default_parameters(&mut self) {
        self.0.as_mut().loadDefaultParameters()
    }
    pub fn truncation(&mut self, enabled: bool, rounding: bool) {
        ffi::truncation(self.0.as_mut(), enabled, rounding)
    }
}

impl<'a> DisplayView<'a> {
    pub fn sharpness(&self) -> f64 {
        self.0.sharpness()
    }
    pub fn contrast_clip_limit(&self) -> f64 {
        self.0.contrastClipLimit()
    }
    pub fn color_correction_gamma(&self) -> f64 {
        self.0.colorCorrectionGamma()
    }
    pub fn color_correction_black_point(&self) -> f64 {
        self.0.colorCorrectionBlackPoint()
    }
    pub fn color_correction_white_point(&self) -> f64 {
        self.0.colorCorrectionWhitePoint()
    }
    pub fn color_gain(&self) -> f64 {
        self.0.colorGain()
    }
}

impl<'a> From<UserView<'a>> for View<'a> {
    fn from(user_view: UserView<'a>) -> Self {
        View(ffi::user_view_as_view(user_view.0))
    }
}

impl<'a> From<DisplayView<'a>> for View<'a> {
    fn from(display_view: DisplayView<'a>) -> Self {
        View(ffi::display_view_as_view(display_view.0))
    }
}

impl<'a> From<SourceView<'a>> for View<'a> {
    fn from(source_view: SourceView<'a>) -> Self {
        View(ffi::source_view_as_view(source_view.0))
    }
}
