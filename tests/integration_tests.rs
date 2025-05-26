use std::io::Write;
use std::process::{Command, Stdio};

fn atp_command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_atp"))
}

fn _run_atp_with_stdin(args: &[&str], input: &[u8]) -> std::process::Output {
    let mut cmd = atp_command();
    for arg in args {
        cmd.arg(arg);
    }

    let mut child = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start atp");

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(input).expect("Failed to write to stdin");
    }

    child.wait_with_output().expect("Failed to get output")
}

// =============================================================================
// BASELINE TEST - This proves our basic setup works
// =============================================================================

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

// =============================================================================
// TDD TESTS - We'll build these up lexicon by lexicon
// =============================================================================

// com.atproto.identity.* tests
#[test]
fn test_identity_resolve_handle_success() {
    let output = atp_command()
        .args(&[
            "atproto",
            "identity",
            "resolve-handle",
            "--handle",
            "jay.bsky.social",
        ])
        .output()
        .expect("Failed to execute resolve-handle");

    assert!(output.status.success(), "Command should succeed");
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should return a DID
    assert!(stdout.contains("did:plc:"), "Should return a DID");
    assert!(stdout.len() > 20, "Output should be substantial");
}

#[test]
fn test_identity_resolve_handle_missing_handle() {
    let output = atp_command()
        .args(&["atproto", "identity", "resolve-handle"])
        .output()
        .expect("Failed to execute resolve-handle");

    assert!(
        !output.status.success(),
        "Command should fail without handle"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("handle") || stderr.contains("required"),
        "Should show missing handle error"
    );
}

#[test]
fn test_identity_resolve_handle_nonexistent() {
    let output = atp_command()
        .args(&[
            "atproto",
            "identity",
            "resolve-handle",
            "--handle",
            "nonexistent-handle-12345.bsky.social",
        ])
        .output()
        .expect("Failed to execute resolve-handle");

    assert!(
        !output.status.success(),
        "Command should fail for nonexistent handle"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Failed to resolve handle") || stderr.contains("error"),
        "Should show appropriate error"
    );
}

#[test]
fn test_identity_resolve_did_missing_did() {
    let output = atp_command()
        .args(&["atproto", "identity", "resolve-did"])
        .output()
        .expect("Failed to execute resolve-did");

    assert!(!output.status.success(), "Command should fail without DID");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("did") || stderr.contains("required"),
        "Should show missing DID error"
    );
}

#[test]
fn test_identity_resolve_did_invalid() {
    let output = atp_command()
        .args(&[
            "atproto",
            "identity",
            "resolve-did",
            "--did",
            "did:invalid:nonexistent123",
        ])
        .output()
        .expect("Failed to execute resolve-did");

    assert!(
        !output.status.success(),
        "Command should fail for invalid DID"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Failed to resolve DID") || stderr.contains("error"),
        "Should show appropriate error"
    );
}

