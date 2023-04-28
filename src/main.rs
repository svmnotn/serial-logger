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
    let mut read_byte_count = 0;
    loop {
        read_byte_count = match port.read(&mut serial_buf[read_byte_count..]) {
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => Ok(0),
            v => v,
        }? + read_byte_count;

        let total_bytes = &mut serial_buf[..read_byte_count];
        for v in total_bytes.iter_mut() {
            if *v == b'\r' {
                *v = b' ';
            }
        }

        let mut last_new_line = 0;
        for (i, _) in total_bytes
            .iter()
            .enumerate()
            .filter(|(_, byte)| **byte == b'\n')
        {
            out.write_all(get_timestamp()?.as_bytes())?;
            out.write_all(b": ")?;
            out.write_all(&total_bytes[last_new_line..(i + 1)])?;
            last_new_line = i + 1;
        }

        if last_new_line > 0 && read_byte_count > 0 {
            serial_buf.copy_within(last_new_line..read_byte_count, 0);
            read_byte_count = (last_new_line..read_byte_count).len();
        }
    }
}
