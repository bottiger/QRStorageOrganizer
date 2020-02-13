extern crate diesel;
extern crate qrstore;
extern crate clap;

use qrstore::models::*;
use diesel::prelude::*;
use qrstore::dynamodb::*;

use std::env;
use clap::{Arg, App, SubCommand};

//use qrcode::im_encoder;

fn main() {
    use qrstore::schema::qrcodes::dsl::*;

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    //let matches = parseArgs(args);

    println!("dynamo:");
    print_dynamodb();

    let title_test = "Hello, World!";
    let add_res = qrstore::models::add(Some(title_test), None, None);

    match add_res {
        Ok(v) => println!("Inserted"),
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
