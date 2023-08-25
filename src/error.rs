use lexopt::Error as ArgsError;
use serialport::Error as SerialError;
use std::{ffi::CString, sync::mpsc::SendError, io::Error as IoError, result::Result as StdResult};
use time::error::{Format as FormattingError, IndeterminateOffset as OffsetError};

pub type Result<T> = StdResult<T, Error>;

/// All the possible Errors in this application
#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Serial(SerialError),
    IndeterminateTimeZone(OffsetError),
    TimeStampFormatting(FormattingError),
    UnableToParseArguments(ArgsError),
    SendError(SendError<CString>),
    /// No serial port given from the command line
    MissingPortArgument,
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

impl From<OffsetError> for Error {
    fn from(value: OffsetError) -> Self {
        Self::IndeterminateTimeZone(value)
    }
}

impl From<FormattingError> for Error {
    fn from(value: FormattingError) -> Self {
        Self::TimeStampFormatting(value)
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
