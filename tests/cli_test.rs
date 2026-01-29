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
    cmd.arg("--config")
        .arg(claude_dir)
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

// ==================== User Story 2 Tests - Detailed Output ====================

#[test]
fn test_detailed_output_with_l_flag() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(claude_dir)
        .arg("--output")
        .arg("detailed");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show version numbers
    assert!(stdout.contains("2.1.0") || stdout.contains("1.0.0"));
    // Should show source (official/third-party)
    assert!(stdout.contains("official") || stdout.contains("third-party"));
}

#[test]
fn test_detailed_output_shows_skills_version() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(claude_dir)
        .arg("--output")
        .arg("detailed")
        .arg("--skills");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show version
    assert!(stdout.contains("1.0.0"));
    // Should show source (test-skill is third-party due to hyphen in name)
    assert!(stdout.contains("third-party"));
}

#[test]
fn test_output_mode_flags_exist() {
    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--help");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Check that output mode options are documented
    assert!(stdout.contains("-l") || stdout.contains("--long"));
}

// ==================== Polish Phase Tests - Edge Cases ====================

#[test]
fn test_malformed_skills_yaml() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();
    std::fs::create_dir_all(claude_dir.join("skills")).unwrap();
    std::fs::create_dir_all(claude_dir.join("skills/test-skill")).unwrap();

    // Malformed YAML
    File::create(claude_dir.join("skills/test-skill/skill.yaml"))
        .unwrap()
        .write_all(b"invalid: yaml: content: [")
        .unwrap();

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

    // Should succeed with graceful degradation (skill skipped)
    cmd.assert().success();
}

#[test]
fn test_empty_skills_directory() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Only skills dir, no skills inside
    std::fs::create_dir_all(claude_dir.join("skills")).unwrap();

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

    cmd.assert().success();
}

#[test]
fn test_empty_session_history() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    let settings = r#"{"installed_plugins": []}"#;
    File::create(claude_dir.join("settings.json"))
        .unwrap()
        .write_all(settings.as_bytes())
        .unwrap();

    // Empty sessions array
    let sessions = r#"{"sessions": []}"#;
    File::create(claude_dir.join("session_history.json"))
        .unwrap()
        .write_all(sessions.as_bytes())
        .unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(&claude_dir);

    // Should not show SESSIONS section when count is 0
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.contains("SESSIONS"));
}

#[test]
fn test_very_long_plugin_name() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Very long plugin name
    let settings = r#"{
        "installed_plugins": [
            {"name": "this-is-a-very-long-plugin-name-with-many-hyphens-and-words"}
        ]
    }"#;
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

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should handle long names gracefully
    assert!(stdout.contains("very-long-plugin-name"));
    assert!(output.status.success());
}

#[test]
fn test_multiple_mcp_servers() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Create multiple MCP servers
    std::fs::create_dir_all(claude_dir.join("mcp-servers")).unwrap();
    std::fs::create_dir_all(claude_dir.join("mcp-servers/server1")).unwrap();
    std::fs::create_dir_all(claude_dir.join("mcp-servers/server2")).unwrap();
    std::fs::create_dir_all(claude_dir.join("mcp-servers/server3")).unwrap();

    // Create smithery.yaml for each
    for server in ["server1", "server2", "server3"] {
        let yaml = format!("name: {}\ncommand: npx -y @test/{}", server, server);
        File::create(claude_dir.join(format!("mcp-servers/{}/smithery.yaml", server)))
            .unwrap()
            .write_all(yaml.as_bytes())
            .unwrap();
    }

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

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show all 3 MCP servers
    assert!(stdout.contains("server1"));
    assert!(stdout.contains("server2"));
    assert!(stdout.contains("server3"));
    assert!(stdout.contains("MCP"));
    assert!(stdout.contains("3 servers"));
}

#[test]
fn test_multiple_hooks() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();
    std::fs::create_dir_all(claude_dir.join("hooks")).unwrap();

    // Create multiple hooks
    for hook_name in ["pre-commit-hook", "post-commit-hook", "pre-push-hook"] {
        let hook_content = format!(
            r#"---
name: {}
type: pre-commit
---
# {}
"#,
            hook_name, hook_name
        );
        File::create(claude_dir.join(format!("hooks/{}.md", hook_name)))
            .unwrap()
            .write_all(hook_content.as_bytes())
            .unwrap();
    }

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

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show all 3 hooks
    assert!(stdout.contains("pre-commit-hook"));
    assert!(stdout.contains("post-commit-hook"));
    assert!(stdout.contains("pre-push-hook"));
    assert!(stdout.contains("HOOKS"));
    assert!(stdout.contains("3 configured"));
}

