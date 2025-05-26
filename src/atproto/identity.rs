use async_trait::async_trait;
use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::{BASE_URL, Client, Config, Process};

#[derive(Parser)]
pub enum Identity {
    /// Resolve a handle to a DID
    ResolveHandle(ResolveHandle),
    /// Resolve a DID to its DID document
    ResolveDid(ResolveDid),
    /// Update the handle for an account
    UpdateHandle(UpdateHandle),
}

#[derive(Parser)]
pub struct ResolveHandle {
    /// Handle to resolve (e.g., alice.bsky.social)
    #[arg(long)]
    pub handle: String,
}

#[derive(Parser)]
pub struct ResolveDid {
    /// DID to resolve (e.g., did:plc:...)
    #[arg(long)]
    pub did: String,
}

#[derive(Parser)]
pub struct UpdateHandle {
    /// New handle to set
    #[arg(long)]
    pub handle: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResolveHandleResponse {
    pub did: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResolveDidResponse {
    pub did: String,
    #[serde(rename = "didDoc")]
    pub did_doc: serde_json::Value,
}

impl Identity {
    pub fn needs_authentication(&self) -> bool {
        match self {
            Identity::ResolveHandle(_) => false, // Public endpoint
            Identity::ResolveDid(_) => true,     // Requires auth (returns 401)
            Identity::UpdateHandle(_) => true,   // Requires auth
        }
    }
}

#[async_trait]
impl Process for Identity {
    type Output = String;

    async fn process(&self, client: &Client, config: &Config) -> anyhow::Result<Self::Output> {
        match self {
            Identity::ResolveHandle(cmd) => {
                let response = cmd.process(client, config).await?;
                Ok(format!("DID: {}", response.did))
            }
            Identity::ResolveDid(cmd) => {
                let response = cmd.process(client, config).await?;
                Ok(format!(
                    "DID: {}\nDocument: {}",
                    response.did,
                    serde_json::to_string_pretty(&response.did_doc)?
                ))
            }
            Identity::UpdateHandle(cmd) => {
                cmd.process(client, config).await?;
                Ok("Handle updated successfully".to_string())
            }
        }
    }
}

#[async_trait]
impl Process for ResolveHandle {
    type Output = ResolveHandleResponse;

    async fn process(&self, client: &Client, _config: &Config) -> anyhow::Result<Self::Output> {
        let url = format!("{}/com.atproto.identity.resolveHandle", BASE_URL);
        let response = client
            .inner()
            .get(&url)
            .query(&[("handle", &self.handle)])
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to resolve handle: {}", response.status());
        }

        let response: ResolveHandleResponse = response.json().await?;
        Ok(response)
    }
}

#[async_trait]
impl Process for ResolveDid {
    type Output = ResolveDidResponse;

    async fn process(&self, client: &Client, config: &Config) -> anyhow::Result<Self::Output> {
        let session = config
            .session
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;

        let url = format!("{}/com.atproto.identity.resolveDid", BASE_URL);
        let response = client
            .inner()
            .get(&url)
            .query(&[("did", &self.did)])
            .header("Authorization", format!("Bearer {}", session.access_jwt))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to resolve DID: {}", response.status());
        }

        let response: ResolveDidResponse = response.json().await?;
        Ok(response)
    }
}

#[async_trait]
impl Process for UpdateHandle {
    type Output = ();

    async fn process(&self, client: &Client, config: &Config) -> anyhow::Result<Self::Output> {
        let session = config
            .session
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;

        let url = format!("{}/com.atproto.identity.updateHandle", BASE_URL);
        let body = serde_json::json!({
            "handle": self.handle
        });

        let response = client
            .inner()
            .post(&url)
            .header("Authorization", format!("Bearer {}", session.access_jwt))
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to update handle: {}", response.status());
        }

        Ok(())
    }
}