#[test]
fn test_identity_resolve_did_requires_auth() {
    // First get a real DID to test with
    let resolve_output = atp_command()
        .args(&[
            "atproto",
            "identity",
            "resolve-handle",
            "--handle",
            "jay.bsky.social",
        ])
        .output()
        .expect("Failed to resolve handle");

    assert!(resolve_output.status.success());
    let stdout = String::from_utf8(resolve_output.stdout).unwrap();
    let did = stdout.trim().replace("DID: ", "");

    // Now test resolving that DID - should fail due to no auth
    let output = atp_command()
        .args(&["atproto", "identity", "resolve-did", "--did", &did])
        .output()
        .expect("Failed to execute resolve-did");

    assert!(
        !output.status.success(),
        "Command should fail without authentication"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();

    // Should show authentication error
    assert!(
        stderr.contains("No such file")
            || stderr.contains("Not logged in")
            || stderr.contains("config"),
        "Should show authentication error"
    );
}

#[test]
fn test_identity_update_handle_requires_auth() {
    let output = atp_command()
        .args(&[
            "atproto",
            "identity",
            "update-handle",
            "--handle",
            "new-handle.bsky.social",
        ])
        .output()
        .expect("Failed to execute update-handle");

    assert!(
        !output.status.success(),
        "Command should fail without authentication"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();

    // Should show authentication error
    assert!(
        stderr.contains("No such file")
            || stderr.contains("Not logged in")
            || stderr.contains("config"),
        "Should show authentication error"
    );
}

#[test]
fn test_identity_update_handle_missing_handle() {
    let output = atp_command()
        .args(&["atproto", "identity", "update-handle"])
        .output()
        .expect("Failed to execute update-handle");

    assert!(
        !output.status.success(),
        "Command should fail without handle"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("handle") || stderr.contains("required"),
        "Should show missing handle error"
    );
}

// com.atproto.repo.* tests
#[test]
fn test_repo_create_record_requires_auth() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "create-record",
            "--repo",
            "test.bsky.social",
            "--collection",
            "app.bsky.feed.post",
            "--record",
            r#"{"text": "Hello world!", "createdAt": "2024-01-01T00:00:00Z"}"#,
        ])
        .output()
        .expect("Failed to execute create-record");

    assert!(
        !output.status.success(),
        "Command should fail without authentication"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();

    // Should show authentication error
    assert!(
        stderr.contains("No such file")
            || stderr.contains("Not logged in")
            || stderr.contains("config"),
        "Should show authentication error"
    );
}

#[test]
fn test_repo_create_record_missing_repo() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "create-record",
            "--collection",
            "app.bsky.feed.post",
            "--record",
            r#"{"text": "Hello world!"}"#,
        ])
        .output()
        .expect("Failed to execute create-record");

    assert!(!output.status.success(), "Command should fail without repo");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("repo") || stderr.contains("required"),
        "Should show missing repo error"
    );
}

#[test]
fn test_repo_create_record_missing_collection() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "create-record",
            "--repo",
            "test.bsky.social",
            "--record",
            r#"{"text": "Hello world!"}"#,
        ])
        .output()
        .expect("Failed to execute create-record");

    assert!(
        !output.status.success(),
        "Command should fail without collection"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("collection") || stderr.contains("required"),
        "Should show missing collection error"
    );
}

#[test]
fn test_repo_create_record_missing_record() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "create-record",
            "--repo",
            "test.bsky.social",
            "--collection",
            "app.bsky.feed.post",
        ])
        .output()
        .expect("Failed to execute create-record");

    assert!(
        !output.status.success(),
        "Command should fail without record"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("record") || stderr.contains("required"),
        "Should show missing record error"
    );
}

#[test]
fn test_repo_create_record_invalid_json() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "create-record",
            "--repo",
            "test.bsky.social",
            "--collection",
            "app.bsky.feed.post",
            "--record",
            "invalid json",
        ])
        .output()
        .expect("Failed to execute create-record");

    assert!(
        !output.status.success(),
        "Command should fail with invalid JSON"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("JSON")
            || stderr.contains("parse")
            || stderr.contains("invalid")
            || stderr.contains("No such file"), // May fail on config loading before JSON parsing
        "Should show JSON parsing error or config error"
    );
}

#[test]
fn test_repo_get_record_success() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "get-record",
            "--repo",
            "bsky.app",
            "--collection",
            "app.bsky.feed.post",
            "--rkey",
            "3lpk2ljkgjd2t",
        ])
        .output()
        .expect("Failed to execute get-record");

    assert!(output.status.success(), "Command should succeed");
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should return record data
    assert!(stdout.contains("URI:"), "Should show URI");
    assert!(stdout.contains("CID:"), "Should show CID");
    assert!(stdout.contains("Value:"), "Should show record value");
    assert!(
        stdout.contains("app.bsky.feed.post"),
        "Should show post type"
    );
}

