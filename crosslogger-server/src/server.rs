use std::io::Read;
use std::net::{TcpListener, TcpStream};
use crosslogger::core::logger::LogType;

pub struct Server {
    listener: TcpListener,
}

pub struct Log {
    pub log_type: LogType,
    pub message: String,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Self {
            listener: TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap(),
        }
    }

    pub fn accept(&self) -> Option<TcpStream> {
        match self.listener.accept() {
            Ok((stream, _)) => {
                Some(stream)
            },
            Err(_) => {
                None
            }
        }
    }

    pub fn receive_packet(&self, stream: &mut TcpStream) -> Option<Log> {
        let mut header_buffer = [0; 5];
        match stream.read(&mut header_buffer) {
            Ok(len) => {
                if len == 0 || len != 5 {
                    return None;
                }
                let length = u32::from_le_bytes([header_buffer[0], header_buffer[1], header_buffer[2], header_buffer[3]]) as usize;
                let mut buffer = vec![0; length];
                match stream.read(&mut buffer) {
                    Ok(len) => {
                        if len == 0 {
                            return None;
                        }
                        let value = String::from_utf8(buffer).unwrap();
                        let log = Log {
                            log_type: LogType::from_number(header_buffer[4]),
                            message: value,
                        };
                        Some(log)
                    },
                    Err(_) => {
                        return None;
                    }
                }
            }
            Err(_) => {
                return None;
            }
        }
    }

}
