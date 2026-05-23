use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use std::slice;
use std::str;

use hyle_compiler_ffi::{
    hyle_compiler_compile, hyle_compiler_last_error_message, hyle_compiler_module_free,
    hyle_compiler_module_from_sole_json, hyle_compiler_module_to_sole_json,
    hyle_compiler_string_free, HyleCompilerModule, HyleCompilerStatus, HyleCompilerString,
};

const GAME: &str = include_str!("../../../../examples/game.hyle");
const GAME_SOLE_JSON: &str = include_str!("../../../../examples/game.sole.json");

#[test]
fn compiles_hyle_source_to_sole_json() {
    let mut module: *mut HyleCompilerModule = ptr::null_mut();
    let status =
        unsafe { hyle_compiler_compile(GAME.as_ptr().cast::<c_char>(), GAME.len(), &mut module) };

    assert_eq!(status, HyleCompilerStatus::Ok);
    assert!(!module.is_null());

    let json = module_to_json(module);
    assert_eq!(json, GAME_SOLE_JSON.trim_end());

    unsafe {
        hyle_compiler_module_free(module);
    }
}

#[test]
fn decodes_sole_json_to_module() {
    let mut module: *mut HyleCompilerModule = ptr::null_mut();
    let status = unsafe {
        hyle_compiler_module_from_sole_json(
            GAME_SOLE_JSON.as_ptr().cast::<c_char>(),
            GAME_SOLE_JSON.len(),
            &mut module,
        )
    };

    assert_eq!(status, HyleCompilerStatus::Ok);
    assert!(!module.is_null());

    let json = module_to_json(module);
    assert_eq!(json, GAME_SOLE_JSON.trim_end());

    unsafe {
        hyle_compiler_module_free(module);
    }
}

#[test]
fn reports_last_error_for_invalid_input() {
    let mut module: *mut HyleCompilerModule = ptr::null_mut();
    let status = unsafe { hyle_compiler_module_from_sole_json(ptr::null(), 1, &mut module) };

    assert_eq!(status, HyleCompilerStatus::Error);
    assert!(module.is_null());

    let error = unsafe { CStr::from_ptr(hyle_compiler_last_error_message()) }
        .to_str()
        .expect("utf8 error");
    assert!(error.contains("input buffer must not be null"));
}

fn module_to_json(module: *mut HyleCompilerModule) -> String {
    let mut json = HyleCompilerString {
        ptr: ptr::null_mut(),
        len: 0,
    };
    let status = unsafe { hyle_compiler_module_to_sole_json(module, &mut json) };

    assert_eq!(status, HyleCompilerStatus::Ok);
    assert!(!json.ptr.is_null());

    let bytes = unsafe { slice::from_raw_parts(json.ptr.cast::<u8>(), json.len) };
    let output = str::from_utf8(bytes).expect("utf8 json").to_owned();

    unsafe {
        hyle_compiler_string_free(json);
    }

    output
}
