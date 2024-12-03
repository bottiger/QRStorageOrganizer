
extern crate qrcode;

use actix_web::HttpRequest;
use actix_web::{HttpResponse, Responder};
use pnet::ipnetwork::IpNetwork;
use serde_json::json;
use actix_web::{web, App, HttpServer};
use std::net::IpAddr;
use crate::model::qruuid::parse_qr_val;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/ping", web::get().to(ping))
            .route("/ip", web::get().to(ip))
			.route("/qr", web::get().to(index2))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

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

async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

async fn ip() -> impl Responder {
    match get_ip_address() {
        Some(ip) => HttpResponse::Ok().body(ip.to_string()),
        None => HttpResponse::InternalServerError().body("Unable to retrieve IP"),
    }
}

fn get_ip_address() -> Option<IpAddr> {
    // Try to fetch the local IP address of the server
    let interfaces = pnet::datalink::interfaces();
    for interface in interfaces {
        for ip in interface.ips {
            if let IpNetwork::V4(ipv4) = ip {
                return Some(IpAddr::V4(ipv4.ip()));
            }
        }
    }
    None
}
