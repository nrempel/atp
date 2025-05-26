mod common;

use common::atp_command;

// =============================================================================
// CLI INTERFACE TESTS - Basic functionality
// =============================================================================

#[test]
fn test_help_flag() {
    let output = atp_command()
        .arg("--help")
        .output()
        .expect("Failed to execute atp");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("auth") || stdout.contains("atproto"));
}

#[test]
fn test_version_flag() {
    let output = atp_command()
        .arg("--version")
        .output()
        .expect("Failed to execute atp");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("atp") && stdout.contains("0.0.1"));
}

#[test]
fn test_atproto_server_describe_server() {
    let output = atp_command()
        .args(&["atproto", "server", "describe-server"])
        .output()
        .expect("Failed to execute atp atproto server describe-server");

    assert!(output.status.success(), "Command should succeed");
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Verify we get real server information
    assert!(
        stdout.contains("Available domains:"),
        "Should show available domains"
    );
    assert!(
        stdout.contains("Invite required:"),
        "Should show invite requirement"
    );
    assert!(
        stdout.contains("bsky.social"),
        "Should include bsky.social domain"
    );
}
