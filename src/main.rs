use clap::{Arg, App};
use serde::{Serialize, Deserialize};
use std::fs;


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
struct Location {
    address: String,
    postal_code: String,
    city: String,
    country_code: String,
    region: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Basics {
    name: String,
    label: String,
    location: Location
}

#[derive(Debug, Deserialize, Serialize)]
struct Resume {
    basics: Basics,
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let matches = App::new("ResuMaester")
                    .about("Resume management for a paperless world")
                    .arg(Arg::with_name("file")
                            .short("f")
                            .long("file")
                            .value_name("FILE")
                            .help("Ingests existing JSON resume")
                            .takes_value(true))
                    .get_matches();

    let file_path = matches.value_of("file").unwrap();
    let file_contents = fs::read_to_string(file_path).unwrap();
    let resume: Resume = serde_json::from_str(&file_contents).unwrap();

    println!("{:?}", resume);

    Ok(())
}
