//! Integration tests for claude-list CLI

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::{self, File};
use std::io::Write;
use tempfile::TempDir;

fn create_mock_claude_dir(base: &tempfile::TempDir) -> std::path::PathBuf {
    let path = base.path().join(".claude");
    std::fs::create_dir_all(&path).unwrap();

    // settings.json with plugins
    let settings = r#"{
        "installed_plugins": [
            {"name": "context7", "version": "2.1.0"},
            {"name": "plugin_test", "version": "1.0.0"}
        ]
    }"#;
    File::create(path.join("settings.json"))
        .unwrap()
        .write_all(settings.as_bytes())
        .unwrap();

    // skills/
    let skills_dir = path.join("skills");
    std::fs::create_dir_all(&skills_dir).unwrap();
    std::fs::create_dir_all(skills_dir.join("test-skill")).unwrap();
    File::create(skills_dir.join("test-skill/skill.yaml"))
        .unwrap()
        .write_all(b"name: test-skill\nversion: 1.0.0\ndescription: A test skill\n")
        .unwrap();

    // session_history.json
    let sessions = r#"{
        "sessions": [
            {"id": "1", "timestamp": "2025-01-28T10:00:00Z"},
            {"id": "2", "timestamp": "2025-01-29T10:00:00Z"}
        ]
    }"#;
    File::create(path.join("session_history.json"))
        .unwrap()
        .write_all(sessions.as_bytes())
        .unwrap();

    path
}

// ==================== User Story 1 Tests ====================

#[test]
fn test_compact_output_displays_all_components() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(claude_dir);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("PLUGINS"))
        .stdout(predicate::str::contains("SKILLS"))
        .stdout(predicate::str::contains("SESSIONS"))
        .stdout(predicate::str::contains("context7"))
        .stdout(predicate::str::contains("test-skill"));
}

#[test]
fn test_compact_output_shows_correct_counts() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(claude_dir);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2 installed"))
        .stdout(predicate::str::contains("1 available"))
        .stdout(predicate::str::contains("2 recorded"));
}

#[test]
fn test_snapshot_compact_output() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(claude_dir);

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Snapshot test - verify output matches expected format
    assert!(stdout.contains("CLAUDE-LIST v"));
    assert!(stdout.contains("CONFIG:"));
    assert!(stdout.contains("PLUGINS"));
    assert!(stdout.contains("SKILLS"));
    assert!(stdout.contains("SESSIONS"));
}

// ==================== User Story 3 Tests - Filtering ====================

#[test]
fn test_filter_plugins_only() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(claude_dir).arg("--plugins");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("PLUGINS"));
    assert!(stdout.contains("context7"));
    assert!(!stdout.contains("SKILLS"));
    assert!(!stdout.contains("test-skill"));
    assert!(!stdout.contains("SESSIONS"));
}

#[test]
fn test_filter_skills_only() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(claude_dir).arg("--skills");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("SKILLS"));
    assert!(stdout.contains("test-skill"));
    assert!(!stdout.contains("PLUGINS"));
    assert!(!stdout.contains("context7"));
    assert!(!stdout.contains("SESSIONS"));
}

#[test]
fn test_filter_sessions_only() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(claude_dir).arg("--sessions");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("SESSIONS"));
    assert!(stdout.contains("2 recorded"));
    assert!(!stdout.contains("PLUGINS"));
    assert!(!stdout.contains("SKILLS"));
}

// ==================== User Story 3 Tests - JSON Output ====================

#[test]
fn test_json_output_format() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(claude_dir).arg("--json");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Verify valid JSON
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    // Check structure
    assert!(json.get("version").is_some());
    assert!(json.get("plugins").is_some());
    assert!(json.get("skills").is_some());
    assert!(json.get("sessions").is_some());

    // Check plugins array
    let plugins = json.get("plugins").unwrap().as_array().unwrap();
    assert_eq!(plugins.len(), 2);
}

#[test]
fn test_json_output_with_filter() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(claude_dir)
        .arg("--json")
        .arg("--plugins");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    // Only plugins should be non-empty
    let plugins = json.get("plugins").unwrap().as_array().unwrap();
    let skills = json.get("skills").unwrap().as_array().unwrap();
    let sessions = json.get("sessions").unwrap();

    assert!(!plugins.is_empty());
    assert!(skills.is_empty());
    assert_eq!(sessions.get("count").unwrap(), 0);
}

// ==================== User Story 3 Tests - Error Handling ====================

#[test]
fn test_missing_claude_directory_error() {
    let dir = TempDir::new().unwrap();
    let missing_dir = dir.path().join("nonexistent");

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(missing_dir);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Directory not found"));
}

#[test]
fn test_malformed_json_settings() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Create malformed JSON
    File::create(claude_dir.join("settings.json"))
        .unwrap()
        .write_all(b"{ invalid json }")
        .unwrap();

    // sessions directory with valid JSON
    let sessions = r#"{"sessions": []}"#;
    File::create(claude_dir.join("session_history.json"))
        .unwrap()
        .write_all(sessions.as_bytes())
        .unwrap();

    // Should still succeed with partial data (graceful degradation)
    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(&claude_dir);

    // The tool should handle errors gracefully
    cmd.assert().success();
}

#[test]
fn test_missing_optional_files() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Only create settings.json
    let settings = r#"{"installed_plugins": []}"#;
    File::create(claude_dir.join("settings.json"))
        .unwrap()
        .write_all(settings.as_bytes())
        .unwrap();

    // No session_history.json, skills/, mcp.json, etc.

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(&claude_dir);

    // Should succeed with empty components
    cmd.assert().success();
}

#[test]
fn test_empty_plugins_array() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    let settings = r#"{"installed_plugins": []}"#;
    File::create(claude_dir.join("settings.json"))
        .unwrap()
        .write_all(settings.as_bytes())
        .unwrap();

    let sessions = r#"{"sessions": []}"#;
    File::create(claude_dir.join("session_history.json"))
        .unwrap()
        .write_all(sessions.as_bytes())
        .unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(&claude_dir);

    // Empty sections are not shown (minimalist design)
    // Should succeed with just header and config
    cmd.assert().success();
}

#[test]
fn test_help_output() {
    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("claude-list"))
        .stdout(predicate::str::contains("--config"))
        .stdout(predicate::str::contains("--plugins"))
        .stdout(predicate::str::contains("--skills"))
        .stdout(predicate::str::contains("--json"));
}

#[test]
fn test_version_output() {
    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("claude-list"));
}
