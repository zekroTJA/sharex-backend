use std::fmt;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    EnvConfig(envconfig::Error),
    Misconfiguration(&'static str),
    Unexpected(Box<dyn std::error::Error>),
    StorageError(storage::Error),
    ImageNotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind() {
            ErrorKind::EnvConfig(err) => write!(f, "failed configuration from environment: {err}"),
            ErrorKind::Misconfiguration(msg) => write!(f, "misconfiguration: {msg}"),
            ErrorKind::StorageError(err) => write!(f, "storage error: {err}"),
            ErrorKind::ImageNotFound => write!(f, "image not found"),
            ErrorKind::Unexpected(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<ErrorKind> for Error {
    fn from(value: ErrorKind) -> Self {
        Error(value.into())
    }
}

impl From<envconfig::Error> for Error {
    fn from(value: envconfig::Error) -> Self {
        ErrorKind::EnvConfig(value).into()
    }
}

impl From<storage::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        ErrorKind::StorageError(value).into()
    }
}
