use std::os::raw::c_char;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HyleCompilerStatus {
    Ok = 0,
    Error = -1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HyleCompilerString {
    pub ptr: *mut c_char,
    pub len: usize,
}

pub struct HyleCompilerModule {
    pub(crate) inner: hyle_sole::SoleModule,
}

impl HyleCompilerModule {
    pub(crate) fn new(inner: hyle_sole::SoleModule) -> Self {
        Self { inner }
    }
}
