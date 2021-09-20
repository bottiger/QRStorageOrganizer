// lambda_http imports
use lambda_http::{

    // runtime related imports
    handler,
    lambda_runtime::{self, Context, Error},

    // imports that define the signature of our lambda
    IntoResponse, Request, RequestExt,
};

// used to calculate sha2 of user's email
//use sha2::{Digest, Sha256};

// used to get user data from s3
//use s3::Client;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(get_user_data)).await?;
    Ok(())
}


// this is our lambda 
// get_user_data is a lambda that returns user data given it's email in query parameters (assuming the user authenticated somewhere else!)
// from the signature you can see that it handles `Request` objects and returns things that can turn `IntoResponse`
async fn get_user_data(event: Request, _: Context) -> Result<impl IntoResponse, Error> {

    // get email from query string params
    let params = event.query_string_parameters();
    let email = params.get("email").unwrap();

        // hash it and encode
    //let hash = Sha256::new().chain(email).chain("some-salt").finalize();
    //let hash = base64::encode(hash);

    // calculate key of s3 object with the hash above
    //let key = format!("user-data/{}/some.json", hash);

    // use s3 API to get this object from s3
    /*
    let s3 = Client::from_env();
    let result = s3
        .get_object()
        .bucket("my-bucket")
        .key(key)
        .response_content_type("application/json")
        .send()
        .await?;
        */

    // return the content as a response
    //let data = result.body.collect().await?;
    //let response = String::from_utf8(data.into_bytes().to_vec())?.into_response();
    let response = format!("return val: /{}/", email);

    Ok(response)
}