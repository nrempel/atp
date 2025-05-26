mod common;

use common::atp_command;

// =============================================================================
// BLUESKY TESTS - app.bsky.*
// =============================================================================

#[test]
fn test_bsky_actor_profile_success() {
    let output = atp_command()
        .args(&["bsky", "actor", "profile", "--actor", "bsky.app"])
        .output()
        .expect("Failed to execute profile");

    assert!(output.status.success(), "Command should succeed");
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should return profile information
    assert!(stdout.contains("Handle:"), "Should show handle");
    assert!(stdout.contains("Display name:"), "Should show display name");
    assert!(stdout.contains("DID:"), "Should show DID");
    assert!(stdout.contains("Followers:"), "Should show follower count");
    assert!(stdout.contains("Following:"), "Should show following count");
}

#[test]
fn test_bsky_actor_profile_missing_actor() {
    let output = atp_command()
        .args(&["bsky", "actor", "profile"])
        .output()
        .expect("Failed to execute profile");

    assert!(
        !output.status.success(),
        "Command should fail without actor"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("actor") || stderr.contains("required"),
        "Should show missing actor error"
    );
}

#[test]
fn test_bsky_actor_search_success() {
    let output = atp_command()
        .args(&["bsky", "actor", "search", "--query", "bsky", "--limit", "3"])
        .output()
        .expect("Failed to execute search");

    assert!(output.status.success(), "Command should succeed");
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should return search results
    assert!(stdout.contains("Found"), "Should show found count");
    assert!(stdout.contains("actors:"), "Should show actors label");
    assert!(stdout.contains("Handle:"), "Should show actor handles");
}

#[test]
fn test_bsky_actor_search_missing_query() {
    let output = atp_command()
        .args(&["bsky", "actor", "search"])
        .output()
        .expect("Failed to execute search");

    assert!(
        !output.status.success(),
        "Command should fail without query"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("query") || stderr.contains("required"),
        "Should show missing query error"
    );
}

#[test]
fn test_bsky_actor_suggestions_requires_auth() {
    let output = atp_command()
        .args(&["bsky", "actor", "suggestions", "--limit", "5"])
        .output()
        .expect("Failed to execute suggestions");

    // With authentication, should succeed
    assert!(
        output.status.success(),
        "Should succeed with authentication"
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Found"), "Should show found count");
    assert!(
        stdout.contains("suggestions:"),
        "Should show suggestions label"
    );
}

#[test]
fn test_bsky_actor_profiles_success() {
    let output = atp_command()
        .args(&[
            "bsky",
            "actor",
            "profiles",
            "--actors",
            "bsky.app,jay.bsky.social",
        ])
        .output()
        .expect("Failed to execute profiles");

    assert!(output.status.success(), "Command should succeed");
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should return multiple profiles
    assert!(stdout.contains("Found"), "Should show found count");
    assert!(stdout.contains("profiles:"), "Should show profiles label");
    assert!(stdout.contains("Handle:"), "Should show handles");
}

#[test]
fn test_bsky_actor_profiles_missing_actors() {
    let output = atp_command()
        .args(&["bsky", "actor", "profiles"])
        .output()
        .expect("Failed to execute profiles");

    assert!(
        !output.status.success(),
        "Command should fail without actors"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("actors") || stderr.contains("required"),
        "Should show missing actors error"
    );
}
