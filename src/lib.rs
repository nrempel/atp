pub mod auth;
pub mod bsky;

use std::fmt::Display;

use anyhow::Ok;
use async_trait::async_trait;
use clap::Parser;
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use tokio::fs::read_to_string;

use crate::auth::LoginResponse;

const BASE_URL: &str = "https://bsky.social/xrpc";

#[derive(Default)]
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

#[derive(Debug, Default, Deserialize, Serialize)]
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
    Profile(bsky::Profile),
    Profiles(bsky::Profiles),
    Preferences(bsky::Preferences),
    Suggestions(bsky::Suggestions),
    SearchActors(bsky::SearchActors),
}

#[async_trait]
pub trait Process {
    type Output: std::fmt::Display;
    async fn process(&self, client: &Client, config: &Config) -> anyhow::Result<Self::Output>;
}

#[async_trait]
impl Process for Server {
    type Output = Box<dyn std::fmt::Display>;

    async fn process(&self, client: &Client, config: &Config) -> anyhow::Result<Self::Output> {
        match self {
            Server::Profile(cmd) => Ok(Box::new(cmd.process(client, config).await?)),
            Server::Profiles(cmd) => Ok(Box::new(cmd.process(client, config).await?)),
            Server::Preferences(cmd) => Ok(Box::new(cmd.process(client, config).await?)),
            Server::Suggestions(cmd) => Ok(Box::new(cmd.process(client, config).await?)),
            Server::SearchActors(cmd) => Ok(Box::new(cmd.process(client, config).await?)),
        }
    }
}
