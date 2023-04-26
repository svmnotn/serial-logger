use crate::{args::Args, error::Result};
use std::{
    fs::File,
    io::{stdout, Write},
};

/// A wrapper around stdout OR a File, allowing us to write to either
/// based on if the `log_file` field of [Args] is filled
pub enum Output {
    /// Locks Standard Out so that we have exclusive writing to it, saving time
    Std(std::io::StdoutLock<'static>),
    /// Creates a file or truncates it
    Fs(std::fs::File),
}

impl Output {
    /// Create the desired output by checking if [Args] has a `log_file` specified
    pub fn from_args(args: &Args) -> Result<Self> {
        Ok(if let Some(log) = &args.log_file {
            Output::Fs(File::create(log)?)
        } else {
            Output::Std(stdout().lock())
        })
    }
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Self::Std(out) => out.write(buf),
            Self::Fs(out) => out.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Self::Std(out) => out.flush(),
            Self::Fs(out) => out.flush(),
        }
    }
}
