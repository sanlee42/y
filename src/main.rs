use clap::{App, Arg};
use std::thread::spawn;

use y_p2p as p2p;


fn main() {
    let matches = App::new("y peer")
        .version("0.1")
        .author("fikgol")
        .about("y peer command line client")
        .arg(Arg::with_name("bind")
            .help("bind tcp listener port")
            .default_value("65535").short("b")
        ).get_matches();

    let port = if let Some(b) = matches.value_of("bind") {
        b.parse::<u16>().expect("Invalid port")
    } else {
        65535
    };
    println!("listen on port:{}", port);
    let p2p_server = p2p::serv::Server::new(port);

    spawn(|| p2p_server.listen()).join().unwrap();
}
