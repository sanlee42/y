use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream, Ipv4Addr};
use std::time::Duration;
use std::io;
use crate::peers::Peers;
use crate::peer::Peer;
use crate::error::Error;

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
        println!("Listen on {}", self.addr);
        let listener = TcpListener::bind(self.addr)?;
        listener.set_nonblocking(true)?;
        let sleep_time = Duration::from_millis(5);
        loop {
            match listener.accept() {
                Ok((stream, peer_addr)) => {
                    self.peers.add_peer(Peer::new(peer_addr, stream))?
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

    pub fn connect(&self, addr: SocketAddr) -> Result<Peer, Error> {
        match TcpStream::connect_timeout(&addr, Duration::from_secs(10)) {
            Ok(stream) => Ok(Peer::new(addr, stream)),
            Err(e) => Err(Error::Connection(e))
        }
    }
}