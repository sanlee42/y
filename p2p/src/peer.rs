use std::net::{TcpStream, SocketAddr};
use std::thread;
use crate::conn::Conn;
use crate::error::Error;
use crate::util;
#[derive(Clone)]
pub struct Peer {
    con: Conn,
    pub addr: SocketAddr,
}


struct PeerDataBuf {
    nonce: u32,
    buf: Vec<String>,
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
                _peer.recv(util::process_msg);
            }
        );
        peer
    }

    pub fn send(&self, msg: String) -> Result<(), Error> {
        self.con.poll.send(msg)?;
        Ok(())
    }

    pub fn recv<F>(&self, op: F)
        where
            F: Fn(&String) -> Result<(), Error>
    {
        let msg = self.con.poll.read_reciver.lock().unwrap().recv().unwrap();
        let msg = String::from_utf8(msg).unwrap();
        if let Err(e) = op(&msg) {
            println!("Failed to process msg:{}, err:{}", msg, e);
        }
    }
}