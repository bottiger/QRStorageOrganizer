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