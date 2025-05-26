mod common;

use common::atp_command;

// =============================================================================
// IDENTITY TESTS - com.atproto.identity.*
// =============================================================================

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

    // Now test resolving that DID - may fail with 404 for some DIDs
    let output = atp_command()
        .args(&["atproto", "identity", "resolve-did", "--did", &did])
        .output()
        .expect("Failed to execute resolve-did");

    if output.status.success() {
        // If it succeeds, verify the response contains the DID
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains(&did), "Should contain the DID in response");
    } else {
        // If it fails, should show a proper error (404 is common for some DIDs)
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(
            stderr.contains("Failed to resolve DID") || stderr.contains("404"),
            "Should show DID resolution error"
        );
    }
}

#[test]
fn test_identity_update_handle_requires_auth() {
    let output = atp_command()
        .args(&[
            "atproto",
            "identity",
            "update-handle",
            "--handle",
            "new-handle-that-doesnt-exist.bsky.social",
        ])
        .output()
        .expect("Failed to execute update-handle");

    // With authentication, should attempt the operation (may fail due to handle availability)
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(
            stderr.contains("Failed to update handle"),
            "Should show handle update error (handle likely unavailable)"
        );
    }
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

#[test]
fn test_authenticated_identity_operations() {
    // Test identity operations that require authentication

    // Test resolve-did with authentication
    let output = atp_command()
        .args(&[
            "atproto",
            "identity",
            "resolve-did",
            "--did",
            "did:plc:bewcbyjd75m7kqc5ykdvtqny",
        ])
        .output()
        .expect("Failed to execute resolve-did");

    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).unwrap();
        if stderr.contains("No such file") || stderr.contains("config") {
            // Skip test if not authenticated
            return;
        }
        // If authenticated but still fails, that's expected for some DIDs
        assert!(
            stderr.contains("Failed to resolve DID"),
            "Should show DID resolution error"
        );
        return;
    }

    // If it succeeds, verify the response
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("did:plc:bewcbyjd75m7kqc5ykdvtqny"),
        "Should contain the DID"
    );
}
