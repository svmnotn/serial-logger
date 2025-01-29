use lexopt::Error as ArgsError;
use serialport::Error as SerialError;
use std::{ffi::CString, io::Error as IoError, result::Result as StdResult, sync::mpsc::SendError};

pub type Result<T> = StdResult<T, Error>;

/// All the possible Errors in this application
#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    Io(IoError),
    Serial(SerialError),
    UnableToParseArguments(ArgsError),
    SendError(SendError<CString>),
    /// The specified command line serial port was not found
    PortNotFound,
    /// The stdin thread disconnected unexpectedly
    StdInThreadDisconnected,
    /// --flow-control was not one of [n, s, h]
    InvalidFlowControlArgument,
    /// --data-bits was not one of [8, 7, 6, 5]
    InvalidDataBitsArgument,
    /// --parity was not one of [n, e, o]
    InvalidParityArgument,
    /// --stop-bits was not one of [1, 2]
    InvalidStopBitsArgument,
    /// --string-parsing was not one of [utf8, utf16be, utf16le]
    InvalidStringParsingArgument,
}

impl From<IoError> for Error {
    fn from(value: IoError) -> Self {
        Self::Io(value)
    }
}

impl From<SerialError> for Error {
    fn from(value: SerialError) -> Self {
        Self::Serial(value)
    }
}

impl From<ArgsError> for Error {
    fn from(value: ArgsError) -> Self {
        Self::UnableToParseArguments(value)
    }
}

impl From<SendError<CString>> for Error {
    fn from(value: SendError<CString>) -> Self {
        Self::SendError(value)
    }
}
