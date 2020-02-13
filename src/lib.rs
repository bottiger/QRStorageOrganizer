#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;
pub mod im_encoder;
pub mod rest;
pub mod dynamodb;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

#[no_mangle]
pub extern fn rust_fn(x: i32) -> i32 {
    println!("Hello from rust\nI'll return: {}", x.pow(2));
    x.pow(2)
 }

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
