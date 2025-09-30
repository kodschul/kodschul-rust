use crate::c_modules;

pub fn main() {
    unsafe {
        println!("2 verdoppelt = {}", c_modules::double_input(2));
    }
}
