use hyle_runtime::{Instance, LoadOptions};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HyleRuntimeLoadOptions {
    pub bound_checks: bool,
}

impl From<HyleRuntimeLoadOptions> for LoadOptions {
    fn from(options: HyleRuntimeLoadOptions) -> Self {
        Self {
            bound_checks: options.bound_checks,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HyleRuntimeStatus {
    Ok = 0,
    Error = -1,
}

pub struct HyleRuntimeInstance {
    pub(crate) inner: Box<dyn Instance>,
}
