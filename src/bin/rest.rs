extern crate qrstore;

use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(crate::qrstore::rest::index))
            .route("/again", web::get().to(crate::qrstore::rest::index2))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
