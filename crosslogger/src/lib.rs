pub mod core;

/// Log a message to crosslogger server.
/// It is only available when crosslogger is connected, and it is not available in release mode.
/// Usage:
/// ```
/// use crosslogger::{connect_logger, log};
/// use crosslogger::core::logger::LogType;
///
/// connect_logger!("127.0.0.1:15635"); // connect to crosslogger server
///
/// log!(LogType::INFO, "{}", 4); // log a message to crosslogger server of type Info.
/// ```
/// which will send '4' to crosslogger server.
///
/// Packet structure:
/// `| length of value (u32) | Log Type (u8) | value (String) |`
#[macro_export]
#[cfg(debug_assertions)]
macro_rules! log {
    ($x:expr, $($arg:tt)*) => {
        {
            $crate::core::logger::print_log(format!($($arg)*), $x);
        }
    };
}

/// It is not available in release mode.
#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! log {
    ($($arg:tt)*) => {
        {
        }
    };
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! connect_logger {
    ($ip:expr) => {
        {
            // connect to tcp server
            let connector: Result<bool, std::io::Error> = $crate::core::writer::Connector::connect($ip);
            connector
        }
    };
}

/// It is not available in release mode.
#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! connect_logger {
    ($x:expr, $reconnect: expr) => {
        {
            // connect to tcp server
            let result: Result<bool, std::io::Error> = Ok(true);
            result
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connection_works() {
        // stopwatch
        let start = std::time::Instant::now();

        let connected = connect_logger!("127.0.0.1:8787");
        assert!(connected.is_ok(), "Failed to connect to crosslogger server.");
        for i in 0..100 {
            log!("{}: {}", i, i);
        }
        let elapsed = start.elapsed();
        println!("Elapsed: {}ms", elapsed.as_millis());
    }
}
