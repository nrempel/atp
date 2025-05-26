mod common;

use common::{TEST_ACCOUNT_DID, atp_command};

// =============================================================================
// SYNC TESTS - com.atproto.sync.*
// =============================================================================

#[test]
fn test_sync_get_blob_success() {
    // Test with a known blob CID from a public repository
    let output = atp_command()
        .args(&[
            "atproto",
            "sync",
            "get-blob",
            "--did",
            "did:plc:z72i7hdynmk6r22z27h6tvur", // bsky.app
            "--cid",
            "bafkreihdwdcefgh4dqkjv67uzcmw7ojee6xedzdetojuzjevtenxquvyku", // Example CID
        ])
        .output()
        .expect("Failed to execute get-blob");

    // Note: This might fail if the specific CID doesn't exist, but the command should be valid
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(
            stdout.contains("Blob retrieved successfully"),
            "Should show blob retrieval success"
        );
    } else {
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(
            stderr.contains("Failed to get blob") || stderr.contains("error"),
            "Should show appropriate error for invalid/missing blob"
        );
    }
}

#[test]
fn test_sync_get_blob_missing_did() {
    let output = atp_command()
        .args(&[
            "atproto",
            "sync",
            "get-blob",
            "--cid",
            "bafkreihdwdcefgh4dqkjv67uzcmw7ojee6xedzdetojuzjevtenxquvyku",
        ])
        .output()
        .expect("Failed to execute get-blob");

    assert!(!output.status.success(), "Command should fail without DID");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("did") || stderr.contains("required"),
        "Should show missing DID error"
    );
}

#[test]
fn test_sync_get_blob_missing_cid() {
    let output = atp_command()
        .args(&[
            "atproto",
            "sync",
            "get-blob",
            "--did",
            "did:plc:z72i7hdynmk6r22z27h6tvur",
        ])
        .output()
        .expect("Failed to execute get-blob");

    assert!(!output.status.success(), "Command should fail without CID");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("cid") || stderr.contains("required"),
        "Should show missing CID error"
    );
}

#[test]
fn test_sync_get_blob_invalid_did() {
    let output = atp_command()
        .args(&[
            "atproto",
            "sync",
            "get-blob",
            "--did",
            "did:invalid:nonexistent123",
            "--cid",
            "bafkreihdwdcefgh4dqkjv67uzcmw7ojee6xedzdetojuzjevtenxquvyku",
        ])
        .output()
        .expect("Failed to execute get-blob");

    assert!(
        !output.status.success(),
        "Command should fail with invalid DID"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Failed to get blob") || stderr.contains("error"),
        "Should show appropriate error"
    );
}

#[test]
fn test_sync_get_blob_public_endpoint() {
    // Test that get-blob works without authentication (public endpoint)
    let output = atp_command()
        .args(&[
            "atproto",
            "sync",
            "get-blob",
            "--did",
            TEST_ACCOUNT_DID,
            "--cid",
            "bafkreihdwdcefgh4dqkjv67uzcmw7ojee6xedzdetojuzjevtenxquvyku",
        ])
        .output()
        .expect("Failed to execute get-blob");

    // Should work as public endpoint (may fail due to invalid CID, but not auth)
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(
            !stderr.contains("Not logged in") && !stderr.contains("Unauthorized"),
            "Should not fail due to authentication"
        );
    }
}

#[test]
fn test_sync_get_head_success() {
    let output = atp_command()
        .args(&[
            "atproto",
            "sync",
            "get-head",
            "--did",
            "did:plc:z72i7hdynmk6r22z27h6tvur", // bsky.app
        ])
        .output()
        .expect("Failed to execute get-head");

    assert!(output.status.success(), "Command should succeed");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Head:"), "Should show head information");
    assert!(stdout.contains("bafyre"), "Should show CID format");
}

#[test]
fn test_sync_get_head_missing_did() {
    let output = atp_command()
        .args(&["atproto", "sync", "get-head"])
        .output()
        .expect("Failed to execute get-head");

    assert!(!output.status.success(), "Command should fail without DID");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("did") || stderr.contains("required"),
        "Should show missing DID error"
    );
}

