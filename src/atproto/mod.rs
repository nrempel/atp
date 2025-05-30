pub mod identity;
pub mod repo;
pub mod server;
pub mod sync;

use async_trait::async_trait;
use clap::Parser;

use crate::{Client, Config, Process};

#[derive(Parser)]
pub enum Atproto {
    #[command(subcommand)]
    Identity(identity::Identity),
    #[command(subcommand)]
    Repo(repo::Repo),
    #[command(subcommand)]
    Server(server::Server),
    #[command(subcommand)]
    Sync(sync::Sync),
}

impl Atproto {
    pub fn needs_authentication(&self) -> bool {
        match self {
            Atproto::Identity(cmd) => cmd.needs_authentication(),
            Atproto::Repo(cmd) => cmd.needs_authentication(),
            Atproto::Server(cmd) => cmd.needs_authentication(),
            Atproto::Sync(cmd) => cmd.needs_authentication(),
        }
    }
}

#[async_trait]
impl Process for Atproto {
    type Output = String;

    async fn process(&self, client: &Client, config: &Config) -> anyhow::Result<Self::Output> {
        match self {
            Atproto::Identity(cmd) => cmd.process(client, config).await,
            Atproto::Repo(cmd) => cmd.process(client, config).await,
            Atproto::Server(cmd) => cmd.process(client, config).await,
            Atproto::Sync(cmd) => cmd.process(client, config).await,
        }
    }
}
