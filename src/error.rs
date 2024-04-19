use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum ErrorKind {
    Antryg
}

mod kinds {
    pub(crate) const ANTRYG: &str = "antryg";
}
#[derive(Debug)]
pub enum Error {
    Original(String),
    Foreign(ErrorKind, Box<dyn std::error::Error>),
    Wrapped(String, Box<Error>)
}


impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::Antryg => write!(f, "{}", kinds::ANTRYG)
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