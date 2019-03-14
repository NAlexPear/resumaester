use dirs::config_dir;
use std::collections::HashMap;
use std::io::prelude::Read;
use std::{fs, io};

pub fn handle() -> Result<HashMap<String, String>, io::Error> {
    let mut configurations = HashMap::new();
    let mut config_base = config_dir().unwrap();

    config_base.push("resumaester");
    config_base.push("config");

    if config_base.exists() {
        fs::File::open(&config_base).and_then(|mut file| {
            let mut contents = String::new();

            file.read_to_string(&mut contents).and_then(|_| {
                for line in contents.lines() {
                    let pair = line.split('=').collect::<Vec<&str>>();

                    configurations.insert(pair[0].to_string(), pair[1].to_string());
                }

                Ok(configurations)
            })
        })
    } else {
        let parent = config_base.parent().unwrap();

        fs::create_dir_all(&parent)
            .and(fs::File::create(&config_base))
            .and_then(|_| Ok(configurations))
    }
}
