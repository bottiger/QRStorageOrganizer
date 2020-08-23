

use rusoto_s3::GetObjectRequest;
use rusoto_s3::DeleteObjectRequest;
use rusoto_core::RusotoError;
use rusoto_s3::PutObjectError;
use rusoto_s3::GetObjectError;
use rusoto_s3::DeleteObjectError;
use rusoto_s3::PutObjectOutput;
use rusoto_s3::GetObjectOutput;
use rusoto_s3::DeleteObjectOutput;
use rusoto_s3::StreamingBody;
use rusoto_core::{HttpClient, Region};
use rusoto_s3::{S3, S3Client, PutObjectRequest};
use rusoto_credential::StaticProvider;


pub mod image_store;


/*
fn get_endpoint() -> Region {
	Region::Custom {
        name: "eu-north-1".to_owned(),
        endpoint: "https://s3.eu-north-1.amazonaws.com".to_owned(),
    }
}
*/

fn get_credential_provider() -> StaticProvider {
	StaticProvider::new_minimal(String::from("9RB1ETUGDVPR8TM87MQA"), String::from("jsCvqZtEsUhm3s8CeDhSkdLpoKT2eAm2A4SeHhPz"))
}

fn get_endpoint() -> Region {
	Region::Custom {
        name: "eu-central-1".to_owned(),
        endpoint: "https://s3.eu-central-1.wasabisys.com".to_owned(),
    }
}

fn get_client() -> S3Client {
	S3Client::new_with(HttpClient::new().unwrap(), get_credential_provider(), get_endpoint())
}

fn get_bucket_name() -> String {
	String::from("qrstorage")
}

fn example(s: String) -> StreamingBody {
    s.into_bytes().into()
}

pub async fn put(obj_key: String, obj_body: Option<StreamingBody>) -> Result<PutObjectOutput, RusotoError<PutObjectError>> {
	let client = get_client();

	let req = PutObjectRequest {
        bucket: get_bucket_name(),
        key: obj_key,
        body: obj_body,
        ..Default::default()
    };
	
	client.put_object(req).await
}

/*
pub async fn put(obj_key: String, obj_body: String) -> Result<PutObjectOutput, RusotoError<PutObjectError>> {
	let client = get_client();

	let req = PutObjectRequest {
        bucket: get_bucket_name(),
        key: obj_key,
        body: Some(example(obj_body)),
        ..Default::default()
    };
	
	client.put_object(req).await
}
*/

pub async fn get(obj_key: String) -> Result<GetObjectOutput, RusotoError<GetObjectError>> {
	let client = get_client();

	let req = GetObjectRequest {
        bucket: get_bucket_name(),
        key: obj_key,
        ..Default::default()
    };
	
	client.get_object(req).await
}

pub async fn delete(obj_key: String) -> Result<DeleteObjectOutput, RusotoError<DeleteObjectError>> {
	let client = get_client();

	let req = DeleteObjectRequest {
        bucket: get_bucket_name(),
        key: obj_key,
        ..Default::default()
    };
	
	client.delete_object(req).await
}

pub async fn get_bucket() -> Result<(), ()> {
	let client = get_client();


	let req = PutObjectRequest {
        bucket: get_bucket_name(),
        key: "types2.json".to_string(),
        body: Some(example("Test".to_owned())),
        ..Default::default()
    };
	
	let output = client.put_object(req).await;

	println!("out: {:?}", output.unwrap().e_tag);

	Ok(())


}