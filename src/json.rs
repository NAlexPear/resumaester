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
#[serde(rename_all="camelCase")]
struct Study {
    institution: String,
    area: String,
    study_type: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Skill {
    name: String,
    level: String,
    keywords: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
struct Language {
    language: String,
    fluency: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Resume {
    basics: Basics,
    work: Vec<Job>,
    education: Vec<Study>,
    skills: Vec<Skill>,
    languages: Vec<Language>
}

pub fn extract<T: AsRef<Path>>(file_path: T) -> Option<Resume>{
    let file_contents = fs::read_to_string(file_path).unwrap(); 
    serde_json::from_str(&file_contents).ok()
}
