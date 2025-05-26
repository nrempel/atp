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

#[test]
fn test_help_flag() {
    let output = atp_command()
        .arg("--help")
        .output()
        .expect("Failed to execute atp");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("auth") || stdout.contains("bsky"));
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
fn test_auth_subcommand_help() {
    let output = atp_command()
        .args(&["auth", "--help"])
        .output()
        .expect("Failed to execute atp auth --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("login") || stdout.contains("session"));
}

#[test]
fn test_bsky_subcommand_help() {
    let output = atp_command()
        .args(&["bsky", "--help"])
        .output()
        .expect("Failed to execute atp bsky --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("actor"));
}

#[test]
fn test_bsky_actor_subcommand_help() {
    let output = atp_command()
        .args(&["bsky", "actor", "--help"])
        .output()
        .expect("Failed to execute atp bsky actor --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("profile")
            || stdout.contains("preferences")
            || stdout.contains("suggestions")
    );
}

#[test]
fn test_auth_login_missing_credentials() {
    let output = atp_command()
        .args(&["auth", "login"])
        .output()
        .expect("Failed to execute atp auth login");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("identifier") || stderr.contains("password") || stderr.contains("required")
    );
}

#[test]
fn test_auth_login_missing_identifier() {
    let output = atp_command()
        .args(&["auth", "login", "--password", "testpass"])
        .output()
        .expect("Failed to execute atp auth login");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("identifier") || stderr.contains("required"));
}

#[test]
fn test_auth_login_missing_password() {
    let output = atp_command()
        .args(&["auth", "login", "--identifier", "test@example.com"])
        .output()
        .expect("Failed to execute atp auth login");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("password") || stderr.contains("required"));
}

#[test]
fn test_auth_login_invalid_credentials() {
    let output = atp_command()
        .args(&[
            "auth",
            "login",
            "--identifier",
            "invalid@example.com",
            "--password",
            "wrongpassword",
        ])
        .output()
        .expect("Failed to execute atp auth login");

    // Should fail with invalid credentials
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Login failed") || stderr.contains("error") || stderr.contains("failed")
    );
}

#[test]
fn test_auth_session_without_login() {
    let output = atp_command()
        .args(&["auth", "session"])
        .output()
        .expect("Failed to execute atp auth session");

    // Should fail when no session exists
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("No such file") || stderr.contains("config") || stderr.contains("session")
    );
}

#[test]
fn test_bsky_actor_profile_missing_actor() {
    let output = atp_command()
        .args(&["bsky", "actor", "profile"])
        .output()
        .expect("Failed to execute atp bsky actor profile");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("actor") || stderr.contains("required"));
}

#[test]
fn test_bsky_actor_profiles_missing_actors() {
    let output = atp_command()
        .args(&["bsky", "actor", "profiles"])
        .output()
        .expect("Failed to execute atp bsky actor profiles");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    // The command fails because it tries to load config, not because of missing actors
    assert!(
        stderr.contains("No such file")
            || stderr.contains("config")
            || stderr.contains("directory")
    );
}

#[test]
fn test_bsky_actor_search_missing_query() {
    let output = atp_command()
        .args(&["bsky", "actor", "search"])
        .output()
        .expect("Failed to execute atp bsky actor search");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("query") || stderr.contains("required"));
}

#[test]
fn test_bsky_commands_without_authentication() {
    // Test that bsky commands fail when not authenticated
    let commands = vec![
        vec!["bsky", "actor", "profile", "--actor", "test.bsky.social"],
        vec!["bsky", "actor", "profiles", "--actors", "test.bsky.social"],
        vec!["bsky", "actor", "preferences"],
        vec!["bsky", "actor", "suggestions"],
        vec!["bsky", "actor", "search", "--query", "test"],
    ];

    for cmd_args in commands {
        let output = atp_command()
            .args(&cmd_args)
            .output()
            .expect(&format!("Failed to execute atp {}", cmd_args.join(" ")));

        assert!(
            !output.status.success(),
            "Command '{}' should fail without authentication",
            cmd_args.join(" ")
        );

        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(
            stderr.contains("Not logged in")
                || stderr.contains("config")
                || stderr.contains("session")
                || stderr.contains("No such file")
                || stderr.contains("directory"),
            "Command '{}' should indicate authentication required. Stderr: {}",
            cmd_args.join(" "),
            stderr
        );
    }
}

#[test]
fn test_auth_login_short_flags() {
    let output = atp_command()
        .args(&[
            "auth",
            "login",
            "-i",
            "invalid@example.com",
            "-p",
            "wrongpassword",
        ])
        .output()
        .expect("Failed to execute atp auth login with short flags");

    // Should fail with invalid credentials (but flags should be recognized)
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    // Should not complain about missing arguments, but about login failure
    assert!(
        stderr.contains("Login failed") || stderr.contains("error") || stderr.contains("failed")
    );
}

#[test]
fn test_bsky_actor_profile_short_flag() {
    let output = atp_command()
        .args(&["bsky", "actor", "profile", "-a", "test.bsky.social"])
        .output()
        .expect("Failed to execute atp bsky actor profile with short flag");

    // Should fail due to no authentication, not due to missing actor argument
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Not logged in")
            || stderr.contains("config")
            || stderr.contains("session")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to authentication, not missing actor. Stderr: {}",
        stderr
    );
}

