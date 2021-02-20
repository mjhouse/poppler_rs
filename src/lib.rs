#![allow(dead_code)]

mod interface;
mod poppler;
mod util;
mod error;

pub use poppler::{PopplerDocument, PopplerPage};
