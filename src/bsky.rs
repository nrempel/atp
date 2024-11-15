use crate::{Client, Config};
use clap::Parser;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Parser)]
pub struct Profile {
    #[clap(short, long)]
    actor: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResponse {
    did: String,
    handle: String,
    display_name: Option<String>,
    description: Option<String>,
    indexed_at: String,
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
