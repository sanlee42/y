use std::sync::mpsc::{sync_channel, Receiver};
use std::{thread, time};

use y_p2p as p2p;
use y_p2p::serv::P2p;

pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }
    pub fn run(&self, broadcaster: std::sync::Arc<p2p::serv::Server>) {
        let receiver = self.listen();
        loop {
            match receiver.recv() {

                Ok(msg) => broadcaster.broadcast(msg),
                Err(_) => break
            }
        }
    }

    fn listen(&self) -> Receiver<String> {
        let (sender, reciver) = sync_channel(10);

        thread::spawn(move ||
            for i in 1..5 {
                thread::sleep(time::Duration::from_millis(10));
                let mock_msg = format!("starcoin see ya: {:?}", i);
                if sender.send(mock_msg).is_err() {
                    break;
                }
            }
        );
        reciver
    }
}