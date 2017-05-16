#![feature(try_from)]

extern crate url;
extern crate tokio_core;
extern crate tokio_proto;

pub mod connection;

pub use connection::Connection;

use std::convert::TryFrom;

#[derive(Debug)]
pub enum CommandType {
    HeartbeatRequestCommand,
    HeartbeatResponseCommand,

    //
}

impl TryFrom<u32> for CommandType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use CommandType::*;
        Ok(match value {
            0x01 => HeartbeatRequestCommand,
            0x02 => HeartbeatResponseCommand,
            _ => return Err( () )
        })
    }
}

//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = CommandType::try_from(0x12).unwrap();
    }
}
