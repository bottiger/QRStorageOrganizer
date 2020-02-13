extern crate qrcode;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}
