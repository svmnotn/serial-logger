use crate::error::Result;
use serialport::SerialPort;
use super::args::Args;

pub type PortName = String;
pub type PortAddress = String;

/// Get an iterator over the available USB ports that have a serial number
pub fn available_ports() -> Result<impl Iterator<Item = (PortName, PortAddress)>> {
    use serialport::{SerialPortInfo, SerialPortType, UsbPortInfo};

    Ok(serialport::available_ports()?.into_iter().filter_map(
        |SerialPortInfo {
             port_name: name,
             port_type: ty,
         }| match ty {
            SerialPortType::UsbPort(UsbPortInfo {
                vid: _,
                pid: _,
                serial_number: Some(serial),
                manufacturer: _,
                product: _,
            }) => Some((serial, name)),
            _ => None,
        },
    ))
}

/// Prints all the available ports with the format: `SERIAL @ PATH`
pub fn print_available_ports() -> Result<()> {
    for (name, path) in available_ports()? {
        println!("{name} @ {path}");
    }

    Ok(())
}

/// Attempt to open the port at the specified path, with the given baud_rate and timeout
pub fn open_port(path: &str, args: &Args) -> Result<impl SerialPort> {
    use std::time::Duration;

    serialport::new(path, args.baud_rate)
        .flow_control(args.flow_control)
        .data_bits(args.data_bits)
        .parity(args.parity)
        .stop_bits(args.stop_bits)
        .timeout(Duration::from_secs(args.timeout_in_seconds))
        .open_native()
        .map_err(Into::into)
}
