use std::error;
use std::io;
use std::fmt;
use std::net::ToSocketAddrs;
use tokio_core::reactor::Handle;
use tokio_proto::{Connect, TcpClient};
use url::Url;

use tokio_proto::multiplex::Multiplex;

#[derive(Debug)]
pub struct Connection {
    url: Url
}

#[derive(Debug)]
pub enum Error {
    InvalidUrlScheme,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidUrlScheme => "Invalid Url Scheme: only tcp and discover are supported",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(error::Error::description(self))
    }
}

impl Connection {
    pub fn new(url: &Url) -> Result<Connection, Error> {
        match url.scheme() {
            "tcp" => Ok(Connection { url: url.clone() }),
            "discover" => unimplemented!(),
            _ => Err(Error::InvalidUrlScheme)
        }
    }

    pub fn connect<Kind>(&self, handle: &Handle) -> io::Result<Connect<Kind, ()>> {
        let socket_addr = self.url.with_default_port(|_| Ok(1113))?
                                  .to_socket_addrs()?
                                  .next().ok_or(io::Error::from(io::ErrorKind::NotFound))?;

        // Ok(TcpClient::new(unimplemented!()).connect(&socket_addr, handle))
        unimplemented!()
    }
}
