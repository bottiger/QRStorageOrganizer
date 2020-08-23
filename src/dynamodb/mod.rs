extern crate futures;
extern crate qrcode;
extern crate tokio;

pub mod crud;
pub mod qruuid;


use dynomite::{
    dynamodb::{
        PutItemOutput, PutItemError,
        QueryOutput, QueryError,
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

#[derive(Debug)]
enum QueryResult {
    QrGroup(QrGroupDB),
    QrCode(QrCodeDB),
}

/*
pub fn insert_sync(qr_entry: &dyn DbItem) -> Result<PutItemOutput, RusotoError<PutItemError>> {
    //let _res = _insert(qr_entry, Rc::new(DynamoDbClient::new(Region::default())));
    let client = DynamoDbClient::new(Region::EuNorth1);

    let item_attr = qr_entry.get_attribute_value_map();

    let put_item_request = PutItemInput {
        table_name: (*TABLE_NAME).to_string(),
        item: item_attr,
        ..PutItemInput::default()
    };

    println!("put req: {:?}", put_item_request);
    println!("put 1");

    //let res = block_on(client.put_item(put_item_request));
    let res = client.put_item(put_item_request).Unpin().Unbox().wait();

    println!("put 123 {:?}", res);

   //_res.await

   res
}
*/


pub async fn get_group(pk: &DynamoPartitionKey) -> Option<QrGroup> {//RusotoFuture<QueryOutput, QueryError> { //impl Future<Output = Qrgroup> {

    //let res:  Pin<Box<dyn Future<Output=Result<QueryOutput, RusotoError<QueryError>>>>> = query(pk, None);
    let res:  Result<QueryOutput, RusotoError<QueryError>> = query(pk, None).await;

    let out = match res {
        Ok(v) => {
            //println!("Group get: {:#?}", v);

            let it = v.items.unwrap();
            let mut group: QrGroup = Default::default();
            let mut codes: Vec<QrCode> = Vec::new();

            for item in it.iter() {
                //println!("start");
                //println!("item: {:#?}", item);

                match QrGroupDB::from_attrs(item.clone()) {
                    Ok(T) => {
                        group = QrGroup::from(T);
                    },
                    Err(E) => {
                        match QrCodeDB::from_attrs(item.clone()) {
                            Ok(T) => {
                                codes.push(QrCode::from(T));
                            },
                            Err(E) => {
                                println!("Error: {}", E);
                            }
                        }
                    }
                }


                group.qrcodes = codes.clone();

                //println!("stream_scan() item {:#?}", group);
                //println!("end");
            }

            Some(group)

        },
        Err(e) => {
            panic!("Error completing futures: {}", e);
            None
        },
    };

    out
}


pub async fn insert_group(item: &QrGroup) -> Result<PutItemOutput, RusotoError<PutItemError>> {
    //let mut puts = Vec::new();

    
    let qr_group_db: QrGroupDB = QrGroupDB::from(item.clone());

    /*
    println!("put 03");

    let r = insert_sync(&qr_group_db);


    println!("put 02");
    r
    */

    
    let group_put: Result<PutItemOutput, RusotoError<PutItemError>> = insert(&qr_group_db).await;
    //puts.push(group_put);

    println!("put 12");

    
    //let qr_codes: Vec<QrCodeDB> = item.qrcodes.clone().into_iter().map(QrCodeDB::from).collect();
    let qr_codes: Vec<QrCode> = item.qrcodes.clone();

    for qrcode in qr_codes {
        //let qr_res: Result<PutItemOutput, RusotoError<PutItemError>> = insert(&qrcode).await;
        //qr_res.await;
        println!("put 11");
        insert_qrcode(&qrcode).await;

        //group_put.then(|_| qr_res)
        //puts.push(qr_res);
    }

    //let result = try_join_all(puts);
    

    let result = group_put;

    result
    
}

pub async fn insert_qrcode(item: &QrCode) -> Result<PutItemOutput, RusotoError<PutItemError>> {
    let qrdb = QrCodeDB::from(item.clone());
    let mut res = insert(&qrdb).await;

    
    for image in item.images.clone() {
        put_image(qrdb.get_primary_key(), image).await;
    }
    


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
