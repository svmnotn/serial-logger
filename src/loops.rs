use crate::{
    args::{Args, StringParsingMode},
    error::{Error::StdInThreadDisconnected, Result},
    output::Output,
    time_stamp::get_timestamp,
};
use serialport::SerialPort;
use std::{
    ffi::CString,
    io::{stdin, BufRead},
    sync::mpsc::{Receiver, Sender, TryRecvError},
    thread::spawn,
};

pub struct ReadLoop {
    buffer: Vec<u8>,
    output: Output,
    mode: StringParsingMode,
    read_bytes: usize,
}

impl ReadLoop {
    pub fn from_args(args: &Args) -> Result<Self> {
        Ok(Self {
            buffer: vec![0; args.buffer_size],
            output: Output::from_args(args)?,
            mode: args.string_parsing_mode,
            read_bytes: 0,
        })
    }

    pub fn run(&mut self, port: &mut impl SerialPort) -> Result<()> {
        use std::io::Write;

        self.read_bytes = match port.read(&mut self.buffer[self.read_bytes..]) {
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::Interrupted | std::io::ErrorKind::TimedOut
                ) =>
            {
                Ok(0)
            }
            v => v,
        }? + self.read_bytes;

        let total_bytes = &mut self.buffer[..self.read_bytes];
        for v in total_bytes.iter_mut() {
            if *v == b'\r' {
                *v = b' ';
            }

            if *v == b'\0' {
                *v = b'?';
            }
        }

        let mut last_new_line = 0;
        for (i, _) in total_bytes
            .iter()
            .enumerate()
            .filter(|(_, byte)| **byte == b'\n')
        {
            self.output.write_all(get_timestamp().as_bytes())?;
            self.output.write_all(b": ")?;
            let line = &total_bytes[last_new_line..(i + 1)];
            match self.mode {
                StringParsingMode::Utf8 => self
                    .output
                    .write_all(String::from_utf8_lossy(line).as_bytes())?,
                // Pending stabilization of the below functions
                // StringParsingMode::Utf16BE => self.output.write_all(String::from_utf16be_lossy(line).as_bytes())?,
                // StringParsingMode::Utf16LE => self.output.write_all(String::from_utf16le_lossy(line).as_bytes())?,
            }
            last_new_line = i + 1;
        }

        if last_new_line > 0 && self.read_bytes > 0 {
            self.buffer.copy_within(last_new_line..self.read_bytes, 0);
            self.read_bytes = (last_new_line..self.read_bytes).len();
        }

        Ok(())
    }
}

pub struct WriteLoop {
    kill_thread: Sender<()>,
    rx_input: Receiver<CString>,
    windows_ending: bool,
}

impl WriteLoop {
    pub fn from_args(args: &Args) -> Self {
        let (tx_user_input, rx_input) = std::sync::mpsc::channel();
        let (kill_thread, kill_command) = std::sync::mpsc::channel();

        spawn(move || -> Result<()> {
            let mut user_input = stdin().lock();
            let mut buf = String::new();
            loop {
                if let Ok(_) | Err(TryRecvError::Disconnected) = kill_command.try_recv() {
                    return Err(StdInThreadDisconnected);
                }

                if user_input.read_line(&mut buf)? > 0 {
                    tx_user_input.send(CString::new(buf.trim_end().as_bytes()).unwrap())?;
                    buf.clear();
                }
            }
        });

        WriteLoop {
            kill_thread,
            rx_input,
            windows_ending: args.windows_line_ending,
        }
    }

    pub fn run(&self, port: &mut impl SerialPort) -> Result<()> {
        let input = self.rx_input.try_recv();

        if let Err(TryRecvError::Disconnected) = input {
            return Err(StdInThreadDisconnected);
        }

        if let Ok(input) = input {
            port.write_all(input.as_bytes())?;
            if self.windows_ending {
                port.write_all(b"\r")?;
            }
            port.write_all(b"\r")?;
        }

        Ok(())
    }
}

impl Drop for WriteLoop {
    fn drop(&mut self) {
        println!("Killing StdIn thread {:?}", self.kill_thread.send(()));
    }
}
