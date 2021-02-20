
use glib;
use glib_sys::GError;
use std::ffi::{CStr,CString, OsString};
use std::{fs, path, ptr};

use glib::{
    FileError,
    error::{Error,ErrorDomain}
};

pub fn to_glib_error(e: *mut GError) -> glib::error::Error {
    unsafe {
        if !e.is_null() {
            Error::new(
                ErrorDomain::from((*e).code)
                    .unwrap_or(FileError::Failed),
                CStr::from_ptr((*e).message).to_str()
                    .unwrap_or("Invalid error message")
            )
        }
        else {
            Error::new(
                ErrorDomain::from(0)
                    .unwrap_or(FileError::Failed),
                CStr::from_bytes_with_nul(b"Error is null\0")
                    .unwrap().to_str()
                    .unwrap_or("Invalid error message")
            )
        }
    }
}









pub fn call_with_gerror<T, F>(f: F) -> Result<*mut T, glib::error::Error>
where
    F: FnOnce(*mut *mut GError) -> *mut T,
{
    // initialize error to a null-pointer
    let mut err = ptr::null_mut();

    // call the c-library function
    let return_value = f(&mut err as *mut *mut GError);

    if return_value.is_null() {
        Err(glib::error::Error::new(
            glib::FileError::Failed,
            "Return value was null",
        ))
    } else {
        Ok(return_value)
    }
}

pub fn path_to_glib_url<P: AsRef<path::Path>>(p: P) -> Result<CString, glib::error::Error> {
    // canonicalize path, try to wrap failures into a glib error
    let canonical = fs::canonicalize(p).map_err(|_| {
        glib::error::Error::new(
            glib::FileError::Noent,
            "Could not turn path into canonical path. Maybe it does not exist?",
        )
    })?;

    // construct path string
    let mut osstr_path: OsString = "file:///".into();
    osstr_path.push(canonical);

    // we need to round-trip to string, as not all os strings are 8 bytes
    let pdf_string = osstr_path.into_string().map_err(|_| {
        glib::error::Error::new(
            glib::FileError::Inval,
            "Path invalid (contains non-utf8 characters)",
        )
    })?;

    CString::new(pdf_string).map_err(|_| {
        glib::error::Error::new(
            glib::FileError::Inval,
            "Path invalid (contains NUL characters)",
        )
    })
}
