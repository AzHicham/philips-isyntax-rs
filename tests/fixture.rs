use rstest::fixture;
use std::path::Path;

#[fixture]
#[once]
pub fn missing_file() -> &'static Path {
    Path::new("missing_file")
}

#[fixture]
#[once]
pub fn unsupported_file() -> &'static Path {
    Path::new("Cargo.toml")
}

#[fixture]
#[once]
pub fn sample() -> &'static Path {
    Path::new("tests/data/sample.isyntax")
}

#[fixture]
#[once]
pub fn sample_i2syntax() -> &'static Path {
    Path::new("tests/data/sample-9b.i2syntax")
}
