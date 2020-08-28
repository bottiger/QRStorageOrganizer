extern crate clap;
extern crate config;
extern crate env_logger;
extern crate qrcode;
extern crate qrstore;
use qrstore::config::get_config;
use crate::qrstore::model::schema::DynamoSearchKey;
use clap::{App, Arg, SubCommand};
use qrstore::config::init_env;
use qrstore::dynamodb::crud::query;
use qrstore::dynamodb::get_group;
use qrstore::dynamodb::insert_group;
use qrstore::fixtures::get_fixture;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _args = parse_args(env::args().collect());

    env_logger::init();

    let fixture = get_fixture();
    log::debug!("Fixture: {:?}", fixture);

    init_env();

    let config = get_config();

    println!("{:?}", config);

    let fixture = get_fixture();
    log::debug!("Fixture: {:?}", fixture);

    let _reginsert = match fixture {
        Ok(mut f) => {
            f.qrcodes[0].title = Some("Pest".to_string());

            let search_key: DynamoSearchKey = 4;

            match query(&f.group_id, Some(&search_key)).await {
                Ok(v) => println!("Query: {:#?}", v),
                Err(e) => panic!("Error completing futures: {}", e),
            }

            match insert_group(&f).await {
                Ok(v) => println!("Inserted: {:#?}", v),
                Err(e) => panic!("Error completing futures: {}", e),
            };

            let group = get_group(&f.group_id).await;
            println!("get_group: {:#?}", group);
        }
        Err(e) => println!("Error: {}", e),
    };

    Ok(())
}

fn parse_args(_args: Vec<String>) -> clap::ArgMatches<'static> {
    let matches = App::new("QR code storage ")
        .version("0.1")
        .author("Arvid Böttiger <bottiger@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .get_matches();

    matches
}
