use std::fs;
use std::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
  pub git_token: String,
  pub git_repo_update_in_days: u16,
  pub gitlab_url: String,
}

impl Config {
  pub fn new(config_file: String) -> Result<Config, Box<dyn Error>> {

    let config_content = fs::read_to_string(config_file)?;

    let config: Config = serde_yaml::from_str(&config_content)?;

    Ok(config)
  }
}
