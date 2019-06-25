use clap::{App, Arg};
use std::thread::spawn;

use y_p2p as p2p;


fn main() {
    let matches = App::new("y")
        .version("v0.1")
        .author("@fikgol")
        .about("y peer command line client")
        .arg(Arg::with_name("bind")
            .help("Bind tcp listener").short("b")
            .default_value("127.0.0.1:65535")
        )
        .arg(Arg::with_name("connect")
            .help("Connect to the peer").short("c")
        )
        .get_matches();

    let listener = if let Some(b) = matches.value_of("bind") {
        b
    } else {
        "127.0.0.1:8558"
    };

    let p2p_server = p2p::serv::Server::new(listener);

    spawn(move || p2p_server.listen()).join().unwrap();
}
