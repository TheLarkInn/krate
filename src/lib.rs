use serde::Deserialize;
use anyhow::Result;
use reqwest::{ClientBuilder, Response};
use std::collections::HashMap;
use thiserror::Error;

const CRATES_IO_URL: &str = "https://crates.io/api/v1/crates";
const UNIQUE_USER_AGENT: &str = "krates/0.1.0";

#[derive(Error, Debug)]
enum KrateError {
    #[error("Crate name is not found. Did you mispell the crate name?")]
    KrateNotFound,
    #[error("Server Status Error: {0}")]
    OtherKrateError(reqwest::Error)
}

impl Krate {
    pub fn get_latest(&self) -> String {
        String::from(&self.versions[0].num)
    }
}

#[derive(Debug, Deserialize)]
pub struct Krate {
    pub categories: Vec<KrateCategory>,
    pub versions: Vec<KrateVersion>,
    #[serde(rename = "crate")]
    pub krate: KrateMetadata,
    pub keywords: Option<Vec<Option<KrateKeyword>>>
}

#[derive(Debug, Deserialize)]
pub struct KrateVersion {
    pub crate_size: Option<i64>,
    pub license: Option<String>,
    pub num: String,
    pub readme_path: String,
    pub yanked: bool,
    pub features: Option<HashMap<String, Vec<String>>>,
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct KrateCategory {
    pub category: String,
    pub crates_cnt: i32,
    pub created_at: String,
    pub description: String,
    pub id: String,
    pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct KrateMetadata {
    pub categories: Vec<String>,
    pub created_at: String,
    pub description: String,
    pub documentation: Option<String>,
    pub downloads: i32,
    pub exact_match: bool,
    pub homepage: Option<String>,
    pub id: String,
    pub keywords: Vec<String>,
    //links:
    pub max_version: String,
    pub max_stable_version: String,
    pub name: String,
    pub newest_version: String,
    pub recent_downloads: i64,
    pub repository: String,
    pub updated_at: String,
    pub versions: Vec<i32>
}

#[derive(Debug, Deserialize)]
pub struct KrateKeyword {
    pub crates_cnt: i64,
    pub created_at: String,
    pub id: String,
    pub keyword: String,
}

fn handle_error(e: reqwest::Error) -> KrateError {
    if e.status() == Some(reqwest::StatusCode::NOT_FOUND) {
        KrateError::KrateNotFound
    } else {
        KrateError::OtherKrateError(e)
    }
}

pub async fn get_async(crate_name: &str) -> Result<Krate>  {
    let url = format!("{CRATES_IO_URL}/{crate_name}");

    let client = ClientBuilder::new()
        .user_agent(UNIQUE_USER_AGENT)
        .build()?;

    let res: Response = client.get(url).send().await?;

    match res.error_for_status() {
        Ok(res) => {
            let krate: Krate = res.json().await?;
            Ok(krate)
        },
        Err(e) => {
           Err(handle_error(e).into()) 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_crate_basic() {
        let krate = get_async("is-wsl").await.unwrap();
        assert_eq!(krate.krate.name, "is-wsl");
    }

    #[tokio::test]
    async fn test_get_latest_version_from_crate() {
        let krate: Krate = get_async("tokio").await.unwrap();
        assert_eq!(krate.get_latest(), "1.24.2");
    }

    #[tokio::test]
    async fn informs_operator_of_not_found_error() {
        let krate = get_async("tokioz").await;
        assert!(krate.is_err());
        assert_eq!(krate.err().unwrap().to_string(), "Crate name is not found. Did you mispell the crate name?");
    }
}

