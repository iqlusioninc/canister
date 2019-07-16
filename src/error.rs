use abscissa_core::Error;
use hyper;
use reqwest;
use serde_json;
use std::fmt::{self, Display};
use std::{io, string::FromUtf8Error};

/// Abscissa error type for canister
pub struct CanisterError(Error<CanisterErrorKind>);

/// Types of errors which occur internally within canister
#[derive(Fail, Clone, Debug, Eq, PartialEq)]
pub enum CanisterErrorKind {
    /// I/O operation failed
    #[fail(display = "I/O operation failed")]
    IoError,

    /// Parse Error
    #[fail(display = "Parse error")]
    ParseError,

    /// Reqwest Error
    #[fail(display = "Reqwest error")]
    ReqwestError,

    /// Content Digest missing
    #[fail(display = "no content digest in response (access control issue?)")]
    ContentDigestMissing,
}

impl Display for CanisterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<abscissa_core::Error<CanisterErrorKind>> for CanisterError {
    fn from(err: abscissa_core::Error<CanisterErrorKind>) -> Self {
        CanisterError(err)
    }
}

impl From<hyper::mime::FromStrError> for CanisterError {
    fn from(_err: hyper::mime::FromStrError) -> Self {
        CanisterError(CanisterErrorKind::ParseError.into())
    }
}

impl From<io::Error> for CanisterError {
    fn from(_err: io::Error) -> Self {
        CanisterError(CanisterErrorKind::IoError.into())
    }
}

impl From<reqwest::Error> for CanisterError {
    fn from(_err: reqwest::Error) -> Self {
        CanisterError(CanisterErrorKind::ReqwestError.into())
    }
}

impl From<serde_json::Error> for CanisterError {
    fn from(_err: serde_json::Error) -> Self {
        CanisterError(CanisterErrorKind::ParseError.into())
    }
}

impl From<FromUtf8Error> for CanisterError {
    fn from(_err: FromUtf8Error) -> Self {
        CanisterError(CanisterErrorKind::ParseError.into())
    }
}
