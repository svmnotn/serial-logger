use crate::{error::{Result, Error::MissingPortArgument}, serial::available_ports};

const USAGE_STRING: &'static str = r#"Usage: serial-logger [--print] [-h|--help] [-b|--baud=NUM] [-t|--timeout=NUM] [-s|--buffer-size=NUM] [-l|--log=LOG_FILE_NAME] [--port=SERIAL_PORT_NAME] SERIAL_PORT_NAME

--help: Prints this message
--print: Prints out all available serial ports

--baud - Default: 115_200
--timeout - Unit: Seconds, Default: 120
--buffer-size - Default: 100000
--log - Optional

--port: Will be used instead of the positional argument if defined"#;

/// Parsed Command Line Arguments
pub struct Args {
    /// The Serial Port's Serial Number, taken to be the Port's Name
    pub port: String,
    /// The baud rate to use for the serial port
    pub baud_rate: u32,
    /// The amount of time to wait to receive data before timing out
    pub timeout_in_seconds: u64,
    /// How large to make the `line` buffer, this should roughly match to the maximum amount output by a single printf, not the size of a single line.
    pub buffer_size: usize,
    /// The path to an optional Log File
    pub log_file: Option<String>,
}

/// Parse the Command Line Arguments into [Args]
pub fn parse_args() -> Result<Args> {
    use lexopt::prelude::*;

    let mut port = None;
    let mut baud_rate = 115_200;
    let mut timeout_in_seconds = 120;
    let mut buffer_size = 100000;
    let mut log_file = None;
    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Short('b') | Long("baud") => {
                baud_rate = parser.value()?.parse()?;
            }
            Short('t') | Long("timeout") => {
                timeout_in_seconds = parser.value()?.parse()?;
            }
            Short('l') | Long("log") => {
                log_file.replace(parser.value()?.string()?);
            }
            Short('s') | Long("buffer-size") => {
                buffer_size = parser.value()?.parse()?;
            }
            Long("port") => {
                port.replace(parser.value()?.string()?);
            }
            Value(val) if port.is_none() => {
                port = Some(val.string()?);
            }
            Long("print") => {
                for (i, (name, _)) in available_ports()?.enumerate() {
                    println!("{i}: {name}");
                }
                std::process::exit(0);
            }
            Short('h') | Long("help") => {
                print!("{}", USAGE_STRING);
                std::process::exit(0);
            }
            _ => Err(arg.unexpected())?,
        }
    }

    Ok(Args {
        port: port.ok_or(MissingPortArgument)?,
        baud_rate,
        timeout_in_seconds,
        buffer_size,
        log_file,
    })
}
