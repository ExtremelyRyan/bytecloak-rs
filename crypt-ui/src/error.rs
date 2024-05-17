//! Main Crate Error
//!

#[allow(dead_code)]
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
    Core(#[from] crypt_cloud::crypt_core::error::Error),

    #[error(transparent)]
    CloudErrorFromSubModulePertainingToOtherIssuesFoundElsewhereInTheProject(
        #[from] crypt_cloud::error::Error,
    ),
}
