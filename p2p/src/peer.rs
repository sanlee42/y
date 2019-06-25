use std::net::{TcpStream, SocketAddr};
use crate::conn::Conn;

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
}