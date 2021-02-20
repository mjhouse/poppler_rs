use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int, c_uint};
use std::path;

use cairo_sys;
use glib_sys;

use crate::interface;
use crate::util;
use crate::error::Error;

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
pub struct PopplerDocument(*mut interface::PopplerDocument);

#[derive(Debug)]
pub struct PopplerPage(*mut interface::PopplerPage);

impl PopplerDocument {
    pub fn new_from_file<P: AsRef<path::Path>>(
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
        unsafe {
            match interface::poppler_document_get_title(self.0) {
                r if !r.is_null()  => {
                    let r = CString::from(CStr::from_ptr(r))
                        .into_string()
                        .ok();
                    r
                }
                _ => None 
            }
        }
    }
    pub fn get_metadata(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_char = interface::poppler_document_get_metadata(self.0);
            if ptr.is_null() {
                None
            } else {
                CString::from_raw(ptr).into_string().ok()
            }
        }
    }
    pub fn get_pdf_version_string(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_char = interface::poppler_document_get_pdf_version_string(self.0);
            if ptr.is_null() {
                None
            } else {
                CString::from_raw(ptr).into_string().ok()
            }
        }
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
    use cairo::*;

    use std::{fs::File, io::Read};

    #[test]
    fn test1() {
        let filename = "/home/mjhouse/Downloads/poppler-rs-master/test/text.pdf";
        let doc = PopplerDocument::new_from_file(filename, "").unwrap();
        let num_pages = doc.get_n_pages();

        println!("Document has {} page(s)", num_pages);

        let mut surface = PdfSurface::new(420.0, 595.0, "output.pdf").unwrap();
        let ctx = Context::new(&mut surface);

        // FIXME: move iterator to poppler
        for page_num in 0..num_pages {
            let page = doc.get_page(page_num).unwrap();
            let (w, h) = page.get_size();
            println!("page {} has size {}, {}", page_num, w, h);
            surface.set_size(w, h);

            ctx.save();
            page.render(&ctx);

            println!("Text: {:?}", page.get_text().unwrap_or(""));

            ctx.restore();
            ctx.show_page();
        }
        // g_object_unref (page);
        //surface.write_to_png("file.png");
        surface.finish();
    }

    #[test]
    fn test2_from_file() {
        let path = "/home/mjhouse/Downloads/poppler-rs-master/test/text.pdf";
        let doc: PopplerDocument = PopplerDocument::new_from_file(path, "upw").unwrap();
        let num_pages = doc.get_n_pages();
        let title = doc.get_title().unwrap();
        let metadata = doc.get_metadata();
        let version_string = doc.get_pdf_version_string();
        let permissions = doc.get_permissions();
        let page: PopplerPage = doc.get_page(0).unwrap();
        let (w, h) = page.get_size();

        println!(
            "Document {} has {} page(s) and is {}x{}",
            title, num_pages, w, h
        );
        println!(
            "Version: {:?}, Permissions: {:x?}",
            version_string, permissions
        );

        assert!(metadata.is_some());
        assert_eq!(version_string, Some("PDF-1.3".to_string()));
        assert_eq!(permissions, 0xff);

        assert_eq!(title, "This is a test PDF file");

        let mut surface = ImageSurface::create(Format::ARgb32, w as i32, h as i32).unwrap();
        let ctx = Context::new(&mut surface);

        ctx.save();
        page.render(&ctx);
        ctx.restore();
        ctx.show_page();

        // let mut f: File = File::create("out.png").unwrap();
        // surface.write_to_png(&mut f).expect("Unable to write PNG");
    }

    #[test]
    fn test2_from_data() {
        let path = "/home/mjhouse/Downloads/poppler-rs-master/test/text.pdf";
        let mut file = File::open(path).unwrap();
        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data).unwrap();
        let doc: PopplerDocument =
            PopplerDocument::new_from_data(&mut data[..], "upw").unwrap();
        let num_pages = doc.get_n_pages();
        let title = doc.get_title().unwrap();
        let metadata = doc.get_metadata();
        let version_string = doc.get_pdf_version_string();
        let permissions = doc.get_permissions();
        let page: PopplerPage = doc.get_page(0).unwrap();
        let (w, h) = page.get_size();

        println!(
            "Document {} has {} page(s) and is {}x{}",
            title, num_pages, w, h
        );
        println!(
            "Version: {:?}, Permissions: {:x?}",
            version_string, permissions
        );

        assert!(metadata.is_some());
        assert_eq!(version_string, Some("PDF-1.3".to_string()));
        assert_eq!(permissions, 0xff);
    }

    #[test]
    fn test3() {
        let mut data = vec![];

        assert!(PopplerDocument::new_from_data(&mut data[..], "upw").is_err());
    }
}