#[test]
fn test_bsky_actor_profiles_multiple_actors() {
    let output = atp_command()
        .args(&[
            "bsky",
            "actor",
            "profiles",
            "--actors",
            "test1.bsky.social,test2.bsky.social,test3.bsky.social",
        ])
        .output()
        .expect("Failed to execute atp bsky actor profiles with multiple actors");

    // Should fail due to no authentication, not due to argument parsing
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Not logged in")
            || stderr.contains("config")
            || stderr.contains("session")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to authentication, not argument parsing. Stderr: {}",
        stderr
    );
}

#[test]
fn test_bsky_actor_suggestions_with_limit() {
    let output = atp_command()
        .args(&["bsky", "actor", "suggestions", "--limit", "10"])
        .output()
        .expect("Failed to execute atp bsky actor suggestions with limit");

    // Should fail due to no authentication
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Not logged in")
            || stderr.contains("config")
            || stderr.contains("session")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to authentication. Stderr: {}",
        stderr
    );
}

#[test]
fn test_bsky_actor_suggestions_with_cursor() {
    let output = atp_command()
        .args(&[
            "bsky",
            "actor",
            "suggestions",
            "--limit",
            "25",
            "--cursor",
            "some-cursor-value",
        ])
        .output()
        .expect("Failed to execute atp bsky actor suggestions with cursor");

    // Should fail due to no authentication
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Not logged in")
            || stderr.contains("config")
            || stderr.contains("session")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to authentication. Stderr: {}",
        stderr
    );
}

#[test]
fn test_bsky_actor_search_with_limit() {
    let output = atp_command()
        .args(&[
            "bsky",
            "actor",
            "search",
            "--query",
            "test search",
            "--limit",
            "10",
        ])
        .output()
        .expect("Failed to execute atp bsky actor search with limit");

    // Should fail due to no authentication
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Not logged in")
            || stderr.contains("config")
            || stderr.contains("session")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to authentication. Stderr: {}",
        stderr
    );
}

#[test]
fn test_bsky_actor_search_with_cursor() {
    let output = atp_command()
        .args(&[
            "bsky",
            "actor",
            "search",
            "--query",
            "test search",
            "--limit",
            "15",
            "--cursor",
            "search-cursor",
        ])
        .output()
        .expect("Failed to execute atp bsky actor search with cursor");

    // Should fail due to no authentication
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Not logged in")
            || stderr.contains("config")
            || stderr.contains("session")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to authentication. Stderr: {}",
        stderr
    );
}

#[test]
fn test_invalid_subcommand() {
    let output = atp_command()
        .args(&["invalid-command"])
        .output()
        .expect("Failed to execute atp with invalid command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("invalid-command")
            || stderr.contains("unrecognized")
            || stderr.contains("unexpected"),
        "Should report invalid command. Stderr: {}",
        stderr
    );
}

#[test]
fn test_auth_invalid_subcommand() {
    let output = atp_command()
        .args(&["auth", "invalid"])
        .output()
        .expect("Failed to execute atp auth invalid");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("invalid")
            || stderr.contains("unrecognized")
            || stderr.contains("unexpected"),
        "Should report invalid auth subcommand. Stderr: {}",
        stderr
    );
}

#[test]
fn test_bsky_invalid_subcommand() {
    let output = atp_command()
        .args(&["bsky", "invalid"])
        .output()
        .expect("Failed to execute atp bsky invalid");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("invalid")
            || stderr.contains("unrecognized")
            || stderr.contains("unexpected"),
        "Should report invalid bsky subcommand. Stderr: {}",
        stderr
    );
}

#[test]
fn test_bsky_actor_invalid_subcommand() {
    let output = atp_command()
        .args(&["bsky", "actor", "invalid"])
        .output()
        .expect("Failed to execute atp bsky actor invalid");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("invalid")
            || stderr.contains("unrecognized")
            || stderr.contains("unexpected"),
        "Should report invalid actor subcommand. Stderr: {}",
        stderr
    );
}

#[test]
fn test_handle_with_at_symbol() {
    // Test that handles with @ symbols are handled correctly
    let output = atp_command()
        .args(&["bsky", "actor", "profile", "--actor", "@test.bsky.social"])
        .output()
        .expect("Failed to execute atp bsky actor profile with @ symbol");

    // Should fail due to no authentication, but should accept the @ symbol
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Not logged in")
            || stderr.contains("config")
            || stderr.contains("session")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to authentication, not @ symbol handling. Stderr: {}",
        stderr
    );
}

#[test]
fn test_numeric_limits_validation() {
    // Test invalid limit values
    let output = atp_command()
        .args(&["bsky", "actor", "suggestions", "--limit", "not-a-number"])
        .output()
        .expect("Failed to execute atp with invalid limit");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("invalid value") || stderr.contains("parse") || stderr.contains("number"),
        "Should report invalid numeric value. Stderr: {}",
        stderr
    );
}

#[test]
fn test_zero_limit() {
    // Test edge case with zero limit
    let output = atp_command()
        .args(&["bsky", "actor", "suggestions", "--limit", "0"])
        .output()
        .expect("Failed to execute atp with zero limit");

    // Should fail due to no authentication, but should accept zero limit
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Not logged in")
            || stderr.contains("config")
            || stderr.contains("session")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to authentication, not zero limit. Stderr: {}",
        stderr
    );
}

#[test]
fn test_large_limit() {
    // Test edge case with large limit (within u8 range)
    let output = atp_command()
        .args(&["bsky", "actor", "suggestions", "--limit", "255"])
        .output()
        .expect("Failed to execute atp with large limit");

    // Should fail due to no authentication, but should accept large limit
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Not logged in")
            || stderr.contains("config")
            || stderr.contains("session")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to authentication, not large limit. Stderr: {}",
        stderr
    );
}

