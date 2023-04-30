use std::fmt::Display;

use anyhow::Ok;
use clap::Parser;
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
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
}

#[derive(Parser, Serialize)]
pub struct Login {
    #[clap(short, long)]
    identifier: String,
    #[clap(short, long)]
    password: String,
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

impl Login {
    pub async fn process(&self, client: &Client) -> anyhow::Result<LoginResponse> {
        let url = format!("{BASE_URL}/com.atproto.server.createSession");
        let res = client.inner().post(url).json(self).send().await?;
        Ok(res.json().await?)
    }
}

#[derive(Parser)]
struct Session;

const BASE_URL: &str = "https://bsky.social/xrpc";
