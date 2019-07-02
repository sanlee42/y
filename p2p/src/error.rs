use std::{fmt, io};
use std::fmt::Formatter;
use std::num::ParseIntError;
use base64::DecodeError;

#[derive(Debug)]
pub enum Error {
    PeerExist,
    Connection(io::Error),
    InvalidNonceError(ParseIntError),
    InvalidBodyError(DecodeError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { write!(f, "{}", self) }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error { Error::Connection(e) }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Error { Error::InvalidNonceError(e) }
}

impl From<base64::DecodeError> for Error {
    fn from(e: DecodeError) -> Error { Error::InvalidBodyError(e) }
}

