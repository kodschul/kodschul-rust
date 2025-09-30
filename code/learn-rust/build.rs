fn main() {
    cc::Build::new().file("src/math.c").compile("mathlib");

    // The bridge is needed to generate C++ bindings for Rust code
    // It processes the #[cxx::bridge] module in src/main.rs
    // to create the necessary FFI interface between Rust and C++
    cxx_build::bridge("src/cpp/mod.rs")
        .file("cpp/add.cpp")
        .include("include")
        .compile("add_cxx");

    println!("cargo:rerun-if-changed=src/cpp/mod.rs");
    println!("cargo:rerun-if-changed=cpp/add.cpp");
    println!("cargo:rerun-if-changed=include/add.h");
}
