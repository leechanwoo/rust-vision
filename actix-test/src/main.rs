
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

use std::{thread, time};

struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {

        for i in 0..500 {
            ctx.binary(format!("testdfasdkfjasfdo{}", i));
            thread::sleep(time::Duration::from_millis(1000));
            println!("send messages {}", i);
        }

        /*
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
        */
    }
}



#[get("/ws")]
async fn wstest(req: HttpRequest, stm: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stm);
    println!("get response: {:?}", resp);
    resp
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(wstest)
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
