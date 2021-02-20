use cairo_sys;
use glib_sys;
use std::os::raw::{c_char, c_double, c_int, c_uint, c_void};

#[repr(C)]
pub struct PopplerDocumentPtr(c_void);

#[repr(C)]
pub struct PopplerPagePtr(c_void);

type GError = *mut *mut glib_sys::GError;

// FIXME: *const instead of mut pointers?
#[link(name = "poppler-glib")]
extern "C" {
    pub fn poppler_document_new_from_file(
        uri: *const c_char,
        password: *const c_char,
        error: GError,
    ) -> *mut PopplerDocumentPtr;

    pub fn poppler_document_new_from_data(
        data: *const c_char,
        length: c_int,
        password: *const c_char,
        error: GError,
    ) -> *mut PopplerDocumentPtr;

    pub fn poppler_document_get_n_pages(document: *const PopplerDocumentPtr) -> c_int;

    pub fn poppler_document_get_page(
        document: *const PopplerDocumentPtr,
        index: c_int,
    ) -> *mut PopplerPagePtr;

    pub fn poppler_document_get_title(document: *const PopplerDocumentPtr) -> *mut c_char;

    pub fn poppler_document_get_metadata(document: *mut PopplerDocumentPtr) -> *mut c_char;

    pub fn poppler_document_get_pdf_version_string(document: *mut PopplerDocumentPtr) -> *mut c_char;

    pub fn poppler_document_get_permissions(document: *mut PopplerDocumentPtr) -> c_uint;

    pub fn poppler_page_get_size(
        page: *mut PopplerPagePtr,
        width: *mut c_double,
        height: *mut c_double,
    );

    pub fn poppler_page_render(page: *mut PopplerPagePtr, cairo: *mut cairo_sys::cairo_t);

    pub fn poppler_page_render_for_printing(page: *mut PopplerPagePtr, cairo: *mut cairo_sys::cairo_t);

    pub fn poppler_page_get_text(page: *mut PopplerPagePtr) -> *mut c_char;
}
