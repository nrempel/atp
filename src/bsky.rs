use crate::{Client, Config};
use clap::Parser;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Parser)]
pub struct Profile {
    #[clap(short, long)]
    actor: String,
}

#[derive(Parser)]
pub struct Preferences {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResponse {
    did: String,
    handle: String,
    display_name: Option<String>,
    description: Option<String>,
    indexed_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreferencesResponse {
    #[serde(rename = "preferences")]
    preferences: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preference {
    #[serde(rename = "$type")]
    type_: String,
    enabled: Option<bool>,
    items: Option<Vec<String>>,
}

impl Profile {
    pub async fn process(
        &self,
        client: &Client,
        config: &Config,
    ) -> anyhow::Result<ProfileResponse> {
        let actor = self.actor.trim_start_matches('@');
        let url = format!("{}/app.bsky.actor.getProfile", super::BASE_URL);

        let session = config
            .session
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;

        let res = client
            .inner()
            .get(url)
            .header("Authorization", format!("Bearer {}", session.access_jwt))
            .query(&[("actor", actor)])
            .send()
            .await?;

        match res.status() {
            reqwest::StatusCode::OK => Ok(res.json().await?),
            reqwest::StatusCode::UNAUTHORIZED => anyhow::bail!("Authentication required"),
            reqwest::StatusCode::NOT_FOUND => anyhow::bail!("Profile not found"),
            _ => {
                let error = res.text().await?;
                anyhow::bail!("Profile lookup failed: {}", error)
            }
        }
    }
}

impl Preferences {
    pub async fn process(
        &self,
        client: &Client,
        config: &Config,
    ) -> anyhow::Result<PreferencesResponse> {
        let url = format!("{}/app.bsky.actor.getPreferences", super::BASE_URL);

        let session = config
            .session
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;

        let res = client
            .inner()
            .get(url)
            .header("Authorization", format!("Bearer {}", session.access_jwt))
            .send()
            .await?;

        match res.status() {
            reqwest::StatusCode::OK => Ok(res.json().await?),
            reqwest::StatusCode::UNAUTHORIZED => anyhow::bail!("Authentication required"),
            _ => {
                let error = res.text().await?;
                anyhow::bail!("Failed to get preferences: {}", error)
            }
        }
    }
}

impl Display for ProfileResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "DID: {}", self.did)?;
        writeln!(f, "Handle: {}", self.handle)?;
        if let Some(name) = &self.display_name {
            writeln!(f, "Display Name: {}", name)?;
        }
        if let Some(desc) = &self.description {
            writeln!(f, "Description: {}", desc)?;
        }
        write!(f, "Indexed At: {}", self.indexed_at)
    }
}

impl Display for PreferencesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            serde_json::to_string_pretty(&self.preferences).unwrap()
        )
    }
}
