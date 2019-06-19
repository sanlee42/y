use futures;
use std::collections::HashMap;
use std::env;
use std::io::BufReader;
use std::iter;
use std::sync::{Arc, Mutex};
use tokio;
use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

pub struct Conn {
    socket: TcpListener,

}


impl Conn {
    pub fn new() -> Self {
        let addr = "127.0.0.1:8080".parse()?;
        let socket = TcpListener::bind(addr)?;
        Self {
            socket,
        }
    }

    pub fn listen(self) {
        let connections = Arc::new(Mutex::new(HashMap::new()));

        self.socket.incoming().map_err(|e| {
            println!("failed to accept socket; error = {:?}", e);
            e
        }).for_each(
            move |stream| {
                let addr = stream.peer_addr()?;
                println!("New Connection: {}", addr);
                let (reader, writer) = stream.split();
                let (tx, rx) = futures::sync::mpsc::unbounded();
                connections.lock().unwrap().insert(addr, tx);
                let connections_inner = connections.clone();
                let reader = BufReader::new(reader);
                let iter = stream::iter_ok::<_, io::Error>(iter::repeat(()));
                let socket_reader = iter.fold(reader, move |reader, _| {
                    // Read a line off the socket, failing if we're at EOF
                    let line = io::read_until(reader, b'\n', Vec::new());
                    let line = line.and_then(|(reader, vec)| {
                        if vec.len() == 0 {
                            Err(io::Error::new(io::ErrorKind::BrokenPipe, "broken pipe"))
                        } else {
                            Ok((reader, vec))
                        }
                    });
                },
                )
            }
    }