use crate::storage::delete;
use rusoto_s3::DeleteObjectOutput;
use rusoto_s3::DeleteObjectError;
use std::error;
use crate::model::qrimage::QrImageHash;
use crate::model::schema::DynamoPrimaryKey;
use rusoto_s3::PutObjectOutput;
use rusoto_core::RusotoError;
use rusoto_s3::PutObjectError;
use crate::storage::put;
use crate::storage::get;
use crate::model::qrimage::QrImage;
use simple_error::SimpleError;
use futures::TryStreamExt;
use bytes::BytesMut;

static SEPARATOR: &str = "-";

pub async fn put_image(primary_key: DynamoPrimaryKey, image: QrImage) -> Result<PutObjectOutput, RusotoError<PutObjectError>> {
	let key = get_key(&primary_key, &image.hash32);
	log::trace!("insert image: {:?} => {:?}", base64::encode(&primary_key.partition_key), key);


 	let bytes = image.image.into_vec();
	let body = Some(bytes.into());

	put(key, body).await
}

pub async fn delete_image(primary_key: DynamoPrimaryKey, image: QrImage) -> Result<DeleteObjectOutput, RusotoError<DeleteObjectError>> {
	
	let key = get_key(&primary_key, &image.hash32);

	delete(key).await
}

pub async fn get_image(primary_key: &DynamoPrimaryKey, hash32: &QrImageHash) -> Result<QrImage, Box<dyn error::Error>> { //RusotoError<GetObjectError>
	let key = get_key(primary_key, hash32);

	let res = get(key).await?;

	let stream = res.body.ok_or_else(|| SimpleError::new("cannot do foo"))?;
    let body = stream.map_ok(|b| BytesMut::from(&b[..])).try_concat().await.unwrap();

    let image = QrImage::new(body.to_vec())?;
	Ok(image)
}

fn get_key(primary_key: &DynamoPrimaryKey, hash32: &QrImageHash) -> String {
	let key = base64::encode(&primary_key.partition_key) 
				+ SEPARATOR
				+ &base64::encode(&primary_key.sort_key)
				+ SEPARATOR
				+ &hash32.to_string();

	key
}