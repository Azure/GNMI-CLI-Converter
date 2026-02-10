//! Integration tests for the gnmi-cli-convert binary (main.rs CLI).
//!
//! These tests exercise the CLI binary via `cargo run` / `std::process::Command`,
//! covering all flags, input/output modes, and error paths.

use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

/// Helper: get the path to the built binary.
fn cargo_bin() -> Command {
    let mut cmd = Command::new(env!("CARGO"));
    cmd.args(["run", "--quiet", "--bin", "gnmi-cli-convert", "--"]);
    cmd
}

/// Helper: create a temp file with the given JSON content and return it.
fn json_file(content: &str) -> NamedTempFile {
    let mut f = NamedTempFile::new().expect("failed to create temp file");
    f.write_all(content.as_bytes())
        .expect("failed to write temp file");
    f
}

// =============================================================================
// --list-commands
// =============================================================================

#[test]
fn test_cli_list_commands() {
    let output = cargo_bin()
        .arg("--list-commands")
        .output()
        .expect("failed to run");
    assert!(output.status.success(), "list commands should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Supported CLI Commands:"),
        "should show header, got: {}",
        stdout
    );
}

#[test]
fn test_cli_list_commands_with_pattern() {
    let output = cargo_bin()
        .args(["--list-commands", "--pattern", "doesnotexist"])
        .output()
        .expect("failed to run");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Total: 0 commands"),
        "should find no matches, got: {}",
        stdout
    );
}

#[test]
fn test_cli_list_commands_pattern_match() {
    let output = cargo_bin()
        .args(["--list-commands", "--pattern", "queue"])
        .output()
        .expect("failed to run");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("queue"), "results should contain 'queue'");
}

// =============================================================================
// --command flag (CLI command conversion)
// =============================================================================

#[test]
fn test_cli_command_with_file_input() {
    let input = json_file(
        r#"{"Ethernet0": {"admin_status": "up", "oper_status": "up", "speed": "100G"}}"#,
    );
    let output = cargo_bin()
        .args([
            "--command",
            "show interfaces status",
            "--input",
            input.path().to_str().unwrap(),
        ])
        .output()
        .expect("failed to run");
    assert!(
        output.status.success(),
        "--command should succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Ethernet0"),
        "output should contain interface data, got: {}",
        stdout
    );
}

#[test]
fn test_cli_command_stdin_input() {
    let output = cargo_bin()
        .args(["--command", "show buffer_pool watermark"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(ref mut stdin) = child.stdin {
                stdin.write_all(
                    br#"{"ingress_lossless_pool": {"Bytes": "1000"}, "egress_lossy_pool": {"Bytes": "2000"}}"#,
                ).ok();
            }
            child.wait_with_output()
        })
        .expect("failed to run");
    assert!(
        output.status.success(),
        "stdin input should succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("ingress_lossless_pool"),
        "output should contain table data"
    );
}

#[test]
fn test_cli_command_invalid_command_fails() {
    let input = json_file(r#"{}"#);
    let output = cargo_bin()
        .args([
            "--command",
            "show nonexistent_command",
            "--input",
            input.path().to_str().unwrap(),
        ])
        .output()
        .expect("failed to run");
    assert!(
        !output.status.success(),
        "invalid command should fail"
    );
}

// =============================================================================
// --key flag (internal key conversion)
// =============================================================================

#[test]
fn test_cli_key_with_file_input() {
    let input = json_file(
        r#"{"ingress_lossless_pool": {"Bytes": "1000"}, "egress_lossy_pool": {"Bytes": "2000"}}"#,
    );
    let output = cargo_bin()
        .args([
            "--key",
            "buffer_pool/watermark",
            "--input",
            input.path().to_str().unwrap(),
        ])
        .output()
        .expect("failed to run");
    assert!(
        output.status.success(),
        "--key should succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("ingress_lossless_pool"),
        "output should contain table data, got: {}",
        stdout
    );
}

#[test]
fn test_cli_key_invalid_format_fails() {
    let input = json_file(r#"{}"#);
    let output = cargo_bin()
        .args([
            "--key",
            "invalid",
            "--input",
            input.path().to_str().unwrap(),
        ])
        .output()
        .expect("failed to run");
    assert!(
        !output.status.success(),
        "invalid key format should fail"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Invalid --key format"),
        "error should mention format, got: {}",
        stderr
    );
}

// =============================================================================
// Output to file
// =============================================================================

#[test]
fn test_cli_output_to_file() {
    let input = json_file(r#"{"ingress_lossless_pool": {"Bytes": "500"}}"#);
    let output_file = NamedTempFile::new().expect("failed to create output file");
    let output = cargo_bin()
        .args([
            "--command",
            "show buffer_pool watermark",
            "--input",
            input.path().to_str().unwrap(),
            "--output",
            output_file.path().to_str().unwrap(),
        ])
        .output()
        .expect("failed to run");
    assert!(
        output.status.success(),
        "output to file should succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let written = std::fs::read_to_string(output_file.path()).unwrap();
    assert!(
        written.contains("ingress_lossless_pool"),
        "output file should contain table data"
    );
}

// =============================================================================
// JSON output format
// =============================================================================

#[test]
fn test_cli_json_format() {
    let input = json_file(r#"{"date": "Mon Feb 04 2026"}"#);
    let output = cargo_bin()
        .args([
            "--command",
            "show clock",
            "--input",
            input.path().to_str().unwrap(),
            "--format",
            "json",
        ])
        .output()
        .expect("failed to run");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\"result\""),
        "json format should wrap in result, got: {}",
        stdout
    );
}

// =============================================================================
// Error handling
// =============================================================================

#[test]
fn test_cli_no_command_no_key_fails() {
    let output = cargo_bin().output().expect("failed to run");
    assert!(
        !output.status.success(),
        "no command or key should fail"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Must provide --command or --key"),
        "error should mention missing args, got: {}",
        stderr
    );
}

#[test]
fn test_cli_invalid_json_fails() {
    let input = json_file("not valid json at all");
    let output = cargo_bin()
        .args([
            "--command",
            "show buffer_pool watermark",
            "--input",
            input.path().to_str().unwrap(),
        ])
        .output()
        .expect("failed to run");
    assert!(
        !output.status.success(),
        "invalid JSON should cause failure"
    );
}

#[test]
fn test_cli_nonexistent_input_file() {
    let output = cargo_bin()
        .args([
            "--command",
            "show buffer_pool watermark",
            "--input",
            "nonexistent_file_xyz_12345.json",
        ])
        .output()
        .expect("failed to run");
    assert!(
        !output.status.success(),
        "nonexistent input file should cause failure"
    );
}
