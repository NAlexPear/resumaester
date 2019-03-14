mod config;
mod json;
mod now;

use clap::{App, Arg};
use console::{style, Term};
use dialoguer::Confirmation;
use json::ExtractionError::{IOError, JSONParseError};
use now::deploy;
use std::error;

fn handle_missing_resume() {
    let term = Term::stdout();
    let styled_prompt = style("JSON resume not found!").yellow();
    let confirmation_text = format!(
        "{} {}",
        styled_prompt, "Would you like to create a new resume instead?"
    );

    let should_create_resume = Confirmation::new()
        .with_text(&confirmation_text)
        .interact()
        .unwrap();

    if should_create_resume {
        term.write_line("Doing the thing!").unwrap();
    }
}

fn main() -> Result<(), Box<dyn error::Error + 'static>> {
    let config = config::handle().unwrap();

    let matches = App::new("ResuMaester")
        .about("Resume management for a paperless world")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Ingests existing JSON resume")
                .takes_value(true),
        )
        .get_matches();

    let file_path = matches.value_of("file").unwrap_or("./resume.json");

    match json::extract(file_path) {
        Ok(resume) => match deploy(resume, config) {
            Ok(response) => println!("{:?}", response),
            Err(deploy_err) => println!("{:?}", deploy_err),
        },
        Err(error) => match error {
            IOError(_) => handle_missing_resume(),
            JSONParseError(parse_error) => println!("{:?}", parse_error),
        },
    }

    Ok(())
}