#[test]
fn test_multiple_agents() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();
    std::fs::create_dir_all(claude_dir.join("agents")).unwrap();

    // Create multiple agents
    for (i, agent_name) in ["database-agent", "frontend-agent", "devops-agent"]
        .iter()
        .enumerate()
    {
        let agent_content = format!(
            r#"---
name: {}
description: Agent number {} for testing purposes
---
# {}
"#,
            agent_name,
            i + 1,
            agent_name
        );
        File::create(claude_dir.join(format!("agents/{}.md", agent_name)))
            .unwrap()
            .write_all(agent_content.as_bytes())
            .unwrap();
    }

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

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show all 3 agents
    assert!(stdout.contains("database-agent"));
    assert!(stdout.contains("frontend-agent"));
    assert!(stdout.contains("devops-agent"));
    assert!(stdout.contains("AGENTS"));
    assert!(stdout.contains("3 defined"));
}

#[test]
fn test_multiple_commands() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();
    std::fs::create_dir_all(claude_dir.join("commands")).unwrap();

    // Create multiple commands
    for cmd_name in ["analyze-code", "generate-tests", "refactor-code"] {
        let cmd_content = format!(
            r#"---
name: {}
description: Command to {}
allowed-tools: ["Read", "Edit", "Bash"]
---
# {}
"#,
            cmd_name,
            cmd_name.replace("-", " "),
            cmd_name
        );
        File::create(claude_dir.join(format!("commands/{}.md", cmd_name)))
            .unwrap()
            .write_all(cmd_content.as_bytes())
            .unwrap();
    }

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

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show all 3 commands
    assert!(stdout.contains("analyze-code"));
    assert!(stdout.contains("generate-tests"));
    assert!(stdout.contains("refactor-code"));
    assert!(stdout.contains("COMMANDS"));
    assert!(stdout.contains("3 available"));
}

#[test]
fn test_unicode_in_skill_description() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();
    std::fs::create_dir_all(claude_dir.join("skills")).unwrap();
    std::fs::create_dir_all(claude_dir.join("skills/test-skill")).unwrap();

    // Unicode description
    let skill_content = r#"---
name: test-skill
version: 1.0.0
description: 中文描述 with emoji 🚀 and special chars "quotes"
---
# Test Skill
"#;
    File::create(claude_dir.join("skills/test-skill/SKILL.md"))
        .unwrap()
        .write_all(skill_content.as_bytes())
        .unwrap();

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

    // Should handle unicode gracefully
    cmd.assert().success();
}

#[test]
fn test_cli_flags_combination() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(claude_dir)
        .arg("--output")
        .arg("detailed")
        .arg("--plugins")
        .arg("--skills");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show plugins and skills, not sessions
    assert!(stdout.contains("PLUGINS"));
    assert!(stdout.contains("SKILLS"));
    assert!(!stdout.contains("SESSIONS"));
    assert!(!stdout.contains("MCP"));
    assert!(output.status.success());
}

#[test]
fn test_json_with_filters() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(claude_dir)
        .arg("--json")
        .arg("--mcp")
        .arg("--hooks");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Parse JSON and verify structure
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    // MCP and hooks should have data
    assert!(json.get("mcp_servers").is_some());
    assert!(json.get("hooks").is_some());

    // Other sections should be empty
    assert!(json.get("plugins").unwrap().as_array().unwrap().is_empty());
    assert!(json.get("skills").unwrap().as_array().unwrap().is_empty());
}

// ==================== CLI Behavior Tests - Filter Flag Descriptions ====================

#[test]
fn test_output_mode_has_only_compact_and_detailed() {
    // Full mode should be removed - only compact and detailed available
    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--help");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should mention compact and detailed
    assert!(stdout.contains("compact"));
    assert!(stdout.contains("detailed"));
    // Full mode should not be mentioned as an option
    // The "- full:" line should not exist under "Possible values:"
    assert!(
        !stdout.contains("- full:"),
        "Full mode should not be available as an option"
    );
}

#[test]
fn test_filter_flags_have_descriptions() {
    // Filter flags should have descriptive help text
    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--help");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Each filter flag should have some description text
    // They should NOT all share the same generic "Filter by component type" text
    // The descriptions should mention what they filter (show only, specific, etc.)
    assert!(stdout.contains("specific") || stdout.contains("only"));
}

