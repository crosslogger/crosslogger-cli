pub mod server;

#[cfg(test)]
mod tests {
    use crosslogger::{connect_logger, log};
    use crosslogger::core::logger::LogType;

    #[test]
    fn it_works() {
        match connect_logger!("172.25.2.205:8787") {
            Ok(_) => {
                for i in 0..1000000 {
                    log!(LogType::INFO, "Hello {}", i);
                }
                // wait for 1 second
                std::thread::sleep(std::time::Duration::from_secs(1));
            },
            Err(e) => {
                println!("Error: {}", e);
                assert!(false, "make sure crosslogger-server is running on port 8787");
            }
        }
    }
}
