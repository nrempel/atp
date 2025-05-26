mod common;

use common::{TEST_ACCOUNT_DID, atp_command, extract_rkey_from_uri};

// =============================================================================
// END-TO-END AUTHENTICATED TESTS - Real network calls with test account
// =============================================================================

#[test]
fn test_authenticated_create_get_delete_cycle() {
    // This test requires authentication and performs a full CRUD cycle
    // Skip if no auth is available (CI environments)

    // Step 1: Create a test record
    let create_output = atp_command()
        .args(&[
            "atproto", "repo", "create-record",
            "--repo", TEST_ACCOUNT_DID,
            "--collection", "app.bsky.feed.post",
            "--record", r#"{"text": "Automated test post - should be deleted", "createdAt": "2025-01-27T20:30:00Z"}"#
        ])
        .output()
        .expect("Failed to execute create-record");

    if !create_output.status.success() {
        let stderr = String::from_utf8(create_output.stderr).unwrap();
        if stderr.contains("No such file") || stderr.contains("config") {
            // Skip test if not authenticated (CI environment)
            return;
        }
        panic!("Create record failed: {}", stderr);
    }

    let create_stdout = String::from_utf8(create_output.stdout).unwrap();
    assert!(
        create_stdout.contains("Created record:"),
        "Should show creation success"
    );

    // Extract the record key from the URI
    let uri_line = create_stdout
        .lines()
        .find(|line| line.contains("at://"))
        .unwrap();
    let rkey = extract_rkey_from_uri(uri_line);

    // Step 2: Retrieve the record we just created
    let get_output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "get-record",
            "--repo",
            TEST_ACCOUNT_DID,
            "--collection",
            "app.bsky.feed.post",
            "--rkey",
            rkey,
        ])
        .output()
        .expect("Failed to execute get-record");

    assert!(
        get_output.status.success(),
        "Should retrieve the created record"
    );
    let get_stdout = String::from_utf8(get_output.stdout).unwrap();
    assert!(
        get_stdout.contains("Automated test post"),
        "Should contain our test text"
    );
    assert!(
        get_stdout.contains("app.bsky.feed.post"),
        "Should show correct type"
    );

    // Step 3: Verify it appears in list-records
    let list_output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "list-records",
            "--repo",
            TEST_ACCOUNT_DID,
            "--collection",
            "app.bsky.feed.post",
            "--limit",
            "10",
        ])
        .output()
        .expect("Failed to execute list-records");

    assert!(
        list_output.status.success(),
        "Should list records successfully"
    );
    let list_stdout = String::from_utf8(list_output.stdout).unwrap();
    assert!(
        list_stdout.contains(rkey),
        "Should find our record in the list"
    );

    // Step 4: Clean up - delete the record
    let delete_output = atp_command()
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
        delete_output.status.success(),
        "Should delete record successfully"
    );
    let delete_stdout = String::from_utf8(delete_output.stdout).unwrap();
    assert!(
        delete_stdout.contains("deleted successfully"),
        "Should show deletion success"
    );

    // Step 5: Verify the record is gone
    let verify_output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "get-record",
            "--repo",
            TEST_ACCOUNT_DID,
            "--collection",
            "app.bsky.feed.post",
            "--rkey",
            rkey,
        ])
        .output()
        .expect("Failed to execute get-record verification");

    assert!(
        !verify_output.status.success(),
        "Should fail to get deleted record"
    );
    let verify_stderr = String::from_utf8(verify_output.stderr).unwrap();
    assert!(
        verify_stderr.contains("Failed to get record"),
        "Should show record not found"
    );
}

#[test]
fn test_authenticated_batch_operations() {
    // Test creating multiple records and managing them

    let mut created_rkeys = Vec::new();

    // Create multiple test records
    for i in 1..=3 {
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
                &format!(
                    r#"{{"text": "Batch test post {}", "createdAt": "2025-01-27T20:30:00Z"}}"#,
                    i
                ),
            ])
            .output()
            .expect("Failed to execute create-record");

        if !create_output.status.success() {
            let stderr = String::from_utf8(create_output.stderr).unwrap();
            if stderr.contains("No such file") || stderr.contains("config") {
                // Skip test if not authenticated
                return;
            }
            panic!("Create record {} failed: {}", i, stderr);
        }

        let create_stdout = String::from_utf8(create_output.stdout).unwrap();
        let uri_line = create_stdout
            .lines()
            .find(|line| line.contains("at://"))
            .unwrap();
        let rkey = extract_rkey_from_uri(uri_line);
        created_rkeys.push(rkey.to_string());
    }

    // Verify all records exist in list
    let list_output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "list-records",
            "--repo",
            TEST_ACCOUNT_DID,
            "--collection",
            "app.bsky.feed.post",
            "--limit",
            "20",
        ])
        .output()
        .expect("Failed to execute list-records");

    assert!(
        list_output.status.success(),
        "Should list records successfully"
    );
    let list_stdout = String::from_utf8(list_output.stdout).unwrap();

    for rkey in &created_rkeys {
        assert!(
            list_stdout.contains(rkey),
            "Should find record {} in list",
            rkey
        );
    }

    // Clean up all created records
    for rkey in created_rkeys {
        let delete_output = atp_command()
            .args(&[
                "atproto",
                "repo",
                "delete-record",
                "--repo",
                TEST_ACCOUNT_DID,
                "--collection",
                "app.bsky.feed.post",
                "--rkey",
                &rkey,
            ])
            .output()
            .expect("Failed to execute delete-record");

        assert!(
            delete_output.status.success(),
            "Should delete record {} successfully",
            rkey
        );
    }
}
