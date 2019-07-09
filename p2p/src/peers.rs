use std::collections::HashMap;
use std::net::SocketAddr;
use crate::error::Error;
use crate::peer::Peer;
use crate::error::Error::PeerExist;
use crate::util;
use std::sync::{RwLock, Arc};
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::thread;


pub struct Peers {
    peers: RwLock<HashMap<SocketAddr, Arc<Peer>>>
}

const BROAD_CAST_NUM: usize = 2;

impl Peers {
    pub fn new() -> Peers {
        Peers {
            peers: RwLock::new(HashMap::new())
        }
    }

    pub fn add_peer(&self, mut peer: Arc<Peer>) -> Result<(), Error> {
        if self.peers.read().unwrap().contains_key(&peer.addr) == true {
            return Err(PeerExist);
        }
        let mut _peer = Arc::make_mut(&mut peer).clone();
        thread::spawn(move ||
            loop {
                let msg = _peer.recv(util::process_msg);
            }
        );
        self.peers.write().unwrap().insert(peer.addr, peer);
        Ok(())
    }

    fn broadcast<F>(&self, op: F)
        where
            F: Fn(&Peer) -> Result<(), Error>
    {
        let mut count = 0;
        let mut peers = self.peers
            .read()
            .unwrap()
            .values()
            .cloned()
            .collect::<Vec<_>>();
        peers.shuffle(&mut thread_rng());

        for peer in peers {
            println!("Pick a peer to broadcast: {:?}", peer.addr);
            match op(&peer) {
                Ok(_) => {
                    count += 1;
                }
                Err(e) => {
                    println!("Failed to broadcast to peer:{:?}", peer.addr);// TODO:Remove from peers
                }
            }
            if count > BROAD_CAST_NUM {
                break;
            }
        }
    }

    pub fn broadcast_msg(&self, msg: Vec<u8>) {
        self.broadcast(|peer| peer.send(msg.clone()));
    }

    pub fn forward(peers: Peers) {
        thread::spawn(|| {
            peers
        }
        );
    }
}