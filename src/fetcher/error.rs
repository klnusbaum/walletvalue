use std::{error, fmt, io};

/// An error that encapsulates all possible fether errors.
#[derive(Debug)]
pub enum Error {
    /// An error that indicates the wallet being fetch cannot be found.
    NotFound(String),

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
            NotFound(ref address) => write!(f, "Couldn't find wallet at address {}", address),
            Io(ref err) => err.fmt(f),
            UnknownCurrency(ref currency) => write!(f, "Unknown Currencty: {}", currency),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match *self {
            NotFound(_) => "Wallet not found",
            Io(ref err) => err.description(),
            UnknownCurrency(_) => "Unknown currency",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        use self::Error::*;
        match *self {
            NotFound(_) => None,
            Io(ref err) => Some(err),
            UnknownCurrency(_) => None,
        }
    }
}
