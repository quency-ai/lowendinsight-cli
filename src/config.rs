use std::fs::{File};
use std::io::{Read};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub rapid_key: String,
}

pub fn read_config(path: &Path) -> Config {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            println!("LEI error: could not read config file.");
            std::process::exit(1);
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: Config = match toml::from_str(&contents) {
        Ok(contents) => contents,
        Err(error) => {
            println!("LEI error: could not read config contents - {}.", error);
            std::process::exit(1);

        }
    };
    config
}

pub fn find_default_config_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".config");
        path.push("lei");
        path.push("config.toml");
        path
    })
}