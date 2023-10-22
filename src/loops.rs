use crate::{
    args::Args,
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

pub trait RunableLoop {
    fn from_args(args: &Args) -> Result<Self> where Self: Sized;
    fn run(&mut self, port: &mut impl SerialPort) -> Result<()>;
}

pub struct ReadLoop {
    buffer: Vec<u8>,
    output: Output,
    read_bytes: usize,
}

impl RunableLoop for ReadLoop {
    fn from_args(args: &Args) -> Result<Self> {
        Ok(Self {
            buffer: vec![0; args.buffer_size],
            output: Output::from_args(args)?,
            read_bytes: 0,
        })
    }

    fn run(&mut self, port: &mut impl SerialPort) -> Result<()> {
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
        }

        let mut last_new_line = 0;
        for (i, _) in total_bytes
            .iter()
            .enumerate()
            .filter(|(_, byte)| **byte == b'\n')
        {
            self.output.write_all(get_timestamp().as_bytes())?;
            self.output.write_all(b": ")?;
            self.output
                .write_all(&total_bytes[last_new_line..(i + 1)])?;
            last_new_line = i + 1;
        }

        if last_new_line > 0 && self.read_bytes > 0 {
            self.buffer.copy_within(last_new_line..self.read_bytes, 0);
            self.read_bytes = (last_new_line..self.read_bytes).len();
        }

        Ok(())
    }
}

pub struct BinaryReadLoop {
    buffer: Vec<u8>,
    output: Output,
}

impl RunableLoop for BinaryReadLoop {
    fn from_args(args: &Args) -> Result<Self> {
        Ok(Self {
            buffer: vec![0; args.buffer_size],
            output: Output::from_args(args)?,
        })
    }

    fn run(&mut self, port: &mut impl SerialPort) -> Result<()> {
        use std::io::Write;

        let read_bytes = match port.read(&mut self.buffer) {
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::Interrupted | std::io::ErrorKind::TimedOut
                ) =>
            {
                Ok(0)
            }
            v => v,
        }?;

        if read_bytes > 0 {
            let total_bytes = &self.buffer[..read_bytes];

            self.output.write_all(get_timestamp().as_bytes())?;
            self.output.write_all(b": ")?;
            for c in total_bytes {
                self.output.write_all(format!("{c:X}").as_bytes())?;
            }
            self.output.write_all(b"\n")?;
        }

        Ok(())
    }
}

pub struct WriteLoop {
    kill_thread: Sender<()>,
    rx_input: Receiver<CString>,
    windows_ending: bool,
}

impl RunableLoop for WriteLoop {
    fn from_args(args: &Args) -> Result<Self> {
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

        Ok(WriteLoop {
            kill_thread,
            rx_input,
            windows_ending: args.windows_line_ending,
        })
    }

    fn run(&mut self, port: &mut impl SerialPort) -> Result<()> {
        let input = self.rx_input.try_recv();

        if let Err(TryRecvError::Disconnected) = input {
            return Err(StdInThreadDisconnected);
        }

        if let Ok(input) = input {
            port.write_all(input.as_bytes())?;
            if self.windows_ending {
                port.write_all(&[b'\r'])?;
            }
            port.write_all(&[b'\n'])?;
        }

        Ok(())
    }
}

impl Drop for WriteLoop {
    fn drop(&mut self) {
        println!("Killing StdIn thread {:?}", self.kill_thread.send(()));
    }
}
