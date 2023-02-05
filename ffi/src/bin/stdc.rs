use std::ffi::CString;
use std::os::raw::c_char;

// See. https://9cguide.appspot.com/r_lib.html
extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn puts(s: *const c_char);
}

fn main() {
    let cs = CString::new("ffi ffi!!!").unwrap();
    unsafe {
        let len = strlen(cs.as_ptr());
        let s = CString::new(len.to_string()).unwrap();
        puts(s.as_ptr());
    }
}
