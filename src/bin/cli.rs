extern crate qrstore;
extern crate clap;
extern crate qrcode;
#[macro_use] extern crate log;
extern crate env_logger;
#[macro_use]
extern crate simple_error;

use log::Level;

use qrstore::fixtures::get_fixture;
use qrstore::fixtures::get_fixture_code;

//use qrstore::dynamodb::insert_group;
use qrstore::dynamodb::crud::query;
use qrstore::dynamodb::insert_group;
use qrstore::dynamodb::get_group;
use clap::{Arg, App, SubCommand};

use futures::executor::block_on;
//use qrcode::im_encoder;

use qrstore::storage::get_bucket;


//use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials};


use crate::qrstore::model::schema::DynamoSearchKey;
use crate::qrstore::model::qrcode::QrCodeDB;
use crate::qrstore::model::qrcode::QrCode;
use crate::qrstore::model::qrgroup::QrGroupDB;
use crate::qrstore::model::qrgroup::QrGroup;
use crate::qrstore::model::schema::DbItem;

use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};

use std::env;

#[tokio::main]
async fn main2() {

    //let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    //let matches = parseArgs(args);
    //println!("uuid1: {:?}", gen_uuid_str("This is a test").to_u128_le());
    //println!("uuid1: {:?}", gen_uuid_str("This is a test").to_u128_le());
    //println!("uuid2: {:?}", gen_uuid_str("This is a test2").to_u128_le());

    //println!("dynamo:");
    //print_dynamodb();

    env_logger::init();
    log::debug!("[foo] debug");

    env::set_var("AWS_ACCESS_KEY_ID", "AKIA3MNCVP2PGTEJOUSX");
    env::set_var("AWS_SECRET_ACCESS_KEY", "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");

    // wasabi
    //env::set_var("AWS_ACCESS_KEY_ID", "9RB1ETUGDVPR8TM87MQA");
    //env::set_var("AWS_SECRET_ACCESS_KEY", "jsCvqZtEsUhm3s8CeDhSkdLpoKT2eAm2A4SeHhPz");

    /*
    let client = DynamoDbClient::new(Region::EuNorth1);
    let list_tables_input: ListTablesInput = Default::default();

    match client.list_tables(list_tables_input).await {
        Ok(output) => match output.table_names {
            Some(table_name_list) => {
                println!("Tables in database:");

                for table_name in table_name_list {
                    println!("{}", table_name);
                }
            }
            None => println!("No tables in database!"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }

    let list_tables_input2: ListTablesInput = Default::default();
    match client.list_tables(list_tables_input2).await {
        Ok(output) => match output.table_names {
            Some(table_name_list) => {
                println!("Tables in database:");

                for table_name in table_name_list {
                    println!("{}", table_name);
                }
            }
            None => println!("No tables in database!"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
    */



    let _title_test = "Hello, World!";

    //get_bucket().await;

}  

