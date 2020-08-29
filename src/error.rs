use std::{io, fmt};
use crate::error::ErrorKind::{SerialPort, Unknown, Io};
use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug, Clone)]
pub struct Error {
    description: String,
    kind: ErrorKind
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    Io(io::ErrorKind),
    SerialPort,
    ParseError,
    Unknown,
}


impl Error {
    /// Instantiates a new error
    pub fn new<T: Into<String>>(kind: ErrorKind, description: T) -> Self {
        Error {
            kind: kind,
            description: description.into(),
        }
    }

    /// Returns the corresponding `ErrorKind` for this error.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        fmt.write_str(&self.description)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.description
    }
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Error {
        Error::new(ErrorKind::Io(io_error.kind()), format!("{}", io_error))
    }
}

impl From<serialport::Error> for Error {
    fn from(serialport_error: serialport::Error) -> Error {
        let kind = match serialport_error.kind {
            serialport::ErrorKind::NoDevice => SerialPort,
            serialport::ErrorKind::InvalidInput => SerialPort,
            serialport::ErrorKind::Unknown => Unknown,
            serialport::ErrorKind::Io(io) => Io(io),
        };
        Error::new(kind, format!("{}", serialport_error))
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(float_parse_error: ParseFloatError) -> Self {
        Error::new(ErrorKind::ParseError, format!("Unable to parse float: {}", float_parse_error))
    }
}

impl From<ParseIntError> for Error {
    fn from(int_parse_error: ParseIntError) -> Self {
        Error::new(ErrorKind::ParseError, format!("Unable to parse float: {}", int_parse_error))
    }
}