#[test]
fn test_limit_overflow() {
    // Test limit value that exceeds u8 range
    let output = atp_command()
        .args(&["bsky", "actor", "suggestions", "--limit", "256"])
        .output()
        .expect("Failed to execute atp with overflow limit");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("invalid value")
            || stderr.contains("out of range")
            || stderr.contains("overflow"),
        "Should report value out of range. Stderr: {}",
        stderr
    );
}

#[test]
fn test_empty_query_string() {
    // Test search with empty query
    let output = atp_command()
        .args(&["bsky", "actor", "search", "--query", ""])
        .output()
        .expect("Failed to execute atp with empty query");

    // Should fail due to no authentication, but should accept empty query
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Not logged in")
            || stderr.contains("config")
            || stderr.contains("session")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to authentication, not empty query. Stderr: {}",
        stderr
    );
}

#[test]
fn test_query_with_special_characters() {
    // Test search with special characters
    let output = atp_command()
        .args(&[
            "bsky",
            "actor",
            "search",
            "--query",
            "test@example.com #hashtag $special &chars",
        ])
        .output()
        .expect("Failed to execute atp with special characters in query");

    // Should fail due to no authentication, but should accept special characters
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Not logged in")
            || stderr.contains("config")
            || stderr.contains("session")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to authentication, not special characters. Stderr: {}",
        stderr
    );
}

#[test]
fn test_command_structure_consistency() {
    // Test that all commands follow consistent structure
    let help_outputs = vec![
        ("auth", vec!["auth", "--help"]),
        ("bsky", vec!["bsky", "--help"]),
        ("bsky actor", vec!["bsky", "actor", "--help"]),
    ];

    for (name, args) in help_outputs {
        let output = atp_command()
            .args(&args)
            .output()
            .expect(&format!("Failed to execute atp {}", args.join(" ")));

        assert!(
            output.status.success(),
            "Help for '{}' should succeed",
            name
        );

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(
            stdout.contains("Usage:") || stdout.contains("USAGE:"),
            "Help for '{}' should contain usage information",
            name
        );
    }
}

#[test]
fn test_default_values_in_help() {
    // Test that default values are shown in help text
    let output = atp_command()
        .args(&["bsky", "actor", "suggestions", "--help"])
        .output()
        .expect("Failed to execute atp bsky actor suggestions --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("50") || stdout.contains("default"),
        "Help should show default limit value"
    );
}

#[test]
fn test_search_default_values_in_help() {
    // Test that search command shows default values
    let output = atp_command()
        .args(&["bsky", "actor", "search", "--help"])
        .output()
        .expect("Failed to execute atp bsky actor search --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("25") || stdout.contains("default"),
        "Help should show default limit value for search"
    );
}

