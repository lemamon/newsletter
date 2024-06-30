use std::net::TcpListener;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;

async fn greet(req: HttpRequest) -> impl
Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    println!("ssssss");
    format!("Hello {}!", &name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(tcp_listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
    })
        // .bind(address)?.
        .listen(tcp_listener)?
        .run();
    Ok(server)
}
