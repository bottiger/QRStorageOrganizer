/*
use lambda_http::{handler, lambda_runtime::{self, Context, Error}, IntoResponse, Request, RequestExt};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(hello)).await?;
    Ok(())
}

async fn hello(
    request: Request,
    _: Context
) -> Result<impl IntoResponse, Error> {
    Ok(format!(
        "hello {}",
        request
            .query_string_parameters()
            .get("name")
            .unwrap_or_else(|| "stranger")
    ))
}
*/
/* 
use rocket::{self, get, routes};
use lambda_web::{is_running_on_lambda, launch_rocket_on_lambda, LambdaError};

#[get("/")]
fn index() -> &'static str {
    "Hi to you"
}

#[get("/")]
fn ttt() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[rocket::main]
async fn main() -> Result<(), LambdaError> {
    let rocket = rocket::build()
        .mount("/t", routes![hello])
        .mount("/hi", routes![index])
        .mount("/", routes![ttt]);
    if is_running_on_lambda() {
        // Launch on AWS Lambda
        launch_rocket_on_lambda(rocket).await?;
    } else {
        // Launch local server
        rocket.launch().await?;
    }
    Ok(())
}
    */