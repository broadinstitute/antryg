use std::fmt::{Debug, Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum ErrorKind {
    Antryg,
    IO,
}

mod kinds {
    pub(crate) const ANTRYG: &str = "antryg";
    pub(crate) const IO: &str = "I/O";
}

#[derive(Debug)]
pub enum Error {
    Original(String),
    Foreign(ErrorKind, Box<dyn std::error::Error>),
    Wrapped(String, Box<Error>),
}

impl Error {
    pub fn from_foreign<E>(kind: ErrorKind, error: E) -> Self
        where E: std::error::Error + 'static {
        Error::Foreign(kind, Box::new(error))
    }
    pub fn wrap<E>(message: String, error: E) -> Self where E: Into<Error> {
        Error::Wrapped(message, Box::new(error.into()))
    }
    pub fn wrap_str<E>(message: &str, error: E) -> Self where E: Into<Error> {
        Error::wrap(message.to_string(), error)
    }
    pub fn wrapped(self, message: String) -> Self {
        Error::Wrapped(message, Box::new(self))
    }
    pub fn wrapped_str(self, message: String) -> Self {
        self.wrapped(message.to_string())
    }
    pub fn wrap_err<T, E, M>(result: Result<T, E>, message: M) -> Result<T, Error>
        where E: Into<Error>, M: FnOnce() -> String {
        result.map_err(|error| Error::wrap(message(), error))
    }
    pub fn wrap_err_str<T, E>(result: Result<T, E>, message: &str) -> Result<T, Error>
        where E: Into<Error> {
        Error::wrap_err(result, || message.to_string())
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::Antryg => write!(f, "{}", kinds::ANTRYG),
            ErrorKind::IO => write!(f, "{}", kinds::IO),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Original(message) => write!(f, "{}", message),
            Error::Foreign(kind, error) => write!(f, "{}: {}", kind, error),
            Error::Wrapped(message, error) => write!(f, "{}: {}", message, error)
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Original(_) => None,
            Error::Foreign(_, error) => Some(error.as_ref()),
            Error::Wrapped(_, error) => Some(error)
        }
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self { Error::Original(message) }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self { Error::from(message.to_string()) }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self { Error::from_foreign(ErrorKind::IO, error) }
}