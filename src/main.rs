mod cli;
mod config;
mod analyze;
use anyhow::anyhow;
use config::*;

use cli::{Action::*, CommandLineArgs};
use structopt::StructOpt;

#[macro_use]
extern crate log;

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
        Analyze { url, verbosity, summary} => {
            match analyze::analyze(config_file, url, verbosity, summary).await {
                Ok(res) => res,
                Err(_) => {
                    println!("LEI Error: asynchronous problem doing analysis.");
                    std::process::exit(1);
                }
            }
        },
        GetAnalysis { uuid, verbosity, summary} => {
            match analyze::get_analysis(config_file, uuid, verbosity, summary).await {
                Ok(res) => res,
                Err(_) => {
                    println!("LEI Error: asynchronous problem doing analysis.");
                    std::process::exit(1);
                }
            }
        },
    };
    Ok(())
}