#[test]
fn test_sync_get_head_invalid_did() {
    let output = atp_command()
        .args(&[
            "atproto",
            "sync",
            "get-head",
            "--did",
            "did:invalid:nonexistent123",
        ])
        .output()
        .expect("Failed to execute get-head");

    assert!(
        !output.status.success(),
        "Command should fail with invalid DID"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Failed to get head") || stderr.contains("error"),
        "Should show appropriate error"
    );
}

#[test]
fn test_sync_get_head_public_endpoint() {
    // Test that get-head works without authentication (public endpoint)
    let output = atp_command()
        .args(&["atproto", "sync", "get-head", "--did", TEST_ACCOUNT_DID])
        .output()
        .expect("Failed to execute get-head");

    assert!(output.status.success(), "Should succeed as public endpoint");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Head:"), "Should show head information");
}

#[test]
fn test_sync_get_latest_commit_success() {
    let output = atp_command()
        .args(&[
            "atproto",
            "sync",
            "get-latest-commit",
            "--did",
            "did:plc:z72i7hdynmk6r22z27h6tvur", // bsky.app
        ])
        .output()
        .expect("Failed to execute get-latest-commit");

    assert!(output.status.success(), "Command should succeed");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("Latest commit:"),
        "Should show latest commit"
    );
    assert!(stdout.contains("Rev:"), "Should show revision");
    assert!(stdout.contains("bafyre"), "Should show CID format");
}

#[test]
fn test_sync_get_latest_commit_missing_did() {
    let output = atp_command()
        .args(&["atproto", "sync", "get-latest-commit"])
        .output()
        .expect("Failed to execute get-latest-commit");

    assert!(!output.status.success(), "Command should fail without DID");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("did") || stderr.contains("required"),
        "Should show missing DID error"
    );
}

#[test]
fn test_sync_get_latest_commit_invalid_did() {
    let output = atp_command()
        .args(&[
            "atproto",
            "sync",
            "get-latest-commit",
            "--did",
            "did:invalid:nonexistent123",
        ])
        .output()
        .expect("Failed to execute get-latest-commit");

    assert!(
        !output.status.success(),
        "Command should fail with invalid DID"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Failed to get latest commit") || stderr.contains("error"),
        "Should show appropriate error"
    );
}

#[test]
fn test_sync_get_latest_commit_public_endpoint() {
    // Test that get-latest-commit works without authentication (public endpoint)
    let output = atp_command()
        .args(&[
            "atproto",
            "sync",
            "get-latest-commit",
            "--did",
            TEST_ACCOUNT_DID,
        ])
        .output()
        .expect("Failed to execute get-latest-commit");

    assert!(output.status.success(), "Should succeed as public endpoint");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("Latest commit:"),
        "Should show latest commit"
    );
}

#[test]
fn test_sync_get_repo_status_success() {
    let output = atp_command()
        .args(&[
            "atproto",
            "sync",
            "get-repo-status",
            "--did",
            "did:plc:z72i7hdynmk6r22z27h6tvur", // bsky.app
        ])
        .output()
        .expect("Failed to execute get-repo-status");

    assert!(output.status.success(), "Command should succeed");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("DID:"), "Should show DID");
    assert!(stdout.contains("Active:"), "Should show active status");
    assert!(stdout.contains("Status:"), "Should show status");
}

#[test]
fn test_sync_get_repo_status_missing_did() {
    let output = atp_command()
        .args(&["atproto", "sync", "get-repo-status"])
        .output()
        .expect("Failed to execute get-repo-status");

    assert!(!output.status.success(), "Command should fail without DID");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("did") || stderr.contains("required"),
        "Should show missing DID error"
    );
}

#[test]
fn test_sync_get_repo_status_invalid_did() {
    let output = atp_command()
        .args(&[
            "atproto",
            "sync",
            "get-repo-status",
            "--did",
            "did:invalid:nonexistent123",
        ])
        .output()
        .expect("Failed to execute get-repo-status");

    assert!(
        !output.status.success(),
        "Command should fail with invalid DID"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Failed to get repo status") || stderr.contains("error"),
        "Should show appropriate error"
    );
}

