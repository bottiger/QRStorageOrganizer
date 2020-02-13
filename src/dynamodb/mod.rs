extern crate qrcode;

extern crate rusoto_core;
extern crate rusoto_dynamodb;

use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeDefinition, AttributeValue, CreateTableInput, CreateTableOutput, DynamoDb,
    DynamoDbClient, GetItemError, GetItemInput, GetItemOutput, KeySchemaElement,
    UpdateItemInput, UpdateItemOutput, ListTablesInput,
};
use std::collections::HashMap;

pub fn print_dynamodb() {
    let client = DynamoDbClient::new(Region::EuNorth1);
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

}

fn make_item() -> HashMap<String, AttributeValue> {
    let item_key = "qr-group-id";
    let mut item = HashMap::new();
    item.insert(
        item_key.to_string(),
        //AttributeValue { },
        AttributeValue {
            s: Some("nesty".to_string()),
            ..Default::default()
        },
    );

    item
}
