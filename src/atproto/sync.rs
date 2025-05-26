use async_trait::async_trait;
use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::{BASE_URL, Client, Config, Process};

#[derive(Parser)]
pub enum Sync {
    /// Get a blob from the repository
    GetBlob(GetBlob),
    /// Get repository head
    GetHead(GetHead),
    /// Get latest commit
    GetLatestCommit(GetLatestCommit),
    /// Get repository status
    GetRepoStatus(GetRepoStatus),
    /// List repositories
    ListRepos(ListRepos),
}

#[derive(Parser)]
pub struct GetBlob {
    /// Repository DID
    #[arg(long)]
    pub did: String,
    /// Blob CID
    #[arg(long)]
    pub cid: String,
}

#[derive(Parser)]
pub struct GetHead {
    /// Repository DID
    #[arg(long)]
    pub did: String,
}

#[derive(Parser)]
pub struct GetLatestCommit {
    /// Repository DID
    #[arg(long)]
    pub did: String,
}

#[derive(Parser)]
pub struct GetRepoStatus {
    /// Repository DID
    #[arg(long)]
    pub did: String,
}

#[derive(Parser)]
pub struct ListRepos {
    /// Maximum number of repos to return
    #[arg(long, default_value = "500")]
    pub limit: u32,
    /// Cursor for pagination
    #[arg(long)]
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetHeadResponse {
    pub root: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetLatestCommitResponse {
    pub cid: String,
    pub rev: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetRepoStatusResponse {
    pub did: String,
    pub active: bool,
    pub status: Option<String>,
    pub rev: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListReposResponse {
    pub repos: Vec<RepoRef>,
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RepoRef {
    pub did: String,
    pub head: String,
    pub rev: String,
    pub active: Option<bool>,
    pub status: Option<String>,
}

#[async_trait]
impl Process for Sync {
    type Output = String;

    async fn process(&self, client: &Client, config: &Config) -> anyhow::Result<Self::Output> {
        match self {
            Sync::GetBlob(cmd) => {
                let _response = cmd.process(client, config).await?;
                Ok("Blob retrieved successfully".to_string())
            }
            Sync::GetHead(cmd) => {
                let response = cmd.process(client, config).await?;
                Ok(format!("Head: {}", response.root))
            }
            Sync::GetLatestCommit(cmd) => {
                let response = cmd.process(client, config).await?;
                Ok(format!(
                    "Latest commit: {}\nRev: {}",
                    response.cid, response.rev
                ))
            }
            Sync::GetRepoStatus(cmd) => {
                let response = cmd.process(client, config).await?;
                Ok(format!(
                    "DID: {}\nActive: {}\nStatus: {}",
                    response.did,
                    response.active,
                    response.status.unwrap_or_else(|| "None".to_string())
                ))
            }
            Sync::ListRepos(cmd) => {
                let response = cmd.process(client, config).await?;
                let mut output = format!("Found {} repositories:\n", response.repos.len());
                for repo in response.repos {
                    output.push_str(&format!("  {}: {} ({})\n", repo.did, repo.head, repo.rev));
                }
                if let Some(cursor) = response.cursor {
                    output.push_str(&format!("Cursor: {}\n", cursor));
                }
                Ok(output)
            }
        }
    }
}

#[async_trait]
impl Process for GetBlob {
    type Output = Vec<u8>;

    async fn process(&self, client: &Client, _config: &Config) -> anyhow::Result<Self::Output> {
        let url = format!("{}/com.atproto.sync.getBlob", BASE_URL);
        let response = client
            .inner()
            .get(&url)
            .query(&[("did", &self.did), ("cid", &self.cid)])
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get blob: {}", response.status());
        }

        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }
}

#[async_trait]
impl Process for GetHead {
    type Output = GetHeadResponse;

    async fn process(&self, client: &Client, _config: &Config) -> anyhow::Result<Self::Output> {
        let url = format!("{}/com.atproto.sync.getHead", BASE_URL);
        let response = client
            .inner()
            .get(&url)
            .query(&[("did", &self.did)])
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get head: {}", response.status());
        }

        let response: GetHeadResponse = response.json().await?;
        Ok(response)
    }
}

#[async_trait]
impl Process for GetLatestCommit {
    type Output = GetLatestCommitResponse;

    async fn process(&self, client: &Client, _config: &Config) -> anyhow::Result<Self::Output> {
        let url = format!("{}/com.atproto.sync.getLatestCommit", BASE_URL);
        let response = client
            .inner()
            .get(&url)
            .query(&[("did", &self.did)])
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get latest commit: {}", response.status());
        }

        let response: GetLatestCommitResponse = response.json().await?;
        Ok(response)
    }
}

#[async_trait]
impl Process for GetRepoStatus {
    type Output = GetRepoStatusResponse;

    async fn process(&self, client: &Client, _config: &Config) -> anyhow::Result<Self::Output> {
        let url = format!("{}/com.atproto.sync.getRepoStatus", BASE_URL);
        let response = client
            .inner()
            .get(&url)
            .query(&[("did", &self.did)])
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get repo status: {}", response.status());
        }

        let response: GetRepoStatusResponse = response.json().await?;
        Ok(response)
    }
}

#[async_trait]
impl Process for ListRepos {
    type Output = ListReposResponse;

    async fn process(&self, client: &Client, _config: &Config) -> anyhow::Result<Self::Output> {
        let url = format!("{}/com.atproto.sync.listRepos", BASE_URL);
        let limit_str = self.limit.to_string();
        let mut query = vec![("limit", limit_str.as_str())];

        if let Some(cursor) = &self.cursor {
            query.push(("cursor", cursor));
        }

        let response = client.inner().get(&url).query(&query).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to list repos: {}", response.status());
        }

        let response: ListReposResponse = response.json().await?;
        Ok(response)
    }
}