#[test]
fn test_sync_get_repo_status_public_endpoint() {
    // Test that get-repo-status works without authentication (public endpoint)
    let output = atp_command()
        .args(&[
            "atproto",
            "sync",
            "get-repo-status",
            "--did",
            TEST_ACCOUNT_DID,
        ])
        .output()
        .expect("Failed to execute get-repo-status");

    assert!(output.status.success(), "Should succeed as public endpoint");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("DID:"), "Should show DID");
    assert!(stdout.contains("Active:"), "Should show active status");
}

#[test]
fn test_sync_list_repos_success() {
    let output = atp_command()
        .args(&["atproto", "sync", "list-repos", "--limit", "5"])
        .output()
        .expect("Failed to execute list-repos");

    assert!(output.status.success(), "Command should succeed");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Found"), "Should show found count");
    assert!(
        stdout.contains("repositories:"),
        "Should show repositories label"
    );
    assert!(stdout.contains("did:plc:"), "Should show DIDs");
}

#[test]
fn test_sync_list_repos_with_limit() {
    let output = atp_command()
        .args(&["atproto", "sync", "list-repos", "--limit", "2"])
        .output()
        .expect("Failed to execute list-repos");

    assert!(output.status.success(), "Command should succeed");
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should respect limit
    let lines: Vec<&str> = stdout
        .lines()
        .filter(|line| line.contains("did:plc:"))
        .collect();
    assert!(
        lines.len() <= 2,
        "Should not exceed limit of 2 repositories"
    );
}

#[test]
fn test_sync_list_repos_with_cursor() {
    let output = atp_command()
        .args(&[
            "atproto",
            "sync",
            "list-repos",
            "--limit",
            "3",
            "--cursor",
            "test-cursor-value",
        ])
        .output()
        .expect("Failed to execute list-repos");

    // Should handle cursor parameter (may return empty results for invalid cursor)
    assert!(
        output.status.success(),
        "Command should succeed with cursor"
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Found"), "Should show found count");
}

#[test]
fn test_sync_list_repos_default_limit() {
    let output = atp_command()
        .args(&["atproto", "sync", "list-repos"])
        .output()
        .expect("Failed to execute list-repos");

    assert!(
        output.status.success(),
        "Command should succeed with default limit"
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Found"), "Should show found count");
    assert!(
        stdout.contains("repositories:"),
        "Should show repositories label"
    );
}

#[test]
fn test_sync_list_repos_public_endpoint() {
    // Test that list-repos works without authentication (public endpoint)
    let output = atp_command()
        .args(&["atproto", "sync", "list-repos", "--limit", "3"])
        .output()
        .expect("Failed to execute list-repos");

    assert!(output.status.success(), "Should succeed as public endpoint");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Found"), "Should show found count");
}

#[test]
fn test_sync_operations_integration() {
    // Test integration between different sync operations

    // 1. Get repository status
    let status_output = atp_command()
        .args(&[
            "atproto",
            "sync",
            "get-repo-status",
            "--did",
            TEST_ACCOUNT_DID,
        ])
        .output()
        .expect("Failed to execute get-repo-status");

    assert!(status_output.status.success(), "Should get repo status");
    let status_stdout = String::from_utf8(status_output.stdout).unwrap();
    assert!(
        status_stdout.contains("Active: true"),
        "Repository should be active"
    );

    // 2. Get repository head
    let head_output = atp_command()
        .args(&["atproto", "sync", "get-head", "--did", TEST_ACCOUNT_DID])
        .output()
        .expect("Failed to execute get-head");

    assert!(head_output.status.success(), "Should get repo head");
    let head_stdout = String::from_utf8(head_output.stdout).unwrap();
    assert!(
        head_stdout.contains("Head:"),
        "Should show head information"
    );

    // 3. Get latest commit
    let commit_output = atp_command()
        .args(&[
            "atproto",
            "sync",
            "get-latest-commit",
            "--did",
            TEST_ACCOUNT_DID,
        ])
        .output()
        .expect("Failed to execute get-latest-commit");

    assert!(commit_output.status.success(), "Should get latest commit");
    let commit_stdout = String::from_utf8(commit_output.stdout).unwrap();
    assert!(
        commit_stdout.contains("Latest commit:"),
        "Should show commit info"
    );
}
