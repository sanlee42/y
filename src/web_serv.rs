use actix_web::{App,Result, http, web, HttpResponse, HttpServer};
use serde_derive::{Deserialize, Serialize};
use std::{thread,time};
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    nonce: String,
}

fn handle_msg(msg: web::Json<Message>) -> HttpResponse {
    thread::sleep(time::Duration::from_secs(5));
    println!("nonce:{:?}", msg.nonce);
    HttpResponse::Ok().json(msg.0)
}

pub fn start_web_srv()-> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/").route(web::post().to(handle_msg))
            )
    }).bind("127.0.0.1:8080")?.run()
}
