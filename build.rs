fn main() -> miette::Result<()> {
    cxx_build::bridge("src/philips_slide.rs")
        .file("cpp/philipsslide.cc")
        .flag_if_supported("-std=c++17")
        .compile("philips-slide");

    println!("cargo:rerun-if-changed=src/philips_slide.rs");
    println!("cargo:rerun-if-changed=cpp/philipsslide.cc");
    println!("cargo:rerun-if-changed=cpp/philipsslide.hpp");

    println!("cargo:rustc-link-lib=pixelengine");
    println!("cargo:rustc-link-lib=softwarerendercontext");
    println!("cargo:rustc-link-lib=eglrendercontext");
    println!("cargo:rustc-link-lib=softwarerenderbackend");
    println!("cargo:rustc-link-lib=gles2renderbackend");
    println!("cargo:rustc-link-lib=gles3renderbackend");
    Ok(())
}
