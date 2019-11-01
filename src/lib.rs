pub mod wimimage;
use wimlib_sys::*;
use std::ffi::CStr;

pub enum WimlibError {
    GeneralLibraryError
}

pub fn get_wimlib_version() -> Result<&'static str, WimlibError> {
    unsafe {
        let c_buf = wimlib_get_version_string();
        let c_str: &CStr = CStr::from_ptr(c_buf);
        c_str.to_str().map_err(|_| WimlibError::GeneralLibraryError)
    }
}