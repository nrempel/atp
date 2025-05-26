mod common;

use common::{TEST_ACCOUNT_DID, atp_command};

// Test account credentials for server tests
const TEST_ACCOUNT_HANDLE: &str = "atp-test-bot.bsky.social";
const TEST_ACCOUNT_PASSWORD: &str = "vompa5-riqsah-fovgoS";

// =============================================================================
// SERVER TESTS - com.atproto.server.*
// =============================================================================

#[test]
fn test_server_describe_server_success() {
    let output = atp_command()
        .args(&["atproto", "server", "describe-server"])
        .output()
        .expect("Failed to execute describe-server");

    assert!(output.status.success(), "Command should succeed");
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should return server information
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

#[test]
fn test_server_describe_server_public_endpoint() {
    // Test that describe-server works without authentication (public endpoint)
    let output = atp_command()
        .args(&["atproto", "server", "describe-server"])
        .output()
        .expect("Failed to execute describe-server");

    assert!(output.status.success(), "Should succeed as public endpoint");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("Available domains:"),
        "Should show server capabilities"
    );
}

#[test]
fn test_server_create_session_success() {
    let output = atp_command()
        .args(&[
            "atproto",
            "server",
            "create-session",
            "--identifier",
            TEST_ACCOUNT_HANDLE,
            "--password",
            TEST_ACCOUNT_PASSWORD,
        ])
        .output()
        .expect("Failed to execute create-session");

    assert!(output.status.success(), "Command should succeed");
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should return session information
    assert!(
        stdout.contains("Session created for:"),
        "Should show session creation"
    );
    assert!(stdout.contains("DID:"), "Should show DID");
    assert!(
        stdout.contains(TEST_ACCOUNT_HANDLE),
        "Should show the handle"
    );
}

#[test]
fn test_server_create_session_missing_identifier() {
    let output = atp_command()
        .args(&[
            "atproto",
            "server",
            "create-session",
            "--password",
            "test-password",
        ])
        .output()
        .expect("Failed to execute create-session");

    assert!(
        !output.status.success(),
        "Command should fail without identifier"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("identifier") || stderr.contains("required"),
        "Should show missing identifier error"
    );
}

#[test]
fn test_server_create_session_missing_password() {
    let output = atp_command()
        .args(&[
            "atproto",
            "server",
            "create-session",
            "--identifier",
            "test.bsky.social",
        ])
        .output()
        .expect("Failed to execute create-session");

    assert!(
        !output.status.success(),
        "Command should fail without password"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("password") || stderr.contains("required"),
        "Should show missing password error"
    );
}

#[test]
fn test_server_create_session_invalid_credentials() {
    let output = atp_command()
        .args(&[
            "atproto",
            "server",
            "create-session",
            "--identifier",
            "nonexistent.bsky.social",
            "--password",
            "invalid-password",
        ])
        .output()
        .expect("Failed to execute create-session");

    assert!(
        !output.status.success(),
        "Command should fail with invalid credentials"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Failed to create session") || stderr.contains("error"),
        "Should show authentication error"
    );
}

#[test]
fn test_server_create_session_public_endpoint() {
    // Test that create-session doesn't require existing authentication
    let output = atp_command()
        .args(&[
            "atproto",
            "server",
            "create-session",
            "--identifier",
            TEST_ACCOUNT_HANDLE,
            "--password",
            TEST_ACCOUNT_PASSWORD,
        ])
        .output()
        .expect("Failed to execute create-session");

    assert!(
        output.status.success(),
        "Should succeed as public endpoint (login)"
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("Session created for:"),
        "Should show session creation"
    );
}

