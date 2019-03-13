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

    if let Some(file_path) = matches.value_of("file") {
        println!("{:?}", json::extract(file_path));
    }


    Ok(())
}
