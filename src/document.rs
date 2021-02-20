use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::path::Path;

use crate::util;
use crate::interface;
use crate::error::Error;

use crate::page::PopplerPage;
use crate::interface::{
    PopplerDocumentPtr,
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
pub struct PopplerDocument(pub(crate) *mut PopplerDocumentPtr);

impl PopplerDocument {
    pub fn new_from_file<P: AsRef<Path>>(
        p: P,
        password: &str,
    ) -> Result<PopplerDocument,Error> {
        let pass = CString::new(password)?;
        let path = util::path_to_glib_url(p)?;

        let doc = call!(
            interface::poppler_document_new_from_file,
            path.as_ptr(), 
            pass.as_ptr())?;

        Ok(PopplerDocument(doc))
    }
    pub fn new_from_data<C: AsRef<[u8]>>(
        content:  C,
        password: &str,
    ) -> Result<PopplerDocument,Error> {
        
        let pass = CString::new(password)?;
        let data = content.as_ref();

        if data.len() == 0 {
            return Err(Error::EmptyData);
        }

        let doc = call!(
            interface::poppler_document_new_from_data,
            data.as_ptr() as *const c_char,
            data.len() as c_int,
            pass.as_ptr())?;

        Ok(PopplerDocument(doc))
    }

    pub fn get_title(&self) -> Option<String> {
        string!(interface::poppler_document_get_title(self.0))
    }
    pub fn get_metadata(&self) -> Option<String> {
        string!(interface::poppler_document_get_metadata(self.0))
    }
    pub fn get_pdf_version_string(&self) -> Option<String> {
        string!(interface::poppler_document_get_pdf_version_string(self.0))
    }
    pub fn get_permissions(&self) -> u8 {
        unsafe { interface::poppler_document_get_permissions(self.0) as u8 }
    }

    pub fn get_n_pages(&self) -> usize {
        (unsafe { interface::poppler_document_get_n_pages(self.0) }) as usize
    }

    pub fn get_page(&self, index: usize) -> Option<PopplerPage> {
        match unsafe { interface::poppler_document_get_page(self.0, index as c_int) } {
            ptr if ptr.is_null() => None,
            ptr => Some(PopplerPage(ptr)),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    
    use std::{
        fs::File, 
        io::Read, 
        io::Bytes
    };

    macro_rules! file {
        ( $f:expr ) => {
            {
                use std::path::PathBuf;
                let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                d.push("test");
                d.push($f);
                d
            }
        }
    }

    macro_rules! data {
        ( $f:expr ) => {
            {
                File::open($f)
                    .expect("Could not open file")
                    .bytes()
                    .map(Result::ok)
                    .filter_map(|v| v)
                    .collect::<Vec<u8>>()
            }
        }
    }

    #[test]
    fn create_document_from_data_success() {
        let path = file!("text.pdf");
        let data = data!(path);
        let file = PopplerDocument::new_from_data(&data[..], "");
        assert!(file.is_ok());
    }

    #[test]
    fn create_document_from_data_failure() {
        let data = vec![];
        let file = PopplerDocument::new_from_data(&data[..], "");
        assert!(file.is_err());
    }

    #[test]
    fn create_document_from_path_success() {
        let path = file!("text.pdf");
        let file = PopplerDocument::new_from_file(&path, "");
        assert!(file.is_ok());
    }

    #[test]
    fn create_document_from_path_failure() {
        let path = file!("NOFILE.pdf");
        let file = PopplerDocument::new_from_file(&path, "");
        assert!(file.is_err());
    }

    #[test]
    fn document_has_title() {
        let path = file!("text.pdf");
        let file = PopplerDocument::new_from_file(&path, "")
            .expect("Could not open file");
        assert!(file.get_title().is_some());
    }

    #[test]
    fn document_has_page_count() {
        let path = file!("text.pdf");
        let file = PopplerDocument::new_from_file(&path, "")
            .expect("Could not open file");
        assert_eq!(file.get_n_pages(),1);
    }

    #[test]
    fn document_has_metadata() {
        let path = file!("text.pdf");
        let file = PopplerDocument::new_from_file(&path, "")
            .expect("Could not open file");
        assert!(file.get_metadata().is_some());
    }

    #[test]
    fn document_has_pdf_version_string() {
        let path = file!("text.pdf");
        let file = PopplerDocument::new_from_file(&path, "")
            .expect("Could not open file");
        assert!(file.get_pdf_version_string().is_some());
    }

    #[test]
    fn document_has_permissions() {
        let path = file!("text.pdf");
        let file = PopplerDocument::new_from_file(&path, "")
            .expect("Could not open file");
        assert_eq!(file.get_permissions(),0xff);
    }

    #[test]
    fn document_has_page() {
        let path = file!("text.pdf");
        let file = PopplerDocument::new_from_file(&path, "")
            .expect("Could not open file");
        assert!(file.get_page(0).is_some());
    }

}