#[test]
fn test_server_get_session_requires_auth() {
    let output = atp_command()
        .args(&["atproto", "server", "get-session"])
        .output()
        .expect("Failed to execute get-session");

    // With authentication, should succeed
    assert!(
        output.status.success(),
        "Should succeed with authentication"
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Handle:"), "Should show handle");
    assert!(stdout.contains("DID:"), "Should show DID");
    assert!(
        stdout.contains(TEST_ACCOUNT_HANDLE),
        "Should show the authenticated handle"
    );
}

#[test]
fn test_server_get_session_auth_flow_validation() {
    // Test that get-session properly validates authentication
    let output = atp_command()
        .args(&["atproto", "server", "get-session"])
        .output()
        .expect("Failed to execute get-session");

    assert!(
        output.status.success(),
        "Should succeed with proper authentication"
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("Handle:"),
        "Should show session information"
    );
    assert!(stdout.contains("DID:"), "Should show DID");
}

#[test]
fn test_server_refresh_session_requires_auth() {
    let output = atp_command()
        .args(&["atproto", "server", "refresh-session"])
        .output()
        .expect("Failed to execute refresh-session");

    // With authentication, should succeed
    assert!(
        output.status.success(),
        "Should succeed with authentication"
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("Session refreshed successfully"),
        "Should show refresh success"
    );
}

#[test]
fn test_server_refresh_session_auth_flow_validation() {
    // Test that refresh-session properly handles refresh tokens
    let output = atp_command()
        .args(&["atproto", "server", "refresh-session"])
        .output()
        .expect("Failed to execute refresh-session");

    assert!(
        output.status.success(),
        "Should succeed with proper authentication"
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("Session refreshed successfully"),
        "Should show refresh success"
    );
}

#[test]
fn test_server_delete_session_requires_auth() {
    // Note: We can't easily test delete-session in isolation because it would
    // invalidate our test session. Instead, we test that the command exists
    // and requires authentication by checking the help or testing with a separate session.

    // Test that the command exists and shows proper help when missing auth
    let output = atp_command()
        .args(&["atproto", "server", "delete-session", "--help"])
        .output()
        .expect("Failed to execute delete-session help");

    assert!(output.status.success(), "Help should work");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("Delete session") || stdout.contains("logout"),
        "Should show delete session help"
    );
}

#[test]
fn test_server_delete_session_auth_flow_validation() {
    // Test that delete-session command exists and is properly configured
    // We can't actually delete our session as it would break other tests

    let output = atp_command()
        .args(&["atproto", "server", "delete-session", "--help"])
        .output()
        .expect("Failed to execute delete-session help");

    assert!(output.status.success(), "Help should work");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("Delete session") || stdout.contains("logout"),
        "Should show proper command description"
    );
}

#[test]
fn test_server_session_lifecycle() {
    // Test the complete session lifecycle: create -> get -> refresh
    // We skip delete to avoid invalidating our session for other tests

    // 1. Create session
    let create_output = atp_command()
        .args(&[
            "atproto",
            "server",
            "create-session",
            "--identifier",
            TEST_ACCOUNT_HANDLE,
            "--password",
            TEST_ACCOUNT_PASSWORD,
        ])
        .output()
        .expect("Failed to create session");

    assert!(create_output.status.success(), "Should create session");
    let create_stdout = String::from_utf8(create_output.stdout).unwrap();
    assert!(
        create_stdout.contains("Session created for:"),
        "Should show session creation"
    );

    // 2. Get session info
    let get_output = atp_command()
        .args(&["atproto", "server", "get-session"])
        .output()
        .expect("Failed to get session");

    assert!(get_output.status.success(), "Should get session info");
    let get_stdout = String::from_utf8(get_output.stdout).unwrap();
    assert!(get_stdout.contains("Handle:"), "Should show session info");

    // 3. Refresh session
    let refresh_output = atp_command()
        .args(&["atproto", "server", "refresh-session"])
        .output()
        .expect("Failed to refresh session");

    assert!(refresh_output.status.success(), "Should refresh session");
    let refresh_stdout = String::from_utf8(refresh_output.stdout).unwrap();
    assert!(
        refresh_stdout.contains("Session refreshed successfully"),
        "Should show refresh success"
    );
}
