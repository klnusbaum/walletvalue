use std::{error, fmt};
use config;
use fetcher;

/// A `Result` type alias for this crate's `Error` type.
pub type Result<T> = ::std::result::Result<T, Error>;

/// An error that encapsulates all possible errors in this crate.
#[derive(Debug)]
pub enum Error {
    /// An error that occured while parsing a configuration.
    Config(config::Error),

    /// An error that occured when fetching a wallet's value.
    Fetch(fetcher::Error),
}

impl From<config::Error> for Error {
    fn from(err: config::Error) -> Error {
        Error::Config(err)
    }
}

impl From<fetcher::Error> for Error {
     fn from(err: fetcher::Error) -> Error {
        Error::Fetch(err)
    }
}
   

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match *self {
            Config(ref err) => write!(f, "Error getting config: {}", err),
            Fetch(ref err) => write!(f, "Error fetching wallet value: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match *self {
            Config(ref err) => err.description(),
            Fetch(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        use self::Error::*;
        match *self {
            Config(ref err) => Some(err),
            Fetch(ref err) => Some(err),
        }
    }
}
