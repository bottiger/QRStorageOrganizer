extern crate futures;
extern crate qrcode;
extern crate tokio;

pub mod crud;
pub mod qruuid;

use crate::storage::image_store::delete_image;
use rusoto_s3::DeleteObjectError;
use rusoto_s3::DeleteObjectOutput;

use dynomite::{
    dynamodb::{PutItemError, PutItemOutput, QueryOutput},
    FromAttributes,
};
use futures::future::join_all;
use std::error;

use rusoto_core::RusotoError;

use crud::*;

use crate::model::qrcode::QrCode;
use crate::model::qrcode::QrCodeDB;
use crate::model::qrgroup::QrGroup;
use crate::model::qrgroup::QrGroupDB;
use crate::model::schema::DbItem;
use crate::model::schema::DynamoPartitionKey;

use crate::storage::image_store::put_image;

/*
#[derive(Debug)]
enum QueryResult {
    QrGroup(QrGroupDB),
    QrCode(QrCodeDB),
}
*/

pub async fn get_group(pk: &DynamoPartitionKey) -> Result<QrGroup, Box<dyn error::Error>> {
    let res: QueryOutput = query(pk, None).await?;

    let it = res.items.unwrap();
    let mut group: QrGroup = Default::default();
    let mut codes: Vec<QrCode> = Vec::new();

    for item in it.iter() {
        match QrGroupDB::from_attrs(item.clone()) {
            Ok(t) => {
                group = QrGroup::from(t);
            }
            Err(_e) => match QrCodeDB::from_attrs(item.clone()) {
                Ok(t) => {
                    codes.push(QrCode::from(t));
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            },
        }

        group.qrcodes = codes.clone();
    }

    Ok(group)
}

pub async fn insert_group(item: &QrGroup) -> Result<PutItemOutput, RusotoError<PutItemError>> {
    log::debug!(
        "insert group: {:?} => {:?}",
        item.group_id,
        base64::encode(item.group_id)
    );

    let qr_group_db: QrGroupDB = QrGroupDB::from(item.clone());
    let group_put: Result<PutItemOutput, RusotoError<PutItemError>> = insert(&qr_group_db).await;

    let futures = item.qrcodes.clone().into_iter().map(insert_qrcode);
    join_all(futures).await;

    group_put
}

pub async fn insert_qrcode(item: QrCode) -> Result<PutItemOutput, RusotoError<PutItemError>> {
    log::warn!(
        "insert qrcode: {:?} => {:?}",
        item.title,
        base64::encode(item.group_id)
    );

    let qrdb = QrCodeDB::from(item.clone());
    let res = insert(&qrdb).await?;

    println!("Inserted qr code {:?}", item.title);

    let futures = item
        .images
        .into_iter()
        .map( |i| put_image(qrdb.get_primary_key(), i) );

    println!("Inserting images");

    for nres in join_all(futures).await.into_iter() {
        match nres {
            Ok(r) => println!("Inserted image"), //log::debug!("Succesfully inserted image: {:?}", r),
            Err(e) => println!("Failed to insert image"), //log::warn!("Failed to insert image: {:?}", e),
        }
    }

    Ok(res)
}

pub async fn delete_images(
    code: &mut QrCode,
) -> Vec<Result<DeleteObjectOutput, RusotoError<DeleteObjectError>>> {
    log::debug!(
        "removing {} images from: {:?}",
        code.images.len(),
        code.title
    );

    let qrdb = QrCodeDB::from(code.clone());
    let futures = code
        .images
        .clone()
        .into_iter()
        .map(|i| delete_image(qrdb.get_primary_key(), i));
    let res = join_all(futures).await;

    code.images.clear();

    res
}

/*

pub fn print_dynamodb() {
    //let client = DynamoDbClient::new(Region::EuNorth1);

    let mut qrcode = QrCodeDB::default();
    qrcode.id = "qrcode-test".to_string();
    qrcode.title = Some("Update test".to_string());
    qrcode.location = Some("Home".to_string());

    let mut qrcode_new = QrCodeDB::default();
    qrcode_new.id = "qrcode-new".to_string();
    qrcode_new.location = Some("Work".to_string());
    qrcode_new.title = Some("New record".to_string());

    //insert();
    let _getres2 = update(&qrcode);
    //let getres = get(&qrcode.id);
    let pk = &qrcode.get_primary_key();
    let _getres = get(pk);
    let _reginsert = insert(&qrcode_new);


}

*/
