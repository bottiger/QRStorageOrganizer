#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<qrid>")]              // <- route attribute
fn getqr(qrid: String) -> &'static str {  // <- request handler
    "hello, world!"
}

#[put("/<qrid>")]              // <- route attribute
fn updateqr(qrid: String) -> &'static str {  // <- request handler
    "hello, world!"
}

#[delete("/<qrid>")]              // <- route attribute
fn removeqr(qrid: String) -> &'static str {  // <- request handler
    "hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, getqr, updateqr, removeqr])
}