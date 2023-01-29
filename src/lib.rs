use anyhow::Result;
use reqwest::{ClientBuilder, Response};
use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

const CRATES_IO_URL: &str = "https://crates.io/api/v1/crates";
const UNIQUE_USER_AGENT: &str = "krates/0.3.0";

#[derive(Error, Debug)]
enum KrateError {
    #[error("Crate name is not found. Did you mispell the crate name?")]
    KrateNotFound,
    #[error("Server Status Error: {0}")]
    OtherKrateError(reqwest::Error),
}

impl Krate {
    pub fn get_latest(&self) -> String {
        String::from(&self.versions[0].num)
    }

    pub fn get_features_for_version(&self, version: &str) -> Option<&HashMap<String, Vec<String>>> {
        for v in &self.versions {
            if v.num == version {
                if let Some(features) = &v.features {
                    return Some(features);
                }
            }
        }
        None
    }
}

#[derive(Debug, Deserialize)]
pub struct Krate {
    pub categories: Vec<KrateCategory>,
    pub versions: Vec<KrateVersion>,
    #[serde(rename = "crate")]
    pub krate: KrateMetadata,
    pub keywords: Option<Vec<Option<KrateKeyword>>>,
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
    pub versions: Vec<i32>,
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

fn has_empty_user_agent(user_agent: &str) -> bool {
    user_agent.trim().len() == 0
}

pub fn get(crate_name: &str, user_agent: &str) -> Result<Krate> {
    if has_empty_user_agent(user_agent) {
        return Err(anyhow::anyhow!(
            "User Agent must be a string with at least one character"
        ));
    }

    let url = format!("{CRATES_IO_URL}/{crate_name}");
    let client = reqwest::blocking::ClientBuilder::new()
        .user_agent(format!(
            "{user_agent} - Brought to you by: {UNIQUE_USER_AGENT}",
        ))
        .build()?;

    let res = client.get(url).send()?;
    match res.error_for_status() {
        Ok(res) => {
            let krate: Krate = res.json()?;
            Ok(krate)
        }
        Err(e) => Err(handle_error(e).into()),
    }
}

pub async fn get_async(crate_name: &str, user_agent: &str) -> Result<Krate> {
    // Enforce a string with actual characters in it
    if has_empty_user_agent(user_agent) {
        return Err(anyhow::anyhow!(
            "User Agent must be a string with at least one character"
        ));
    }

    let url = format!("{CRATES_IO_URL}/{crate_name}");

    let client = ClientBuilder::new()
        .user_agent(format!(
            "{user_agent} - Brought to you by: {UNIQUE_USER_AGENT}",
        ))
        .build()?;

    let res: Response = client.get(url).send().await?;

    match res.error_for_status() {
        Ok(res) => {
            let krate: Krate = res.json().await?;
            Ok(krate)
        }
        Err(e) => Err(handle_error(e).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_async_crate_basic() {
        let krate = get_async("is-wsl", "Test Mocks for TheLarkInn/krate")
            .await
            .unwrap();
        assert_eq!(krate.krate.name, "is-wsl");
    }

    #[tokio::test]
    async fn test_get_async_latest_version_from_crate() {
        let krate: Krate = get_async("tokio", "Test Mocks for TheLarkInn/krate")
            .await
            .unwrap();
        assert_eq!(krate.get_latest(), "1.24.2");
    }

    #[tokio::test]
    async fn test_get_async_informs_operator_of_not_found_error() {
        let krate = get_async("tokioz", "Test Mocks for TheLarkInn/krate").await;
        assert!(krate.is_err());
        assert_eq!(
            krate.err().unwrap().to_string(),
            "Crate name is not found. Did you mispell the crate name?"
        );
    }

    #[tokio::test]
    async fn test_get_async_errors_on_empty_user_agent() {
        let krate = get_async("is-docker", "    ").await;
        assert_eq!(
            krate.err().unwrap().to_string(),
            "User Agent must be a string with at least one character"
        );
    }

    #[test]
    fn test_get_crate_basic() {
        let krate = get("is-interactive", "Test Mocks for TheLarkInn/krate").unwrap();
        assert_eq!(krate.krate.name, "is-interactive");
        assert_eq!(krate.versions[0].num, "0.1.0");
        assert_eq!(
            krate.krate.description,
            "Checks if stdout or stderr is interactive"
        );
    }

    #[test]
    fn test_get_get_latest() {
        let krate: Krate = get("syn", "Test Mocks for TheLarkInn/krate").unwrap();
        assert_eq!(krate.get_latest(), "1.0.107");
    }

    #[test]
    fn test_get_features_for_version() {
        let krate: Krate = get("tokio", "Test Mocks for TheLarkInn/krate").unwrap();
        let features = krate.get_features_for_version("1.24.2");
        assert_eq!(features.unwrap().len(), 15);
    }

    #[test]
    fn test_get_features_for_wrong_version() {
        let krate: Krate = get("cargo-outdated", "Test Mocks for TheLarkInn/krate").unwrap();
        let features = krate.get_features_for_version("9999.0.00");
        assert!(features.is_none());
    }
}
