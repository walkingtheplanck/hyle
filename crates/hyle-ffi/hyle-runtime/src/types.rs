use hyle_runtime::Instance;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HyleRuntimeStatus {
    Ok = 0,
    Error = -1,
}

pub struct HyleRuntimeInstance {
    pub(crate) inner: Box<dyn Instance>,
}
