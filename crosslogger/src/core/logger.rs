use std::io::Write;

pub enum LogType {
    INFO,
    WARN,
    ERROR,
}

impl LogType {
    pub fn to_string(&self) -> String {
        match self {
            LogType::INFO => "INFO".to_string(),
            LogType::WARN => "WARN".to_string(),
            LogType::ERROR => "ERROR".to_string(),
        }
    }
    pub fn to_number(&self) -> u8 {
        match self {
            LogType::INFO => 0,
            LogType::WARN => 1,
            LogType::ERROR => 2,
        }
    }

    pub fn from_number(number: u8) -> LogType {
        match number {
            0 => LogType::INFO,
            1 => LogType::WARN,
            2 => LogType::ERROR,
            _ => LogType::INFO,
        }
    }
}

pub fn print_log(value: String, log_type: LogType) {
    let packet = create_log_packet(value, log_type);
    loop {
        let connector = super::writer::Connector::get();
        match connector {
            Some(connector) => {
                match connector.client.write(&packet) {
                    Ok(_) => {
                        connector.client.flush().unwrap();
                        break;
                    },
                    Err(_) => {
                        connector.reconnect();
                    }
                }
            },
            None => { }
        }
    }
}

fn create_log_packet(value: String, log_type: LogType) -> Vec<u8> {
    let mut packet = Vec::new();
    // push length of value as u32
    packet.extend_from_slice(&(value.len() as u32).to_le_bytes());

    // push log type as u8
    packet.push(log_type.to_number());

    // push value
    packet.extend(value.as_bytes());
    packet
}