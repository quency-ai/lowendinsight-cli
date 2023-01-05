use std::io::Result;
use std::path::PathBuf;
use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{Value};
use prettytable::{Row};

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
  if summary {
    summarize(j);
  } else {
    println!("{}", j);
  }
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
  if summary {
    summarize(j);
  } else {
    println!("{}", j);
  }
  Ok(())
}

fn summarize(json: Value) {
  let results = &json["report"]["repos"][0]["data"]["results"];
  let files = &json["report"]["repos"][0]["data"]["files"];
  let version = "LowEndInsight: ".to_owned() + &drop_quotes(json["report"]["repos"][0]["header"]["library_version"].to_string());
  let mut table = table!();

  // Header
  table.add_row(row!["repo",
    drop_quotes(json["report"]["repos"][0]["data"]["repo"].to_string())
  ]);

  // Overall Risk
  let overall_risk = drop_quotes(json["report"]["repos"][0]["data"]["risk"].to_string());
  let row = create_row("overall risk".to_string(), overall_risk);
  table.add_row(row);

  // Commit Currency Risk
  let currency_risk = drop_quotes(results["commit_currency_risk"].to_string());
  let row = create_row("commit currency risk".to_string(), currency_risk);
  table.add_row(row);

  // Functional Contributors Risk
  let functional_contributor_risk = drop_quotes(results["functional_contributors_risk"].to_string());
  let row = create_row("functional contributor risk".to_string(), functional_contributor_risk);
  table.add_row(row);

  // Contributor Count Risk
  let contributor_risk = drop_quotes(results["contributor_risk"].to_string());
  let row = create_row("contributor count risk".to_string(), contributor_risk);
  table.add_row(row);

  // Large Commits/Change Risk
  let large_recent_commit_risk = drop_quotes(results["large_recent_commit_risk"].to_string());
  let row = create_row("large recent commit risk".to_string(), large_recent_commit_risk);
  table.add_row(row);

  // SBOM Risk
  let sbom_risk = drop_quotes(results["sbom_risk"].to_string());
  let row = create_row("sbom risk".to_string(), sbom_risk);
  table.add_row(row);
  
  table.add_row(row!["has readme file", drop_quotes(files["has_readme"].to_string())]);
  table.add_row(row!["has license file", drop_quotes(files["has_license"].to_string())]);
  table.add_row(row!["has contributing file", drop_quotes(files["has_contributing"].to_string())]);
  table.add_row(row!["binary files", drop_quotes(files["binary_files_count"].to_string())]);
  table.add_row(row!["total file count",drop_quotes(files["total_file_count"].to_string())]);
  table.add_row(row![version, "https://github.com/gtri/lowendinsight"]);

  table.printstd();
}

fn create_row(key: String, value: String) -> Row {
  let row = match value.as_str() {
    "critical" => {row![Fr => key, "critical"]}
    "high" => {row![Fy => key, "high"]}
    "medium" => {row![Fc => key, "medium"]}
    "low" => {row![Fg => key, "low"]}
    _ => {row![Fr => key, "unknown"]}
  };
  row
}
fn drop_quotes(value: String) -> String {
  let quoteless = value.trim_end_matches('"').trim_start_matches('"');
  quoteless.to_string()
}