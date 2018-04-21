use std::{error, fmt, io};

/// An error that encapsulates all possible fether errors.
#[derive(Debug)]
pub enum Error {
    /// An error that occured while doing fetching related I/O.
    Io(io::Error),

    /// An error that occured due to an unknown currency.
    UnknownCurrency(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match *self {
            Io(ref err) => err.fmt(f),
            UnknownCurrency(ref currency) => write!(f, "Unknown Currencty: {}", currency),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match *self {
            Io(ref err) => err.description(),
            UnknownCurrency(_) => "Unknown currency",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        use self::Error::*;
        match *self {
            Io(ref err) => Some(err),
            UnknownCurrency(_) => None,
        }
    }
}
