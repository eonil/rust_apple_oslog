//use std::ffi::c_void;
use libc::c_char;
//use libc::c_int;


#[link(name = "RE_stub")]
extern "C" {
    pub fn RE_stub_oslog_default_public(msg: *const c_char);
}

