use crate::error::Result;
use serialport::SerialPort;

pub type PortName = String;
pub type PortAddress = String;

/// Get an iterator over the available USB ports that have a serial number
pub fn available_ports() -> Result<impl Iterator<Item = (PortName, PortAddress)>> {
    use serialport::{SerialPortInfo, SerialPortType};

    Ok(serialport::available_ports()?
        .into_iter()
        .filter(
            |SerialPortInfo {
                 port_name: _,
                 port_type: ty,
             }| matches!(ty, SerialPortType::UsbPort(p) if p.serial_number.is_some()),
        )
        .map(
            |SerialPortInfo {
                 port_name,
                 port_type,
             }| {
                (
                    match port_type {
                        SerialPortType::UsbPort(port) => port.serial_number.unwrap(),
                        _ => unreachable!(),
                    },
                    port_name,
                )
            },
        ))
}

/// Attempt to open the port at the specified path, with the given baud_rate and timeout
pub fn open_port(path: &str, baud_rate: u32, timeout_in_seconds: u64) -> Result<impl SerialPort> {
    use serialport::{DataBits, FlowControl, Parity, StopBits};
    use std::time::Duration;

    serialport::new(path, baud_rate)
        .flow_control(FlowControl::None)
        .data_bits(DataBits::Eight)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .timeout(Duration::from_secs(timeout_in_seconds))
        .open_native()
        .map_err(Into::into)
}
