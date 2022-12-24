use std::fs::{File};
use std::io::{Read};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub rapid_key: String,
}

pub fn read_config(path: &Path) -> Config {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: Config = toml::from_str(&contents).unwrap();
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