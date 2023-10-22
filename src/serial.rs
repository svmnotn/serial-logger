use crate::error::Result;
use serialport::SerialPort;

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
pub fn open_port(path: &str, baud_rate: u32, timeout_in_seconds: u64) -> Result<impl SerialPort> {
    use serialport::{DataBits, FlowControl, Parity, StopBits};
    use std::time::Duration;

    serialport::new(path, baud_rate)
        .flow_control(FlowControl::Software)
        .data_bits(DataBits::Eight)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .timeout(Duration::from_secs(timeout_in_seconds))
        .open_native()
        .map_err(Into::into)
}
