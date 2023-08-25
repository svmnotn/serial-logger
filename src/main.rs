use serialport::SerialPort;

mod args;
use args::{parse_args, Args};

mod error;
use error::{
    Error::PortNotFound,
    Result,
};

mod loops;
use loops::ReadLoop;

mod output;

mod serial;
use serial::{available_ports, open_port};

use crate::loops::WriteLoop;

mod time_stamp;

fn open_user_specified_port(args: &Args) -> Result<impl SerialPort> {
    available_ports()?
        .find(|(port, _)| port.eq_ignore_ascii_case(&args.port))
        .ok_or(PortNotFound)
        .and_then(|(_, path)| open_port(&path, args.baud_rate, args.timeout_in_seconds))
}

fn main() -> Result<()> {
    let args = parse_args()?;
    let mut port = open_user_specified_port(&args)?;

    println!(
        "Receiving data on {} at {} baud",
        &args.port, args.baud_rate
    );

    let mut read_loop = ReadLoop::from_args(&args)?;
    let write_loop = WriteLoop::from_args(&args);

    port.write_data_terminal_ready(true)?;
    port.write_request_to_send(true)?;

    loop {
        write_loop.run(&mut port)?;
        read_loop.run(&mut port)?;
    }
}
