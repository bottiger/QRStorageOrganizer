extern crate dotenv;
#[macro_use]
extern crate lazy_static;

pub mod model;
//pub mod schema;
pub mod im_encoder;
pub mod rest;
pub mod dynamodb;
//pub mod types;
pub mod fixtures;
pub mod storage;
pub mod config;

#[no_mangle]
pub extern fn rust_fn(x: i32) -> i32 {
    println!("Hello from rust\nI'll return: {}", x.pow(2));
    x.pow(2)
 }