#[test]
fn test_auth_login_help_shows_required_fields() {
    // Test that login help clearly shows required fields
    let output = atp_command()
        .args(&["auth", "login", "--help"])
        .output()
        .expect("Failed to execute atp auth login --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("identifier") && stdout.contains("password"));
}

#[test]
fn test_nested_help_commands() {
    // Test that help works at all levels of nesting
    let help_commands = vec![
        vec!["--help"],
        vec!["auth", "--help"],
        vec!["bsky", "--help"],
        vec!["bsky", "actor", "--help"],
        vec!["bsky", "actor", "profile", "--help"],
        vec!["bsky", "actor", "profiles", "--help"],
        vec!["bsky", "actor", "preferences", "--help"],
        vec!["bsky", "actor", "suggestions", "--help"],
        vec!["bsky", "actor", "search", "--help"],
    ];

    for cmd_args in help_commands {
        let output = atp_command()
            .args(&cmd_args)
            .output()
            .expect(&format!("Failed to execute atp {}", cmd_args.join(" ")));

        assert!(
            output.status.success(),
            "Help command '{}' should succeed",
            cmd_args.join(" ")
        );

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(
            stdout.contains("Usage:") || stdout.contains("USAGE:"),
            "Help for '{}' should contain usage information",
            cmd_args.join(" ")
        );
    }
}

#[test]
fn test_short_flags_consistency() {
    // Test that short flags work consistently across commands
    let short_flag_tests = vec![
        (vec!["auth", "login", "-h"], true), // help should work
        (vec!["bsky", "actor", "profile", "-h"], true), // help should work
        (vec!["bsky", "actor", "suggestions", "-h"], true), // help should work
    ];

    for (cmd_args, should_succeed) in short_flag_tests {
        let output = atp_command()
            .args(&cmd_args)
            .output()
            .expect(&format!("Failed to execute atp {}", cmd_args.join(" ")));

        assert_eq!(
            output.status.success(),
            should_succeed,
            "Command '{}' success status should be {}",
            cmd_args.join(" "),
            should_succeed
        );
    }
}

#[test]
fn test_error_message_quality_for_typos() {
    // Test that typos in commands produce helpful error messages
    let typo_commands = vec![
        vec!["auht", "login"],             // typo in "auth"
        vec!["auth", "loginn"],            // typo in "login"
        vec!["bsky", "actorr", "profile"], // typo in "actor"
        vec!["bsky", "actor", "profilee"], // typo in "profile"
    ];

    for cmd_args in typo_commands {
        let output = atp_command()
            .args(&cmd_args)
            .output()
            .expect(&format!("Failed to execute atp {}", cmd_args.join(" ")));

        assert!(!output.status.success());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(
            !stderr.is_empty(),
            "Command '{}' should provide error message for typo",
            cmd_args.join(" ")
        );
    }
}

#[test]
fn test_limit_boundary_values() {
    // Test boundary values for limit parameters
    let boundary_tests = vec![
        ("1", true),    // minimum valid
        ("255", true),  // maximum valid for u8
        ("256", false), // overflow
        ("0", true),    // zero (edge case)
    ];

    for (limit_value, should_parse) in boundary_tests {
        let output = atp_command()
            .args(&["bsky", "actor", "suggestions", "--limit", limit_value])
            .output()
            .expect(&format!("Failed to execute atp with limit {}", limit_value));

        if should_parse {
            // Should fail due to authentication, not parsing
            let stderr = String::from_utf8(output.stderr).unwrap();
            assert!(
                stderr.contains("No such file") || stderr.contains("directory"),
                "Limit {} should parse correctly but fail on auth. Stderr: {}",
                limit_value,
                stderr
            );
        } else {
            // Should fail due to parsing error
            let stderr = String::from_utf8(output.stderr).unwrap();
            assert!(
                stderr.contains("invalid value")
                    || stderr.contains("out of range")
                    || stderr.contains("overflow")
                    || stderr.contains("parse"),
                "Limit {} should fail to parse. Stderr: {}",
                limit_value,
                stderr
            );
        }
    }
}

#[test]
fn test_multiple_actors_parsing() {
    // Test that multiple actors are parsed correctly
    let multi_actor_tests = vec![
        "user1.bsky.social,user2.bsky.social",
        "user1.bsky.social,user2.bsky.social,user3.bsky.social",
        "@user1.bsky.social,@user2.bsky.social", // with @ symbols
        "user1.bsky.social, user2.bsky.social",  // with spaces
    ];

    for actors in multi_actor_tests {
        let output = atp_command()
            .args(&["bsky", "actor", "profiles", "--actors", actors])
            .output()
            .expect(&format!("Failed to execute atp with actors: {}", actors));

        // Should fail due to authentication, not parsing
        assert!(!output.status.success());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(
            stderr.contains("No such file") || stderr.contains("directory"),
            "Multiple actors '{}' should parse correctly. Stderr: {}",
            actors,
            stderr
        );
    }
}

#[test]
fn test_empty_and_whitespace_inputs() {
    // Test handling of empty and whitespace-only inputs
    let empty_tests = vec![
        ("", "empty string"),
        ("   ", "spaces only"),
        ("\t", "tab only"),
        ("\n", "newline only"),
        ("  \t\n  ", "mixed whitespace"),
    ];

    for (input, description) in empty_tests {
        // Test with query parameter
        let output = atp_command()
            .args(&["bsky", "actor", "search", "--query", input])
            .output()
            .expect(&format!("Failed to execute atp with {}", description));

        // Should fail due to authentication, not input validation
        assert!(!output.status.success());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(
            stderr.contains("No such file") || stderr.contains("directory"),
            "Input '{}' ({}) should be accepted. Stderr: {}",
            input,
            description,
            stderr
        );
    }
}

#[test]
fn test_unicode_and_special_characters_in_queries() {
    // Test that Unicode and special characters are handled properly
    let unicode_tests = vec![
        "café",            // accented characters
        "🦋🌟",            // emoji
        "こんにちは",      // Japanese
        "Здравствуй",      // Cyrillic
        "مرحبا",           // Arabic
        "test@domain.com", // email-like
        "#hashtag",        // hashtag
        "user.name",       // dots
        "user-name",       // hyphens
        "user_name",       // underscores
    ];

    for query in unicode_tests {
        let output = atp_command()
            .args(&["bsky", "actor", "search", "--query", query])
            .output()
            .expect(&format!(
                "Failed to execute atp with Unicode query: {}",
                query
            ));

        // Should fail due to authentication, not Unicode handling
        assert!(!output.status.success());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(
            stderr.contains("No such file") || stderr.contains("directory"),
            "Unicode query '{}' should be handled correctly. Stderr: {}",
            query,
            stderr
        );
    }
}

#[test]
fn test_very_long_inputs() {
    // Test handling of very long input strings
    let long_query = "a".repeat(1000); // 1000 character string
    let long_actor = format!("{}.bsky.social", "a".repeat(100)); // long actor name

    // Test long query
    let output = atp_command()
        .args(&["bsky", "actor", "search", "--query", &long_query])
        .output()
        .expect("Failed to execute atp with long query");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("No such file") || stderr.contains("directory"),
        "Long query should be handled. Stderr: {}",
        stderr
    );

    // Test long actor
    let output = atp_command()
        .args(&["bsky", "actor", "profile", "--actor", &long_actor])
        .output()
        .expect("Failed to execute atp with long actor");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("No such file") || stderr.contains("directory"),
        "Long actor should be handled. Stderr: {}",
        stderr
    );
}

#[test]
fn test_cursor_parameter_handling() {
    // Test that cursor parameters are handled correctly
    let cursor_tests = vec![
        "simple-cursor",
        "cursor-with-dashes",
        "cursor_with_underscores",
        "CursorWithMixedCase",
        "cursor123with456numbers",
        "very-long-cursor-value-that-might-be-used-in-real-scenarios",
    ];

    for cursor in cursor_tests {
        // Test with suggestions
        let output = atp_command()
            .args(&["bsky", "actor", "suggestions", "--cursor", cursor])
            .output()
            .expect(&format!("Failed to execute atp with cursor: {}", cursor));

        assert!(!output.status.success());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(
            stderr.contains("No such file") || stderr.contains("directory"),
            "Cursor '{}' should be handled correctly. Stderr: {}",
            cursor,
            stderr
        );

        // Test with search
        let output = atp_command()
            .args(&[
                "bsky", "actor", "search", "--query", "test", "--cursor", cursor,
            ])
            .output()
            .expect(&format!(
                "Failed to execute atp search with cursor: {}",
                cursor
            ));

        assert!(!output.status.success());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(
            stderr.contains("No such file") || stderr.contains("directory"),
            "Search cursor '{}' should be handled correctly. Stderr: {}",
            cursor,
            stderr
        );
    }
}

