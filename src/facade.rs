#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("philips-sys/cpp/facade.hpp");

        pub type Facade;
        type SubImage = crate::subimage::ffi::SubImage;

        pub(crate) fn open(
            facade: Pin<&mut Facade>,
            url: &CxxString,
            container_name: &CxxString,
            cache_name: &CxxString,
        ) -> Result<()>;
        pub(crate) fn close(self: Pin<&mut Facade>) -> usize;
        pub(crate) fn abort(self: Pin<&mut Facade>);
        pub(crate) fn remainingPixelsToEncode(self: &Facade) -> usize;

        pub(crate) fn numImages(self: &Facade) -> usize;
        pub(crate) fn sub_image<'a, 'b>(
            facade: &'a Facade,
            image_type: &'b CxxString,
        ) -> &'a SubImage;

        pub(crate) fn iSyntaxFileVersion(self: &Facade) -> &CxxString;
        pub(crate) fn id(self: &Facade) -> &CxxString;
        pub(crate) fn barcode(self: &Facade) -> &CxxString;
        pub(crate) fn scannerCalibrationStatus(self: &Facade) -> &CxxString;
        pub(crate) fn softwareVersions(self: &Facade) -> &CxxVector<CxxString>;
        pub(crate) fn derivationDescription(self: &Facade) -> &CxxString;
        pub(crate) fn acquisitionDateTime(self: &Facade) -> &CxxString;
        pub(crate) fn manufacturer(self: &Facade) -> &CxxString;
        pub(crate) fn modelName(self: &Facade) -> &CxxString;
        pub(crate) fn deviceSerialNumber(self: &Facade) -> &CxxString;
        pub(crate) fn scannerRackNumber(self: &Facade) -> u16;
        pub(crate) fn scannerSlotNumber(self: &Facade) -> u16;
        pub(crate) fn scannerOperatorId(self: &Facade) -> &CxxString;
        pub(crate) fn scannerRackPriority(self: &Facade) -> u16;
        pub(crate) fn dateOfLastCalibration(self: &Facade) -> &CxxVector<CxxString>;
        pub(crate) fn timeOfLastCalibration(self: &Facade) -> &CxxVector<CxxString>;

        pub(crate) fn isPhilips(self: &Facade) -> bool;
        pub(crate) fn isHamamatsu(self: &Facade) -> bool;
        pub(crate) fn isUFS(self: &Facade) -> bool;
        pub(crate) fn isUFSb(self: &Facade) -> bool;
        pub(crate) fn isUVS(self: &Facade) -> bool;
    }
}

#[cfg(test)]
mod tests {
    use crate::facade::ffi as ffi_facade;
    use crate::pixelengine::ffi;
    use core::ops::DerefMut;
    use cxx::let_cxx_string;
    use std::pin::Pin;

    #[test]
    fn it_workss() {
        let render_context = ffi::make_render_context();
        let render_backend = ffi::make_render_backend();
        let mut pixel_engine = ffi::make_pixel_engine(&render_context, &render_backend);
        assert_eq!(ffi::pe_version().to_str().unwrap(), "5.1.0");
        let containers = pixel_engine.containers();
        assert_eq!(
            containers
                .iter()
                .map(|cxx_str| cxx_str.to_str().unwrap())
                .collect::<Vec<&str>>(),
            vec!["ficom", "dicom", "caching-ficom", "s3", "legacy"]
        );
        let_cxx_string!(container = "ficom");
        assert_eq!(
            pixel_engine
                .containerVersion(&container)
                .unwrap()
                .to_str()
                .unwrap(),
            "100.5"
        );
        let facade_name = "in";
        let_cxx_string!(facade_name = facade_name);
        let facade = ffi::facade(pixel_engine.pin_mut(), &facade_name);

        let_cxx_string!(filename = "");
        let_cxx_string!(container_name = "");
        let_cxx_string!(cache_name = "");
        assert_eq!(facade.numImages(), 5);
        assert!(ffi_facade::open(facade, &filename, &container_name, &cache_name).is_ok());
    }
}
