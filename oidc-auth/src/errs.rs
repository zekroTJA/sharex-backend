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
    EnvInitFailed(envconfig::Error),
    Request(reqwest::Error),
    JwtValidation(jsonwebtoken::errors::Error),
    TokenNoKidHeader,
    TokenNoKeyInJwks,
    TokenJwkInvalidAlgorithm,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind() {
            ErrorKind::EnvInitFailed(err) => write!(f, "init from env failed: {}", err),
            ErrorKind::Request(err) => write!(f, "request failed: {}", err),
            ErrorKind::JwtValidation(err) => write!(f, "jwt validation failed: {}", err),
            ErrorKind::TokenNoKidHeader => write!(f, "the identity token has no kid header"),
            ErrorKind::TokenNoKeyInJwks => {
                write!(f, "no key was found in JWKS with the id tokens kid")
            }
            ErrorKind::TokenJwkInvalidAlgorithm => {
                write!(f, "key algorithm is not RSA")
            }
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
        ErrorKind::EnvInitFailed(value).into()
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        ErrorKind::Request(value).into()
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        ErrorKind::JwtValidation(value).into()
    }
}