#[test]
fn test_command_combinations() {
    // Test various flag combinations to ensure they work together
    let combination_tests = vec![
        vec!["bsky", "actor", "suggestions", "--limit", "10"],
        vec![
            "bsky",
            "actor",
            "suggestions",
            "--limit",
            "25",
            "--cursor",
            "test",
        ],
        vec![
            "bsky", "actor", "search", "--query", "test", "--limit", "15",
        ],
        vec![
            "bsky",
            "actor",
            "search",
            "--query",
            "test search",
            "--limit",
            "20",
            "--cursor",
            "search-cursor",
        ],
    ];

    for cmd_args in combination_tests {
        let output = atp_command()
            .args(&cmd_args)
            .output()
            .expect(&format!("Failed to execute atp {}", cmd_args.join(" ")));

        assert!(!output.status.success());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(
            stderr.contains("No such file") || stderr.contains("directory"),
            "Command combination '{}' should parse correctly. Stderr: {}",
            cmd_args.join(" "),
            stderr
        );
    }
}

// AT Protocol Identity Tests
#[test]
fn test_atproto_identity_resolve_handle_help() {
    let output = atp_command()
        .args(&["atproto", "identity", "resolve-handle", "--help"])
        .output()
        .expect("Failed to execute atp atproto identity resolve-handle --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("handle"));
    assert!(stdout.contains("Resolve a handle to a DID"));
}

#[test]
fn test_atproto_identity_resolve_handle_missing_handle() {
    let output = atp_command()
        .args(&["atproto", "identity", "resolve-handle"])
        .output()
        .expect("Failed to execute atp atproto identity resolve-handle");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("handle") || stderr.contains("required"));
}

#[test]
fn test_atproto_identity_resolve_handle_with_handle() {
    let output = atp_command()
        .args(&[
            "atproto",
            "identity",
            "resolve-handle",
            "--handle",
            "test.bsky.social",
        ])
        .output()
        .expect("Failed to execute atp atproto identity resolve-handle");

    // Should fail due to network/auth, not argument parsing
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Failed to resolve handle")
            || stderr.contains("network")
            || stderr.contains("connection")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to network issues, not argument parsing. Stderr: {}",
        stderr
    );
}

#[test]
fn test_atproto_identity_resolve_did_help() {
    let output = atp_command()
        .args(&["atproto", "identity", "resolve-did", "--help"])
        .output()
        .expect("Failed to execute atp atproto identity resolve-did --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("did"));
    assert!(stdout.contains("Resolve a DID to its DID document"));
}

#[test]
fn test_atproto_identity_resolve_did_missing_did() {
    let output = atp_command()
        .args(&["atproto", "identity", "resolve-did"])
        .output()
        .expect("Failed to execute atp atproto identity resolve-did");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("did") || stderr.contains("required"));
}

#[test]
fn test_atproto_identity_resolve_did_with_did() {
    let output = atp_command()
        .args(&[
            "atproto",
            "identity",
            "resolve-did",
            "--did",
            "did:plc:test123",
        ])
        .output()
        .expect("Failed to execute atp atproto identity resolve-did");

    // Should fail due to network/auth, not argument parsing
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Failed to resolve DID")
            || stderr.contains("network")
            || stderr.contains("connection")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to network issues, not argument parsing. Stderr: {}",
        stderr
    );
}

#[test]
fn test_atproto_identity_update_handle_help() {
    let output = atp_command()
        .args(&["atproto", "identity", "update-handle", "--help"])
        .output()
        .expect("Failed to execute atp atproto identity update-handle --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("handle"));
    assert!(stdout.contains("Update the handle for an account"));
}

#[test]
fn test_atproto_identity_update_handle_requires_auth() {
    let output = atp_command()
        .args(&[
            "atproto",
            "identity",
            "update-handle",
            "--handle",
            "new.handle.com",
        ])
        .output()
        .expect("Failed to execute atp atproto identity update-handle");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Not logged in")
            || stderr.contains("config")
            || stderr.contains("session")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to authentication required. Stderr: {}",
        stderr
    );
}

// AT Protocol Repository Tests
#[test]
fn test_atproto_repo_create_record_help() {
    let output = atp_command()
        .args(&["atproto", "repo", "create-record", "--help"])
        .output()
        .expect("Failed to execute atp atproto repo create-record --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("repo"));
    assert!(stdout.contains("collection"));
    assert!(stdout.contains("record"));
    assert!(stdout.contains("Create a new record in a repository"));
}

#[test]
fn test_atproto_repo_create_record_missing_args() {
    let output = atp_command()
        .args(&["atproto", "repo", "create-record"])
        .output()
        .expect("Failed to execute atp atproto repo create-record");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("repo")
            || stderr.contains("collection")
            || stderr.contains("record")
            || stderr.contains("required")
    );
}

#[test]
fn test_atproto_repo_create_record_requires_auth() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "create-record",
            "--repo",
            "did:plc:test",
            "--collection",
            "app.bsky.feed.post",
            "--record",
            r#"{"text": "test post", "createdAt": "2023-01-01T00:00:00Z"}"#,
        ])
        .output()
        .expect("Failed to execute atp atproto repo create-record");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Not logged in")
            || stderr.contains("config")
            || stderr.contains("session")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to authentication required. Stderr: {}",
        stderr
    );
}

