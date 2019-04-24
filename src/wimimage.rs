use std::path::Path;
use std::ffi::CString;
use std::ptr;
use wimlib_sys::*;

pub struct WimImage {
    c_struct: *mut wimlib_sys::WIMStruct
}

pub enum WimImageOpenError {
    InvalidCharacters,
    UnknownWimlibError(i32)
}

impl WimImage {
    fn open_from_file(wim_file_path: &Path) -> Result<Self, WimImageOpenError> {
        let path = wim_file_path.to_str().ok_or_else(|| WimImageOpenError::InvalidCharacters)?;
        let cpath = CString::new(path).map_err(|_| WimImageOpenError::InvalidCharacters)?;
        let wim_struct_ptr_box = Box::new(ptr::null_mut());
        let wim_struct_ptr_box_ptr = Box::into_raw(wim_struct_ptr_box);
        unsafe {
            let open_res = wimlib_open_wim(cpath.as_ptr() as *const i8, 0, wim_struct_ptr_box_ptr);
            match open_res {
                0 => {
                    // success
                    let wim_struct_ptr_box = Box::from_raw(wim_struct_ptr_box_ptr);
                    let wim_struct_ptr = *wim_struct_ptr_box;
                    Ok(WimImage { c_struct: wim_struct_ptr })
                },
                _ => {
                    // unknown Wimlib error
                    Err(WimImageOpenError::UnknownWimlibError(open_res))
                }
            }
        }
    }
}

impl Drop for WimImage {
    fn drop(&mut self) {
        unsafe {
            wimlib_free(self.c_struct);
        }
    }
}