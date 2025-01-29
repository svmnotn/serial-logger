use serialport::SerialPort;
use std::process::exit;

mod args;
use args::{parse_args, print_help};

mod error;
use error::{Error::PortNotFound, Result};

mod loops;
use loops::{ReadLoop, WriteLoop};

mod output;

mod serial;
use serial::{available_ports, open_port, print_available_ports};

mod time_stamp;

fn get_path_from_port(specified_port: &str) -> Result<String> {
    available_ports()?
        .find(|(port, _)| port.eq_ignore_ascii_case(specified_port))
        .ok_or(PortNotFound)
        .map(|(_, path)| path)
}

fn main() -> Result<()> {
    let args = parse_args()?;
    if args.port.is_none() && args.path.is_none() {
        println!("Missing path argument or --port\n");
        println!("Here is a list of available ports:\n");
        print_available_ports()?;
        println!();
        print_help();
        exit(1);
    }

    let path = if let Some(p) = &args.port {
        get_path_from_port(p)?
    } else if let Some(p) = &args.path {
        p.clone()
    } else {
        unreachable!("Somehow we didn't have either a path or a port and we didn't exit before trying to use them...")
    };
    let mut port = open_port(&path, &args)?;

    println!("Receiving data on {} at {} baud", &path, args.baud_rate);

    let mut read_loop = ReadLoop::from_args(&args)?;
    let write_loop = WriteLoop::from_args(&args);

    port.write_data_terminal_ready(true)?;
    port.write_request_to_send(true)?;

    loop {
        write_loop.run(&mut port)?;
        read_loop.run(&mut port)?;
    }
}
