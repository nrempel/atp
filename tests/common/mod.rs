use std::io::Write;
use std::process::{Command, Stdio};

/// Create a new ATP command for testing
pub fn atp_command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_atp"))
}

/// Run ATP command with stdin input (currently unused but kept for future use)
#[allow(dead_code)]
pub fn run_atp_with_stdin(args: &[&str], input: &[u8]) -> std::process::Output {
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

/// Test account DID for authenticated tests
pub const TEST_ACCOUNT_DID: &str = "did:plc:bewcbyjd75m7kqc5ykdvtqny";

/// Extract record key from AT URI
pub fn extract_rkey_from_uri(uri: &str) -> &str {
    uri.split('/').last().unwrap()
}

/// Clean up a test record by deleting it
pub fn cleanup_test_record(repo: &str, collection: &str, rkey: &str) {
    let _cleanup = atp_command()
        .args(&[
            "atproto",
            "repo",
            "delete-record",
            "--repo",
            repo,
            "--collection",
            collection,
            "--rkey",
            rkey,
        ])
        .output();
}
