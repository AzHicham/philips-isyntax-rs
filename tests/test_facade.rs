use philips_isyntax_rs::PixelEngine;
use rstest::rstest;

#[rstest]
#[should_panic(
    expected = "CoreError(\"PixelEngine internal error: iSyntaxFileVersion not available on uninitialized or unopened stream\")"
)]
fn test_error_not_opend_file_1() {
    let mut pe = PixelEngine::new();
    let facade = pe.facade("in");
    facade.isyntax_file_version().unwrap();
}

#[rstest]
#[should_panic(
    expected = "CoreError(\"encoding remaining pixels cannot be called at this stage\")"
)]
fn test_error_not_opend_file_2() {
    let mut pe = PixelEngine::new();
    let facade = pe.facade("in");
    facade.remaining_pixels_to_encode().unwrap();
}

#[rstest]
#[should_panic(
    expected = "CoreError(\"PixelEngine internal error: cannot open file for reading: <not_a_file.isyntax>\")"
)]
fn test_error_open_file() {
    let mut pe = PixelEngine::new();
    let mut facade = pe.facade("in");

    facade.open("not_a_file.isyntax", "", "").unwrap();
}