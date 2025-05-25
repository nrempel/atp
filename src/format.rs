use chrono::DateTime;
use colored::Colorize;
use image::load_from_memory;
use serde::Serialize;
use std::io::Write;
use textwrap::fill;
use viuer::Config as ViuerConfig;

use crate::bsky::actor::{
    Label, PreferencesResponse, ProfileResponse, ProfilesResponse, SearchActorsResponse,
    SuggestionsResponse, ViewerState,
};

pub(super) async fn format_profile(profile: &ProfileResponse) -> String {
    let mut output = String::new();
    output.push_str("\n\n");

    // Try to display banner if available
    if let Some(banner_url) = &profile.banner {
        if let Ok(image_data) = download_image(banner_url).await {
            if let Ok(image) = load_from_memory(&image_data) {
                let conf = ViuerConfig {
                    width: Some(80),
                    height: Some(12),
                    ..Default::default()
                };
                let _ = viuer::print(&image, &conf);
                std::io::stdout().flush().unwrap();
            }
        }
    }

    // Try to display avatar if available
    if let Some(avatar_url) = &profile.avatar {
        if let Ok(image_data) = download_image(avatar_url).await {
            if let Ok(image) = load_from_memory(&image_data) {
                let height = 12; // Fixed height
                let width = height * 2; // Double the width to account for terminal character aspect ratio
                let conf = ViuerConfig {
                    width: Some(width),
                    height: Some(height),
                    absolute_offset: true,
                    x: 5,
                    y: (height / 2) as i16,
                    ..Default::default()
                };
                let _ = viuer::print(&image, &conf);
            }
        }
    }

    // Display name and handle section
    if let Some(name) = &profile.display_name {
        output.push_str(&format!("{}\n", name.bold()));
    }
    output.push_str(&format!("@{}\n\n", profile.handle));

    // Bio/Description with text wrapping
    if let Some(desc) = &profile.description {
        output.push_str(&format!("{}\n\n", fill(desc, 70)));
    }

    // Stats in a single line
    let stats = format_stats(
        profile.followers_count,
        profile.follows_count,
        profile.posts_count,
    );
    output.push_str(&format!("{}\n", stats));

    // Viewer state as badges
    if let Some(viewer) = &profile.viewer {
        let viewer_state = format_viewer_state(viewer);
        if !viewer_state.is_empty() {
            output.push_str(&format!("\n{}\n", viewer_state));
        }
    }

    // Labels as tags
    if let Some(labels) = &profile.labels {
        let label_text = format_labels(labels);
        if !label_text.is_empty() {
            output.push_str(&format!("\n{}\n", label_text));
        }
    }

    // Small metadata footer
    output.push_str(&format!(
        "\n{}\n\n",
        format!(
            "Joined {}",
            DateTime::parse_from_rfc3339(&profile.indexed_at)
                .map(|dt| dt.format("%B %d, %Y").to_string())
                .unwrap_or_else(|_| profile.indexed_at.clone())
        )
        .dimmed()
    ));

    output
}

fn format_stats(followers: Option<i64>, follows: Option<i64>, posts: Option<i64>) -> String {
    let mut stats = Vec::new();

    if let Some(followers) = followers {
        stats.push(format!("{} Followers", followers));
    }
    if let Some(follows) = follows {
        stats.push(format!("{} Following", follows));
    }
    if let Some(posts) = posts {
        stats.push(format!("{} Posts", posts));
    }

    stats.join(" · ")
}

fn format_viewer_state(viewer: &ViewerState) -> String {
    let mut badges = Vec::new();

    if let Some(true) = viewer.following {
        badges.push("Following".green());
    }
    if let Some(true) = viewer.followed_by {
        badges.push("Follows you".blue());
    }
    if let Some(true) = viewer.muted {
        badges.push("Muted".yellow());
    }
    if let Some(true) = viewer.blocking {
        badges.push("Blocked".red());
    }
    if let Some(true) = viewer.blocked_by {
        badges.push("Has blocked you".red());
    }

    badges
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn format_labels(labels: &[Label]) -> String {
    labels
        .iter()
        .map(|label| format!("#{}", label.val.replace(' ', "_")))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn format_json<T: Serialize>(value: &T) -> String {
    serde_json::to_string_pretty(value).unwrap()
}

async fn download_image(url: &str) -> anyhow::Result<Vec<u8>> {
    let response = reqwest::get(url).await?;
    Ok(response.bytes().await?.to_vec())
}

pub(super) async fn format_profiles(response: &ProfilesResponse) -> String {
    let mut output = String::new();
    for (i, profile) in response.profiles.iter().enumerate() {
        if i > 0 {
            output.push_str("\n---\n");
        }
        output.push_str(&format_profile(profile).await);
    }
    output
}

pub(super) async fn format_preferences(response: &PreferencesResponse) -> String {
    serde_json::to_string_pretty(&response.preferences)
        .unwrap_or_else(|_| "Failed to format preferences".to_string())
}

pub(super) async fn format_suggestions(response: &SuggestionsResponse) -> String {
    let mut output = String::new();
    for (i, profile) in response.actors.iter().enumerate() {
        if i > 0 {
            output.push_str("\n---\n");
        }
        output.push_str(&format_profile(profile).await);
    }
    if let Some(cursor) = &response.cursor {
        output.push_str(&format!("\n\nNext cursor: {}", cursor));
    }
    output
}

pub(super) async fn format_search_actors(response: &SearchActorsResponse) -> String {
    let mut output = String::new();
    for (i, profile) in response.actors.iter().enumerate() {
        if i > 0 {
            output.push_str("\n---\n");
        }
        output.push_str(&format_profile(profile).await);
    }
    if let Some(cursor) = &response.cursor {
        output.push_str(&format!("\n\nNext cursor: {}", cursor));
    }
    output
}
