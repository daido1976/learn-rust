use my_sqlite_sys::*;

fn main() {
    unsafe {
        let v = sqlite3_libversion_number();
        println!("{}", v);
        let v = sqlite3_libversion();
        let v = std::ffi::CStr::from_ptr(v);
        println!("{:?}", v);
    }
}
