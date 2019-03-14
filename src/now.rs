use super::json::Resume;
use reqwest::{Client, Response, Result};
use serde::Serialize;
use std::clone::Clone;
use std::collections::HashMap;

type Config = HashMap<String, String>;

#[derive(Debug, Serialize)]
struct Build {
    src: String,

    #[serde(rename = "use")]
    _use: String,
}

#[derive(Debug, Serialize)]
struct File {
    file: String,
    data: String,
}

#[derive(Debug, Serialize)]
struct Deployment {
    name: String,
    version: u16,
    files: Vec<File>,
    builds: Vec<Build>,
}

fn get_api_token(config: Config) -> Option<String> {
    config
        .get("ZEIT_API_TOKEN")
        .map(Clone::clone)
        .or_else(|| None)
}

fn get_lambda(_resume: &Resume) -> String {
    let template = "
        use http::{Request, Response, StatusCode, header};

        fn handler(request: Request<()>) -> http::Result<Response<String>> {
             let response = Response::builder()
                 .status(StatusCode::OK)
                 .header(header::CONTENT_TYPE, \"text/html\")
                 .body(\"<!doctype html><html><head><title>A simple deployment with Now!</title></head><body><h1>Welcome to Rust on Now</h1></body></html>\".to_string())
                 .expect(\"failed to render response\");
             
              Ok(response)
          }
     ";

    template.to_string()
}

pub fn deploy(resume: Resume, config: Config) -> Result<Response> {
    let client = Client::new();
    let token = get_api_token(config).unwrap();
    let authorization = format!("bearer {}", token);
    let builds = vec![Build {
        src: "index.rs".into(),
        _use: "@now/rust".into(),
    }];

    let files = vec![File {
        file: "index.rs".into(),
        data: get_lambda(&resume),
    }];

    let deployment = Deployment {
        name: "test-resume-deployment".into(),
        version: 2,
        builds,
        files,
    };

    client
        .post("https://api.zeit.co/v6/now/deployments")
        .header("Authorization", authorization)
        .json(&deployment)
        .send()
}
