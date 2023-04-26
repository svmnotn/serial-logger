use std::io::{Read, Write};

mod args;
use args::parse_args;

mod error;
use error::{Error::PortNotFound, Result};

mod output;
use output::Output;

mod serial;
use serial::{available_ports, open_port};

mod time;
use crate::time::get_timestamp;

fn main() -> Result<()> {
    let args = parse_args()?;
    let mut port = available_ports()?
        .find(|(port, _)| port.eq_ignore_ascii_case(&args.port))
        .ok_or(PortNotFound)
        .and_then(|(_, path)| open_port(&path, args.baud_rate, args.timeout_in_seconds))?;

    println!(
        "Receiving data on {} at {} baud",
        &args.port, args.baud_rate
    );

    let mut serial_buf: Vec<u8> = vec![0; args.buffer_size];
    let mut out = Output::from_args(&args)?;
    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(read_byte_count) => {
                out.write_all(get_timestamp()?.as_bytes())?;
                out.write_all(": ".as_bytes())?;
                out.write_all(&serial_buf[..read_byte_count])?;
            }
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => (),
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => eprintln!("Timed Out!"),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
