mod fixture;

use fixture::{missing_file, sample, sample_i2syntax, unsupported_file};
use std::path::{Path, PathBuf};

use philips_isyntax_rs::{ContainerName, PhilipsEngine};
use rstest::rstest;

#[rstest]
#[should_panic(
    expected = "CoreError(Exception { what: \"PixelEngine internal error: cannot open file for reading: <missing_file>\" })"
)]
#[case(missing_file())]
fn test_error_missing_file(#[case] filename: &Path) {
    let engine = PhilipsEngine::new();
    let _ = engine
        .facade(filename, &ContainerName::CachingFicom)
        .unwrap();
}

// Error message too long ..
#[rstest]
#[case(unsupported_file())]
fn test_error_unsupported_file(#[case] filename: &Path) {
    let engine = PhilipsEngine::new();
    let facade = engine.facade(filename, &ContainerName::CachingFicom);
    assert!(facade.is_err());
}

#[rstest]
#[case(sample())]
fn test_properties(#[case] filename: &Path) {
    let engine = PhilipsEngine::new();
    let facade = engine
        .facade(filename, &ContainerName::CachingFicom)
        .unwrap();

    assert_eq!(facade.isyntax_file_version().unwrap(), "100.5");
    assert_eq!(facade.num_images().unwrap(), 3);
    assert!(facade.id().is_ok());
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
#[case(sample_i2syntax())]
fn test_properties_2(#[case] filename: &Path) {
    let engine = PhilipsEngine::new();
    let facade = engine
        .facade(filename, &ContainerName::CachingFicom)
        .unwrap();

    assert_eq!(facade.isyntax_file_version().unwrap(), "100.5");
    assert_eq!(facade.num_images().unwrap(), 3);
    assert!(facade.id().is_ok());
    assert_eq!(facade.barcode().unwrap(), "");
    assert_eq!(facade.scanner_calibration_status().unwrap(), "OK");
    assert_eq!(
        facade.software_versions().unwrap().collect::<Vec<_>>(),
        vec!["1.1.17312", "29.0.10579-1", "3.5.2+0001"]
    );
    assert_eq!(
        facade.derivation_description().unwrap(),
        "PHILIPS UFS B60 V1.1.1.1 | Quality=2 | DWT=LeGall53 | Compressor=Hulsken2"
    );
    assert_eq!(
        facade.acquisition_date_time().unwrap(),
        "20230620143951.991000+0000"
    );
    assert_eq!(facade.manufacturer().unwrap(), "PHILIPS");
    assert_eq!(facade.model_name().unwrap(), "UFS B60");
    assert_eq!(facade.device_serial_number().unwrap(), "FMTS0117");
    assert_eq!(facade.scanner_rack_number().unwrap(), 2);
    assert_eq!(facade.scanner_slot_number().unwrap(), 7);
    assert_eq!(facade.scanner_operator_id().unwrap(), "");
    assert_eq!(facade.scanner_rack_priority().unwrap(), 0);
    assert_eq!(
        facade
            .date_of_last_calibration()
            .unwrap()
            .collect::<Vec<_>>(),
        Vec::<String>::new()
    );
    assert_eq!(
        facade
            .time_of_last_calibration()
            .unwrap()
            .collect::<Vec<_>>(),
        Vec::<String>::new()
    );

    assert!(facade.is_philips().unwrap());
    assert!(!facade.is_hamamatsu().unwrap());
    assert!(!facade.is_ufs().unwrap());
    assert!(facade.is_ufsb().unwrap());
    assert!(!facade.is_uvs().unwrap());
}

#[rstest]
#[case(sample())]
fn test_multiple_file(#[case] filename: &Path) {
    let engine = PhilipsEngine::new();
    let facade = engine
        .facade(filename, &ContainerName::CachingFicom)
        .unwrap();
    assert_eq!(facade.isyntax_file_version().unwrap(), "100.5");
    assert_eq!(facade.num_images().unwrap(), 3);

    // On the other hand you can create a new facade and open the same file
    // The two facade should be independent !
    // eg. you can drop one of them without any consequences on the other
    let facade2 = engine
        .facade(filename, &ContainerName::CachingFicom)
        .unwrap();

    drop(facade);

    assert_eq!(facade2.isyntax_file_version().unwrap(), "100.5");
    assert_eq!(facade2.num_images().unwrap(), 3);
}

#[rstest]
#[case(sample())]
fn test_facade_with_cache_file(#[case] filename: &Path) {
    let engine = PhilipsEngine::new();

    let cache_file = PathBuf::from("/tmp/sample-cache-file.fic");
    assert!(!cache_file.exists());

    let facade = engine
        .facade_with_cache_file(
            filename,
            &ContainerName::CachingFicom,
            "/tmp/sample-cache-file.fic",
        )
        .expect("Cannot open file");
    assert_eq!(facade.isyntax_file_version().unwrap(), "100.5");

    assert!(cache_file.exists());
}
