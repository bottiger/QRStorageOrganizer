extern crate clap;
extern crate config;
extern crate env_logger;
extern crate qrcode;
extern crate qrstore;
use crate::qrstore::model::schema::DynamoSearchKey;
use clap::{Parser, arg, command};
use qrstore::config::get_config;
use qrstore::config::init_env;
use qrstore::fixtures::get_fixture;
use std::env;
use std::error::Error;
use std::path::PathBuf;

/*
fn parse_args(_args: Vec<String>) -> clap::ArgMatches<'static> {
    App::new("QR code storage ")
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
        .get_matches()
}
        */

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "INPUT")]
    input: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //let _args = parse_args(env::args().collect());
    let cli = Cli::parse();

    env_logger::init();

    let fixture = get_fixture();
    log::debug!("Fixture: {:?}", fixture);

    init_env();

    let config = get_config();

    println!("{:?}", config);

    let fixture = get_fixture();
    log::debug!("Fixture: {:?}", fixture);

    /* 
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
    */

    Ok(())
}
