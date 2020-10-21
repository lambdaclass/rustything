use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tracing::{span, event, instrument, Level};

#[get("/")]
async fn hello() -> impl Responder {
    event!(Level::TRACE, "hello");
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    event!(Level::TRACE, "echo");
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    event!(Level::TRACE, "manual_hello");
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let span = span!(Level::TRACE, "main");
    let _guard = span.enter();
    event!(Level::TRACE, "starting server");
    // event!(Level::DEBUG, "");
    // event!(Level::INFO, "");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

