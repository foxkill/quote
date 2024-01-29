#![allow(unused_imports)]
use std::ffi::c_double;
use std::ffi::c_char;
use std::ffi::CStr;

use super::parse;
use super::Style;

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn cparse(s_ptr: *const c_char, style: Style) -> c_double {
    unsafe { 
        let s = CStr::from_ptr(s_ptr);
        let result = parse(s.to_str().unwrap(), style);
        std::ffi::c_double::from(result.unwrap_or(0.0))
    }
}