//! Main Crate Error
//! 

use crypt_core::error::Error as core_error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// For starter, to remove as code matures.
    #[error("Generic error: {0}")]
    Generic(String),
    /// For starter, to remove as code matures.
    #[error("Static error: {0}")]
    Static(&'static str),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Core(#[from] core_error),
}

