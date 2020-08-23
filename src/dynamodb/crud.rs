//extern crate rusoto_dynamodb;
extern crate futures;

use rusoto_dynamodb::CreateTableInput;
use rusoto_dynamodb::KeySchemaElement;
use rusoto_dynamodb::AttributeDefinition;
use rusoto_dynamodb::ProvisionedThroughput;
use rusoto_dynamodb::ListTablesInput;

use dynomite::{
    attr_map, 
    dynamodb::{
        DynamoDb, DynamoDbClient, AttributeValue,
        PutItemInput, PutItemOutput, PutItemError,
        GetItemInput, GetItemOutput, GetItemError,
        UpdateItemInput, UpdateItemOutput, UpdateItemError,
        DeleteItemInput, DeleteItemOutput, DeleteItemError,
        QueryInput, QueryOutput, QueryError,
    }, 
    Attribute, Attributes, DynamoDbExt, Retries
};

use dynomite::{
    retry::Policy,
    FromAttributes, Item, 
};
use futures::{TryStreamExt};
#[cfg(feature = "default")]
use rusoto_core_default::Region;
#[cfg(feature = "rustls")]
use rusoto_core_rustls::Region;

use rusoto_core::region::Region;
use rusoto_core::RusotoError;
//use rusoto_core::RusotoFuture;

use std::rc::Rc;
use std::collections::HashMap;


use crate::model::schema::DynamoPrimaryKey;
use crate::model::schema::DynamoPartitionKey;
use crate::model::schema::DynamoSearchKey;
use crate::model::schema::DbItem;



use crate::model::schema::*;

// work in 0.41
thread_local!(static DB: Rc<DynamoDbClient> = Rc::new(DynamoDbClient::new(Region::EuNorth1)));


pub async fn get(item: &DynamoPrimaryKey) -> Result<GetItemOutput, RusotoError<GetItemError>> {
   let res = DB.with(|odb_cell| {
      _get(item, Rc::clone(odb_cell))
  });

  res.await
}


pub async fn insert(item: &dyn DbItem) -> Result<PutItemOutput, RusotoError<PutItemError>> {
    //let _res = _insert(qr_entry, Rc::new(DynamoDbClient::new(Region::EuNorth1)));

    println!("put 123");

   //_res.await

    //let res = DB.with(|odb_cell| {
      //_insert(qr_entry, Rc::clone(odb_cell))
        println!("_insert");
        let client = DynamoDbClient::new(Region::EuNorth1).with_retries(Policy::default());

        let item_attr = item.get_attribute_value_map();

        let put_item_request = PutItemInput {
            table_name: (*TABLE_NAME).to_string(),
            item: item_attr,
            ..PutItemInput::default()
        };

       //println!("put req: {:?}", put_item_request);
        println!("put 1");

        let res2 = client.put_item(put_item_request).await;

        println!("put 2");

        //res.await2
        let res = res2;
    //});

    res
}

pub async fn update(qr_entry: &dyn DbItem) -> Result<UpdateItemOutput, RusotoError<UpdateItemError>> {
    let _res = DB.with(|odb_cell| {
       _update(qr_entry, Rc::clone(odb_cell))
   });

   _res.await
}


pub async fn delete(pk: &DynamoPrimaryKey) -> Result<DeleteItemOutput, RusotoError<DeleteItemError>> {
    let _res = DB.with(|odb_cell| {
       _delete(pk, Rc::clone(odb_cell))
   });

   _res.await
}

pub async fn query(partition_key: &DynamoPartitionKey, sort_key: Option<&DynamoSearchKey>) -> Result<QueryOutput, RusotoError<QueryError>> {

    let client = DynamoDbClient::new(Region::EuNorth1);
    let mut val_map: HashMap<String, AttributeValue> = HashMap::new();
    val_map.insert(
            ":partitionkeyval".to_string(),
            partition_key.to_vec().into_attr()
        );

    let key_cond_expr = match sort_key {
        Some(s) => {
            val_map.insert(
                    ":sortkeyval".to_string(),
                    s.to_owned().into_attr()
                );
            format!("{} = :partitionkeyval AND begins_with({}, :sortkeyval)", *PARTITION_KEY_NAME, *SORT_KEY_NAME)
        },
        None    => format!("{} = :partitionkeyval", *PARTITION_KEY_NAME) //format!("{} = :partitionkeyval", *PARTITION_KEY_NAME),
    };

    let attr_vals = Some(val_map);

    let query_input = QueryInput {
        table_name: String::from(*TABLE_NAME),
        consistent_read: Some(true),
        key_condition_expression: Some(key_cond_expr),
        expression_attribute_values: attr_vals,
        ..Default::default()
    };

    client.query(query_input).await
}

