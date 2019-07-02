use actix_web::{App, web, HttpResponse, HttpServer};
use serde_derive::{Deserialize, Serialize};
use std::sync::{mpsc, Arc};
use std::thread;
use y_p2p as p2p;
use y_p2p::serv::P2p;

pub fn run(bind: String, broadcaster: std::sync::Arc<p2p::serv::Server>) {
    thread::spawn(move || {
        let srv_reciver = start_web_srv(bind);
        loop {
            match srv_reciver.recv() {
                Ok(msg) => broadcaster.broadcast(msg),
                Err(_) => break
            }
        };
    });
}


#[derive(Debug, Serialize, Deserialize)]
struct Message {
    nonce: u32,
    bytes: String,
}


fn handle_msg(sender: web::Data<Arc<mpsc::SyncSender<String>>>, msg: web::Json<Message>) -> HttpResponse {
    println!("nonce:{:?}", msg.nonce);

    let msg_str = format!("{:?},{}", msg.nonce, msg.bytes);
    sender.send(msg_str).unwrap();
    HttpResponse::Ok().json(msg.nonce)
}

pub fn start_web_srv(addr: String) -> Arc<mpsc::Receiver<String>> {
    println!("Start web server on {}", addr);
    let (sender, reciver) = mpsc::sync_channel(10);
    let sender = web::Data::new(Arc::new(sender));
    thread::spawn(|| {
        HttpServer::new(move || {
            App::new().register_data(sender.clone())
                .service(
                    web::resource("/").route(web::post().to(handle_msg))
                )
        }).bind(addr).expect("Failed to bind http listener").run()
    }
    );
    Arc::new(reciver)
}