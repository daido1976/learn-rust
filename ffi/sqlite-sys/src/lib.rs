use std::os::raw::{c_char, c_int};

// On macOS, it is bundled by default.
#[link(name = "sqlite3")]
extern "C" {
    pub fn sqlite3_libversion() -> *const c_char;
    pub fn sqlite3_libversion_number() -> c_int;
}