#[test]
fn test_repo_get_record_missing_repo() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "get-record",
            "--collection",
            "app.bsky.actor.profile",
            "--rkey",
            "self",
        ])
        .output()
        .expect("Failed to execute get-record");

    assert!(!output.status.success(), "Command should fail without repo");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("repo") || stderr.contains("required"),
        "Should show missing repo error"
    );
}

#[test]
fn test_repo_get_record_missing_collection() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "get-record",
            "--repo",
            "jay.bsky.social",
            "--rkey",
            "self",
        ])
        .output()
        .expect("Failed to execute get-record");

    assert!(
        !output.status.success(),
        "Command should fail without collection"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("collection") || stderr.contains("required"),
        "Should show missing collection error"
    );
}

#[test]
fn test_repo_get_record_missing_rkey() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "get-record",
            "--repo",
            "jay.bsky.social",
            "--collection",
            "app.bsky.actor.profile",
        ])
        .output()
        .expect("Failed to execute get-record");

    assert!(!output.status.success(), "Command should fail without rkey");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("rkey") || stderr.contains("required"),
        "Should show missing rkey error"
    );
}

#[test]
fn test_repo_get_record_nonexistent() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "get-record",
            "--repo",
            "jay.bsky.social",
            "--collection",
            "app.bsky.feed.post",
            "--rkey",
            "nonexistent-record-12345",
        ])
        .output()
        .expect("Failed to execute get-record");

    assert!(
        !output.status.success(),
        "Command should fail for nonexistent record"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Failed to get record") || stderr.contains("error"),
        "Should show appropriate error"
    );
}

#[test]
fn test_repo_list_records_success() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "list-records",
            "--repo",
            "bsky.app",
            "--collection",
            "app.bsky.feed.post",
            "--limit",
            "3",
        ])
        .output()
        .expect("Failed to execute list-records");

    assert!(output.status.success(), "Command should succeed");
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should return record list
    assert!(stdout.contains("Found"), "Should show found count");
    assert!(stdout.contains("records:"), "Should show records label");
    assert!(stdout.contains("at://"), "Should show AT URIs");
    assert!(stdout.contains("bafyre"), "Should show CIDs");
}

#[test]
fn test_repo_list_records_missing_repo() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "list-records",
            "--collection",
            "app.bsky.feed.post",
        ])
        .output()
        .expect("Failed to execute list-records");

    assert!(!output.status.success(), "Command should fail without repo");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("repo") || stderr.contains("required"),
        "Should show missing repo error"
    );
}

#[test]
fn test_repo_list_records_missing_collection() {
    let output = atp_command()
        .args(&["atproto", "repo", "list-records", "--repo", "bsky.app"])
        .output()
        .expect("Failed to execute list-records");

    assert!(
        !output.status.success(),
        "Command should fail without collection"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("collection") || stderr.contains("required"),
        "Should show missing collection error"
    );
}

#[test]
fn test_repo_list_records_empty_collection() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "list-records",
            "--repo",
            "bsky.app",
            "--collection",
            "app.bsky.nonexistent.collection",
        ])
        .output()
        .expect("Failed to execute list-records");

    assert!(
        output.status.success(),
        "Command should succeed for empty collection"
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("Found 0 records:"),
        "Should show zero records"
    );
}

#[test]
fn test_repo_list_records_with_limit() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "list-records",
            "--repo",
            "bsky.app",
            "--collection",
            "app.bsky.feed.post",
            "--limit",
            "2",
        ])
        .output()
        .expect("Failed to execute list-records");

    assert!(output.status.success(), "Command should succeed");
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should respect limit
    let lines: Vec<&str> = stdout
        .lines()
        .filter(|line| line.contains("at://"))
        .collect();
    assert!(lines.len() <= 2, "Should not exceed limit of 2 records");
}

// TODO: com.atproto.repo.deleteRecord tests will go here
