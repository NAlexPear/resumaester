mod json;

use clap::{Arg, App};


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

    let file_path = matches.value_of("file").unwrap_or("./resume.json");

    match json::extract(file_path) {
        Ok(resume ) => println!("{:?}", resume),
        Err(error) => match error {
            json::ExtractionError::IOError(_) => println!("JSON resume not found! Would you like to create a new resume instead?"),
            json::ExtractionError::JSONParseError(parse_error) => println!("{:?}", parse_error)
        }
    }

    Ok(())
}
