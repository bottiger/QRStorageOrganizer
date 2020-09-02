extern crate qrcode;

use crate::dynamodb::qruuid::parse_qr_val;
use actix_web::HttpRequest;
use actix_web::{HttpResponse, Responder};
use serde_json::json;
use actix_web::web;

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn index2(info: web::Path<String>, _req: HttpRequest) -> impl Responder {

	match parse_qr_val(info.to_string()) {
		Ok(v) => {
			let resp_json = json!(v).to_string();
			HttpResponse::Ok().body(resp_json)
		},
		Err(_e) => HttpResponse::BadRequest().body("Unable to parse input")
	}
}