#[tokio::main]
async fn main() {

    env::set_var("AWS_ACCESS_KEY_ID", "AKIA3MNCVP2PGTEJOUSX");
    env::set_var("AWS_SECRET_ACCESS_KEY", "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");

    let fixture = get_fixture();
    println!("Fixture: {:?}", fixture);


    println!("put 02");


    let _reginsert = match fixture {
        Ok(mut f) => {

            /*
            let qr_item_1 = get_fixture_code().ok();
            let qr_group_db: QrCodeDB = QrCodeDB::from(qr_item_1.unwrap());

            println!("--------------------------");
            println!("put 03: {:#?}", qr_group_db);

            let r = insert_sync(&qr_group_db);

            println!("r: {:#?}", r);
            */

            f.qrcodes[0].title = Some("Pest".to_string());

            let search_key: DynamoSearchKey = String::from("s");
            /*
            match query(&f.get_partition_key(), Some(&search_key)).sync() {
                Ok(v) => println!("Query: {:#?}", v),
                Err(e) => panic!("Error completing futures: {}", e),
            }
            */
            match query(&f.group_id, Some(&search_key)).await {
                Ok(v) => println!("Query: {:#?}", v),
                Err(e) => panic!("Error completing futures: {}", e),
            }


            
            match insert_group(&f).await {
                Ok(v) => println!("Inserted: {:#?}", v),
                Err(e) => panic!("Error completing futures: {}", e),
            };
            

            let group = block_on(get_group(&f.group_id));
            println!("get_group: {:#?}", group);

            /*
            match get_group(&f.group_id).sync() {
                Ok(v) => {
                    //let v2 = v.map(|result| result.item.map(Qrgroup::from_attrs));
                    println!("Group get: {:#?}", v);

                    let it = v.items.unwrap();

                    /*
                    let out = it.iter().map(|item| {//.for_each(|item| {
                                    println!("start");
                                    println!("stream_scan() item {:#?}", Qrgroup::from_attrs(item.clone()));
                                    println!("end");
                                    //Ok(())
                                });

                    out.clone()
                    */
                    let mut group: QrGroup = Default::default();
                    let mut codes: Vec<QrCode> = Vec::new();

                    for item in it.iter() {
                        println!("start");
                        println!("item: {:#?}", item);

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

                        println!("stream_scan() item {:#?}", group);
                        println!("end");
                    }

                },
                Err(e) => panic!("Error completing futures: {}", e),
            };
            */



            /*
            match insert(&f).sync() {
                Ok(v) => println!("Inserted: {:#?}", v),
                Err(e) => panic!("Error completing futures: {}", e),
            };
            */

            /*
            f.qrcodes[0].title = Some("Pest".to_string());

            let search_key: DynamoSearchKey = String::from("s");
            match query(&f.get_partition_key(), Some(&search_key)).sync() {
                Ok(v) => println!("Query: {:#?}", v),
                Err(e) => panic!("Error completing futures: {}", e),
            }
            */

            /*
            match update(&f).sync() {
                Ok(v) => println!("Updated: {:#?}", v),
                Err(e) => panic!("Error completing futures: {}", e),
            };

            match get(&f.get_primary_key()).sync() {
                Ok(v) => println!("Get: {:#?}", v),
                Err(e) => panic!("Error completing futures: {:?}", e),
            };
            */

            /*
            match delete(&f.get_primary_key()).sync() {
                Ok(v) => println!("Deleted: {:#?}", v),
                Err(e) => panic!("Error completing futures: {:?}", e),
            };
            */
        },
        Err(e) => println!("Error: {}", e),
    };


    /*
    let add_res = qrstore::types::add(Some(title_test), None, None);

    match add_res {
        Ok(_v) => println!("Inserted"),
        Err(e) => println!("error parsing header: {:?}", e),
    }

    let connection = qrstore::establish_connection();
    let results = qrcodes
        //.filter(published.eq(true))
        .limit(5)
        .load::<QrEntry>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    //for qr in results {
    //    println!("{}", qr.title);
    //    println!("----------\n");
    //    println!("{}", qr.body;
    //}
    if results.len() > 0 {
        println!("Test");
        let im = qrstore::im_encoder::to_img(results.first().unwrap());
        qrstore::im_encoder::write("C:\\Users\\abottiger\\qrcode\\out\\qr.png", im);
        let read_im = qrstore::im_encoder::from_img();
        println!("Read res: {}", read_im);
    }
    */
}

#[allow(dead_code)]
fn parse_args(_args: Vec<String>) -> clap::ArgMatches<'static> {

    let matches = App::new("QR code storage ")
                          .version("0.1")
                          .author("Arvid Böttiger <bottiger@gmail.com>")
                          .about("Does awesome things")
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Sets a custom config file")
                               .takes_value(true))
                          .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(false)
                               .index(1))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .subcommand(SubCommand::with_name("test")
                                      .about("controls testing features")
                                      .version("1.3")
                                      .author("Someone E. <someone_else@other.com>")
                                      .arg(Arg::with_name("debug")
                                          .short("d")
                                          .help("print debug information verbosely")))
                          .get_matches();

    matches

}
