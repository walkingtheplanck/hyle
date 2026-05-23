use std::ffi::CStr;
use std::ptr;

use hyle_runtime_ffi::{
    hyle_runtime_instance_free, hyle_runtime_instance_step, hyle_runtime_last_error_message,
    HyleRuntimeInstance, HyleRuntimeStatus,
};

#[test]
fn reports_last_error_for_null_instance_step() {
    let status = unsafe { hyle_runtime_instance_step(ptr::null_mut::<HyleRuntimeInstance>()) };

    assert_eq!(status, HyleRuntimeStatus::Error);

    let error = unsafe { CStr::from_ptr(hyle_runtime_last_error_message()) }
        .to_str()
        .expect("utf8 error");
    assert!(error.contains("instance must not be null"));
}

#[test]
fn frees_null_instance() {
    unsafe {
        hyle_runtime_instance_free(ptr::null_mut::<HyleRuntimeInstance>());
    }
}
