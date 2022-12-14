mod fixture;

use fixture::{missing_file, sample, unsupported_file};
use std::path::Path;

use philips_isyntax_rs::PhilipsSlide;
use rstest::rstest;

#[rstest]
#[should_panic(
    expected = "CoreError(\"PixelEngine internal error: cannot open file for reading: <missing_file>\")"
)]
#[case(missing_file())]
fn test_error_missing_file(#[case] filename: &Path) {
    let _ = PhilipsSlide::new(filename.to_str().unwrap()).unwrap();
}

// Error message too long ..
#[rstest]
#[case(unsupported_file())]
fn test_error_unsupported_file(#[case] filename: &Path) {
    let slide = PhilipsSlide::new(filename.to_str().unwrap());
    assert!(slide.is_err());
}

#[rstest]
#[case(sample())]
fn test_properties(#[case] filename: &Path) {
    let slide = PhilipsSlide::new(filename.to_str().unwrap()).unwrap();

    assert_eq!(slide.isyntax_file_version().unwrap(), "100.5");
    assert_eq!(slide.num_images().unwrap(), 3);
    assert_eq!(slide.id().unwrap(), "in");
    assert_eq!(slide.barcode().unwrap(), "LMS-2-12306355");
    assert_eq!(slide.scanner_calibration_status().unwrap(), "OK");
    assert_eq!(
        slide.software_versions().unwrap().collect::<Vec<_>>(),
        vec!["1.8.6824", "20180906_R51"]
    );
    assert_eq!(
        slide.derivation_description().unwrap(),
        "PHILIPS UFS V1.8.6824 | Quality=1 | DWT=1 | Compressor=16"
    );
    assert_eq!(
        slide.acquisition_date_time().unwrap(),
        "20210125181858.000000"
    );
    assert_eq!(slide.manufacturer().unwrap(), "PHILIPS");
    assert_eq!(slide.model_name().unwrap(), "UFS Scanner");
    assert_eq!(slide.device_serial_number().unwrap(), "FMT0296");
    assert_eq!(slide.scanner_rack_number().unwrap(), 4);
    assert_eq!(slide.scanner_slot_number().unwrap(), 19);
    assert_eq!(slide.scanner_operator_id().unwrap(), "");
    assert_eq!(slide.scanner_rack_priority().unwrap(), 0);
    assert_eq!(
        slide
            .date_of_last_calibration()
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["20210125"]
    );
    assert_eq!(
        slide
            .time_of_last_calibration()
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["174955"]
    );

    assert!(slide.is_philips().unwrap());
    assert!(!slide.is_hamamatsu().unwrap());
    assert!(slide.is_ufs().unwrap());
    assert!(!slide.is_ufsb().unwrap());
    assert!(!slide.is_uvs().unwrap());
}
