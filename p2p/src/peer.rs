use std::net::{TcpStream, SocketAddr};
use std::thread;
use crate::conn::Conn;
use crate::error::Error;
use crate::util;
use std::sync::Arc;
use core::borrow::BorrowMut;

#[derive(Clone)]
pub struct Peer {
    con: Conn,
    pub addr: SocketAddr,
    pub data: Option<PeerData>,
    latest: u32,
    hash: String,
}

#[derive(Clone)]
pub struct PeerData {
    latest: u32,
    buf: Vec<String>,
    hash: String,
}

impl Peer {
    pub fn new(addr: SocketAddr, stream: TcpStream) -> Peer {
        let peer = Peer {
            con: Conn::new(stream).unwrap(),
            addr,
            data: Option::None,
            latest: 0,
            hash: "".to_string(),
        };
        peer
    }

     pub fn send(&self, msg: Vec<u8>) -> Result<(), Error> {
        self.con.poll.send(msg)?;
        Ok(())
    }

    pub fn recv<F>(&mut self, op: F) -> Vec<u8>
        where
            F: Fn(&[u8], String) -> Result<((u32, String)), Error>
    {
        let msg = self.con.poll.read_reciver.lock().unwrap().recv().unwrap();
        match op(&msg, self.hash.clone()) {
            Err(e) => {
                println!("Failed to process msg:{:?}, err:{}", msg, e);
                msg
            }
            Ok((nonce, hash)) => {
                println!("Final nonce:{},{}", nonce, hash);
                self.latest = nonce;
                self.hash = hash;
                msg
            }
        }
    }
}

