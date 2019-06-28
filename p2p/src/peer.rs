use std::net::{TcpStream, SocketAddr};
use std::thread;
use crate::conn::Conn;
use crate::error::Error;

#[derive(Clone)]
pub struct Peer {
    con: Conn,
    pub addr: SocketAddr,
}

impl Peer {
    pub fn new(addr: SocketAddr, stream: TcpStream) -> Peer {
        let peer = Peer {
            con: Conn::new(stream).unwrap(),
            addr,
        };
        let _peer = peer.clone();
        thread::spawn(move ||
            loop {
                _peer.recv();
            }
        );
        peer
    }

    pub fn send(&self, msg: String) -> Result<(), Error> {
        self.con.poll.send(msg)?;
        Ok(())
    }


    pub fn recv(&self) {
        let msg = self.con.poll.read_reciver.lock().unwrap().recv().unwrap();
        let msg= String::from_utf8(msg).unwrap();
        println!("recv:{:?}", msg);
    }
}