use std::{io, path::PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PkiError {
    #[error("File operation failed")]
    FileOperation(#[from] io::Error),
    #[error("File `{0}` does not exsit")]
    FileNotExsit(PathBuf),
    #[error("File format error")]
    InvalidFormat,
    #[error("Unknown error")]
    Unknown,
}

pub type PkiResult<T> = Result<T, PkiError>;
