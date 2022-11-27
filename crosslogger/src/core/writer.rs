use std::net::TcpStream;

pub struct Connector {
    pub client: TcpStream,
    pub address: String,
}

static mut CONNECTOR: Option<Connector> = None;

impl Connector {
    // singleton
    pub fn connect<A: std::net::ToSocketAddrs>(addr: A) -> Result<bool, std::io::Error> {
        let address = addr.to_socket_addrs().unwrap().next().unwrap().to_string();
        unsafe {
            if CONNECTOR.is_none() {
                let client = TcpStream::connect(&address)?;
                // client.set_nonblocking(true)?;
                CONNECTOR = Some(Connector {
                    client,
                    address,
                });
            }
        }
        Ok(true)
    }

    pub fn get() -> Option<&'static mut Connector> {
        unsafe {
            CONNECTOR.as_mut()
        }
    }

    pub fn reconnect(&mut self) {
        let client = match TcpStream::connect(&self.address) {
            Ok(client) => client,
            Err(_) => return,
        };
        self.client = client;
    }
}