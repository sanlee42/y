use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream, Ipv4Addr};
use std::time::Duration;
use std::{io, thread, time};
use crate::peers::Peers;
use crate::peer::Peer;
use crate::error::Error;

pub trait P2p {
    fn broadcast(&self, msg: String);
}

pub struct Server {
    addr: SocketAddr,
    peers: Peers,
}


impl Server {
    pub fn new(addr: &str) -> Server {
        Server {
            addr: addr.parse().unwrap(),
            peers: Peers::new(),
        }
    }

    pub fn listen(&self) -> Result<(), Error> {
        println!("Binding p2p server on {}", self.addr);
        let listener = TcpListener::bind(self.addr)?;
        listener.set_nonblocking(true)?;
        let sleep_time = Duration::from_millis(10);
        loop {
            thread::sleep(sleep_time);
            match listener.accept() {
                Ok((stream, peer_addr)) => {
                    println!("Find new peer connecting:{:?}", peer_addr);
                    self.peers.add_peer(Peer::new(peer_addr, stream))?;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // Wait util network socket is ready or just retry later
                }
                Err(e) => {
                    println!("Failed to establish client connection: {:?}", e);
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn connect(&self, addr: &str) -> Result<Peer, Error> {
        let addr = addr.parse().unwrap();
        match TcpStream::connect_timeout(&addr, Duration::from_secs(10)) {
            Ok(stream) => Ok(Peer::new(addr, stream)),
            Err(e) => Err(Error::Connection(e))
        }
    }
}

impl P2p for Server {
    fn broadcast(&self, msg: String) {
        println!("To broadcast msg:{:?}", msg);
        self.peers.broadcast_msg(msg);
    }
}