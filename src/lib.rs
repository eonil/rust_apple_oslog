#![allow(dead_code)]


use std::ffi::CStr;
use std::ffi::CString;
mod c;

fn os_log_default(msg:&str) {
    let s = CString::new(msg).expect("Converting into C string failed.");
    let x: &CStr = &s;
    unsafe {
        c::RE_stub_oslog_default_public(x.as_ptr());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        super::os_log_default("AAAA");
    }
}



