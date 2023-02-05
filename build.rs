use std::env;

fn main() {
    if env::var("DOCS_RS").is_err() {
        cxx_build::bridge("src/bindings.rs")
            .file("cpp/philipsslide.cc")
            .flag_if_supported("-std=c++17")
            .compile("philips-bindings");

        println!("cargo:rerun-if-changed=src/bindings.rs");
        println!("cargo:rerun-if-changed=cpp/philipsslide.cc");
        println!("cargo:rerun-if-changed=cpp/philipsslide.hpp");

        println!("cargo:rustc-link-lib=pixelengine");
        println!("cargo:rustc-link-lib=softwarerendercontext");
        println!("cargo:rustc-link-lib=eglrendercontext");
        println!("cargo:rustc-link-lib=softwarerenderbackend");
        println!("cargo:rustc-link-lib=gles2renderbackend");
        println!("cargo:rustc-link-lib=gles3renderbackend");
    }
}
