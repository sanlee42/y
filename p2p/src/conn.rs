use std::net::TcpStream;
use std::thread;
use std::sync::mpsc::{SyncSender, sync_channel, Receiver};
use std::time;

use crate::io::{read_exact, write_all};
use crate::error::Error;

pub struct Conn {
    poll: Polled,
}

const WRITE_CHANNEL_CAP: usize = 10;
const READ_CHANNEL_CAP: usize = 10;
const MSG_LEN: usize = 16;

pub struct Polled {
    write_sender: SyncSender<Vec<u8>>,
    read_reciver: Receiver<Vec<u8>>,
}

impl Conn {
    pub fn new(stream: TcpStream) -> Result<Conn, Error> {
        poll(stream).map(|polled| Conn { poll: polled })
        //match poll(stream) {
        //Ok(polled) => Ok(Conn { poll: polled }),
        //Err(e) => Err(e),
        //}
    }
}

fn poll(stream: TcpStream) -> Result<Polled, Error> {
    let (write_sender, write_reciver) = sync_channel::<Vec<u8>>(WRITE_CHANNEL_CAP);
    let (read_sender, read_reciver) = sync_channel::<Vec<u8>>(READ_CHANNEL_CAP);
    let mut read_stream = stream.try_clone().expect("Clone conn for reader failed");
    let mut write_stream = stream.try_clone().expect("Clone conn for reader failed");
// TODO: error in thread
    let _ = thread::Builder::new().name("peer_poll_read".to_string()).
        spawn(move || {
            loop {
// Read
                let mut data = vec![0u8; MSG_LEN];
                let _ = read_exact(&mut read_stream, &mut data, time::Duration::from_secs(10), true);
                read_sender.send(data);
                thread::sleep(time::Duration::from_millis(5));
            }
        });
    let _ = thread::Builder::new().name("peer_poll_write".to_string()).
        spawn(move || {
            loop {
// Write
                if let Ok(data) = write_reciver.recv() {
                    let _ = write_all(&mut write_stream, &data, time::Duration::from_secs(10));
                }
                thread::sleep(time::Duration::from_millis(5));
            }
        }
        );
    Ok(Polled {
        write_sender,
        read_reciver,
    })
}
