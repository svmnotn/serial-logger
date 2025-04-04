use crate::{
    error::{Error, Result},
    serial::print_available_ports,
};
use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::process::exit;

const USAGE_STRING: &str = r"Usage: serial-logger [--print] [-h|--help] [-b|--baud=NUM] [--flow-control=n|s|h] [--data-bits=5|6|7|8] [--parity=n|o|e] [--stop-bits=1|2] [-t|--timeout=NUM] [--buffer-size=NUM] [-w|--windows-line-ending] [--string-parsing=utf8] [-l|--log=LOG_FILE_NAME] [-s|--silent] [--port=SERIAL_PORT_NAME] SERIAL_PORT_PATH

--help: Prints this message
--print: Prints out all available serial ports

--baud: The baud rate to use for the serial port - Default: 115_200
--flow-control: How to handle flow control, n: None, s: Software, h: Hardware - Accepted Values: [n,s,h] Default: Software
--data-bits: How many data bits - Accepted Values: [5,6,7,8] Default: 8
--parity: Parity checking modes, n: None, o: Odd, e: Even - Accepted Values: [n,o,e] Default: None
--stop-bits: Number of Stop Bits - Accepted Values: [1,2] Default: 1
--timeout: Set the amount of time to wait to receive data before timing out - Unit: Seconds, Default: 1
--buffer-size: How large to make the `line` buffer, this should roughly match to the maximum amount output by a single printf, not the size of a single line - Default: 100000
--windows-line-ending: if this is present, when sending through the serial port it will interpret newlines as '\r\n' instead of just '\n' - Default off
--log: The path to a log file - Optional
--silent: Don't write output to stdout - Optional
--string-parsing: Assume the incoming data is utf8[/utf16le/utf16be, pending rust update] - Default: utf8

--port: Will be used instead of the positional argument if defined, should just be the serial port's serial number.";

#[derive(Debug, Clone, Copy)]
pub enum StringParsingMode {
    Utf8,
    // Utf16,
    // Utf16LE,
    // Utf16BE,
}

/// Parsed Command Line Arguments
#[derive(Debug)]
pub struct Args {
    /// The Serial Port's Path
    pub path: Option<String>,
    /// The Serial Port's Serial Number, taken to be the Port's Name
    pub port: Option<String>,
    /// The baud rate to use for the serial port
    pub baud_rate: u32,
    /// How to handle flow control
    pub flow_control: FlowControl,
    /// How many data bits
    pub data_bits: DataBits,
    /// Parity checking modes
    pub parity: Parity,
    /// Number of Stop Bits
    pub stop_bits: StopBits,
    /// The amount of time to wait to receive data before timing out
    pub timeout_in_seconds: u64,
    /// How large to make the `line` buffer, this should roughly match to the maximum amount output by a single printf, not the size of a single line
    pub buffer_size: usize,
    /// When sending through the serial port it will interpret newlines as '\r\n' instead of just '\n'
    pub windows_line_ending: bool,
    /// Don't output to stdout
    pub silent: bool,
    /// The path to an optional Log File
    pub log_file: Option<String>,
    // Assume UTF-8/UTF-16LE/UTF-16BE
    pub string_parsing_mode: StringParsingMode,
}

pub fn print_help() {
    println!("{USAGE_STRING}");
}

/// Parse the Command Line Arguments into [Args]
pub fn parse_args() -> Result<Args> {
    use lexopt::prelude::*;

    let mut path = None;
    let mut port = None;
    let mut baud_rate = 115_200;
    let mut flow_control = FlowControl::Software;
    let mut data_bits = DataBits::Eight;
    let mut parity = Parity::None;
    let mut stop_bits = StopBits::One;
    let mut timeout_in_seconds = 1;
    let mut buffer_size = 100000;
    let mut silent = false;
    let mut windows_line_ending = false;
    let mut string_parsing_mode = StringParsingMode::Utf8;
    let mut log_file = None;
    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Long("print") => {
                print_available_ports()?;
                exit(0);
            }
            Short('h') | Long("help") => {
                print_help();
                exit(0);
            }
            Short('b') | Long("baud") => {
                baud_rate = parser.value()?.parse()?;
            }
            Long("flow-control") => {
                flow_control = match &*parser.value()?.to_string_lossy() {
                    "n" => FlowControl::None,
                    "s" => FlowControl::Software,
                    "h" => FlowControl::Hardware,
                    _ => return Err(Error::InvalidFlowControlArgument),
                };
            }
            Long("data-bits") => {
                data_bits = match &*parser.value()?.to_string_lossy() {
                    "8" => DataBits::Eight,
                    "7" => DataBits::Seven,
                    "6" => DataBits::Six,
                    "5" => DataBits::Five,
                    _ => return Err(Error::InvalidDataBitsArgument),
                };
            }
            Long("parity") => {
                parity = match &*parser.value()?.to_string_lossy() {
                    "n" => Parity::None,
                    "e" => Parity::Even,
                    "o" => Parity::Odd,
                    _ => return Err(Error::InvalidParityArgument),
                };
            }
            Long("stop-bits") => {
                stop_bits = match &*parser.value()?.to_string_lossy() {
                    "1" => StopBits::One,
                    "2" => StopBits::Two,
                    _ => return Err(Error::InvalidStopBitsArgument),
                };
            }
            Short('t') | Long("timeout") => {
                timeout_in_seconds = parser.value()?.parse()?;
            }
            Long("buffer-size") => {
                buffer_size = parser.value()?.parse()?;
            }
            Long("string-parsing") => {
                string_parsing_mode = match &*parser.value()?.to_string_lossy() {
                    "utf8" => StringParsingMode::Utf8,
                    // "utf16be" => StringParsingMode::Utf16BE,
                    // "utf16le" => StringParsingMode::Utf16LE,
                    _ => return Err(Error::InvalidStringParsingArgument),
                };
            }
            Short('w') | Long("windows-line-ending") => {
                windows_line_ending = true;
            }
            Short('s') | Long("silent") => {
                silent = true;
            }
            Short('l') | Long("log") => {
                log_file.replace(parser.value()?.string()?);
            }
            Long("port") => {
                port.replace(parser.value()?.string()?);
            }
            Value(val) if port.is_none() => {
                path = Some(val.string()?);
            }
            _ => Err(arg.unexpected())?,
        }
    }

    Ok(Args {
        path,
        port,
        baud_rate,
        flow_control,
        data_bits,
        parity,
        stop_bits,
        timeout_in_seconds,
        buffer_size,
        windows_line_ending,
        string_parsing_mode,
        silent,
        log_file,
    })
}
