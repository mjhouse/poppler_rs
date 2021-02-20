use std::ffi::{CStr, CString};
use std::os::raw::{c_double, c_void};

use crate::util;
use crate::interface;
use crate::error::Error;

use crate::interface::{
    PopplerPagePtr
};

macro_rules! string {
    ( $p: expr ) => {
        unsafe {
            match $p {
                r if !r.is_null()  => {
                    let c = CString::from(CStr::from_ptr(r))
                        .into_string()
                        .ok();
                    libc::free(r as *mut c_void);
                    c
                }
                _ => None 
            }
        }
    }
}

macro_rules! call {
    ( $f:path $(, $v:expr )* ) => {
        {
            let mut e: *mut glib_sys::GError = std::ptr::null_mut();
            unsafe {
                match $f( $( $v, )* &mut e as *mut *mut glib_sys::GError) {
                    r if !r.is_null() => Ok(r),
                    _ => Err(util::to_glib_error(e))
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct PopplerPage(pub(crate) *mut PopplerPagePtr);

impl PopplerPage {
    pub fn get_size(&self) -> (f64, f64) {
        let mut width: f64 = 0.0;
        let mut height: f64 = 0.0;

        unsafe {
            interface::poppler_page_get_size(
                self.0,
                &mut width as *mut f64 as *mut c_double,
                &mut height as *mut f64 as *mut c_double,
            )
        }

        (width, height)
    }

    pub fn render(&self, ctx: &cairo::Context) {
        let ctx_raw = ctx.to_raw_none();
        unsafe { interface::poppler_page_render(self.0, ctx_raw) }
    }

    pub fn render_for_printing(&self, ctx: &cairo::Context) {
        let ctx_raw = ctx.to_raw_none();
        unsafe { interface::poppler_page_render_for_printing(self.0, ctx_raw) }
    }

    pub fn get_text(&self) -> Option<&str> {
        match unsafe { interface::poppler_page_get_text(self.0) } {
            ptr if ptr.is_null() => None,
            ptr => unsafe { Some(CStr::from_ptr(ptr).to_str().unwrap()) },
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
    }
}
