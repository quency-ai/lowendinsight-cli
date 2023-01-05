use std::io::Result;
use std::path::PathBuf;
use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{Value};

use crate::config;

use env_logger::{Builder, Env, Target};

pub async fn analyze(config_file: PathBuf, url: String, verbosity: u8, summary: bool) -> Result<()> {
  // Get Config from config file
  let config = config::read_config(&config_file);
  let rapid_key = &config.rapid_key;

  let log_level = match verbosity {
      0 => "error",
      1 => "info",
      _ => "debug"
  };
  Builder::from_env(Env::default().default_filter_or(log_level))
      .target(Target::Stdout)
      .init();
  info!("Analyzing {}", url);

  let mut body = HashMap::new();
  let urls = vec![url];
  body.insert("urls", urls);

  let mut headers = HeaderMap::new();
  headers.insert("X-RapidApi-Key", HeaderValue::from_str(rapid_key).unwrap());
  headers.insert("X-RapidApi-Host", HeaderValue::from_str("lowendinsight2.p.rapidapi.com").unwrap());
  let client = reqwest::Client::new();
  let resp = client.post("https://lowendinsight2.p.rapidapi.com/analyze")
      .headers(headers)
      .json(&body)
      .send()
      .await
      .unwrap()
      .text()
      .await
      .unwrap();

  let j: Value = serde_json::from_str(&resp)?;
  println!("{}", j);
  Ok(())
}

pub async fn get_analysis(config_file: PathBuf, uuid:String, verbosity: u8, summary: bool) -> Result<()> {
  // Get Config from config file
  let config = config::read_config(&config_file);
  let rapid_key = &config.rapid_key;

  let log_level = match verbosity {
      0 => "error",
      1 => "info",
      _ => "debug"
  };
  Builder::from_env(Env::default().default_filter_or(log_level))
      .target(Target::Stdout)
      .init();
  info!("Getting analysis for job: {}", uuid);

  let mut headers = HeaderMap::new();
  headers.insert("X-RapidApi-Key", HeaderValue::from_str(rapid_key).unwrap());
  headers.insert("X-RapidApi-Host", HeaderValue::from_str("lowendinsight2.p.rapidapi.com").unwrap());
  let client = reqwest::Client::new();
  let resp = client.get("https://lowendinsight2.p.rapidapi.com/analyze".to_owned() + "/" + &uuid)
      .headers(headers)
      .send()
      .await
      .unwrap()
      .text()
      .await
      .unwrap();

  let j: Value = serde_json::from_str(&resp)?;
  println!("{}", j);
  Ok(())
}