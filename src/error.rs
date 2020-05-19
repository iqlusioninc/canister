use abscissa_core::error::{BoxError, Context};
use std::fmt::{self, Display};
use std::ops::Deref;
use std::{io, string::FromUtf8Error};
use thiserror::Error;

/// Abscissa error type for canister
pub struct Error(Box<Context<ErrorKind>>);

/// Types of errors which occur internally within canister
#[derive(Copy, Clone, Debug, Eq, Error, PartialEq)]
pub enum ErrorKind {
    /// I/O operation failed
    #[error("I/O operation failed")]
    IoError,

    /// Parse Error
    #[error("Parse error")]
    ParseError,

    /// Reqwest Error
    #[error("Reqwest error")]
    ReqwestError,

    /// Content Digest missing
    #[error("no content digest in response (access control issue?)")]
    ContentDigestMissing,
}

impl ErrorKind {
    /// Create an error context from this error
    pub fn context(self, source: impl Into<BoxError>) -> Context<ErrorKind> {
        Context::new(self, Some(source.into()))
    }
}

impl Deref for Error {
    type Target = Context<ErrorKind>;

    fn deref(&self) -> &Context<ErrorKind> {
        &self.0
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(ctx: Context<ErrorKind>) -> Self {
        Error(Box::new(ctx))
    }
}

impl From<hyper::header::ToStrError> for Error {
    fn from(err: hyper::header::ToStrError) -> Self {
        ErrorKind::ParseError.context(err).into()
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        ErrorKind::IoError.context(err).into()
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        ErrorKind::ReqwestError.context(err).into()
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        ErrorKind::ParseError.context(err).into()
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Error(ErrorKind::ParseError.context(err).into())
    }
}

impl From<reqwest::UrlError> for Error {
    fn from(err: reqwest::UrlError) -> Self {
        Error(ErrorKind::ParseError.context(err).into())
    }
}
