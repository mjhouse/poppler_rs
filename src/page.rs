use std::ffi::{CStr};
use std::os::raw::{c_double};

use crate::interface;
use crate::interface::PopplerPagePtr;

/// A single page from a PDF document ([see freedesktop.org])
///
/// [see freedesktop.org]: https://poppler.freedesktop.org/api/glib/poppler-Poppler-Page.html
pub struct PopplerPage(pub(crate) *mut PopplerPagePtr);

impl PopplerPage {
    /// Get the size of the page (width,height)
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

    // Render a page
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

    use cairo::*;

    use std::fs::File;
    use crate::document::PopplerDocument;

    macro_rules! path {
        ( $f:expr ) => {{
            use std::path::PathBuf;
            let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            d.push("test");
            d.push($f);
            d
        }};
    }

    macro_rules! file {
        ( $f:expr ) => {{
            PopplerDocument::new_from_file(&path!($f), "").expect("Could not open file")
        }};
    }

    #[test]
    fn page_get_size() {
        let file = file!("text.pdf");
        let (w, h) = file.get_page(0).unwrap().get_size();

        assert_eq!(w, 595.0);
        assert_eq!(h, 842.0);
    }

    #[test]
    fn page_render() {
        let file1 = file!("text.pdf").get_page(0).unwrap();

        let (w, h) = file1.get_size();

        {
            let mut surface = PdfSurface::new(w, h, path!("output/render1.pdf")).unwrap();
            let ctx = Context::new(&mut surface);
            file1.render(&ctx);
        }

        let file2 = file!("output/render1.pdf").get_page(0).unwrap();

        assert_eq!(file1.get_text(), file2.get_text());
    }

    #[test]
    fn page_render_for_printing_png() {
        let file1 = file!("text.pdf").get_page(0).unwrap();

        let (w, h) = file1.get_size();
        let mut image = File::create(path!("output/render2.png")).unwrap();

        let result = {
            let mut surface = ImageSurface::create(Format::ARgb32, w as i32, h as i32).unwrap();
            let ctx = Context::new(&mut surface);
            file1.render_for_printing(&ctx);
            surface.write_to_png(&mut image)
        };

        assert!(result.is_ok());
    }

    #[test]
    fn page_render_for_printing_svg() {
        let file1 = file!("text.pdf").get_page(0).unwrap();

        let (w, h) = file1.get_size();
        let image = File::create(path!("output/render3.svg")).unwrap();

        let mut surface = SvgSurface::for_stream(w, h, image).unwrap();
        let ctx = Context::new(&mut surface);
        file1.render_for_printing(&ctx);
    }

    #[test]
    fn page_get_text() {
        let file = file!("short_text.pdf").get_page(0).unwrap();
        assert_eq!(file.get_text(), Some("TEST"))
    }
}
