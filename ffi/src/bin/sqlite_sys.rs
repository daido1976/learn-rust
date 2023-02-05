use std::os::raw::{c_char, c_int};

// On macOS, it is bundled by default.
#[link(name = "sqlite3")]
extern "C" {
    pub fn sqlite3_libversion() -> *const c_char;
    pub fn sqlite3_libversion_number() -> c_int;
}

fn main() {
    unsafe {
        let v = sqlite3_libversion_number();
        println!("{}", v);
        let v = sqlite3_libversion();
        let v = std::ffi::CStr::from_ptr(v);
        println!("{:?}", v);
    }
}
