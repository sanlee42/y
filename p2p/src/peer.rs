use std::net::{TcpStream, SocketAddr};
use crate::conn::Conn;
use crate::error::Error;

#[derive(Clone)]
pub struct Peer {
    con: Conn,
    pub addr: SocketAddr,
}

impl Peer {
    pub fn new(addr: SocketAddr, stream: TcpStream) -> Peer {
        Peer {
            con: Conn::new(stream).unwrap(),
            addr,
        }
    }

    pub fn send(&self, msg: String) -> Result<(), Error> {
        self.con.poll.send(msg)?;
        Ok(())
    }
}