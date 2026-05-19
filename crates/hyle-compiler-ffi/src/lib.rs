use std::os::raw::c_char;

use hyle_compiler::{compile, CompileInput, CompileOptions, SourceFile};
use hyle_sole::{decode_sole_json, encode_sole_json};

mod error;
mod raw;
mod types;

use error::ffi_call;
pub use error::hyle_compiler_last_error_message;
use raw::{free_box, free_string, read_utf8, write_out, write_string};
pub use types::{HyleCompilerModule, HyleCompilerStatus, HyleCompilerString};

#[no_mangle]
/// Compiles a UTF-8 Hyle source buffer into an owned compiler module handle.
///
/// # Safety
///
/// `source` must point to `source_len` readable bytes, unless `source_len` is
/// zero. `out_module` must be a valid writable pointer. The returned module
/// must be released with `hyle_compiler_module_free`.
pub unsafe extern "C" fn hyle_compiler_compile(
    source: *const c_char,
    source_len: usize,
    out_module: *mut *mut HyleCompilerModule,
) -> HyleCompilerStatus {
    ffi_call(|| {
        let source = read_utf8(source, source_len)?;
        let output = compile(
            CompileInput {
                source: SourceFile::new("<ffi>", source),
                module_name: None,
            },
            CompileOptions::default(),
        )
        .map_err(|error| error.to_string())?;

        write_out(out_module, HyleCompilerModule::new(output.module))
    })
}

#[no_mangle]
/// Decodes a UTF-8 `.sole.json` buffer into an owned compiler module handle.
///
/// # Safety
///
/// `json` must point to `json_len` readable bytes, unless `json_len` is zero.
/// `out_module` must be a valid writable pointer. The returned module must be
/// released with `hyle_compiler_module_free`.
pub unsafe extern "C" fn hyle_compiler_module_from_sole_json(
    json: *const c_char,
    json_len: usize,
    out_module: *mut *mut HyleCompilerModule,
) -> HyleCompilerStatus {
    ffi_call(|| {
        let json = read_utf8(json, json_len)?;
        let module = decode_sole_json(&json).map_err(|error| error.to_string())?;

        write_out(out_module, HyleCompilerModule::new(module))
    })
}

#[no_mangle]
/// Encodes a compiler module handle as a newly allocated `.sole.json` string.
///
/// # Safety
///
/// `module` must be a valid pointer returned by this library. `out_json` must
/// be a valid writable pointer. The returned string must be released with
/// `hyle_compiler_string_free`.
pub unsafe extern "C" fn hyle_compiler_module_to_sole_json(
    module: *const HyleCompilerModule,
    out_json: *mut HyleCompilerString,
) -> HyleCompilerStatus {
    ffi_call(|| {
        let module = module.as_ref().ok_or("module must not be null")?;
        let json = encode_sole_json(&module.inner).map_err(|error| error.to_string())?;
        write_string(out_json, json)
    })
}

#[no_mangle]
/// Releases a compiler module handle.
///
/// # Safety
///
/// `module` must be null or a pointer returned by this library. Passing the
/// same pointer more than once is undefined behavior.
pub unsafe extern "C" fn hyle_compiler_module_free(module: *mut HyleCompilerModule) {
    free_box(module);
}

#[no_mangle]
/// Releases a string returned by this library.
///
/// # Safety
///
/// `string.ptr` must be null or a pointer returned by this library. Passing the
/// same pointer more than once is undefined behavior.
pub unsafe extern "C" fn hyle_compiler_string_free(string: HyleCompilerString) {
    free_string(string);
}
