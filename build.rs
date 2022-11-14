fn main() -> miette::Result<()> {
    cxx_build::bridge("src/pixelengine.rs")
        .file("cpp/pixelengine.cc")
        .flag_if_supported("-std=c++17")
        .compile("philips-pe-sys");

    cxx_build::bridge("src/view.rs")
        .file("cpp/view.cc")
        .flag_if_supported("-std=c++17")
        .compile("philips-view-sys");

    cxx_build::bridge("src/facade.rs")
        .file("cpp/facade.cc")
        .flag_if_supported("-std=c++17")
        .compile("philips-facade-sys");

    cxx_build::bridge("src/subimage.rs")
        .file("cpp/subimage.cc")
        .flag_if_supported("-std=c++17")
        .compile("philips-subimage-sys");

    cxx_build::bridge("src/dataenvelopes.rs")
        .file("cpp/dataenvelopes.cc")
        .flag_if_supported("-std=c++17")
        .compile("philips-dataenvelopes-sys");

    println!("cargo:rerun-if-changed=src/pixel_engine.rs");
    println!("cargo:rerun-if-changed=src/view.rs");
    println!("cargo:rerun-if-changed=src/facade.rs");
    println!("cargo:rerun-if-changed=src/subimage.rs");
    println!("cargo:rerun-if-changed=src/dataenvelopes.rs");

    println!("cargo:rerun-if-changed=cpp/pixelengine.cc");
    println!("cargo:rerun-if-changed=cpp/pixelengine.hpp");
    println!("cargo:rerun-if-changed=cpp/view.cc");
    println!("cargo:rerun-if-changed=cpp/view.hpp");
    println!("cargo:rerun-if-changed=cpp/facade.cc");
    println!("cargo:rerun-if-changed=cpp/facade.hpp");
    println!("cargo:rerun-if-changed=cpp/subimage.cc");
    println!("cargo:rerun-if-changed=cpp/subimage.hpp");

    println!("cargo:rustc-link-lib=pixelengine");
    println!("cargo:rustc-link-lib=softwarerendercontext");
    println!("cargo:rustc-link-lib=eglrendercontext");
    println!("cargo:rustc-link-lib=softwarerenderbackend");
    println!("cargo:rustc-link-lib=gles2renderbackend");
    println!("cargo:rustc-link-lib=gles3renderbackend");
    Ok(())
}
