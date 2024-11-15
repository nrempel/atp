use async_trait::async_trait;
use clap::Parser;
use serde::Deserialize;

use crate::{auth::make_authenticated_request, format, Client, Config, Process};

impl Profile {
    pub(super) async fn process(
        &self,
        client: &Client,
        config: &Config,
    ) -> anyhow::Result<ProfileResponse> {
        let actor = self.actor.trim_start_matches('@');
        make_authenticated_request(
            client,
            config,
            "getProfile",
            &[("actor", actor.to_string())],
        )
        .await
    }
}

impl Preferences {
    pub(super) async fn process(
        &self,
        client: &Client,
        config: &Config,
    ) -> anyhow::Result<PreferencesResponse> {
        make_authenticated_request(client, config, "getPreferences", &[]).await
    }
}

impl Profiles {
    pub(super) async fn process(
        &self,
        client: &Client,
        config: &Config,
    ) -> anyhow::Result<ProfilesResponse> {
        let actors: Vec<_> = self
            .actors
            .iter()
            .map(|a| a.trim_start_matches('@'))
            .collect();

        let query: Vec<(&str, String)> = actors
            .iter()
            .map(|actor| ("actors", actor.to_string()))
            .collect();

        make_authenticated_request(client, config, "getProfiles", &query).await
    }
}

impl Suggestions {
    pub(super) async fn process(
        &self,
        client: &Client,
        config: &Config,
    ) -> anyhow::Result<SuggestionsResponse> {
        let mut query = vec![("limit", self.limit.to_string())];
        if let Some(cursor) = &self.cursor {
            query.push(("cursor", cursor.to_string()));
        }

        make_authenticated_request(client, config, "getSuggestions", &query).await
    }
}

impl SearchActors {
    pub(super) async fn process(
        &self,
        client: &Client,
        config: &Config,
    ) -> anyhow::Result<SearchActorsResponse> {
        let mut query = vec![("q", self.query.clone()), ("limit", self.limit.to_string())];
        if let Some(cursor) = &self.cursor {
            query.push(("cursor", cursor.to_string()));
        }

        make_authenticated_request(client, config, "searchActors", &query).await
    }
}

#[derive(Parser)]
pub enum Bsky {
    #[command(subcommand)]
    Actor(Actor),
}

#[derive(Parser)]
pub enum Actor {
    Profile(Profile),
    Profiles(Profiles),
    Preferences(Preferences),
    Suggestions(Suggestions),
    Search(SearchActors),
}

#[derive(Parser)]
pub struct Profile {
    #[clap(short, long)]
    actor: String,
}

#[derive(Parser)]
pub struct Preferences {}

#[derive(Parser)]
pub struct Profiles {
    #[clap(short, long)]
    #[clap(value_delimiter = ',')]
    actors: Vec<String>,
}

#[derive(Parser)]
pub struct Suggestions {
    #[clap(short, long)]
    #[clap(default_value = "50")]
    limit: u8,
    #[clap(short, long)]
    cursor: Option<String>,
}

#[derive(Parser)]
pub struct SearchActors {
    #[clap(short, long)]
    query: String,
    #[clap(short, long)]
    #[clap(default_value = "25")]
    limit: u8,
    #[clap(short, long)]
    cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ProfileResponse {
    #[allow(dead_code)]
    pub(super) did: String,
    pub(super) handle: String,
    pub(super) display_name: Option<String>,
    pub(super) description: Option<String>,
    pub(super) avatar: Option<String>,
    pub(super) banner: Option<String>,
    pub(super) followers_count: Option<i64>,
    pub(super) follows_count: Option<i64>,
    pub(super) posts_count: Option<i64>,
    pub(super) indexed_at: String,
    pub(super) viewer: Option<ViewerState>,
    pub(super) labels: Option<Vec<Label>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PreferencesResponse {
    pub(super) preferences: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ViewerState {
    pub(super) muted: Option<bool>,
    pub(super) blocked_by: Option<bool>,
    pub(super) blocking: Option<bool>,
    pub(super) following: Option<bool>,
    pub(super) followed_by: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Label {
    pub(super) val: String,
    #[allow(dead_code)]
    pub(super) src: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ProfilesResponse {
    pub(super) profiles: Vec<ProfileResponse>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct SuggestionsResponse {
    pub(super) actors: Vec<ProfileResponse>,
    pub(super) cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct SearchActorsResponse {
    pub(super) actors: Vec<ProfileResponse>,
    pub(super) cursor: Option<String>,
}

#[async_trait]
impl Process for Bsky {
    type Output = String;

    async fn process(&self, client: &Client, config: &Config) -> anyhow::Result<Self::Output> {
        match self {
            Bsky::Actor(Actor::Profile(cmd)) => {
                let response = cmd.process(client, config).await?;
                Ok(format::format_profile(&response).await)
            }
            Bsky::Actor(Actor::Profiles(cmd)) => {
                let response = cmd.process(client, config).await?;
                Ok(format::format_profiles(&response).await)
            }
            Bsky::Actor(Actor::Preferences(cmd)) => {
                let response = cmd.process(client, config).await?;
                Ok(format::format_preferences(&response).await)
            }
            Bsky::Actor(Actor::Suggestions(cmd)) => {
                let response = cmd.process(client, config).await?;
                Ok(format::format_suggestions(&response).await)
            }
            Bsky::Actor(Actor::Search(cmd)) => {
                let response = cmd.process(client, config).await?;
                Ok(format::format_search_actors(&response).await)
            }
        }
    }
}
