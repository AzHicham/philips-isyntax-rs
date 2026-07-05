use std::env;

fn main() {
    let docs_rs = env::var("DOCS_RS").is_ok();
    let native_sdk = env::var("CARGO_FEATURE_NATIVE_SDK").is_ok();

    println!("cargo:rerun-if-env-changed=DOCS_RS");
    println!("cargo:rerun-if-changed=src/bindings.rs");
    println!("cargo:rerun-if-changed=src/native_bridge.rs");

    if docs_rs || !native_sdk {
        return;
    }

    cxx_build::bridge("src/native_bridge.rs")
        .file("cpp/philipsslide.cc")
        .cpp(true)
        .std("c++17")
        .compile("philips-bindings");

    println!("cargo:rerun-if-changed=cpp/philipsslide.cc");
    println!("cargo:rerun-if-changed=cpp/philipsslide.hpp");

    println!("cargo:rustc-link-lib=pixelengine");
    println!("cargo:rustc-link-lib=softwarerendercontext");
    println!("cargo:rustc-link-lib=eglrendercontext");
    println!("cargo:rustc-link-lib=softwarerenderbackend");
    println!("cargo:rustc-link-lib=gles2renderbackend");
    println!("cargo:rustc-link-lib=gles3renderbackend");
}
