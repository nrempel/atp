use async_trait::async_trait;
use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::{BASE_URL, Client, Config, Process};

#[derive(Parser)]
pub enum Repo {
    /// Create a new record in a repository
    CreateRecord(CreateRecord),
    /// Get a record from a repository
    GetRecord(GetRecord),
    /// List records in a collection
    ListRecords(ListRecords),
    /// Delete a record from a repository
    DeleteRecord(DeleteRecord),
    /// Upload a blob to the repository
    UploadBlob(UploadBlob),
    /// Describe a repository
    DescribeRepo(DescribeRepo),
}

#[derive(Parser)]
pub struct CreateRecord {
    /// Repository DID or handle
    #[arg(long)]
    pub repo: String,
    /// Collection name (e.g., app.bsky.feed.post)
    #[arg(long)]
    pub collection: String,
    /// Record data as JSON
    #[arg(long)]
    pub record: String,
    /// Optional record key
    #[arg(long)]
    pub rkey: Option<String>,
}

#[derive(Parser)]
pub struct GetRecord {
    /// Repository DID or handle
    #[arg(long)]
    pub repo: String,
    /// Collection name
    #[arg(long)]
    pub collection: String,
    /// Record key
    #[arg(long)]
    pub rkey: String,
}

#[derive(Parser)]
pub struct ListRecords {
    /// Repository DID or handle
    #[arg(long)]
    pub repo: String,
    /// Collection name
    #[arg(long)]
    pub collection: String,
    /// Maximum number of records to return
    #[arg(long, default_value = "50")]
    pub limit: u32,
    /// Cursor for pagination
    #[arg(long)]
    pub cursor: Option<String>,
}

#[derive(Parser)]
pub struct DeleteRecord {
    /// Repository DID or handle
    #[arg(long)]
    pub repo: String,
    /// Collection name
    #[arg(long)]
    pub collection: String,
    /// Record key
    #[arg(long)]
    pub rkey: String,
}

#[derive(Parser)]
pub struct UploadBlob {
    /// Path to file to upload
    #[arg(long)]
    pub file: String,
}

