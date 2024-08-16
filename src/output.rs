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
    /// Output the same line to both a file and stdout
    Both {
        file: std::fs::File,
        stdout: std::io::StdoutLock<'static>,
    },
    /// Silence all output
    None,
}

impl Output {
    /// Create the desired output by checking if [Args] has a `log_file` specified
    pub fn from_args(args: &Args) -> Result<Self> {
        Ok(match (args.log_file.as_ref(), args.silent) {
            (None, true) => Self::None,
            (None, false) => Self::Std(stdout().lock()),
            (Some(f), true) => Self::Fs(File::create(f)?),
            (Some(f), false) => Self::Both {
                file: File::create(f)?,
                stdout: stdout().lock(),
            },
        })
    }
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Self::None => Ok(buf.len()),
            Self::Std(out) => out.write(buf),
            Self::Fs(out) => out.write(buf),
            Self::Both { file, stdout } => {
                file.write(buf)?;
                stdout.write(buf)
            }
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Self::None => Ok(()),
            Self::Std(out) => out.flush(),
            Self::Fs(out) => out.flush(),
            Self::Both { file, stdout } => {
                file.flush()?;
                stdout.flush()
            }
        }
    }
}
