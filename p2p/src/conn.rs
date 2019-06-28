use std::net::TcpStream;
use std::{thread, io};
use std::sync::mpsc::{SyncSender, sync_channel, Receiver};
use std::time;

use crate::io::{read_exact, write_all};
use crate::error::Error;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Conn {
    pub poll: Polled,
}

const WRITE_CHANNEL_CAP: usize = 10;
const READ_CHANNEL_CAP: usize = 10;
const MSG_LEN: usize = 18;

#[derive(Clone)]
pub struct Polled {
    write_sender: SyncSender<Vec<u8>>,
    pub read_reciver: Arc<Mutex<Receiver<Vec<u8>>>>,
}

impl Polled {
    pub fn send(&self, msg: String) -> Result<(), Error> {
        let body_buf = msg.as_bytes();
        self.write_sender.send(Vec::from(body_buf)).unwrap();
        Ok(())
    }
}

impl Conn {
    pub fn new(stream: TcpStream) -> Result<Conn, Error> {
        poll(stream).map(|polled| Conn { poll: polled })
    }
}

fn poll(stream: TcpStream) -> Result<Polled, Error> {
    let (read_sender, read_reciver) = sync_channel::<Vec<u8>>(READ_CHANNEL_CAP);
    let mut read_stream = stream.try_clone().expect("Clone conn for reader failed");
    // TODO: error in thread
    let _ = thread::Builder::new().name("peer_poll_read".to_string()).
        spawn(move || {
            loop {
                thread::sleep(time::Duration::from_millis(5));
                // Read
                let mut data = vec![0u8; MSG_LEN];
                if let Err(e) = read_exact(&mut read_stream,
                                           &mut data,
                                           time::Duration::from_secs(1),
                                           true) {
                    continue;
                }
                println!("read from stream:{:?}", data);
                read_sender.send(data).unwrap();
            }
        });

    let (write_sender, write_reciver) = sync_channel::<Vec<u8>>(WRITE_CHANNEL_CAP);
    let mut write_stream = stream.try_clone().expect("Clone conn for reader failed");
    let _ = thread::Builder::new().name("peer_poll_write".to_string()).
        spawn(move || {
            loop {
                // Write
                if let Ok(data) = write_reciver.recv() {
                    println!("writing tcpstream:{:?}", data);
                    let _ = write_all(&mut write_stream, &data, time::Duration::from_secs(10));
                }
                thread::sleep(time::Duration::from_millis(5));
            }
        }
        );
    let read_reciver = Arc::new(Mutex::new(read_reciver));
    Ok(Polled {
        write_sender,
        read_reciver,
    })
}
