extern crate futures;
extern crate qrcode;
extern crate tokio;

pub mod crud;
pub mod qruuid;


use rusoto_s3::DeleteObjectOutput;
use rusoto_s3::DeleteObjectError;
use crate::storage::image_store::delete_image;

use std::error;
use futures::future::join_all;
use dynomite::{
    dynamodb::{
        PutItemOutput, PutItemError,
        QueryOutput,
    }, FromAttributes
};

use rusoto_core::RusotoError;

use crud::*;

use crate::model::qrcode::QrCodeDB;
use crate::model::qrcode::QrCode;
use crate::model::qrgroup::QrGroupDB;
use crate::model::qrgroup::QrGroup;
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
            },
            Err(_e) => {
                match QrCodeDB::from_attrs(item.clone()) {
                    Ok(t) => {
                        codes.push(QrCode::from(t));
                    },
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        }
        
        group.qrcodes = codes.clone();
    }

    Ok(group)
}

pub async fn insert_group(item: &QrGroup) -> Result<PutItemOutput, RusotoError<PutItemError>> {
    log::debug!("insert group: {:?} => {:?}", item.group_id, base64::encode(item.group_id));

    
    let qr_group_db: QrGroupDB = QrGroupDB::from(item.clone());
    let group_put: Result<PutItemOutput, RusotoError<PutItemError>> = insert(&qr_group_db).await;

    let futures = item.qrcodes.clone().into_iter().map(|q| insert_qrcode(q) );
    join_all(futures).await;
    

    let result = group_put;

    result
    
}

pub async fn insert_qrcode(item: QrCode) -> Result<PutItemOutput, RusotoError<PutItemError>> {
    log::warn!("insert qrcode: {:?} => {:?}", item.title, base64::encode(item.group_id));


    let qrdb = QrCodeDB::from(item.clone());
    let res = insert(&qrdb).await?;

    let futures = item.images.into_iter().map({ |i| 
        put_image(qrdb.get_primary_key(), i)
    });
    
    for nres in join_all(futures).await.into_iter() {
        match nres {
            Ok(r) => log::debug!("Succesfully inserted image: {:?}", r),
            Err(e) => log::warn!("Failed to insert image: {:?}", e),
        } 
    }

    Ok(res)
}

pub async fn delete_images(code: &mut QrCode) -> Vec<Result<DeleteObjectOutput, RusotoError<DeleteObjectError>>> {
    log::debug!("removing {} images from: {:?}", code.images.len(), code.title);

    let qrdb = QrCodeDB::from(code.clone());
    let futures = code.images.clone().into_iter().map(|i| delete_image(qrdb.get_primary_key(), i));
    let res = join_all(futures).await;

    code.images.clear();

    res
}


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
    let getres = get(pk);
    let _reginsert = insert(&qrcode_new);
    //let _regdel = delete(&qrcode_new.id);

    //let getres = get("qrcode-test".to_string());

    //println!("getres: {:?}", getres);

/*
    let person = Qrcode { id: "123".into(), title: None, location: None, items: None, images: None };
    // convert person to string keys and attribute values
    let attributes: Attributes = person.clone().into();
    // convert attributes into person type
    assert_eq!(person, Qrcode::from_attrs(attributes).unwrap());

    // dynamodb types require only primary key attributes and may contain
    // other fields. when looking up items only those key attributes are required
    // dynomite derives a new {Name}Key struct for your which contains
    // only those and also implements Item
    let key = QrcodeKey { id: "123".into() };
    let key_attributes: Attributes = key.clone().into(); // `std::collections::HashMap<std::string::String, types::dynomite::rusoto_dynamodb::AttributeValue>
    // convert attributes into person type
    assert_eq!(key, QrcodeKey::from_attrs(key_attributes.clone()).unwrap());

    //let k: String = key;

    println!("qrcode {:?}", person);
    println!("qrcode attr {:?}", key_attributes);
    println!("done");
    */

    //let mut runtime = tokio::runtime::Runtime::new().unwrap();

    /*
    let mut rt = Runtime::new().expect("failed to initialize futures runtime");
    let item_from_dynamo = match rt.block_on(getres) {
            Ok(item) => item,
            Err(e) => panic!("Error completing futures: {}", e),
        };
        */
    //println!("getres: {:?}", getres);


    //println!("getres: {:?}", block_on(getres));

    /*
    let list_tables_input: ListTablesInput = Default::default();
    match client.list_tables(list_tables_input).sync() {
        Ok(output) => {
            match output.table_names {
                Some(table_name_list) => {
                    println!("Tables in database:");

                    for table_name in table_name_list {
                        println!("{}", table_name);
                    }
                },
                None => println!("No tables in database!"),
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
        },
    }

    let item = make_item();

    let get_item_request = GetItemInput {
        key: item.clone(),
        table_name: "qr-storage".to_string(),
        ..Default::default()
    };

    let res = client.get_item(get_item_request).sync();

    println!("item: {:?}", res);

    match res {
        Ok(v) => match v.item {
            Some(v) => println!("v: {:?}", v),
            None => println!("no items found"),
        },
        Err(e) => println!("e: {:?}", e),
    }
    */

}
