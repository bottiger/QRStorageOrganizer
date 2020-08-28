use crate::config::get_config;
use rusoto_core::RusotoError;
use rusoto_core::{HttpClient, Region};
use rusoto_credential::StaticProvider;
use rusoto_s3::DeleteObjectError;
use rusoto_s3::DeleteObjectOutput;
use rusoto_s3::DeleteObjectRequest;
use rusoto_s3::GetObjectError;
use rusoto_s3::GetObjectOutput;
use rusoto_s3::GetObjectRequest;
use rusoto_s3::ListObjectsV2Error;
use rusoto_s3::ListObjectsV2Output;
use rusoto_s3::ListObjectsV2Request;
use rusoto_s3::PutObjectError;
use rusoto_s3::PutObjectOutput;
use rusoto_s3::StreamingBody;
use rusoto_s3::{PutObjectRequest, S3Client, S3};

pub mod image_store;

fn get_credential_provider() -> StaticProvider {
    let db_key = get_config().get::<String>(&"storage_key").unwrap();
    let db_secret = get_config().get::<String>(&"storage_secret").unwrap();
    StaticProvider::new_minimal(db_key, db_secret)
}

fn get_endpoint() -> Region {
    let storage_region = get_config().get::<String>(&"storage_region").unwrap();
    let storage_endpoint = get_config().get::<String>(&"storage_endpoint").unwrap();
    Region::Custom {
        name: storage_region,
        endpoint: storage_endpoint,
    }
}

fn get_client() -> S3Client {
    S3Client::new_with(
        HttpClient::new().unwrap(),
        get_credential_provider(),
        get_endpoint(),
    )
}

fn get_bucket_name() -> String {
    String::from("qrstorage")
}

fn example(s: String) -> StreamingBody {
    s.into_bytes().into()
}

pub async fn put(
    obj_key: String,
    obj_body: Option<StreamingBody>,
) -> Result<PutObjectOutput, RusotoError<PutObjectError>> {
    log::info!("put obj: {:?}", obj_key);
    let client = get_client();

    let req = PutObjectRequest {
        bucket: get_bucket_name(),
        key: obj_key,
        body: obj_body,
        ..Default::default()
    };

    client.put_object(req).await
}

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

pub async fn list_objects(
    obj_prefix: String,
) -> Result<ListObjectsV2Output, RusotoError<ListObjectsV2Error>> {
    log::info!("count objects with prefix: {:?}", obj_prefix);

    let client = get_client();

    let req = ListObjectsV2Request {
        bucket: get_bucket_name(),
        prefix: Some(obj_prefix),
        ..Default::default()
    };

    let res = client.list_objects_v2(req).await;

    res
}

pub async fn count_objects(obj_prefix: String) -> Result<i64, RusotoError<ListObjectsV2Error>> {
    let objs = list_objects(obj_prefix).await?;

    let count = match objs.key_count {
        Some(v) => v,
        None => 0,
    };

    Ok(count)
}
