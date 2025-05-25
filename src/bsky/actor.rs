use async_trait::async_trait;
use clap::Parser;
use serde::Deserialize;

use crate::{auth::make_authenticated_request, format, Client, Config, Process};

impl Profile {
    pub(crate) async fn process(
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
    pub(crate) async fn process(
        &self,
        client: &Client,
        config: &Config,
    ) -> anyhow::Result<PreferencesResponse> {
        make_authenticated_request(client, config, "getPreferences", &[]).await
    }
}

impl Profiles {
    pub(crate) async fn process(
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
    pub(crate) async fn process(
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
    pub(crate) async fn process(
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
pub(crate) struct ProfileResponse {
    #[allow(dead_code)]
    pub(crate) did: String,
    pub(crate) handle: String,
    pub(crate) display_name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) avatar: Option<String>,
    pub(crate) banner: Option<String>,
    pub(crate) followers_count: Option<i64>,
    pub(crate) follows_count: Option<i64>,
    pub(crate) posts_count: Option<i64>,
    pub(crate) indexed_at: String,
    pub(crate) viewer: Option<ViewerState>,
    pub(crate) labels: Option<Vec<Label>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PreferencesResponse {
    pub(crate) preferences: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ViewerState {
    pub(crate) muted: Option<bool>,
    pub(crate) blocked_by: Option<bool>,
    pub(crate) blocking: Option<bool>,
    pub(crate) following: Option<bool>,
    pub(crate) followed_by: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Label {
    pub(crate) val: String,
    #[allow(dead_code)]
    pub(crate) src: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProfilesResponse {
    pub(crate) profiles: Vec<ProfileResponse>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SuggestionsResponse {
    pub(crate) actors: Vec<ProfileResponse>,
    pub(crate) cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SearchActorsResponse {
    pub(crate) actors: Vec<ProfileResponse>,
    pub(crate) cursor: Option<String>,
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
