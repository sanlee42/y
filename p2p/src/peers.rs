use std::collections::HashMap;
use std::net::SocketAddr;
use crate::error::Error;
use crate::peer::Peer;
use crate::error::Error::PeerExist;
use std::sync::RwLock;

pub struct Peers {
    peers: RwLock<HashMap<SocketAddr, Peer>>
}


impl Peers {
    pub fn new() -> Peers {
        Peers {
            peers: RwLock::new(HashMap::new())
        }
    }

    pub fn add_peer(&self, peer: Peer) -> Result<(), Error> {
        if self.peers.read().unwrap().contains_key(&peer.addr) == true {
            return Err(PeerExist);
        }
        self.peers.write().unwrap().insert(peer.addr, peer);
        Ok(())
    }
}