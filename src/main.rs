#![feature(rustc_private)]
extern crate getopts;
use getopts::Options;
use serde::{Serialize, Deserialize};
use std::fs;
use std::env;


#[derive(Deserialize, Serialize)]
struct Basics {
    name: String,
    label: String
}

#[derive(Deserialize, Serialize)]
struct Location {
    address: String,
    postal_code: u32,
    city: String,
    country_code: String,
    region: String
}

#[derive(Deserialize, Serialize)]
struct Resume {
    basics: Basics,
    location: Location
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args: Vec<String> = env::args().collect();
    let mut options = Options::new();

    options.optopt("f", "file", "use existing file", "FILE");

    let matches = match options.parse(&args[1..]) {
        Ok(arg) => { arg }
        Err(err) => { panic!(err.to_string()) }
    };

    let filepath = matches.opt_str("f").unwrap();
    let file_contents = fs::read_to_string(filepath).unwrap();
    
    println!("{:?}", file_contents);
    Ok(())
}
