use tokio::net::TcpStream;
use std::io::Write;

pub struct Peer {
    addr: String,
    conns: Vec<TcpStream>,
}

trait P2pPeer {
    fn broadcast(&self, message: String);
    fn recv(&self) -> String;
}


impl P2pPeer for Peer {
    fn broadcast(&self, message: String) {
        self.conns.into_iter().for_each(
            |con| self.handle_con(con)
        )
    }

    fn recv(&self) -> String {
        unimplemented!()
    }
}

impl Peer {
    fn handle_con(&self, stream: TcpStream) {}
}