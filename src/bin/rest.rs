extern crate qrstore;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(qrstore::rest::index))
            .route("/again", web::get().to(qrstore::rest::index2))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
