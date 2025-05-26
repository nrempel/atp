use async_trait::async_trait;
use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::{BASE_URL, Client, Config, Process};

#[derive(Parser)]
pub enum Server {
    /// Create a new session (login)
    CreateSession(CreateSession),
    /// Get current session info
    GetSession(GetSession),
    /// Refresh session tokens
    RefreshSession(RefreshSession),
    /// Delete session (logout)
    DeleteSession(DeleteSession),
    /// Describe server capabilities
    DescribeServer(DescribeServer),
}

#[derive(Parser)]
pub struct CreateSession {
    /// Account identifier (handle or email)
    #[arg(long)]
    pub identifier: String,
    /// Account password
    #[arg(long)]
    pub password: String,
}

#[derive(Parser)]
pub struct GetSession;

#[derive(Parser)]
pub struct RefreshSession;

#[derive(Parser)]
pub struct DeleteSession;

#[derive(Parser)]
pub struct DescribeServer;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSessionResponse {
    pub did: String,
    pub handle: String,
    pub email: Option<String>,
    #[serde(rename = "accessJwt")]
    pub access_jwt: String,
    #[serde(rename = "refreshJwt")]
    pub refresh_jwt: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetSessionResponse {
    pub did: String,
    pub handle: String,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshSessionResponse {
    #[serde(rename = "accessJwt")]
    pub access_jwt: String,
    #[serde(rename = "refreshJwt")]
    pub refresh_jwt: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DescribeServerResponse {
    #[serde(rename = "availableUserDomains")]
    pub available_user_domains: Vec<String>,
    #[serde(rename = "inviteCodeRequired")]
    pub invite_code_required: Option<bool>,
    pub links: Option<ServerLinks>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerLinks {
    #[serde(rename = "privacyPolicy")]
    pub privacy_policy: Option<String>,
    #[serde(rename = "termsOfService")]
    pub terms_of_service: Option<String>,
}

#[async_trait]
impl Process for Server {
    type Output = String;

    async fn process(&self, client: &Client, config: &Config) -> anyhow::Result<Self::Output> {
        match self {
            Server::CreateSession(cmd) => {
                let response = cmd.process(client, config).await?;
                Ok(format!(
                    "Session created for: {}\nDID: {}",
                    response.handle, response.did
                ))
            }
            Server::GetSession(cmd) => {
                let response = cmd.process(client, config).await?;
                Ok(format!(
                    "Handle: {}\nDID: {}",
                    response.handle, response.did
                ))
            }
            Server::RefreshSession(cmd) => {
                cmd.process(client, config).await?;
                Ok("Session refreshed successfully".to_string())
            }
            Server::DeleteSession(cmd) => {
                cmd.process(client, config).await?;
                Ok("Session deleted successfully".to_string())
            }
            Server::DescribeServer(cmd) => {
                let response = cmd.process(client, config).await?;
                Ok(format!(
                    "Available domains: {}\nInvite required: {}",
                    response.available_user_domains.join(", "),
                    response.invite_code_required.unwrap_or(false)
                ))
            }
        }
    }
}

#[async_trait]
impl Process for CreateSession {
    type Output = CreateSessionResponse;

    async fn process(&self, client: &Client, _config: &Config) -> anyhow::Result<Self::Output> {
        let url = format!("{}/com.atproto.server.createSession", BASE_URL);
        let body = serde_json::json!({
            "identifier": self.identifier,
            "password": self.password
        });

        let response = client.inner().post(&url).json(&body).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to create session: {}", response.status());
        }

        let response: CreateSessionResponse = response.json().await?;
        Ok(response)
    }
}

#[async_trait]
impl Process for GetSession {
    type Output = GetSessionResponse;

    async fn process(&self, client: &Client, config: &Config) -> anyhow::Result<Self::Output> {
        let session = config
            .session
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;

        let url = format!("{}/com.atproto.server.getSession", BASE_URL);
        let response = client
            .inner()
            .get(&url)
            .header("Authorization", format!("Bearer {}", session.access_jwt))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get session: {}", response.status());
        }

        let response: GetSessionResponse = response.json().await?;
        Ok(response)
    }
}

#[async_trait]
impl Process for RefreshSession {
    type Output = RefreshSessionResponse;

    async fn process(&self, client: &Client, config: &Config) -> anyhow::Result<Self::Output> {
        let session = config
            .session
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;

        let url = format!("{}/com.atproto.server.refreshSession", BASE_URL);
        let response = client
            .inner()
            .post(&url)
            .header("Authorization", format!("Bearer {}", session.refresh_jwt))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to refresh session: {}", response.status());
        }

        let response: RefreshSessionResponse = response.json().await?;
        Ok(response)
    }
}

#[async_trait]
impl Process for DeleteSession {
    type Output = ();

    async fn process(&self, client: &Client, config: &Config) -> anyhow::Result<Self::Output> {
        let session = config
            .session
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;

        let url = format!("{}/com.atproto.server.deleteSession", BASE_URL);
        let response = client
            .inner()
            .post(&url)
            .header("Authorization", format!("Bearer {}", session.refresh_jwt))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to delete session: {}", response.status());
        }

        Ok(())
    }
}

#[async_trait]
impl Process for DescribeServer {
    type Output = DescribeServerResponse;

    async fn process(&self, client: &Client, _config: &Config) -> anyhow::Result<Self::Output> {
        let url = format!("{}/com.atproto.server.describeServer", BASE_URL);
        let response = client.inner().get(&url).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to describe server: {}", response.status());
        }

        let response: DescribeServerResponse = response.json().await?;
        Ok(response)
    }
}
