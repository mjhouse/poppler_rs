//! # Overview
//!
//! This library is a partial (very partial) wrapper around the [Poppler](https://poppler.freedesktop.org/) 
//! library available on many flavors of linux.
//!
//! # Installation
//!
//! You'll need to install poppler and cairo, if you don't have them:
//!
//! ```sh
//! sudo apt install -y     \
//!     libpoppler-glib-dev \
//!     libpoppler-dev      \
//!     libcairo2-dev       \
//!     libglib2.0-dev
//! ```
//! Add this crate to your Cargo.toml, import document and page:
//!
//! ```rust
//! use poppler_rs::{PopplerDocument,PopplerPage};
//! ```
//!
//! -and go nuts. Most of Poppler is not available wrapped but If you need
//! something specific, submit a pull request.

mod error;
mod interface;
mod util;

mod document;
mod page;

pub use document::PopplerDocument;
pub use page::PopplerPage;