/*
pub fn _query(partition_key: &DynamoPartitionKey, sort_key: Option<&DynamoSearchKey>, client: &'static DynamoDbClient) -> Pin<Box<dyn Future<Output=Result<QueryOutput, RusotoError<QueryError>>>>> {

    let mut val_map: HashMap<String, AttributeValue> = HashMap::new();
    val_map.insert(
            ":partitionkeyval".to_string(),
            partition_key.to_vec().into_attr()
        );

    let key_cond_expr = match sort_key {
        Some(s) => {
            val_map.insert(
                    ":sortkeyval".to_string(),
                    s.to_owned().into_attr()
                );
            format!("{} = :partitionkeyval AND begins_with({}, :sortkeyval)", *PARTITION_KEY_NAME, *SORT_KEY_NAME)
        },
        None    => format!("{} = :partitionkeyval", *PARTITION_KEY_NAME) //format!("{} = :partitionkeyval", *PARTITION_KEY_NAME),
    };

    let attr_vals = Some(val_map);

    let query_input = QueryInput {
        table_name: String::from(*TABLE_NAME),
        consistent_read: Some(true),
        key_condition_expression: Some(key_cond_expr),
        expression_attribute_values: attr_vals,
        ..Default::default()
    };

    
    /*
    DB.with(|odb_cell| {
       Rc::clone(odb_cell).query(query_input)
    })
    */

    client.query(query_input)
    
}
*/

async fn get_tables(client: Rc<DynamoDbClient>) {
    let list_tables_input: ListTablesInput = Default::default();
    match client.list_tables(list_tables_input).await {
        Ok(output) => {
            match output.table_names {
                Some(table_name_list) => {
                    println!("Tables in database:");
                    //println!("Tables in database: {}", item);

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
}

/*
async fn _insert(item: &dyn DbItem, client: Rc<DynamoDbClient>) -> Result<PutItemOutput, RusotoError<PutItemError>> {
    //println!("_insert");

    let item_attr = item.get_attribute_value_map();

    let put_item_request = PutItemInput {
        table_name: (*TABLE_NAME).to_string(),
        item: item_attr,
        ..PutItemInput::default()
    };

    //println!("put req: {:?}", put_item_request);
    println!("put 1");

    let res = client.put_item(put_item_request);

    println!("put 2");

    res.await
}
*/

async fn _get(item: &DynamoPrimaryKey, client: Rc<DynamoDbClient>) -> Result<GetItemOutput, RusotoError<GetItemError>> { //, Error=GetItemError> {
    println!("_get: partition: {:?} sort: {:?}", item.partition_key, item.sort_key);

    let key = get_key(item);

    let get_item_request = GetItemInput {
        key: key.clone(),
        table_name: (*TABLE_NAME).to_string(),
        ..GetItemInput::default()
    };

    //println!("get req: {:?}", get_item_request);
 
    let res = client.get_item(get_item_request);

    res.await
}

async fn _update(item: &dyn DbItem, client: Rc<DynamoDbClient>) -> Result<UpdateItemOutput, RusotoError<UpdateItemError>> { //Result<Qrcode, AttributeError> {


    println!("_update");
    //println!("_update: {:?}", item);
    //println!("attr: {:?}", item.into_attr());

    let (attr_val, update_expr) = item.get_update_expr();

    /*
    let mut expression_attribute_values = HashMap::new();
    expression_attribute_values.insert(
            ":title_val".to_string(),
            item.title.to_owned().into_attr()
        );
    expression_attribute_values.insert(
            ":location_val".to_string(),
            item.location.to_owned().unwrap().into_attr()
        );


    let update_item_request = UpdateItemInput {
            attribute_updates: None,
            condition_expression: None, //Some("owner_id = :null_attribute_type".to_string()),
            conditional_operator: None,
            expected: None,
            expression_attribute_names: None,
            expression_attribute_values: Some(expression_attribute_values),
            key: get_key(&item.id),
            return_consumed_capacity: None,
            return_item_collection_metrics: None,
            return_values: Some("ALL_NEW".to_string()), //None,
            table_name: TABLE.to_string(),
            update_expression: Some("SET title = :title_val, location2 = :location_val".to_string())
        };
        */
        let update_item_request = UpdateItemInput {
                attribute_updates: None,
                condition_expression: None, //Some("owner_id = :null_attribute_type".to_string()),
                conditional_operator: None,
                expected: None,
                expression_attribute_names: None,
                expression_attribute_values: attr_val,
                key: get_key(&item.get_primary_key()),
                return_consumed_capacity: None,
                return_item_collection_metrics: None,
                return_values: Some("ALL_NEW".to_string()), //None,
                table_name: (*TABLE_NAME).to_string(),
                update_expression: update_expr,
            };

    let res = client.update_item(update_item_request);

    //println!("_update resp: {:?}", res);

    res.await


}

async fn _delete(pk: &DynamoPrimaryKey, client: Rc<DynamoDbClient>) -> Result<DeleteItemOutput, RusotoError<DeleteItemError>> {
    //println!("_delete: {:?}", pk);
    let key = get_key(pk);

    let delete_item_request = DeleteItemInput {
        key: key.clone(),
        table_name: (*TABLE_NAME).to_string(),
        ..DeleteItemInput::default()
    };

    println!("del req: {:?}", delete_item_request);

    let res = client.delete_item(delete_item_request);

    //println!("_del resp: {:?}", res);

    res.await

}

fn get_key(pk: &DynamoPrimaryKey) -> HashMap<String, AttributeValue> {
    let mut key: HashMap<String, AttributeValue> = HashMap::new();
    key.insert(
        (*PARTITION_KEY_NAME).to_string(),
        pk.partition_key.to_vec().into_attr()
    );

    key.insert(
        (*SORT_KEY_NAME).to_string(),
        pk.sort_key.to_owned().into_attr()
    );

    key
}