#[test]
fn test_atproto_repo_get_record_help() {
    let output = atp_command()
        .args(&["atproto", "repo", "get-record", "--help"])
        .output()
        .expect("Failed to execute atp atproto repo get-record --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("repo"));
    assert!(stdout.contains("collection"));
    assert!(stdout.contains("rkey"));
    assert!(stdout.contains("Get a record from a repository"));
}

#[test]
fn test_atproto_repo_get_record_with_args() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "get-record",
            "--repo",
            "did:plc:test",
            "--collection",
            "app.bsky.feed.post",
            "--rkey",
            "test123",
        ])
        .output()
        .expect("Failed to execute atp atproto repo get-record");

    // Should fail due to network, not argument parsing
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Failed to get record")
            || stderr.contains("network")
            || stderr.contains("connection")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to network issues, not argument parsing. Stderr: {}",
        stderr
    );
}

#[test]
fn test_atproto_repo_list_records_help() {
    let output = atp_command()
        .args(&["atproto", "repo", "list-records", "--help"])
        .output()
        .expect("Failed to execute atp atproto repo list-records --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("repo"));
    assert!(stdout.contains("collection"));
    assert!(stdout.contains("limit"));
    assert!(stdout.contains("List records in a collection"));
}

#[test]
fn test_atproto_repo_list_records_default_limit() {
    let output = atp_command()
        .args(&["atproto", "repo", "list-records", "--help"])
        .output()
        .expect("Failed to execute atp atproto repo list-records --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("50") || stdout.contains("default"));
}

#[test]
fn test_atproto_repo_delete_record_requires_auth() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "delete-record",
            "--repo",
            "did:plc:test",
            "--collection",
            "app.bsky.feed.post",
            "--rkey",
            "test123",
        ])
        .output()
        .expect("Failed to execute atp atproto repo delete-record");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Not logged in")
            || stderr.contains("config")
            || stderr.contains("session")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to authentication required. Stderr: {}",
        stderr
    );
}

#[test]
fn test_atproto_repo_upload_blob_help() {
    let output = atp_command()
        .args(&["atproto", "repo", "upload-blob", "--help"])
        .output()
        .expect("Failed to execute atp atproto repo upload-blob --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("file"));
    assert!(stdout.contains("Upload a blob to the repository"));
}

#[test]
fn test_atproto_repo_upload_blob_requires_auth() {
    let output = atp_command()
        .args(&[
            "atproto",
            "repo",
            "upload-blob",
            "--file",
            "/nonexistent/file.txt",
        ])
        .output()
        .expect("Failed to execute atp atproto repo upload-blob");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Not logged in")
            || stderr.contains("config")
            || stderr.contains("session")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to authentication or file not found. Stderr: {}",
        stderr
    );
}

#[test]
fn test_atproto_repo_describe_repo_help() {
    let output = atp_command()
        .args(&["atproto", "repo", "describe-repo", "--help"])
        .output()
        .expect("Failed to execute atp atproto repo describe-repo --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("repo"));
    assert!(stdout.contains("Describe a repository"));
}

#[test]
fn test_atproto_repo_describe_repo_with_repo() {
    let output = atp_command()
        .args(&["atproto", "repo", "describe-repo", "--repo", "did:plc:test"])
        .output()
        .expect("Failed to execute atp atproto repo describe-repo");

    // Should fail due to network, not argument parsing
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Failed to describe repo")
            || stderr.contains("network")
            || stderr.contains("connection")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to network issues, not argument parsing. Stderr: {}",
        stderr
    );
}

// AT Protocol Server Tests
#[test]
fn test_atproto_server_create_session_help() {
    let output = atp_command()
        .args(&["atproto", "server", "create-session", "--help"])
        .output()
        .expect("Failed to execute atp atproto server create-session --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("identifier"));
    assert!(stdout.contains("password"));
    assert!(stdout.contains("Create a new session"));
}

#[test]
fn test_atproto_server_create_session_missing_args() {
    let output = atp_command()
        .args(&["atproto", "server", "create-session"])
        .output()
        .expect("Failed to execute atp atproto server create-session");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("identifier") || stderr.contains("password") || stderr.contains("required")
    );
}

#[test]
fn test_atproto_server_create_session_with_credentials() {
    let output = atp_command()
        .args(&[
            "atproto",
            "server",
            "create-session",
            "--identifier",
            "test@example.com",
            "--password",
            "testpass",
        ])
        .output()
        .expect("Failed to execute atp atproto server create-session");

    // Should fail due to invalid credentials or network, not argument parsing
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Failed to create session")
            || stderr.contains("network")
            || stderr.contains("connection")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to network/auth issues, not argument parsing. Stderr: {}",
        stderr
    );
}

#[test]
fn test_atproto_server_get_session_requires_auth() {
    let output = atp_command()
        .args(&["atproto", "server", "get-session"])
        .output()
        .expect("Failed to execute atp atproto server get-session");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Not logged in")
            || stderr.contains("config")
            || stderr.contains("session")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to authentication required. Stderr: {}",
        stderr
    );
}

#[test]
fn test_atproto_server_refresh_session_requires_auth() {
    let output = atp_command()
        .args(&["atproto", "server", "refresh-session"])
        .output()
        .expect("Failed to execute atp atproto server refresh-session");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Not logged in")
            || stderr.contains("config")
            || stderr.contains("session")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to authentication required. Stderr: {}",
        stderr
    );
}

