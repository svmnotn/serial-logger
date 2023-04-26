use serialport::Error as SerialError;
use std::{io::Error as IoError, result::Result as StdResult};
use time::error::Format as FormattingError;
use time::error::IndeterminateOffset as OffsetError;
use lexopt::Error as ArgsError;

pub type Result<T> = StdResult<T, Error>;

/// All the possible Errors in this application
#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Serial(SerialError),
    IndeterminateTimeZone(OffsetError),
    TimeStampFormatting(FormattingError),
    UnableToParseArguments(ArgsError),
    /// No serial port given from the command line
    MissingPortArgument,
    /// The specified command line serial port was not found
    PortNotFound,
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
