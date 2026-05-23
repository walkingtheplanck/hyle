use std::cell::RefCell;
use std::ffi::CString;
use std::os::raw::c_char;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;

use crate::HyleRuntimeStatus;

thread_local! {
    static LAST_ERROR: RefCell<Option<CString>> = const { RefCell::new(None) };
}

#[no_mangle]
pub extern "C" fn hyle_runtime_last_error_message() -> *const c_char {
    LAST_ERROR.with(|last_error| {
        last_error
            .borrow()
            .as_ref()
            .map_or(ptr::null(), |message| message.as_ptr())
    })
}

pub(crate) fn ffi_call(function: impl FnOnce() -> Result<(), String>) -> HyleRuntimeStatus {
    match catch_unwind(AssertUnwindSafe(function)) {
        Ok(Ok(())) => {
            clear_last_error();
            HyleRuntimeStatus::Ok
        }
        Ok(Err(error)) => {
            set_last_error(error);
            HyleRuntimeStatus::Error
        }
        Err(_) => {
            set_last_error("panic crossed Hyle runtime FFI boundary".to_owned());
            HyleRuntimeStatus::Error
        }
    }
}

fn clear_last_error() {
    LAST_ERROR.with(|last_error| {
        *last_error.borrow_mut() = None;
    });
}

fn set_last_error(error: String) {
    let sanitized = error.replace('\0', "\\0");
    LAST_ERROR.with(|last_error| {
        *last_error.borrow_mut() =
            Some(CString::new(sanitized).expect("sanitized error must not contain nul"));
    });
}
