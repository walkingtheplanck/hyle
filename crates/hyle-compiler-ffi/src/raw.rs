use std::ffi::CString;
use std::os::raw::c_char;
use std::slice;
use std::str;

use crate::HyleCompilerString;

pub(crate) unsafe fn read_utf8(data: *const c_char, len: usize) -> Result<String, String> {
    if data.is_null() {
        if len == 0 {
            return Ok(String::new());
        }

        return Err("input buffer must not be null".to_owned());
    }

    let bytes = slice::from_raw_parts(data.cast::<u8>(), len);
    str::from_utf8(bytes)
        .map(str::to_owned)
        .map_err(|error| error.to_string())
}

pub(crate) unsafe fn write_out<T>(out: *mut *mut T, value: T) -> Result<(), String> {
    if out.is_null() {
        return Err("output pointer must not be null".to_owned());
    }

    *out = Box::into_raw(Box::new(value));
    Ok(())
}

pub(crate) unsafe fn write_string(
    out: *mut HyleCompilerString,
    value: String,
) -> Result<(), String> {
    if out.is_null() {
        return Err("output string must not be null".to_owned());
    }

    let string = CString::new(value).map_err(|error| error.to_string())?;
    let len = string.as_bytes().len();

    *out = HyleCompilerString {
        ptr: string.into_raw(),
        len,
    };

    Ok(())
}

pub(crate) unsafe fn free_box<T>(value: *mut T) {
    if !value.is_null() {
        drop(Box::from_raw(value));
    }
}

pub(crate) unsafe fn free_string(string: HyleCompilerString) {
    if !string.ptr.is_null() {
        drop(CString::from_raw(string.ptr));
    }
}
