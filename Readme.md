# Crosslogger CLI
 The server for Rust crosslogger library, which is a remote logging library for Rust.

 Crosslogger is a remote-logger library that allows you to log messages from your Rust application to a remote server. This server is the server that receives the logs from the library. It is written in Rust.


## Installation
### From source
```bash
git clone
cd crosslogger-server
cargo build --release
```

### From crates.io
```bash
cargo install crosslogger-server
```

## Usage
### Running the server
```bash
crosslogger-cli
```

### Using the library
```rust
 // import the library
 use crosslogger::{connect_logger, log};
 use crosslogger::core::logger::LogType;

 // replace the address with the address of the server
 connect_logger!("127.0.0.1:15635"); // connect to crosslogger server

 log!(LogType::INFO, "{}", 4); // log a message to crosslogger server of type Info.
```