use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Given data is empty")]
    EmptyData,

    #[error(transparent)]
    NullError(#[from] std::ffi::NulError),

    #[error(transparent)]
    GlibError(#[from] glib::error::Error),
}
