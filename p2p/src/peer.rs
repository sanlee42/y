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

    pub fn recv<F>(&mut self, op: F) -> Option<Vec<u8>>
        where
            F: Fn(&[u8], String) -> Result<((u32, String)), Error>
    {
        let msg = self.con.poll.read_reciver.lock().unwrap().recv().unwrap();
        match op(&msg, self.hash.clone()) {
            Err(e) => {
                println!("Failed to process msg: {:?}, err: {}", msg, e);
                Option::None
            }
            Ok((nonce, hash)) => {
                //println!("Recive nonce, hash: {}, {}", nonce, hash);
                match self.latest >= nonce {
                    true => Option::None,
                    false => {
                        match self.latest + 1 == nonce {
                            true => {
                                self.latest += 1;
                                self.hash = util::update_hash(self.hash.clone(), hash);
                                println!("The latest nonce: {}, hash: {}", self.latest, self.hash)
                            }
                            false => {
                                //TODO: Buffer it.
                            }
                        }
                        Option::Some(msg)
                    }
                }
            }
        }
    }
}

