use std::fmt::Display;

use anyhow::Ok;
use clap::Parser;
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::fs::read_to_string;

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub fn inner(&self) -> &reqwest::Client {
        &self.client
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub session: Option<LoginResponse>,
}

impl Config {
    pub async fn write(&self, base_dirs: &BaseDirs) -> anyhow::Result<()> {
        let dest = base_dirs.config_local_dir().join("atp");
        tokio::fs::create_dir_all(&dest).await?;

        let file = dest.join("config.toml");
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file)
            .await?;

        let toml = toml::to_string(&self)?;
        tokio::io::copy(&mut toml.as_bytes(), &mut file).await?;
        Ok(())
    }

    pub async fn load(base_dirs: &BaseDirs) -> anyhow::Result<Self> {
        let file = base_dirs.config_local_dir().join("atp").join("config.toml");
        let file = read_to_string(file).await?;
        Ok(toml::from_str(&file)?)
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(session) = &self.session {
            writeln!(f, "did: {}", session.did)?;
            writeln!(f, "handle: {}", session.handle)?;
            write!(f, "email: {}", session.email)
        } else {
            write!(f, "No session")
        }
    }
}

#[derive(Parser)]
pub enum Server {
    Login(Login),
    Profile(Profile),
}

#[derive(Parser, Serialize)]
pub struct Login {
    #[clap(short, long)]
    identifier: String,
    #[clap(short, long)]
    password: String,
}

#[derive(Parser)]
pub struct Profile {
    #[clap(short, long)]
    actor: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    did: String,
    handle: String,
    email: String,
    access_jwt: String,
    refresh_jwt: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResponse {
    did: String,
    handle: String,
    display_name: Option<String>,
    description: Option<String>,
    avatar: Option<String>,
    indexed_at: String,
}

impl Login {
    pub async fn process(&self, client: &Client) -> anyhow::Result<LoginResponse> {
        let identifier = self.identifier.trim_start_matches('@');
        let url = format!("{BASE_URL}/com.atproto.server.createSession");
        let res = client
            .inner()
            .post(url)
            .json(&json!({
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

impl Profile {
    pub async fn process(
        &self,
        client: &Client,
        config: &Config,
    ) -> anyhow::Result<ProfileResponse> {
        let actor = self.actor.trim_start_matches('@');
        let url = format!("{BASE_URL}/app.bsky.actor.getProfile");

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

#[derive(Parser)]
struct Session;

const BASE_URL: &str = "https://bsky.social/xrpc";