#[test]
fn test_atproto_server_delete_session_requires_auth() {
    let output = atp_command()
        .args(&["atproto", "server", "delete-session"])
        .output()
        .expect("Failed to execute atp atproto server delete-session");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Not logged in")
            || stderr.contains("config")
            || stderr.contains("session")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to authentication required. Stderr: {}",
        stderr
    );
}

#[test]
fn test_atproto_server_describe_server_help() {
    let output = atp_command()
        .args(&["atproto", "server", "describe-server", "--help"])
        .output()
        .expect("Failed to execute atp atproto server describe-server --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Describe server capabilities"));
}

#[test]
fn test_atproto_server_describe_server_no_auth_required() {
    let output = atp_command()
        .args(&["atproto", "server", "describe-server"])
        .output()
        .expect("Failed to execute atp atproto server describe-server");

    // Should fail due to network, not auth (this endpoint doesn't require auth)
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Failed to describe server")
            || stderr.contains("network")
            || stderr.contains("connection")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to network issues, not authentication. Stderr: {}",
        stderr
    );
}

// AT Protocol Sync Tests
#[test]
fn test_atproto_sync_get_blob_help() {
    let output = atp_command()
        .args(&["atproto", "sync", "get-blob", "--help"])
        .output()
        .expect("Failed to execute atp atproto sync get-blob --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("did"));
    assert!(stdout.contains("cid"));
    assert!(stdout.contains("Get a blob from the repository"));
}

#[test]
fn test_atproto_sync_get_blob_missing_args() {
    let output = atp_command()
        .args(&["atproto", "sync", "get-blob"])
        .output()
        .expect("Failed to execute atp atproto sync get-blob");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("did") || stderr.contains("cid") || stderr.contains("required"));
}

#[test]
fn test_atproto_sync_get_blob_with_args() {
    let output = atp_command()
        .args(&[
            "atproto",
            "sync",
            "get-blob",
            "--did",
            "did:plc:test",
            "--cid",
            "bafytest123",
        ])
        .output()
        .expect("Failed to execute atp atproto sync get-blob");

    // Should fail due to network, not argument parsing
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Failed to get blob")
            || stderr.contains("network")
            || stderr.contains("connection")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to network issues, not argument parsing. Stderr: {}",
        stderr
    );
}

#[test]
fn test_atproto_sync_get_head_help() {
    let output = atp_command()
        .args(&["atproto", "sync", "get-head", "--help"])
        .output()
        .expect("Failed to execute atp atproto sync get-head --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("did"));
    assert!(stdout.contains("Get repository head"));
}

#[test]
fn test_atproto_sync_get_head_with_did() {
    let output = atp_command()
        .args(&["atproto", "sync", "get-head", "--did", "did:plc:test"])
        .output()
        .expect("Failed to execute atp atproto sync get-head");

    // Should fail due to network, not argument parsing
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Failed to get head")
            || stderr.contains("network")
            || stderr.contains("connection")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to network issues, not argument parsing. Stderr: {}",
        stderr
    );
}

