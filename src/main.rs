extern crate reqwest;
extern crate env_logger;

use log::*;
use std::env;
use std::process;
use env_logger::Env;
use std::collections::HashMap;
use serde::{Deserialize};

mod config;
mod gitlab;

#[derive(Deserialize, Debug)]
struct Pkg {
  dependencies: Option<HashMap<String, String>>,

  #[serde(rename = "devDependencies")]
  dev_dependencies: Option<HashMap<String, String>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  env_logger::from_env(Env::default().default_filter_or("warn")).init();
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    error!("Missing config file path");
    process::exit(1);
  }

  let config: config::Config = config::Config::new(args[1].clone()).unwrap_or_else(|err|{
    error!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  let repos: Vec<gitlab::Repo> = gitlab::get_all_reposities(
    &config.gitlab_url,
    &config.git_token,
    &config.git_repo_update_in_days
  ).unwrap_or_else(|err|{
    error!("Getting repos from Gitlab: {}", err);
    process::exit(1);
  });


  let mut package_total: HashMap<String, u32> = HashMap::new();

  for repo in repos {
    let package_json: String = gitlab::read_package_file_from_repo(
      &config.gitlab_url,
      &config.git_token,
      &repo.default_branch,
      &repo.id
    ).unwrap();

    if package_json.len() > 0 {
      let pkg: Pkg = serde_json::from_str(&package_json)?;

      match pkg.dev_dependencies {
        Some(dev_dependencies) => {
          for key in dev_dependencies.keys() {
            if !package_total.contains_key(key) {
              package_total.insert(key.clone(), 0);
            }
    
            if let Some(x) = package_total.get_mut(key) {
              *x = *x + 1;
            }
          }
        },
        _ => (),
      }

      match pkg.dependencies {
        Some(dependencies) => {
          for key in dependencies.keys() {
            if !package_total.contains_key(key) {
              package_total.insert(key.clone(), 0);
            }
    
            if let Some(x) = package_total.get_mut(key) {
              *x = *x + 1;
            }
          }
        },
        _ => (),
      }
    }
  }

  println!("{:?}", package_total);

  Ok(())
}
