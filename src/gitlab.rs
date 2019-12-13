use reqwest::StatusCode;
use log::*;
use chrono::prelude::*;
use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
pub struct Repo {
  pub id: u16,
  pub name: String,
  pub default_branch: String,
  pub last_activity_at: String,
}

#[derive(Deserialize, Debug, Clone)]
struct UnhandleRepo {
  id: u16,
  name: String,
  default_branch: Option<String>,
  last_activity_at: String,
}

pub fn get_all_reposities(gitlab_url: &String, token: &String, days: &u16) -> Result<Vec<Repo>, Box<dyn std::error::Error>> {
  let today: DateTime<Utc> = Utc::now();
  let mut repos: Vec<Repo> = Vec::new();
  let mut page: u8 = 1;
  let per_page: u8 = 100;

  loop {
    let uri = format!("{}/api/v4/projects?per_page={}&page={}&private_token={}&order_by=last_activity_at&simple=true", gitlab_url, per_page, page.to_string(), token);
    let mut resp: reqwest::Response = reqwest::get(&uri)?;
    
    match resp.status() {
      StatusCode::OK => {
        let response_text = resp.text()?;
        let total_pages_header_value = resp.headers().get("X-Total-Pages").unwrap();
        let total_pages = total_pages_header_value.to_str().unwrap().parse::<u8>().unwrap();

        let tmp: Vec<UnhandleRepo> = serde_json::from_str(&response_text)?;

        for repo in tmp {
          match repo.default_branch {
            Some(branch) => {
              let last_activity_at = DateTime::parse_from_rfc3339(&repo.last_activity_at).unwrap();

              let duration = today.signed_duration_since(last_activity_at);

              if duration.num_days() <= *days as i64 {
                repos.push(Repo {
                  id: repo.id,
                  name: repo.name,
                  default_branch: branch,
                  last_activity_at: repo.last_activity_at,
                });
              } else {
                break;
              }
            },
            _ => (),
          }
        }

        if total_pages > page {
          page = page + 1;
        } else {
          break;
        }
      },
      s => error!("Received response status when list repositories: {:?}", s),
    };
  }

  Ok(repos)
}

pub fn read_package_file_from_repo(gitlab_url: &String, token: &String, branch: &String, id: &u16) -> Result<String, Box<dyn std::error::Error>> {
  let uri = format!("{}/api/v4/projects/{}/repository/files/package%2Ejson/raw?ref={}&private_token={}", gitlab_url, id.to_string(), branch, token);
  let mut resp: reqwest::Response = reqwest::get(&uri)?;

  Ok(match resp.status() {
    StatusCode::OK => {
      let response_text = resp.text()?;
      response_text
    },
    s => {
      if StatusCode::NOT_FOUND != s {
        error!("Received response status when read package.json: {:?}", s)
      }

      String::new()
    },
  })
}
