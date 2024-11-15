use crate::{Client, Config};
use clap::Parser;
use serde::Deserialize;
use std::fmt::Display;

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
pub struct ProfileResponse {
    did: String,
    handle: String,
    display_name: Option<String>,
    description: Option<String>,
    avatar: Option<String>,
    banner: Option<String>,
    followers_count: Option<i64>,
    follows_count: Option<i64>,
    posts_count: Option<i64>,
    indexed_at: String,
    viewer: Option<ViewerState>,
    labels: Option<Vec<Label>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreferencesResponse {
    #[serde(rename = "preferences")]
    preferences: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewerState {
    muted: Option<bool>,
    blocked_by: Option<bool>,
    blocking: Option<bool>,
    following: Option<String>,
    followed_by: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    val: String,
    src: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfilesResponse {
    profiles: Vec<ProfileResponse>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuggestionsResponse {
    actors: Vec<ProfileResponse>,
    cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchActorsResponse {
    actors: Vec<ProfileResponse>,
    cursor: Option<String>,
}

impl Profile {
    pub async fn process(
        &self,
        client: &Client,
        config: &Config,
    ) -> anyhow::Result<ProfileResponse> {
        let actor = self.actor.trim_start_matches('@');
        let url = format!("{}/app.bsky.actor.getProfile", super::BASE_URL);

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

impl Preferences {
    pub async fn process(
        &self,
        client: &Client,
        config: &Config,
    ) -> anyhow::Result<PreferencesResponse> {
        let url = format!("{}/app.bsky.actor.getPreferences", super::BASE_URL);

        let session = config
            .session
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;

        let res = client
            .inner()
            .get(url)
            .header("Authorization", format!("Bearer {}", session.access_jwt))
            .send()
            .await?;

        match res.status() {
            reqwest::StatusCode::OK => Ok(res.json().await?),
            reqwest::StatusCode::UNAUTHORIZED => anyhow::bail!("Authentication required"),
            _ => {
                let error = res.text().await?;
                anyhow::bail!("Failed to get preferences: {}", error)
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
        if let Some(avatar) = &self.avatar {
            writeln!(f, "Avatar: {}", avatar)?;
        }
        if let Some(banner) = &self.banner {
            writeln!(f, "Banner: {}", banner)?;
        }
        if let Some(followers) = &self.followers_count {
            writeln!(f, "Followers: {}", followers)?;
        }
        if let Some(follows) = &self.follows_count {
            writeln!(f, "Following: {}", follows)?;
        }
        if let Some(posts) = &self.posts_count {
            writeln!(f, "Posts: {}", posts)?;
        }
        if let Some(viewer) = &self.viewer {
            writeln!(f, "\nViewer State:")?;
            if let Some(muted) = viewer.muted {
                writeln!(f, "  Muted: {}", muted)?;
            }
            if let Some(blocked_by) = viewer.blocked_by {
                writeln!(f, "  Blocked by: {}", blocked_by)?;
            }
            if let Some(blocking) = viewer.blocking {
                writeln!(f, "  Blocking: {}", blocking)?;
            }
            if let Some(following) = &viewer.following {
                writeln!(f, "  Following: {}", following)?;
            }
            if let Some(followed_by) = &viewer.followed_by {
                writeln!(f, "  Followed by: {}", followed_by)?;
            }
        }
        if let Some(labels) = &self.labels {
            writeln!(f, "\nLabels:")?;
            for label in labels {
                writeln!(f, "  {} (from: {})", label.val, label.src)?;
            }
        }
        write!(f, "Indexed At: {}", self.indexed_at)
    }
}

impl Display for PreferencesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            serde_json::to_string_pretty(&self.preferences).unwrap()
        )
    }
}

impl Display for ProfilesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, profile) in self.profiles.iter().enumerate() {
            if i > 0 {
                writeln!(f, "\n---\n")?;
            }
            write!(f, "{}", profile)?;
        }
        Ok(())
    }
}

impl Display for SuggestionsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, profile) in self.actors.iter().enumerate() {
            if i > 0 {
                writeln!(f, "\n---\n")?;
            }
            write!(f, "{}", profile)?;
        }
        if let Some(cursor) = &self.cursor {
            writeln!(f, "\n\nNext cursor: {}", cursor)?;
        }
        Ok(())
    }
}

impl Display for SearchActorsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, profile) in self.actors.iter().enumerate() {
            if i > 0 {
                writeln!(f, "\n---\n")?;
            }
            write!(f, "{}", profile)?;
        }
        if let Some(cursor) = &self.cursor {
            writeln!(f, "\n\nNext cursor: {}", cursor)?;
        }
        Ok(())
    }
}

impl Profiles {
    pub async fn process(
        &self,
        client: &Client,
        config: &Config,
    ) -> anyhow::Result<ProfilesResponse> {
        let actors: Vec<_> = self
            .actors
            .iter()
            .map(|a| a.trim_start_matches('@'))
            .collect();

        let url = format!("{}/app.bsky.actor.getProfiles", super::BASE_URL);

        let session = config
            .session
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;

        let query: Vec<(&str, String)> = actors
            .iter()
            .map(|actor| ("actors", actor.to_string()))
            .collect();

        let res = client
            .inner()
            .get(url)
            .header("Authorization", format!("Bearer {}", session.access_jwt))
            .query(&query)
            .send()
            .await?;

        match res.status() {
            reqwest::StatusCode::OK => Ok(res.json().await?),
            reqwest::StatusCode::UNAUTHORIZED => anyhow::bail!("Authentication required"),
            _ => {
                let error = res.text().await?;
                anyhow::bail!("Profiles lookup failed: {}", error)
            }
        }
    }
}

impl Suggestions {
    pub async fn process(
        &self,
        client: &Client,
        config: &Config,
    ) -> anyhow::Result<SuggestionsResponse> {
        let url = format!("{}/app.bsky.actor.getSuggestions", super::BASE_URL);

        let session = config
            .session
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;

        let mut query = vec![("limit", self.limit.to_string())];
        if let Some(cursor) = &self.cursor {
            query.push(("cursor", cursor.to_string()));
        }

        let res = client
            .inner()
            .get(url)
            .header("Authorization", format!("Bearer {}", session.access_jwt))
            .query(&query)
            .send()
            .await?;

        match res.status() {
            reqwest::StatusCode::OK => Ok(res.json().await?),
            reqwest::StatusCode::UNAUTHORIZED => anyhow::bail!("Authentication required"),
            _ => {
                let error = res.text().await?;
                anyhow::bail!("Failed to get suggestions: {}", error)
            }
        }
    }
}

impl SearchActors {
    pub async fn process(
        &self,
        client: &Client,
        config: &Config,
    ) -> anyhow::Result<SearchActorsResponse> {
        let url = format!("{}/app.bsky.actor.searchActors", super::BASE_URL);

        let session = config
            .session
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;

        let mut query = vec![("q", self.query.clone()), ("limit", self.limit.to_string())];

        if let Some(cursor) = &self.cursor {
            query.push(("cursor", cursor.to_string()));
        }

        let res = client
            .inner()
            .get(url)
            .header("Authorization", format!("Bearer {}", session.access_jwt))
            .query(&query)
            .send()
            .await?;

        match res.status() {
            reqwest::StatusCode::OK => Ok(res.json().await?),
            reqwest::StatusCode::UNAUTHORIZED => anyhow::bail!("Authentication required"),
            _ => {
                let error = res.text().await?;
                anyhow::bail!("Search failed: {}", error)
            }
        }
    }
}
