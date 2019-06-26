use clap::{App, Arg};
use std::thread;

use y_p2p as p2p;
use std::sync::Arc;


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
            .help("Connect to the peer").short("c").empty_values(false)
        )
        .get_matches();

    let listener = if let Some(b) = matches.value_of("bind") {
        b
    } else {
        "127.0.0.1:8558"
    };

    let peer = matches.value_of("peer").unwrap();

    let p2p_server = Arc::new(p2p::serv::Server::new(listener));
    let listen_srv = p2p_server.clone();
    let srv_handle = thread::Builder::new().name("p2p_server".to_string()).spawn(move ||
        listen_srv.listen()
    ).unwrap();
    p2p_server.connect(peer).expect("Failed to connect peer");

    srv_handle.join();
}
