
use actix:: { Actor, 
    StreamHandler };

use actix_web::{ get, 
    web, 
    Error, 
    App, 
    HttpServer, 
    HttpRequest, 
    HttpResponse };

use actix_files as fs;
use actix_web_actors::ws;

use futures::{future::ok, stream::once};

use std::{thread, time};

struct MonitorWs;

impl Actor for MonitorWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MonitorWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}



#[get("/monitor")]
async fn monitor(req: HttpRequest, stm: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MonitorWs {}, &req, stm);
    println!("monitor get response: {:?}", resp);
    resp
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(monitor)
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
