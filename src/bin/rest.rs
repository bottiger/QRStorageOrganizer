extern crate qrstore;
extern crate env_logger;

use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

	env_logger::init();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(crate::qrstore::rest::index))
            .route("/{qr}", web::get().to(crate::qrstore::rest::index2))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
