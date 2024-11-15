use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::{Client, Config, BASE_URL};

#[derive(Parser)]
pub enum Auth {
    Login(Login),
    Session,
}

#[derive(Parser, Serialize)]
pub struct Login {
    #[clap(short, long)]
    identifier: String,
    #[clap(short, long)]
    password: String,
}

impl Login {
    pub async fn process(&self, client: &Client) -> anyhow::Result<LoginResponse> {
        let identifier = self.identifier.trim_start_matches('@');
        let url = format!("{BASE_URL}/com.atproto.server.createSession");
        let res = client
            .inner()
            .post(url)
            .json(&serde_json::json!({
                "identifier": identifier,
                "password": self.password
            }))
            .send()
            .await?;

        if !res.status().is_success() {
            let error = res.text().await?;
            anyhow::bail!("Login failed: {}", error);
        }

        Ok(res.json().await?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub(crate) did: String,
    pub(crate) handle: String,
    pub(crate) email: String,
    pub(crate) access_jwt: String,
    pub(crate) refresh_jwt: String,
}

pub(super) async fn make_authenticated_request<T: serde::de::DeserializeOwned>(
    client: &Client,
    config: &Config,
    endpoint: &str,
    query: &[(&str, String)],
) -> anyhow::Result<T> {
    let url = format!("{}/app.bsky.actor.{}", super::BASE_URL, endpoint);

    let session = config
        .session
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;

    let res = client
        .inner()
        .get(url)
        .header("Authorization", format!("Bearer {}", session.access_jwt))
        .query(query)
        .send()
        .await?;

    match res.status() {
        reqwest::StatusCode::OK => Ok(res.json().await?),
        reqwest::StatusCode::UNAUTHORIZED => anyhow::bail!("Authentication required"),
        reqwest::StatusCode::NOT_FOUND => anyhow::bail!("Not found"),
        _ => {
            let error = res.text().await?;
            anyhow::bail!("Request failed: {}", error)
        }
    }
}