#[test]
fn test_all_filter_flags_have_descriptions() {
    // Each filter flag should have its own non-empty description in --help
    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--help");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();

    // Find lines with filter flags and check they have descriptions (not just whitespace)
    let filter_flags = [
        "--plugins",
        "--skills",
        "--sessions",
        "--mcp",
        "--hooks",
        "--agents",
        "--commands",
    ];

    for flag in &filter_flags {
        let flag_line = lines.iter().find(|line| line.trim() == *flag);
        assert!(flag_line.is_some(), "Flag {} not found in help", flag);

        // Find the index and check next line has content (description)
        if let Some(idx) = lines.iter().position(|line| line.trim() == *flag) {
            // Description should be on the next line (after indentation)
            if idx + 1 < lines.len() {
                let desc_line = lines[idx + 1].trim();
                // Description should not be empty or just another flag
                assert!(
                    !desc_line.is_empty() && !desc_line.starts_with("--"),
                    "Flag {} has empty description",
                    flag
                );
            }
        }
    }
}

#[test]
fn test_detailed_output_works() {
    // Verify detailed output mode works correctly
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(claude_dir)
        .arg("--output")
        .arg("detailed");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show version numbers
    assert!(stdout.contains("2.1.0") || stdout.contains("1.0.0"));
    // Should show source
    assert!(stdout.contains("official") || stdout.contains("third-party"));
}

// ==================== Fixtures Verification Tests ====================

#[test]
fn test_fixtures_directory_contains_all_component_types() {
    // Verify the fixtures directory has all necessary component types
    let fixtures_path = std::path::PathBuf::from("tests/fixtures/.claude");

    // Must exist
    assert!(fixtures_path.exists(), "Fixtures directory must exist");

    // Check each component type directory/file
    assert!(
        fixtures_path.join("settings.json").exists(),
        "settings.json must exist"
    );
    assert!(
        fixtures_path.join("skills").exists() && fixtures_path.join("skills").is_dir(),
        "skills/ directory must exist"
    );

    // Verify skills subdirectory has content
    let skills_dir = fixtures_path.join("skills");
    assert!(
        skills_dir.read_dir().unwrap().next().is_some(),
        "skills/ must have at least one skill"
    );

    // Check plugins directory (new format)
    let plugins_dir = fixtures_path.join("plugins");
    if plugins_dir.exists() {
        assert!(
            plugins_dir.join("installed_plugins.json").exists(),
            "plugins/installed_plugins.json must exist if plugins/ exists"
        );
    }

    // Check mcp-servers directory (new format)
    let mcp_servers_dir = fixtures_path.join("mcp-servers");
    if mcp_servers_dir.exists() {
        assert!(
            mcp_servers_dir.read_dir().unwrap().next().is_some(),
            "mcp-servers/ must have content if it exists"
        );
    }

    // Check hooks directory
    let hooks_dir = fixtures_path.join("hooks");
    if hooks_dir.exists() {
        assert!(
            hooks_dir.read_dir().unwrap().next().is_some(),
            "hooks/ must have content if it exists"
        );
    }

    // Check agents directory
    let agents_dir = fixtures_path.join("agents");
    if agents_dir.exists() {
        assert!(
            agents_dir.read_dir().unwrap().next().is_some(),
            "agents/ must have content if it exists"
        );
    }

    // Check commands directory
    let commands_dir = fixtures_path.join("commands");
    if commands_dir.exists() {
        assert!(
            commands_dir.read_dir().unwrap().next().is_some(),
            "commands/ must have content if it exists"
        );
    }
}

#[test]
fn test_fixtures_supports_all_output_modes() {
    // Verify fixtures work with all output modes
    let fixtures_path = std::path::PathBuf::from("tests/fixtures/.claude");
    if !fixtures_path.exists() {
        return; // Skip if fixtures don't exist
    }

    // Test compact mode
    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(&fixtures_path);
    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "Compact mode should work with fixtures"
    );

    // Test detailed mode
    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&fixtures_path)
        .arg("--output")
        .arg("detailed");
    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "Detailed mode should work with fixtures"
    );

    // Test JSON mode
    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(&fixtures_path).arg("--json");
    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "JSON mode should work with fixtures"
    );
}

#[test]
fn test_fixtures_supports_all_filters() {
    // Verify fixtures work with all filter flags
    let fixtures_path = std::path::PathBuf::from("tests/fixtures/.claude");
    if !fixtures_path.exists() {
        return; // Skip if fixtures don't exist
    }

    // Verify commands directory exists
    let commands_dir = fixtures_path.join("commands");
    assert!(
        commands_dir.exists() && commands_dir.is_dir(),
        "commands/ directory must exist in fixtures"
    );

    let filters = [
        "--plugins",
        "--skills",
        "--sessions",
        "--mcp",
        "--hooks",
        "--agents",
        "--commands",
    ];

    for filter in &filters {
        let mut cmd = Command::cargo_bin("claude-list").unwrap();
        cmd.arg("--config").arg(&fixtures_path).arg(filter);
        let output = cmd.output().unwrap();
        assert!(
            output.status.success(),
            "Filter {} should work with fixtures",
            filter
        );
    }
}
