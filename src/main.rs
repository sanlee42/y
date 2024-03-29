use clap::{App, Arg};
use std::{thread, time};
use std::sync::Arc;

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
            .help("Connect to the peer").short("c").empty_values(false)
        )
        .arg(Arg::with_name("http").short("t").default_value("127.0.0.1:8080")
            .help("Bind web server for receiving msg"))
        .get_matches();

    let listener = if let Some(b) = matches.value_of("bind") {
        b
    } else {
        "127.0.0.1:8558"
    };


    // Start p2p network server
    let p2p_server = Arc::new(p2p::serv::Server::new(listener));
    let listen_srv = p2p_server.clone();
    let srv_handle = thread::Builder::new().name("p2p_server".to_string()).spawn(move ||
        listen_srv.listen()
    ).expect("Failed to start p2p server");
    thread::sleep(time::Duration::from_secs(2));

    // Connect to the peer
    if let Some(peer) = matches.value_of("connect") {
        p2p_server.connect(peer).expect("Failed to connect peer")
    }

    // Start message receive web server
    if let Some(addr) = matches.value_of("http") {
        y::web_serv::run(addr.to_string(), p2p_server);
    }

    // Join server
    let _ = srv_handle.join().expect("Failed to join p2p server thread");
}
