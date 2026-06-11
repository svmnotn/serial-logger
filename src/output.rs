use crate::{
    args::Args,
    error::Result,
    serial::{available_ports, PortName},
};
use std::{
    fs::File,
    io::{stdout, Write},
    path::Path,
};

fn canonical_port_name(args: &Args) -> Result<Option<PortName>> {
    if let Some(requested_name) = args.port.as_deref() {
        return Ok(available_ports()?
            .find(|(name, _)| name.eq_ignore_ascii_case(requested_name))
            .map(|(name, _)| name)
            .or_else(|| Some(requested_name.to_owned())));
    }

    if let Some(path) = args.path.as_deref() {
        return Ok(available_ports()?
            .find(|(_, port_path)| port_path == path)
            .map(|(name, _)| name));
    }

    Ok(None)
}

fn default_log_file_name(args: &Args) -> Result<String> {
    use chrono::{Local, SecondsFormat};

    let timestamp = Local::now().to_rfc3339_opts(SecondsFormat::Secs, false);
    let port_name = canonical_port_name(args)?
        .as_deref()
        .or_else(|| {
            args.path
                .as_deref()
                .and_then(|path| Path::new(path).file_name()?.to_str())
        })
        .unwrap_or("unknown-port")
        .replace('/', "_");

    Ok(format!("{timestamp}-{port_name}.log"))
}

/// A wrapper around stdout OR a File, allowing us to write to either
/// based on if logging is enabled and if [Args] has a `log_file` specified
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
    /// Create the desired output by checking if logging is enabled
    pub fn from_args(args: &Args) -> Result<Self> {
        Ok(match (args.log_enabled, args.silent) {
            (false, true) => Self::None,
            (false, false) => Self::Std(stdout().lock()),
            (true, true) => {
                let log_file = args
                    .log_file
                    .as_deref()
                    .map(ToOwned::to_owned)
                    .map(Ok)
                    .unwrap_or_else(|| default_log_file_name(args))?;
                Self::Fs(File::create(&log_file)?)
            }
            (true, false) => {
                let log_file = args
                    .log_file
                    .as_deref()
                    .map(ToOwned::to_owned)
                    .map(Ok)
                    .unwrap_or_else(|| default_log_file_name(args))?;
                Self::Both {
                    file: File::create(&log_file)?,
                    stdout: stdout().lock(),
                }
            }
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
