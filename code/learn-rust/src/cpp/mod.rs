#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("add.h");

        fn add(a: i32, b: i32) -> i32;
    }
}
