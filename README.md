# philips-isyntax-rs

![CI](https://github.com/AzHicham/philips-isyntax-rs/actions/workflows/workflow.yml/badge.svg)
[![codecov](https://codecov.io/gh/AzHicham/philips-isyntax-rs/branch/main/graph/badge.svg?token=RZMS9U1W97)](https://codecov.io/gh/AzHicham/philips-isyntax-rs)

Rust bindings to Philips Open Pathology C++ library ([http://openpathology.philips.com/](http://openpathology.philips.com/)).

This work has no affiliations with Philips.

The Philips Pathology SDK software toolset provides software developers full access to the data and information stored in iSyntax file format.

Philips-isyntax-rs can read virtual slides in these formats:

* [isyntax] (`.isyntax`)
* [i2syntax] (`.i2syntax`)

## Requirements

* Rust &ge; 1.87
* Philips Open Pathology C++ SDK 2.0
* Ubuntu 20.04 or 22.04

## Feature Flags

Default features are `image` and `native-sdk`.

* `native-sdk`: builds and links the Philips C++ SDK bridge.
* `image`: enables JPEG decoding, `RgbImage` helpers, and thumbnail generation.

If the Philips SDK is not installed locally, disable default features to build the
Rust-only API and unit tests:

```bash
cargo test --workspace --no-default-features
```

In this mode, SDK-backed methods return `PhilipsSlideError::SdkUnavailable`.
Use `--no-default-features --features native-sdk` to test the native bindings
without the optional image helpers.

## Thread Safety

Engine, facade, image, and view handles are `Send` but not `Sync`: a handle may
be moved to another thread, but it must not be accessed concurrently from
multiple threads.

## More Information

- [GitHub](https://github.com/AzHicham/philips-isyntax-rs)
