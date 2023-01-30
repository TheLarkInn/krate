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

#[derive(Debug)]
pub struct SyncKrateClient {
    client: reqwest::blocking::Client,
}

#[derive(Debug)]
pub struct AsyncKrateClient {
    client: reqwest::Client,
}

impl SyncKrateClient {
    pub fn get(&self, crate_name: &str) -> anyhow::Result<Krate> {
        let url = format!("{CRATES_IO_URL}/{crate_name}");

        let res = self.client.get(url).send()?;
        match res.error_for_status() {
            Ok(res) => {
                let krate: Krate = res.json()?;
                Ok(krate)
            }
            Err(e) => Err(handle_error(e).into()),
        }
    }
}

impl AsyncKrateClient {
    pub async fn get_async(&self, crate_name: &str) -> anyhow::Result<Krate> {
        let url = format!("{CRATES_IO_URL}/{crate_name}");
        let res: Response = self.client.get(url).send().await?;

        match res.error_for_status() {
            Ok(res) => {
                let krate: Krate = res.json().await?;
                Ok(krate)
            }
            Err(e) => Err(handle_error(e).into()),
        }
    }
}

pub struct KrateClientBuilder {
    user_agent: String,
}

impl KrateClientBuilder {
    pub fn new(user_agent: &str) -> KrateClientBuilder {
        KrateClientBuilder {
            user_agent: user_agent.to_string(),
        }
    }

    pub fn build_sync(&self) -> anyhow::Result<SyncKrateClient> {
        if has_empty_user_agent(&self.user_agent) {
            return Err(anyhow::anyhow!(
                "User Agent must be a string with at least one character"
            ));
        }

        let operator_user_agent = format!(
            "{} - Brought to you by: {UNIQUE_USER_AGENT}",
            self.user_agent
        );

        let client = reqwest::blocking::ClientBuilder::new()
            .user_agent(&operator_user_agent)
            .build()?;

        return Ok(SyncKrateClient { client: client });
    }

    pub fn build_asnyc(&self) -> anyhow::Result<AsyncKrateClient> {
        if has_empty_user_agent(&self.user_agent) {
            return Err(anyhow::anyhow!(
                "User Agent must be a string with at least one character"
            ));
        }

        let operator_user_agent = format!(
            "{} - Brought to you by: {UNIQUE_USER_AGENT}",
            self.user_agent
        );

        let client = reqwest::ClientBuilder::new()
            .user_agent(&operator_user_agent)
            .build()?;

        return Ok(AsyncKrateClient { client: client });
    }
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

    fn client_builder() -> KrateClientBuilder {
        KrateClientBuilder::new("Test Mocks for TheLarkInn/krate")
    }

    fn get_sync_krate_client() -> SyncKrateClient {
        client_builder().build_sync().unwrap()
    }

    fn get_async_krate_client() -> AsyncKrateClient {
        client_builder().build_asnyc().unwrap()
    }

    #[tokio::test]
    async fn test_get_async_crate_basic() {
        let krate = get_async_krate_client().get_async("is-wsl").await.unwrap();
        assert_eq!(krate.krate.name, "is-wsl");
    }

    #[tokio::test]
    async fn test_get_async_latest_version_from_crate() {
        let krate: Krate = get_async_krate_client().get_async("tokio").await.unwrap();
        assert_eq!(krate.get_latest(), krate.versions[0].num);
    }

    #[tokio::test]
    async fn test_get_async_informs_operator_of_not_found_error() {
        let krate = get_async_krate_client().get_async("tokioz").await;
        assert!(krate.is_err());
        assert_eq!(
            krate.err().unwrap().to_string(),
            "Crate name is not found. Did you mispell the crate name?"
        );
    }

    #[tokio::test]
    async fn test_get_async_errors_on_empty_user_agent() {
        let builder = KrateClientBuilder::new("     ").build_asnyc();

        assert_eq!(
            builder.err().unwrap().to_string(),
            "User Agent must be a string with at least one character"
        );
    }

    #[test]
    fn test_get_crate_basic() {
        let krate = get_sync_krate_client().get("is-interactive").unwrap();
        assert_eq!(krate.krate.name, "is-interactive");
        assert_eq!(krate.versions[0].num, "0.1.0");
        assert_eq!(
            krate.krate.description,
            "Checks if stdout or stderr is interactive"
        );
    }

    #[test]
    fn test_get_get_latest() {
        let krate: Krate = get_sync_krate_client().get("syn").unwrap();
        assert_eq!(krate.get_latest(), krate.versions[0].num);
    }

    #[test]
    fn test_get_features_for_version() {
        let krate: Krate = get_sync_krate_client().get("tokio").unwrap();
        let features = krate.get_features_for_version("1.24.2");
        assert_eq!(features.unwrap().len(), 15);
    }

    #[test]
    fn test_get_features_for_wrong_version() {
        let krate: Krate = get_sync_krate_client().get("cargo-outdated").unwrap();
        let features = krate.get_features_for_version("9999.0.00");
        assert!(features.is_none());
    }
}
