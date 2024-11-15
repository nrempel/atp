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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub(crate) did: String,
    pub(crate) handle: String,
    pub(crate) email: Option<String>,
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

    // First attempt with current access token
    let res = client
        .inner()
        .get(url.clone())
        .header("Authorization", format!("Bearer {}", session.access_jwt))
        .query(query)
        .send()
        .await?;

    if res.status().is_success() {
        return Ok(res.json().await?);
    }

    // Check if token is expired
    let error_text = res.text().await?;
    if error_text.contains("ExpiredToken") {
        // Try to refresh the token
        let new_session = refresh_session(client, &session.refresh_jwt).await?;

        // Update the config with the new session
        let mut new_config = config.clone();
        new_config.session = Some(new_session);
        new_config
            .write(&directories::BaseDirs::new().unwrap())
            .await?;

        // Retry the request with the new token

        let retry_res = client
            .inner()
            .get(url)
            .header(
                "Authorization",
                format!("Bearer {}", new_config.session.as_ref().unwrap().access_jwt),
            )
            .query(query)
            .send()
            .await?;

        if retry_res.status().is_success() {
            return Ok(retry_res.json().await?);
        }

        let retry_error = retry_res.text().await?;
        anyhow::bail!("Request failed after token refresh: {}", retry_error);
    }

    anyhow::bail!("Request failed: {}", error_text)
}

pub(super) async fn refresh_session(
    client: &Client,
    refresh_jwt: &str,
) -> anyhow::Result<LoginResponse> {
    let url = format!("{BASE_URL}/com.atproto.server.refreshSession");
    let res = client
        .inner()
        .post(url)
        .header("Authorization", format!("Bearer {}", refresh_jwt))
        .send()
        .await?;

    if !res.status().is_success() {
        let error = res.text().await?;

        anyhow::bail!("Session refresh failed: {}", error);
    }

    Ok(res.json().await?)
}
