use std::collections::HashMap;
mod cli;
mod config;
mod analyze;
use anyhow::anyhow;
use config::*;

use cli::{Action::*, CommandLineArgs};
use structopt::StructOpt;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{Value};

#[macro_use]
extern crate log;
use env_logger::{Builder, Env, Target};
#[tokio::main]
async fn main() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    let CommandLineArgs {
            action,
            config_file,
    } = CommandLineArgs::from_args();

    let config_file = config_file
        .or_else(find_default_config_file)
        .ok_or_else(|| anyhow!("Failed to read default config file"))?;

    match action {
        Analyze {
            url,
            verbosity
        } => {
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
        }
    };
    Ok(())
}
