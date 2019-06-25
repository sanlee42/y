use std::{fmt, io};
use std::fmt::Formatter;

#[derive(Debug)]
pub enum Error {
    PeerExist,
    Connection(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { write!(f, "{}", self) }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error { Error::Connection(e) }
}