#[derive(Parser)]
pub struct DescribeRepo {
    /// Repository DID or handle
    #[arg(long)]
    pub repo: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRecordResponse {
    pub uri: String,
    pub cid: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetRecordResponse {
    pub uri: String,
    pub cid: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListRecordsResponse {
    pub records: Vec<RecordItem>,
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordItem {
    pub uri: String,
    pub cid: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UploadBlobResponse {
    pub blob: BlobRef,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlobRef {
    #[serde(rename = "$type")]
    pub type_: String,
    #[serde(rename = "ref")]
    pub ref_: serde_json::Value,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub size: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DescribeRepoResponse {
    pub handle: String,
    pub did: String,
    #[serde(rename = "didDoc")]
    pub did_doc: serde_json::Value,
    pub collections: Vec<String>,
    #[serde(rename = "handleIsCorrect")]
    pub handle_is_correct: bool,
}

impl Repo {
    pub fn needs_authentication(&self) -> bool {
        match self {
            Repo::CreateRecord(_) => true,  // Requires auth
            Repo::GetRecord(_) => false,    // Public endpoint
            Repo::ListRecords(_) => false,  // Public endpoint
            Repo::DeleteRecord(_) => true,  // Requires auth
            Repo::UploadBlob(_) => true,    // Requires auth
            Repo::DescribeRepo(_) => false, // Public endpoint
        }
    }
}

#[async_trait]
impl Process for Repo {
    type Output = String;

    async fn process(&self, client: &Client, config: &Config) -> anyhow::Result<Self::Output> {
        match self {
            Repo::CreateRecord(cmd) => {
                let response = cmd.process(client, config).await?;
                Ok(format!(
                    "Created record: {}\nCID: {}",
                    response.uri, response.cid
                ))
            }
            Repo::GetRecord(cmd) => {
                let response = cmd.process(client, config).await?;
                Ok(format!(
                    "URI: {}\nCID: {}\nValue: {}",
                    response.uri,
                    response.cid,
                    serde_json::to_string_pretty(&response.value)?
                ))
            }
            Repo::ListRecords(cmd) => {
                let response = cmd.process(client, config).await?;
                let mut output = format!("Found {} records:\n", response.records.len());
                for record in response.records {
                    output.push_str(&format!("  {}: {}\n", record.uri, record.cid));
                }
                if let Some(cursor) = response.cursor {
                    output.push_str(&format!("Cursor: {}\n", cursor));
                }
                Ok(output)
            }
            Repo::DeleteRecord(cmd) => {
                cmd.process(client, config).await?;
                Ok("Record deleted successfully".to_string())
            }
            Repo::UploadBlob(cmd) => {
                let response = cmd.process(client, config).await?;
                Ok(format!(
                    "Uploaded blob: {} bytes, type: {}",
                    response.blob.size, response.blob.mime_type
                ))
            }
            Repo::DescribeRepo(cmd) => {
                let response = cmd.process(client, config).await?;
                Ok(format!(
                    "Handle: {}\nDID: {}\nCollections: {}\nHandle correct: {}",
                    response.handle,
                    response.did,
                    response.collections.join(", "),
                    response.handle_is_correct
                ))
            }
        }
    }
}

#[async_trait]
impl Process for CreateRecord {
    type Output = CreateRecordResponse;

    async fn process(&self, client: &Client, config: &Config) -> anyhow::Result<Self::Output> {
        let session = config
            .session
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;

        let record: serde_json::Value = serde_json::from_str(&self.record)?;

        let mut body = serde_json::json!({
            "repo": self.repo,
            "collection": self.collection,
            "record": record
        });

        if let Some(rkey) = &self.rkey {
            body["rkey"] = serde_json::Value::String(rkey.clone());
        }

        let url = format!("{}/com.atproto.repo.createRecord", BASE_URL);
        let response = client
            .inner()
            .post(&url)
            .header("Authorization", format!("Bearer {}", session.access_jwt))
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to create record: {}", response.status());
        }

        let response: CreateRecordResponse = response.json().await?;
        Ok(response)
    }
}

#[async_trait]
impl Process for GetRecord {
    type Output = GetRecordResponse;

    async fn process(&self, client: &Client, _config: &Config) -> anyhow::Result<Self::Output> {
        let url = format!("{}/com.atproto.repo.getRecord", BASE_URL);
        let response = client
            .inner()
            .get(&url)
            .query(&[
                ("repo", &self.repo),
                ("collection", &self.collection),
                ("rkey", &self.rkey),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get record: {}", response.status());
        }

        let response: GetRecordResponse = response.json().await?;
        Ok(response)
    }
}

#[async_trait]
impl Process for ListRecords {
    type Output = ListRecordsResponse;

    async fn process(&self, client: &Client, _config: &Config) -> anyhow::Result<Self::Output> {
        let url = format!("{}/com.atproto.repo.listRecords", BASE_URL);
        let limit_str = self.limit.to_string();
        let mut query = vec![
            ("repo", self.repo.as_str()),
            ("collection", self.collection.as_str()),
            ("limit", limit_str.as_str()),
        ];

        if let Some(cursor) = &self.cursor {
            query.push(("cursor", cursor));
        }

        let response = client.inner().get(&url).query(&query).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to list records: {}", response.status());
        }

        let response: ListRecordsResponse = response.json().await?;
        Ok(response)
    }
}

#[async_trait]
impl Process for DeleteRecord {
    type Output = ();

    async fn process(&self, client: &Client, config: &Config) -> anyhow::Result<Self::Output> {
        let session = config
            .session
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;

        let body = serde_json::json!({
            "repo": self.repo,
            "collection": self.collection,
            "rkey": self.rkey
        });

        let url = format!("{}/com.atproto.repo.deleteRecord", BASE_URL);
        let response = client
            .inner()
            .post(&url)
            .header("Authorization", format!("Bearer {}", session.access_jwt))
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to delete record: {}", response.status());
        }

        Ok(())
    }
}

#[async_trait]
impl Process for UploadBlob {
    type Output = UploadBlobResponse;

    async fn process(&self, client: &Client, config: &Config) -> anyhow::Result<Self::Output> {
        let session = config
            .session
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;

        let file_data = tokio::fs::read(&self.file).await?;

        // Try to determine MIME type from file extension
        let mime_type = match std::path::Path::new(&self.file)
            .extension()
            .and_then(|ext| ext.to_str())
        {
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("png") => "image/png",
            Some("gif") => "image/gif",
            Some("webp") => "image/webp",
            Some("txt") => "text/plain",
            Some("json") => "application/json",
            _ => "application/octet-stream",
        };

        let url = format!("{}/com.atproto.repo.uploadBlob", BASE_URL);
        let response = client
            .inner()
            .post(&url)
            .header("Authorization", format!("Bearer {}", session.access_jwt))
            .header("Content-Type", mime_type)
            .body(file_data)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to upload blob: {}", response.status());
        }

        let response: UploadBlobResponse = response.json().await?;
        Ok(response)
    }
}

#[async_trait]
impl Process for DescribeRepo {
    type Output = DescribeRepoResponse;

    async fn process(&self, client: &Client, _config: &Config) -> anyhow::Result<Self::Output> {
        let url = format!("{}/com.atproto.repo.describeRepo", BASE_URL);
        let response = client
            .inner()
            .get(&url)
            .query(&[("repo", &self.repo)])
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to describe repo: {}", response.status());
        }

        let response: DescribeRepoResponse = response.json().await?;
        Ok(response)
    }
}
