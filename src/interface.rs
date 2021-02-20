use cairo_sys;
use glib_sys;
use std::os::raw::{c_char, c_double, c_int, c_uint, c_void};

#[repr(C)]
pub struct PopplerDocument(c_void);

#[repr(C)]
pub struct PopplerPage(c_void);

type GError = *mut *mut glib_sys::GError;

// FIXME: *const instead of mut pointers?
#[link(name = "poppler-glib")]
extern "C" {
    pub fn poppler_document_new_from_file(
        uri: *const c_char,
        password: *const c_char,
        error: GError,
    ) -> *mut PopplerDocument;

    pub fn poppler_document_new_from_data(
        data: *const c_char,
        length: c_int,
        password: *const c_char,
        error: GError,
    ) -> *mut PopplerDocument;

    pub fn poppler_document_get_n_pages(document: *const PopplerDocument) -> c_int;

    pub fn poppler_document_get_page(
        document: *const PopplerDocument,
        index: c_int,
    ) -> *mut PopplerPage;

    pub fn poppler_document_get_title(document: *const PopplerDocument) -> *const c_char;

    pub fn poppler_document_get_metadata(document: *mut PopplerDocument) -> *mut c_char;

    pub fn poppler_document_get_pdf_version_string(document: *mut PopplerDocument) -> *mut c_char;

    pub fn poppler_document_get_permissions(document: *mut PopplerDocument) -> c_uint;

    pub fn poppler_page_get_size(
        page: *mut PopplerPage,
        width: *mut c_double,
        height: *mut c_double,
    );

    pub fn poppler_page_render(page: *mut PopplerPage, cairo: *mut cairo_sys::cairo_t);

    pub fn poppler_page_render_for_printing(page: *mut PopplerPage, cairo: *mut cairo_sys::cairo_t);

    pub fn poppler_page_get_text(page: *mut PopplerPage) -> *mut c_char;
}
