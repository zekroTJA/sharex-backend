use core::fmt;

#[derive(Debug)]
pub struct StatusError(Box<StatusErrorKind>);

#[derive(Debug)]
pub enum StatusErrorKind {
    NotFound,
    Forbidden,
    Unauthorized,
    Unknown(u16),
}

impl StatusError {
    pub fn kind(&self) -> &StatusErrorKind {
        &self.0
    }
}

impl fmt::Display for StatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind() {
            StatusErrorKind::Unauthorized => write!(f, "unauthorized"),
            StatusErrorKind::Forbidden => write!(f, "forbidden"),
            StatusErrorKind::NotFound => write!(f, "not found"),
            StatusErrorKind::Unknown(status) => write!(f, "status {status}"),
        }
    }
}

impl std::error::Error for StatusError {}

impl From<u16> for StatusError {
    fn from(value: u16) -> Self {
        Self(Box::new(match value {
            401 => StatusErrorKind::Unauthorized,
            403 => StatusErrorKind::Forbidden,
            404 => StatusErrorKind::NotFound,
            _ => StatusErrorKind::Unknown(value),
        }))
    }
}
