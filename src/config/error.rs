use std::{error, fmt, io};
use serde_yaml;

/// An error that encapsulates all possible configuration errors.
#[derive(Debug)]
pub enum Error {
    /// An error that occured while doing configuration related I/O.
    Io(io::Error),

    /// An error that occured while parsing a yaml configuration.
    Yaml(serde_yaml::Error),

    /// No configuration file was specified so we attempted to load configuration
    /// from the default location in the user's home directory. However, no home
    /// directory could be found.
    NoHomeDir,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Error {
        Error::Yaml(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match *self {
            Io(ref err) => err.fmt(f),
            Yaml(ref err) => err.fmt(f),
            NoHomeDir => write!(f, "\
No configuration file was specified.
We instead looked for a configuration file in your home directory.
However, we could not find your home directory."),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match *self {
            Io(ref err) => err.description(),
            Yaml(ref err) => err.description(),
            NoHomeDir => "No home directory found",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        use self::Error::*;
        match *self {
            Io(ref err) => Some(err),
            Yaml(ref err) => Some(err),
            NoHomeDir => None,
        }
    }
}
