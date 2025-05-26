mod common;

use common::{TEST_ACCOUNT_DID, atp_command, cleanup_test_record, extract_rkey_from_uri};

// =============================================================================
// REPOSITORY TESTS - com.atproto.repo.*
// =============================================================================

// createRecord tests
#[test]
fn test_repo_create_record_requires_auth() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "create-record",
            "--repo",
            TEST_ACCOUNT_DID,
            "--collection",
            "app.bsky.feed.post",
            "--record",
            r#"{"text": "Test auth post", "createdAt": "2025-01-27T20:30:00Z"}"#,
        ])
        .output()
        .expect("Failed to execute create-record");

    // With authentication, should succeed and create record
    assert!(
        output.status.success(),
        "Should succeed with authentication"
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("Created record:"),
        "Should show creation success"
    );

    // Clean up - extract rkey and delete the record
    if let Some(uri_line) = stdout.lines().find(|line| line.contains("at://")) {
        let rkey = extract_rkey_from_uri(uri_line);
        cleanup_test_record(TEST_ACCOUNT_DID, "app.bsky.feed.post", rkey);
    }
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
            TEST_ACCOUNT_DID,
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
        stderr.contains("expected value")
            || stderr.contains("JSON")
            || stderr.contains("parse")
            || stderr.contains("invalid"),
        "Should show JSON parsing error"
    );
}

// getRecord tests
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

// listRecords tests
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

// deleteRecord tests
#[test]
fn test_repo_delete_record_requires_auth() {
    // First create a record to delete
    let create_output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "create-record",
            "--repo",
            TEST_ACCOUNT_DID,
            "--collection",
            "app.bsky.feed.post",
            "--record",
            r#"{"text": "Test delete post", "createdAt": "2025-01-27T20:30:00Z"}"#,
        ])
        .output()
        .expect("Failed to create test record");

    assert!(create_output.status.success(), "Should create test record");
    let create_stdout = String::from_utf8(create_output.stdout).unwrap();
    let uri_line = create_stdout
        .lines()
        .find(|line| line.contains("at://"))
        .unwrap();
    let rkey = extract_rkey_from_uri(uri_line);

    // Now test deleting it
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "delete-record",
            "--repo",
            TEST_ACCOUNT_DID,
            "--collection",
            "app.bsky.feed.post",
            "--rkey",
            rkey,
        ])
        .output()
        .expect("Failed to execute delete-record");

    // With authentication, should succeed
    assert!(
        output.status.success(),
        "Should succeed with authentication"
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("deleted successfully"),
        "Should show deletion success"
    );
}

#[test]
fn test_repo_delete_record_missing_repo() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "delete-record",
            "--collection",
            "app.bsky.feed.post",
            "--rkey",
            "test-record-key",
        ])
        .output()
        .expect("Failed to execute delete-record");

    assert!(!output.status.success(), "Command should fail without repo");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("repo") || stderr.contains("required"),
        "Should show missing repo error"
    );
}

#[test]
fn test_repo_delete_record_missing_collection() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "delete-record",
            "--repo",
            "test.bsky.social",
            "--rkey",
            "test-record-key",
        ])
        .output()
        .expect("Failed to execute delete-record");

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
fn test_repo_delete_record_missing_rkey() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "delete-record",
            "--repo",
            "test.bsky.social",
            "--collection",
            "app.bsky.feed.post",
        ])
        .output()
        .expect("Failed to execute delete-record");

    assert!(!output.status.success(), "Command should fail without rkey");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("rkey") || stderr.contains("required"),
        "Should show missing rkey error"
    );
}

#[test]
fn test_repo_delete_record_nonexistent() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "delete-record",
            "--repo",
            TEST_ACCOUNT_DID,
            "--collection",
            "app.bsky.feed.post",
            "--rkey",
            "nonexistent-record-12345",
        ])
        .output()
        .expect("Failed to execute delete-record");

    // Delete operations are idempotent - they succeed even for nonexistent records
    assert!(
        output.status.success(),
        "Delete should be idempotent (succeed even for nonexistent records)"
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("deleted successfully"),
        "Should show deletion success"
    );
}

#[test]
fn test_repo_delete_record_auth_flow_validation() {
    // This test validates that the deleteRecord command properly handles authenticated requests

    // First create a record to delete
    let create_output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "create-record",
            "--repo",
            TEST_ACCOUNT_DID,
            "--collection",
            "app.bsky.feed.post",
            "--record",
            r#"{"text": "Auth flow test post", "createdAt": "2025-01-27T20:30:00Z"}"#,
        ])
        .output()
        .expect("Failed to create test record");

    assert!(create_output.status.success(), "Should create test record");
    let create_stdout = String::from_utf8(create_output.stdout).unwrap();
    let uri_line = create_stdout
        .lines()
        .find(|line| line.contains("at://"))
        .unwrap();
    let rkey = extract_rkey_from_uri(uri_line);

    // Test the authenticated delete flow
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "delete-record",
            "--repo",
            TEST_ACCOUNT_DID,
            "--collection",
            "app.bsky.feed.post",
            "--rkey",
            rkey,
        ])
        .output()
        .expect("Failed to execute delete-record");

    assert!(
        output.status.success(),
        "Should succeed with proper authentication"
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("deleted successfully"),
        "Should show deletion success"
    );
}
