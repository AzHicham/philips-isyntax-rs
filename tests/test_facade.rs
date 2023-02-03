mod fixture;

use fixture::{missing_file, sample, unsupported_file};
use std::path::Path;

use philips_isyntax_rs::PhilipsEngine;
use rstest::rstest;

#[rstest]
#[should_panic(
    expected = "CoreError(\"PixelEngine internal error: cannot open file for reading: <missing_file>\")"
)]
#[case(missing_file())]
fn test_error_missing_file(#[case] filename: &Path) {
    let engine = PhilipsEngine::new().unwrap();
    let facade = engine.facade("facade_name").unwrap();
    facade.open(filename).unwrap()
}

// Error message too long ..
#[rstest]
#[case(unsupported_file())]
fn test_error_unsupported_file(#[case] filename: &Path) {
    let engine = PhilipsEngine::new().unwrap();
    let facade = engine.facade("facade_name").unwrap();
    assert!(facade.open(filename).is_err());
}

#[rstest]
#[case(sample())]
fn test_properties(#[case] filename: &Path) {
    let engine = PhilipsEngine::new().unwrap();
    let facade = engine.facade("facade_name").unwrap();
    facade.open(filename).unwrap();

    assert_eq!(facade.isyntax_file_version().unwrap(), "100.5");
    assert_eq!(facade.num_images().unwrap(), 3);
    assert_eq!(facade.id().unwrap(), "facade_name");
    assert_eq!(facade.barcode().unwrap(), "LMS-2-12306355");
    assert_eq!(facade.scanner_calibration_status().unwrap(), "OK");
    assert_eq!(
        facade.software_versions().unwrap().collect::<Vec<_>>(),
        vec!["1.8.6824", "20180906_R51"]
    );
    assert_eq!(
        facade.derivation_description().unwrap(),
        "PHILIPS UFS V1.8.6824 | Quality=1 | DWT=1 | Compressor=16"
    );
    assert_eq!(
        facade.acquisition_date_time().unwrap(),
        "20210125181858.000000"
    );
    assert_eq!(facade.manufacturer().unwrap(), "PHILIPS");
    assert_eq!(facade.model_name().unwrap(), "UFS Scanner");
    assert_eq!(facade.device_serial_number().unwrap(), "FMT0296");
    assert_eq!(facade.scanner_rack_number().unwrap(), 4);
    assert_eq!(facade.scanner_slot_number().unwrap(), 19);
    assert_eq!(facade.scanner_operator_id().unwrap(), "");
    assert_eq!(facade.scanner_rack_priority().unwrap(), 0);
    assert_eq!(
        facade
            .date_of_last_calibration()
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["20210125"]
    );
    assert_eq!(
        facade
            .time_of_last_calibration()
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["174955"]
    );

    assert!(facade.is_philips().unwrap());
    assert!(!facade.is_hamamatsu().unwrap());
    assert!(facade.is_ufs().unwrap());
    assert!(!facade.is_ufsb().unwrap());
    assert!(!facade.is_uvs().unwrap());
}

#[rstest]
#[case(sample())]
fn test_properties_new(#[case] filename: &Path) {
    let engine = PhilipsEngine::new().unwrap();
    let facade = engine.facade("facade_name2").unwrap();
    facade.open(filename).unwrap();

    assert_eq!(facade.isyntax_file_version().unwrap(), "100.5");
    assert_eq!(facade.num_images().unwrap(), 3);
    assert_eq!(facade.id().unwrap(), "facade_name2");
    assert_eq!(facade.barcode().unwrap(), "LMS-2-12306355");
    assert_eq!(facade.scanner_calibration_status().unwrap(), "OK");
    assert_eq!(
        facade.software_versions().unwrap().collect::<Vec<_>>(),
        vec!["1.8.6824", "20180906_R51"]
    );
    assert_eq!(
        facade.derivation_description().unwrap(),
        "PHILIPS UFS V1.8.6824 | Quality=1 | DWT=1 | Compressor=16"
    );
    assert_eq!(
        facade.acquisition_date_time().unwrap(),
        "20210125181858.000000"
    );
    assert_eq!(facade.manufacturer().unwrap(), "PHILIPS");
    assert_eq!(facade.model_name().unwrap(), "UFS Scanner");
    assert_eq!(facade.device_serial_number().unwrap(), "FMT0296");
    assert_eq!(facade.scanner_rack_number().unwrap(), 4);
    assert_eq!(facade.scanner_slot_number().unwrap(), 19);
    assert_eq!(facade.scanner_operator_id().unwrap(), "");
    assert_eq!(facade.scanner_rack_priority().unwrap(), 0);
    assert_eq!(
        facade
            .date_of_last_calibration()
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["20210125"]
    );
    assert_eq!(
        facade
            .time_of_last_calibration()
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["174955"]
    );

    assert!(facade.is_philips().unwrap());
    assert!(!facade.is_hamamatsu().unwrap());
    assert!(facade.is_ufs().unwrap());
    assert!(!facade.is_ufsb().unwrap());
    assert!(!facade.is_uvs().unwrap());
}
