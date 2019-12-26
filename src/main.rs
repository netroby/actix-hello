use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello World")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Listen on 127.0.0.1:8080");
    HttpServer::new(|| App::new().service(index))
        .backlog(65536)
        .maxconn(65536)
        .maxconnrate(65536)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}