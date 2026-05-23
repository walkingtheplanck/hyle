mod error;
mod types;

use error::ffi_call;
pub use error::hyle_runtime_last_error_message;
pub use types::{HyleRuntimeInstance, HyleRuntimeStatus};

#[no_mangle]
/// Advances a runtime instance by one step.
///
/// # Safety
///
/// `instance` must be a valid pointer returned by this library and must not be
/// used concurrently while this call is running.
pub unsafe extern "C" fn hyle_runtime_instance_step(
    instance: *mut HyleRuntimeInstance,
) -> HyleRuntimeStatus {
    ffi_call(|| {
        let instance = instance.as_mut().ok_or("instance must not be null")?;
        instance.inner.step().map_err(|error| error.to_string())
    })
}

#[no_mangle]
/// Releases an instance handle.
///
/// # Safety
///
/// `instance` must be null or a pointer returned by this library. Passing the
/// same pointer more than once is undefined behavior.
pub unsafe extern "C" fn hyle_runtime_instance_free(instance: *mut HyleRuntimeInstance) {
    if !instance.is_null() {
        drop(Box::from_raw(instance));
    }
}
