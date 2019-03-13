use clap::{Arg, App};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;


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
struct Profile {
    network: String,
    username: String,
    url: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Basics {
    name: String,
    label: String,
    location: Location,
    profiles: Vec<Profile>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
struct Job {
    company: String,
    position: String,
    website: String,
    start_date: String,
    end_date: Option<String>,
    summary: String,
    highlights: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
struct Resume {
    basics: Basics,
    work: Vec<Job>
}

fn extract_file<T: AsRef<Path>>(file_path: T) -> Option<Resume>{
    let file_contents = fs::read_to_string(file_path).unwrap(); 
    serde_json::from_str(&file_contents).ok()
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

    if let Some(file_path) = matches.value_of("file") {
        println!("{:?}", extract_file(file_path));
    }


    Ok(())
}
