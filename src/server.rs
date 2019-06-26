use std::sync::mpsc::{sync_channel, Receiver};
use std::thread::spawn;

use y_p2p as p2p;
use y_p2p::serv::P2p;

pub struct Server {}


impl Server {
    pub fn new() -> Server {
        Server{}
    }
    pub fn run(&self, broadcaster: std::sync::Arc<p2p::serv::Server>) {
        let receiver = self.listen();
        loop {
            match receiver.recv() {
                Ok(msg) => broadcaster.broadcast(msg),
                Err(e) => break
            }
        }
    }

    fn listen(&self) -> Receiver<&'static str> {
        let (sender, reciver) = sync_channel(10);
        let mock_msg = "hello starcoin";
        spawn(move ||
            loop {
                if sender.send(mock_msg).is_err() {
                    break;
                }
            }
        );
        reciver
    }
}