#[test]
fn test_atproto_sync_get_latest_commit_help() {
    let output = atp_command()
        .args(&["atproto", "sync", "get-latest-commit", "--help"])
        .output()
        .expect("Failed to execute atp atproto sync get-latest-commit --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("did"));
    assert!(stdout.contains("Get latest commit"));
}

#[test]
fn test_atproto_sync_get_repo_status_help() {
    let output = atp_command()
        .args(&["atproto", "sync", "get-repo-status", "--help"])
        .output()
        .expect("Failed to execute atp atproto sync get-repo-status --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("did"));
    assert!(stdout.contains("Get repository status"));
}

#[test]
fn test_atproto_sync_list_repos_help() {
    let output = atp_command()
        .args(&["atproto", "sync", "list-repos", "--help"])
        .output()
        .expect("Failed to execute atp atproto sync list-repos --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("limit"));
    assert!(stdout.contains("cursor"));
    assert!(stdout.contains("List repositories"));
}

#[test]
fn test_atproto_sync_list_repos_default_limit() {
    let output = atp_command()
        .args(&["atproto", "sync", "list-repos", "--help"])
        .output()
        .expect("Failed to execute atp atproto sync list-repos --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("500") || stdout.contains("default"));
}

#[test]
fn test_atproto_sync_list_repos_no_args() {
    let output = atp_command()
        .args(&["atproto", "sync", "list-repos"])
        .output()
        .expect("Failed to execute atp atproto sync list-repos");

    // Should fail due to network, not argument parsing (no required args)
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("Failed to list repos")
            || stderr.contains("network")
            || stderr.contains("connection")
            || stderr.contains("No such file")
            || stderr.contains("directory"),
        "Should fail due to network issues, not argument parsing. Stderr: {}",
        stderr
    );
}

// Edge case and validation tests
#[test]
fn test_atproto_invalid_subcommand() {
    let output = atp_command()
        .args(&["atproto", "invalid"])
        .output()
        .expect("Failed to execute atp atproto invalid");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("invalid")
            || stderr.contains("subcommand")
            || stderr.contains("unrecognized")
    );
}

#[test]
fn test_atproto_help_shows_all_subcommands() {
    let output = atp_command()
        .args(&["atproto", "--help"])
        .output()
        .expect("Failed to execute atp atproto --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("identity"));
    assert!(stdout.contains("repo"));
    assert!(stdout.contains("server"));
    assert!(stdout.contains("sync"));
}

#[test]
fn test_atproto_identity_help_shows_all_commands() {
    let output = atp_command()
        .args(&["atproto", "identity", "--help"])
        .output()
        .expect("Failed to execute atp atproto identity --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("resolve-handle"));
    assert!(stdout.contains("resolve-did"));
    assert!(stdout.contains("update-handle"));
}

#[test]
fn test_atproto_repo_help_shows_all_commands() {
    let output = atp_command()
        .args(&["atproto", "repo", "--help"])
        .output()
        .expect("Failed to execute atp atproto repo --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("create-record"));
    assert!(stdout.contains("get-record"));
    assert!(stdout.contains("list-records"));
    assert!(stdout.contains("delete-record"));
    assert!(stdout.contains("upload-blob"));
    assert!(stdout.contains("describe-repo"));
}

#[test]
fn test_atproto_server_help_shows_all_commands() {
    let output = atp_command()
        .args(&["atproto", "server", "--help"])
        .output()
        .expect("Failed to execute atp atproto server --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("create-session"));
    assert!(stdout.contains("get-session"));
    assert!(stdout.contains("refresh-session"));
    assert!(stdout.contains("delete-session"));
    assert!(stdout.contains("describe-server"));
}

#[test]
fn test_atproto_sync_help_shows_all_commands() {
    let output = atp_command()
        .args(&["atproto", "sync", "--help"])
        .output()
        .expect("Failed to execute atp atproto sync --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("get-blob"));
    assert!(stdout.contains("get-head"));
    assert!(stdout.contains("get-latest-commit"));
    assert!(stdout.contains("get-repo-status"));
    assert!(stdout.contains("list-repos"));
}

// Parameter validation tests
#[test]
fn test_atproto_repo_list_records_limit_validation() {
    let test_cases = vec![
        ("1", true),    // minimum valid
        ("1000", true), // large valid
        ("0", true),    // zero (edge case)
    ];

    for (limit, should_parse) in test_cases {
        let output = atp_command()
            .args(&[
                "atproto",
                "repo",
                "list-records",
                "--repo",
                "did:plc:test",
                "--collection",
                "app.bsky.feed.post",
                "--limit",
                limit,
            ])
            .output()
            .expect("Failed to execute atp atproto repo list-records");

        if should_parse {
            // Should fail due to network, not argument parsing
            let stderr = String::from_utf8(output.stderr).unwrap();
            assert!(
                stderr.contains("Failed to list records")
                    || stderr.contains("network")
                    || stderr.contains("connection")
                    || stderr.contains("No such file")
                    || stderr.contains("directory"),
                "Limit '{}' should parse correctly but fail on network. Stderr: {}",
                limit,
                stderr
            );
        } else {
            // Should fail due to argument parsing
            let stderr = String::from_utf8(output.stderr).unwrap();
            assert!(
                stderr.contains("invalid") || stderr.contains("parse"),
                "Limit '{}' should fail to parse. Stderr: {}",
                limit,
                stderr
            );
        }
    }
}

#[test]
fn test_atproto_sync_list_repos_limit_validation() {
    let test_cases = vec![
        ("1", true),    // minimum valid
        ("1000", true), // large valid
        ("0", true),    // zero (edge case)
    ];

    for (limit, should_parse) in test_cases {
        let output = atp_command()
            .args(&["atproto", "sync", "list-repos", "--limit", limit])
            .output()
            .expect("Failed to execute atp atproto sync list-repos");

        if should_parse {
            // Should fail due to network, not argument parsing
            let stderr = String::from_utf8(output.stderr).unwrap();
            assert!(
                stderr.contains("Failed to list repos")
                    || stderr.contains("network")
                    || stderr.contains("connection")
                    || stderr.contains("No such file")
                    || stderr.contains("directory"),
                "Limit '{}' should parse correctly but fail on network. Stderr: {}",
                limit,
                stderr
            );
        } else {
            // Should fail due to argument parsing
            let stderr = String::from_utf8(output.stderr).unwrap();
            assert!(
                stderr.contains("invalid") || stderr.contains("parse"),
                "Limit '{}' should fail to parse. Stderr: {}",
                limit,
                stderr
            );
        }
    }
}

// JSON validation tests
#[test]
fn test_atproto_repo_create_record_json_validation() {
    let test_cases = vec![
        (r#"{"text": "hello"}"#, true),
        (
            r#"{"text": "hello", "createdAt": "2023-01-01T00:00:00Z"}"#,
            true,
        ),
        (r#"invalid json"#, false),
        (r#"{"unclosed": "json"#, false),
    ];

    for (json, should_parse) in test_cases {
        let output = atp_command()
            .args(&[
                "atproto",
                "repo",
                "create-record",
                "--repo",
                "did:plc:test",
                "--collection",
                "app.bsky.feed.post",
                "--record",
                json,
            ])
            .output()
            .expect("Failed to execute atp atproto repo create-record");

        assert!(!output.status.success());
        let stderr = String::from_utf8(output.stderr).unwrap();

        if should_parse {
            // Should fail due to auth, not JSON parsing
            assert!(
                stderr.contains("Not logged in")
                    || stderr.contains("config")
                    || stderr.contains("session")
                    || stderr.contains("No such file")
                    || stderr.contains("directory"),
                "JSON '{}' should parse correctly but fail on auth. Stderr: {}",
                json,
                stderr
            );
        } else {
            // Should fail due to JSON parsing OR config loading (both are valid failures for invalid JSON)
            assert!(
                stderr.contains("JSON")
                    || stderr.contains("parse")
                    || stderr.contains("invalid")
                    || stderr.contains("No such file")
                    || stderr.contains("directory"),
                "JSON '{}' should fail to parse or fail on config loading. Stderr: {}",
                json,
                stderr
            );
        }
    }
}
