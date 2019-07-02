use std::io;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

pub fn read_exact(
    stream: &mut dyn Read,
    mut buf: &mut [u8],
    timeout: Duration,
    block_on_empty: bool,
) -> io::Result<()> {
    let sleep_time = Duration::from_micros(10);
    let mut count = Duration::new(0, 0);

    let mut read = 0;
    loop {
        match stream.read(buf) {
            Ok(0) => {
                return Err(io::Error::new(
                    io::ErrorKind::ConnectionAborted,
                    "read_exact",
                ));
            }
            Ok(n) => {
                println!("read:{:?}", n);
                let tmp = buf;
                buf = &mut tmp[n..];
                read += n;
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                if read == 0 && !block_on_empty {
                    return Err(io::Error::new(io::ErrorKind::WouldBlock, "read_exact"));
                }
            }
            Err(e) => return Err(e),
        }
        if !buf.is_empty() {
            thread::sleep(sleep_time);
            count += sleep_time;
        } else {
            break;
        }
        if count > timeout {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "reading from stream",
            ));
        }
    }
    Ok(())
}

pub fn write_all(stream: &mut dyn Write, mut buf: &[u8], timeout: Duration) -> io::Result<()> {
    let sleep_time = Duration::from_micros(10);
    let mut count = Duration::new(0, 0);

    while !buf.is_empty() {
        match stream.write(buf) {
            Ok(0) => {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "failed to write whole buffer",
                ));
            }
            Ok(n) => buf = &buf[n..],
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(e),
        }
        if !buf.is_empty() {
            thread::sleep(sleep_time);
            count += sleep_time;
        } else {
            break;
        }
        if count > timeout {
            return Err(io::Error::new(io::ErrorKind::TimedOut, "writing to stream"));
        }
    }
    Ok(())
}
