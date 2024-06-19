extern crate dotenv;
#[macro_use]
extern crate lazy_static;

pub mod config;
pub mod fixtures;
pub mod im_encoder;
pub mod model;
pub mod pdf_generator;
pub mod rest;
pub mod storage;

#[no_mangle]
pub extern "C" fn rust_fn(x: i32) -> i32 {
    println!("Hello from rust\nI'll return: {}", x.pow(2));
    x.pow(2)
}
