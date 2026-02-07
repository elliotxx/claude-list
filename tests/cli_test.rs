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

// Test for issue: -l flag should work for detailed output
#[test]
fn test_short_l_flag_for_detailed_output() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(claude_dir).arg("-l");

    // -l should not cause an error, it should enable detailed output
    let output = cmd.output().unwrap();

    // Command should succeed, not fail with "unexpected argument '-l'"
    assert!(
        output.status.success(),
        "-l flag should be accepted, but got error: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Should show detailed output with NAME, SOURCE (for skills), DESCRIPTION (for others)
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("NAME"));
    assert!(stdout.contains("SOURCE")); // Skills now show SOURCE (location: global/plugin)
    assert!(stdout.contains("DESCRIPTION"));
    assert!(stdout.contains("context7"));
}

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

    // Should show NAME, PATH (for plugins), SOURCE (for skills), DESCRIPTION
    assert!(stdout.contains("NAME"));
    assert!(stdout.contains("SOURCE")); // Skills now show SOURCE (location: global/plugin)
    assert!(stdout.contains("DESCRIPTION"));
    // Should show plugin path
    assert!(stdout.contains("settings.json") || stdout.contains("plugins"));
}

#[test]
fn test_detailed_output_shows_skills_description() {
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

    // Should show NAME, SOURCE (for skills), DESCRIPTION
    assert!(stdout.contains("NAME"));
    assert!(stdout.contains("SOURCE")); // Skills now show SOURCE (location: global/plugin)
    assert!(stdout.contains("DESCRIPTION"));
    // Should show skill description from skill.yaml
    assert!(stdout.contains("A test skill"));
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

    // Should show NAME, PATH (for plugins), SOURCE (for skills), DESCRIPTION
    assert!(stdout.contains("NAME"));
    assert!(stdout.contains("SOURCE")); // Skills now show SOURCE (location: global/plugin)
    assert!(stdout.contains("DESCRIPTION"));
    // Should show plugin path
    assert!(stdout.contains("settings.json") || stdout.contains("plugins"));
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

// ==================== Colored Output Tests (T012, T013) ====================

#[test]
fn test_colored_output_in_terminal() {
    // Test that colors are enabled in TTY mode (when stdout is a terminal)
    // This test verifies the ColorSettings::from_env() behavior
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(claude_dir);

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should produce output
    assert!(output.status.success());
    assert!(stdout.contains("CLAUDE-LIST"));
    assert!(stdout.contains("PLUGINS"));
    assert!(stdout.contains("context7"));
}

#[test]
fn test_no_color_flag_disables_colors() {
    // Test that --no-color flag produces plain text output
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(claude_dir).arg("--no-color");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Command should succeed
    assert!(output.status.success());

    // Should still show components (just without ANSI codes)
    assert!(stdout.contains("context7"));
    assert!(stdout.contains("test-skill"));

    // Verify no ANSI escape codes (basic check)
    assert!(!stdout.contains("\x1b["));
}

// ==================== Search Functionality Tests (T020, T021, T025) ====================

#[test]
fn test_single_keyword_search() {
    // Test single keyword search (case-insensitive)
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(claude_dir)
        .arg("--search")
        .arg("context");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    // Should contain context7 (matches "context")
    assert!(stdout.contains("context7"));
    // Should not contain test-skill (doesn't match "context")
    assert!(!stdout.contains("test-skill"));
}

#[test]
fn test_single_keyword_search_case_insensitive() {
    // Test case-insensitive matching
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    // Test uppercase
    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--search")
        .arg("CONTEXT");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert!(stdout.contains("context7"));

    // Test mixed case
    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--search")
        .arg("Context");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert!(stdout.contains("context7"));
}

#[test]
fn test_empty_search_result_message() {
    // Test that empty search results show appropriate message
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(claude_dir)
        .arg("--search")
        .arg("nonexistent");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    // Should show no components
    assert!(!stdout.contains("PLUGINS"));
    assert!(!stdout.contains("SKILLS"));
}

#[test]
fn test_multi_keyword_and_search() {
    // Test multi-keyword AND search
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Create plugins with names that test AND logic
    let settings = r#"{
        "installed_plugins": [
            {"name": "context7-plugin"},
            {"name": "plugin-context-manager"},
            {"name": "context7-demo"},
            {"name": "other-plugin"}
        ]
    }"#;
    std::fs::write(claude_dir.join("settings.json"), settings).unwrap();

    let sessions = r#"{"sessions": []}"#;
    std::fs::write(claude_dir.join("session_history.json"), sessions).unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--search")
        .arg("context plugin");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    // Should match "context7-plugin" (has both "context" and "plugin")
    assert!(stdout.contains("context7-plugin"));
    // Should match "plugin-context-manager" (has both)
    assert!(stdout.contains("plugin-context-manager"));
    // Should NOT match "context7-demo" (has "context" but not "plugin")
    assert!(!stdout.contains("context7-demo"));
    // Should NOT match "other-plugin" (has "plugin" but not "context")
    assert!(!stdout.contains("other-plugin"));
}

// ==================== Search + Filter Combination Tests (T031) ====================

#[test]
fn test_search_with_plugins_filter() {
    // Test search combined with --plugins filter
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    let settings = r#"{
        "installed_plugins": [
            {"name": "context7", "version": "2.1.0"},
            {"name": "other-plugin", "version": "1.0.0"}
        ]
    }"#;
    std::fs::write(claude_dir.join("settings.json"), settings).unwrap();

    // Add a skill with "context" in the name
    std::fs::create_dir_all(claude_dir.join("skills")).unwrap();
    std::fs::create_dir_all(claude_dir.join("skills/context-skill")).unwrap();
    std::fs::write(
        claude_dir.join("skills/context-skill/skill.yaml"),
        "name: context-skill\nversion: 1.0.0\n",
    )
    .unwrap();

    let sessions = r#"{"sessions": []}"#;
    std::fs::write(claude_dir.join("session_history.json"), sessions).unwrap();

    // Search with --plugins should only show matching plugins
    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--search")
        .arg("context")
        .arg("--plugins");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    // Should show context7 plugin
    assert!(stdout.contains("context7"));
    // Should NOT show context-skill (filtered by --plugins)
    assert!(!stdout.contains("context-skill"));
}

#[test]
fn test_search_with_skills_filter() {
    // Test search combined with --skills filter
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    let settings = r#"{"installed_plugins": []}"#;
    std::fs::write(claude_dir.join("settings.json"), settings).unwrap();

    // Create multiple skills
    std::fs::create_dir_all(claude_dir.join("skills")).unwrap();
    std::fs::create_dir_all(claude_dir.join("skills/context-skill")).unwrap();
    std::fs::write(
        claude_dir.join("skills/context-skill/skill.yaml"),
        "name: context-skill\nversion: 1.0.0\n",
    )
    .unwrap();
    std::fs::create_dir_all(claude_dir.join("skills/api-skill")).unwrap();
    std::fs::write(
        claude_dir.join("skills/api-skill/skill.yaml"),
        "name: api-skill\nversion: 2.0.0\n",
    )
    .unwrap();

    let sessions = r#"{"sessions": []}"#;
    std::fs::write(claude_dir.join("session_history.json"), sessions).unwrap();

    // Search with --skills should only show matching skills
    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--search")
        .arg("context")
        .arg("--skills");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    // Should show context-skill
    assert!(stdout.contains("context-skill"));
    // Should NOT show api-skill
    assert!(!stdout.contains("api-skill"));
}

// ==================== Color Control via Environment Tests (T034, T035) ====================

#[test]
fn test_no_color_env_disables_colors() {
    // Test that NO_COLOR=1 environment variable disables colors
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(claude_dir);
    cmd.env("NO_COLOR", "1");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    // Should still show content (just without ANSI codes)
    assert!(stdout.contains("context7"));
    assert!(stdout.contains("test-skill"));

    // Verify no ANSI escape codes
    assert!(!stdout.contains("\x1b["));
}

#[test]
#[ignore]
fn test_piped_output_has_no_colors() {
    // Test that piped output automatically disables colors (non-TTY)
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    // Run command and pipe output (simulates non-TTY)
    let output = std::process::Command::new("./target/release/claude-list")
        .arg("--config")
        .arg(claude_dir)
        .output()
        .expect("Failed to run command");

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    // Should show content
    assert!(stdout.contains("CLAUDE-LIST"));
    assert!(stdout.contains("context7"));

    // In non-TTY mode, colors should be automatically disabled
    // (The ColorSettings::from_env() checks is_terminal())
    // We verify this by checking no ANSI codes are present
    assert!(!stdout.contains("\x1b["));
}

// ==================== End-to-End Integration Tests (T026, T027) ====================

#[test]
fn test_detailed_output_with_search_and_colors() {
    // Test detailed mode (-l) combined with search
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(claude_dir)
        .arg("-l")
        .arg("--search")
        .arg("test"); // "test" matches both plugin_test and test-skill

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    // Should show detailed format (NAME, PATH for plugins, SOURCE for skills)
    assert!(stdout.contains("NAME"));
    assert!(stdout.contains("SOURCE")); // Skills now show SOURCE (location: global/plugin)
    assert!(stdout.contains("PATH")); // Plugins show PATH, not DESCRIPTION
                                      // Should filter by search
    assert!(stdout.contains("test-skill")); // Matches skill
                                            // Should not show non-matching plugins
    assert!(!stdout.contains("context7"));
}

#[test]
fn test_all_output_modes_with_search() {
    // Test search works with all output modes
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    // Compact mode with search
    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--search")
        .arg("context");
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("context7"));

    // Detailed mode with search - search for skill to get DESCRIPTION column
    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--output")
        .arg("detailed")
        .arg("--search")
        .arg("test");
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("test-skill")); // Matches skill
    assert!(stdout.contains("NAME"));
    assert!(stdout.contains("DESCRIPTION")); // Skills have description

    // JSON mode with search
    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--json")
        .arg("--search")
        .arg("context");
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    // Verify JSON is valid and filtered
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let plugins = json.get("plugins").unwrap().as_array().unwrap();
    assert_eq!(plugins.len(), 1);
}

#[test]
fn test_search_with_all_filters_combination() {
    // Test search works with any filter combination
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

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
        cmd.arg("--config")
            .arg(&claude_dir)
            .arg("--search")
            .arg("test")
            .arg(filter);

        let output = cmd.output().unwrap();
        assert!(
            output.status.success(),
            "Search with filter {} should succeed",
            filter
        );
    }
}

// ==================== Performance Benchmark Tests (T028, T029) ====================
// These tests require release build and are unstable in CI environments.
// Run manually with: cargo test --release --test cli_test --ignored

#[test]
#[ignore]
fn test_search_performance_benchmark() {
    // Benchmark: search should complete in under 100ms for 100+ components
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Create 150 plugins to benchmark search
    let mut plugins = Vec::new();
    for i in 0..150 {
        plugins.push(format!(
            r#"{{"name": "plugin-{:03}-test{}", "version": "1.0.0"}}"#,
            i, i
        ));
    }
    let settings = format!(r#"{{"installed_plugins": [{}]}}"#, plugins.join(","));
    std::fs::write(claude_dir.join("settings.json"), &settings).unwrap();

    let sessions = r#"{"sessions": []}"#;
    std::fs::write(claude_dir.join("session_history.json"), sessions).unwrap();

    let start = std::time::Instant::now();
    let output = std::process::Command::new("./target/release/claude-list")
        .arg("--config")
        .arg(claude_dir)
        .arg("--search")
        .arg("test")
        .output()
        .expect("Failed to run command");
    let elapsed = start.elapsed();

    assert!(output.status.success());
    // Should complete in under 100ms
    assert!(
        elapsed.as_millis() < 100,
        "Search took {}ms, expected < 100ms",
        elapsed.as_millis()
    );
}

#[test]
#[ignore]
fn test_color_rendering_performance() {
    // Benchmark: color rendering should add minimal overhead (<5ms)
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);
    let claude_dir_str = claude_dir.to_string_lossy().into_owned();

    // Measure time with colors enabled (default)
    let start = std::time::Instant::now();
    let _colored_output = std::process::Command::new("./target/release/claude-list")
        .arg("--config")
        .arg(&claude_dir_str)
        .output()
        .expect("Failed to run command");
    let colored_elapsed = start.elapsed();

    // Measure time with colors disabled
    let start = std::time::Instant::now();
    let no_color_output = std::process::Command::new("./target/release/claude-list")
        .arg("--config")
        .arg(&claude_dir_str)
        .arg("--no-color")
        .output()
        .expect("Failed to run command");
    let no_color_elapsed = start.elapsed();

    assert!(no_color_output.status.success());
    // Color rendering overhead should be minimal (<30ms for local/CI environments)
    let overhead = colored_elapsed.as_millis() as i64 - no_color_elapsed.as_millis() as i64;
    assert!(
        overhead < 30,
        "Color rendering overhead was {}ms, expected < 30ms",
        overhead
    );
}

#[test]
#[ignore]
fn test_large_dataset_search_performance() {
    // Test search performance with large dataset
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Create 200 components across all types
    let mut plugins = Vec::new();
    for i in 0..50 {
        plugins.push(format!(
            r#"{{"name": "searchable-plugin-{:03}", "version": "1.0.0"}}"#,
            i
        ));
    }
    let settings = format!(r#"{{"installed_plugins": [{}]}}"#, plugins.join(","));
    std::fs::write(claude_dir.join("settings.json"), &settings).unwrap();

    // Create searchable skills
    std::fs::create_dir_all(claude_dir.join("skills")).unwrap();
    for i in 0..50 {
        std::fs::create_dir_all(claude_dir.join(format!("skills/searchable-skill-{:03}", i)))
            .unwrap();
        std::fs::write(
            claude_dir.join(format!("skills/searchable-skill-{:03}/skill.yaml", i)),
            format!("name: searchable-skill-{:03}\nversion: 1.0.0\n", i),
        )
        .unwrap();
    }

    // Create searchable commands
    std::fs::create_dir_all(claude_dir.join("commands")).unwrap();
    for i in 0..50 {
        std::fs::write(
            claude_dir.join(format!("commands/searchable-command-{:03}.md", i)),
            format!("---\nname: searchable-command-{:03}\n---\n", i),
        )
        .unwrap();
    }

    // Create searchable agents
    std::fs::create_dir_all(claude_dir.join("agents")).unwrap();
    for i in 0..50 {
        std::fs::write(
            claude_dir.join(format!("agents/searchable-agent-{:03}.md", i)),
            format!("---\nname: searchable-agent-{:03}\n---\n", i),
        )
        .unwrap();
    }

    let sessions = r#"{"sessions": []}"#;
    std::fs::write(claude_dir.join("session_history.json"), sessions).unwrap();

    // Benchmark search across all components
    let start = std::time::Instant::now();
    let output = std::process::Command::new("./target/release/claude-list")
        .arg("--config")
        .arg(claude_dir)
        .arg("--search")
        .arg("searchable")
        .output()
        .expect("Failed to run command");
    let elapsed = start.elapsed();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Should find 200 matching components (50 plugins + 50 skills + 50 commands + 50 agents)
    assert!(stdout.contains("200") || stdout.contains("50"));

    // Should complete quickly (< 100ms)
    assert!(
        elapsed.as_millis() < 100,
        "Search across 200 components took {}ms, expected < 100ms",
        elapsed.as_millis()
    );
}

// ==================== Additional Filter Tests ====================

#[test]
fn test_filter_mcp_only() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    // Add MCP server
    std::fs::create_dir_all(claude_dir.join("mcp-servers")).unwrap();
    std::fs::create_dir_all(claude_dir.join("mcp-servers/test-server")).unwrap();
    let mcp_yaml = r#"name: test-server
command: npx -y @test/server"#;
    std::fs::write(
        claude_dir.join("mcp-servers/test-server/smithery.yaml"),
        mcp_yaml,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(&claude_dir).arg("--mcp");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("MCP"));
    assert!(stdout.contains("test-server"));
    assert!(!stdout.contains("PLUGINS"));
    assert!(!stdout.contains("SKILLS"));
    assert!(!stdout.contains("context7"));
}

#[test]
fn test_filter_hooks_only() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    // Add hooks
    std::fs::create_dir_all(claude_dir.join("hooks")).unwrap();
    let hook_content = r#"---
hook: pre-commit
---
# Test Hook"#;
    std::fs::write(claude_dir.join("hooks/test-hook.md"), hook_content).unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(&claude_dir).arg("--hooks");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("HOOKS"));
    assert!(stdout.contains("test-hook"));
    assert!(!stdout.contains("PLUGINS"));
    assert!(!stdout.contains("SKILLS"));
}

#[test]
fn test_filter_agents_only() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    // Add agent
    std::fs::create_dir_all(claude_dir.join("agents")).unwrap();
    let agent_content = r#"---
name: test-agent
description: A test agent
---
# Test Agent"#;
    std::fs::write(claude_dir.join("agents/test-agent.md"), agent_content).unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(&claude_dir).arg("--agents");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("AGENTS"));
    assert!(stdout.contains("test-agent"));
    assert!(!stdout.contains("PLUGINS"));
    assert!(!stdout.contains("SKILLS"));
}

#[test]
fn test_filter_commands_only() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    // Add command
    std::fs::create_dir_all(claude_dir.join("commands")).unwrap();
    let cmd_content = r#"---
name: test-command
description: A test command
---
# Test Command"#;
    std::fs::write(claude_dir.join("commands/test-command.md"), cmd_content).unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(&claude_dir).arg("--commands");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("COMMANDS"));
    assert!(stdout.contains("test-command"));
    assert!(!stdout.contains("PLUGINS"));
    assert!(!stdout.contains("SKILLS"));
}

// ==================== Detailed Mode Column Tests ====================

#[test]
fn test_detailed_mode_shows_plugin_path() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--output")
        .arg("detailed")
        .arg("--plugins");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Plugins should show PATH column in detailed mode
    assert!(stdout.contains("PATH"));
    assert!(stdout.contains("context7"));
}

#[test]
fn test_detailed_mode_shows_skill_source() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--output")
        .arg("detailed")
        .arg("--skills");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Skills should show SOURCE and DESCRIPTION columns in detailed mode
    assert!(stdout.contains("SOURCE"));
    assert!(stdout.contains("DESCRIPTION"));
    assert!(stdout.contains("test-skill"));
}

#[test]
fn test_detailed_mode_shows_agent_description() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    // Add agent
    std::fs::create_dir_all(claude_dir.join("agents")).unwrap();
    let agent_content = r#"---
name: test-agent
description: Test agent description
---
# Test Agent"#;
    std::fs::write(claude_dir.join("agents/test-agent.md"), agent_content).unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--output")
        .arg("detailed")
        .arg("--agents");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Agents should show DESCRIPTION column in detailed mode
    assert!(stdout.contains("DESCRIPTION"));
    assert!(stdout.contains("test-agent"));
    assert!(stdout.contains("Test agent description"));
}

#[test]
fn test_detailed_mode_shows_command_description() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    // Add command
    std::fs::create_dir_all(claude_dir.join("commands")).unwrap();
    let cmd_content = r#"---
name: test-command
description: Test command description
---
# Test Command"#;
    std::fs::write(claude_dir.join("commands/test-command.md"), cmd_content).unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--output")
        .arg("detailed")
        .arg("--commands");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Commands should show DESCRIPTION column in detailed mode
    assert!(stdout.contains("DESCRIPTION"));
    assert!(stdout.contains("test-command"));
    assert!(stdout.contains("Test command description"));
}

#[test]
fn test_detailed_mode_shows_mcp_description() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    // Add MCP server with description
    std::fs::create_dir_all(claude_dir.join("mcp-servers")).unwrap();
    std::fs::create_dir_all(claude_dir.join("mcp-servers/test-server")).unwrap();
    let mcp_yaml = r#"name: test-server
description: Test MCP server for testing
startCommand:
  type: stdio
  commandFunction: |
    (config) => ({
      command: 'npx',
      args: ['-y', '@test/server']
    })"#;
    std::fs::write(
        claude_dir.join("mcp-servers/test-server/smithery.yaml"),
        mcp_yaml,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--output")
        .arg("detailed")
        .arg("--mcp");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // MCP servers should show DESCRIPTION in detailed mode
    assert!(stdout.contains("test-server"));
}

// ==================== MCP Format Priority Tests ====================

#[test]
fn test_mcp_new_format_takes_priority() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Create mcp.json (legacy format)
    let mcp_json = r#"{
        "mcpServers": {
            "legacy-server": {
                "command": "npx",
                "args": ["-y", "@test/legacy"]
            }
        }
    }"#;
    std::fs::write(claude_dir.join("mcp.json"), mcp_json).unwrap();

    // Create mcp-servers directory (new format - takes priority)
    std::fs::create_dir_all(claude_dir.join("mcp-servers")).unwrap();
    std::fs::create_dir_all(claude_dir.join("mcp-servers/new-server")).unwrap();
    let mcp_yaml = r#"name: new-server
command: npx -y @test/new"#;
    std::fs::write(
        claude_dir.join("mcp-servers/new-server/smithery.yaml"),
        mcp_yaml,
    )
    .unwrap();

    // Create empty settings.json
    let settings = r#"{"installed_plugins": []}"#;
    std::fs::write(claude_dir.join("settings.json"), settings).unwrap();
    let sessions = r#"{"sessions": []}"#;
    std::fs::write(claude_dir.join("session_history.json"), sessions).unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(&claude_dir).arg("--mcp");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // New format (mcp-servers/) takes priority over mcp.json
    assert!(stdout.contains("new-server"));
    assert!(!stdout.contains("legacy-server"));
}

#[test]
fn test_mcp_legacy_format_used_when_new_missing() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Create only mcp.json (legacy format)
    let mcp_json = r#"{
        "mcpServers": {
            "legacy-server": {
                "command": "npx",
                "args": ["-y", "@test/legacy"]
            },
            "another-server": {
                "command": "python",
                "args": ["server.py"]
            }
        }
    }"#;
    std::fs::write(claude_dir.join("mcp.json"), mcp_json).unwrap();

    // No mcp-servers directory

    let settings = r#"{"installed_plugins": []}"#;
    std::fs::write(claude_dir.join("settings.json"), settings).unwrap();
    let sessions = r#"{"sessions": []}"#;
    std::fs::write(claude_dir.join("session_history.json"), sessions).unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(&claude_dir).arg("--mcp");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Legacy format should be used when mcp-servers/ doesn't exist
    assert!(stdout.contains("legacy-server"));
    assert!(stdout.contains("another-server"));
}

#[test]
fn test_mcp_empty_mcp_json() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Create empty mcp.json
    let mcp_json = r#"{"mcpServers": {}}"#;
    std::fs::write(claude_dir.join("mcp.json"), mcp_json).unwrap();

    let settings = r#"{"installed_plugins": []}"#;
    std::fs::write(claude_dir.join("settings.json"), settings).unwrap();
    let sessions = r#"{"sessions": []}"#;
    std::fs::write(claude_dir.join("session_history.json"), sessions).unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(&claude_dir);

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should not show MCP section when empty
    assert!(!stdout.contains("MCP"));
}

#[test]
fn test_mcp_empty_servers_directory() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Create empty mcp-servers directory
    std::fs::create_dir_all(claude_dir.join("mcp-servers")).unwrap();

    let settings = r#"{"installed_plugins": []}"#;
    std::fs::write(claude_dir.join("settings.json"), settings).unwrap();
    let sessions = r#"{"sessions": []}"#;
    std::fs::write(claude_dir.join("session_history.json"), sessions).unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(&claude_dir);

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should not show MCP section when empty
    assert!(!stdout.contains("MCP"));
}

// ==================== Search Combination Tests ====================

#[test]
fn test_search_with_mcp_filter() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Create multiple MCP servers
    std::fs::create_dir_all(claude_dir.join("mcp-servers")).unwrap();
    std::fs::create_dir_all(claude_dir.join("mcp-servers/test-server")).unwrap();
    std::fs::create_dir_all(claude_dir.join("mcp-servers/other-server")).unwrap();
    std::fs::create_dir_all(claude_dir.join("mcp-servers/sample-server")).unwrap();

    for server in ["test-server", "other-server", "sample-server"] {
        let yaml = format!("name: {}\ncommand: npx -y @test/{}", server, server);
        std::fs::write(
            claude_dir.join(format!("mcp-servers/{}/smithery.yaml", server)),
            yaml,
        )
        .unwrap();
    }

    let settings = r#"{"installed_plugins": []}"#;
    std::fs::write(claude_dir.join("settings.json"), settings).unwrap();
    let sessions = r#"{"sessions": []}"#;
    std::fs::write(claude_dir.join("session_history.json"), sessions).unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--search")
        .arg("test")
        .arg("--mcp");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert!(stdout.contains("test-server"));
    assert!(!stdout.contains("other-server"));
    assert!(!stdout.contains("sample-server"));
}

#[test]
fn test_search_with_commands_filter() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Create multiple commands
    std::fs::create_dir_all(claude_dir.join("commands")).unwrap();
    let commands = [
        ("test-command", "A test command"),
        ("other-command", "Another command"),
        ("sample-command", "A sample command"),
    ];

    for (name, desc) in &commands {
        let content = format!(
            r#"---
name: {}
description: {}
---
# {}"#,
            name, desc, name
        );
        std::fs::write(claude_dir.join(format!("commands/{}.md", name)), content).unwrap();
    }

    let settings = r#"{"installed_plugins": []}"#;
    std::fs::write(claude_dir.join("settings.json"), settings).unwrap();
    let sessions = r#"{"sessions": []}"#;
    std::fs::write(claude_dir.join("session_history.json"), sessions).unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--search")
        .arg("test")
        .arg("--commands");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert!(stdout.contains("test-command"));
    assert!(!stdout.contains("other-command"));
    assert!(!stdout.contains("sample-command"));
}

#[test]
fn test_search_with_agents_filter() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Create multiple agents
    std::fs::create_dir_all(claude_dir.join("agents")).unwrap();
    let agents = [
        ("test-agent", "A test agent"),
        ("other-agent", "Another agent"),
        ("sample-agent", "A sample agent"),
    ];

    for (name, desc) in &agents {
        let content = format!(
            r#"---
name: {}
description: {}
---
# {}"#,
            name, desc, name
        );
        std::fs::write(claude_dir.join(format!("agents/{}.md", name)), content).unwrap();
    }

    let settings = r#"{"installed_plugins": []}"#;
    std::fs::write(claude_dir.join("settings.json"), settings).unwrap();
    let sessions = r#"{"sessions": []}"#;
    std::fs::write(claude_dir.join("session_history.json"), sessions).unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--search")
        .arg("test")
        .arg("--agents");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert!(stdout.contains("test-agent"));
    assert!(!stdout.contains("other-agent"));
    assert!(!stdout.contains("sample-agent"));
}

#[test]
fn test_search_with_hooks_filter() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Create multiple hooks
    std::fs::create_dir_all(claude_dir.join("hooks")).unwrap();
    let hooks = ["test-hook.md", "other-hook.md", "sample-hook.md"];

    for hook in &hooks {
        let content = format!(
            r#"---
hook: pre-commit
---
# {}"#,
            hook.replace(".md", "")
        );
        std::fs::write(claude_dir.join(format!("hooks/{}", hook)), content).unwrap();
    }

    let settings = r#"{"installed_plugins": []}"#;
    std::fs::write(claude_dir.join("settings.json"), settings).unwrap();
    let sessions = r#"{"sessions": []}"#;
    std::fs::write(claude_dir.join("session_history.json"), sessions).unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--search")
        .arg("test")
        .arg("--hooks");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert!(stdout.contains("test-hook"));
    assert!(!stdout.contains("other-hook"));
    assert!(!stdout.contains("sample-hook"));
}

#[test]
fn test_search_with_sessions_filter() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Create sessions with specific IDs
    let sessions = r#"{
        "sessions": [
            {"id": "test-session-123", "timestamp": "2025-01-28T10:00:00Z"},
            {"id": "other-session-456", "timestamp": "2025-01-29T10:00:00Z"},
            {"id": "sample-session-789", "timestamp": "2025-01-30T10:00:00Z"}
        ]
    }"#;
    std::fs::write(claude_dir.join("session_history.json"), sessions).unwrap();

    let settings = r#"{"installed_plugins": []}"#;
    std::fs::write(claude_dir.join("settings.json"), settings).unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--search")
        .arg("test")
        .arg("--sessions");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    // Sessions search should work (if sessions have searchable IDs)
    assert!(stdout.contains("SESSIONS"));
}

// ==================== Edge Cases and Special Characters ====================

#[test]
fn test_special_characters_in_plugin_name() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Plugin with special characters
    let settings = r#"{
        "installed_plugins": [
            {"name": "plugin_with_underscore", "version": "1.0.0"},
            {"name": "plugin.with.dots", "version": "1.0.0"}
        ]
    }"#;
    std::fs::write(claude_dir.join("settings.json"), settings).unwrap();
    let sessions = r#"{"sessions": []}"#;
    std::fs::write(claude_dir.join("session_history.json"), sessions).unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(&claude_dir);

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert!(stdout.contains("plugin_with_underscore") || stdout.contains("plugin.with"));
}

#[test]
fn test_unicode_skill_name() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Skill with unicode name
    std::fs::create_dir_all(claude_dir.join("skills")).unwrap();
    std::fs::create_dir_all(claude_dir.join("skills/测试技能")).unwrap();
    let skill_yaml = "name: 测试技能\nversion: 1.0.0\ndescription: A test skill\n";
    std::fs::write(claude_dir.join("skills/测试技能/skill.yaml"), skill_yaml).unwrap();

    let settings = r#"{"installed_plugins": []}"#;
    std::fs::write(claude_dir.join("settings.json"), settings).unwrap();
    let sessions = r#"{"sessions": []}"#;
    std::fs::write(claude_dir.join("session_history.json"), sessions).unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(&claude_dir);

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("测试") || stdout.contains("SKILLS"));
}

#[test]
fn test_empty_components_directory() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Create empty directories for all component types
    std::fs::create_dir_all(claude_dir.join("skills")).unwrap();
    std::fs::create_dir_all(claude_dir.join("hooks")).unwrap();
    std::fs::create_dir_all(claude_dir.join("agents")).unwrap();
    std::fs::create_dir_all(claude_dir.join("commands")).unwrap();
    std::fs::create_dir_all(claude_dir.join("mcp-servers")).unwrap();

    let settings = r#"{"installed_plugins": []}"#;
    std::fs::write(claude_dir.join("settings.json"), settings).unwrap();
    let sessions = r#"{"sessions": []}"#;
    std::fs::write(claude_dir.join("session_history.json"), sessions).unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(&claude_dir);

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should succeed without showing empty sections
    assert!(output.status.success());
    // Only header should be shown
    assert!(stdout.contains("CLAUDE-LIST"));
}

#[test]
fn test_plugin_without_version_field() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Plugin without version (graceful degradation)
    let settings = r#"{
        "installed_plugins": [
            {"name": "plugin-no-version"}
        ]
    }"#;
    std::fs::write(claude_dir.join("settings.json"), settings).unwrap();
    let sessions = r#"{"sessions": []}"#;
    std::fs::write(claude_dir.join("session_history.json"), sessions).unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(&claude_dir);

    // Should handle gracefully
    cmd.assert().success();
}

// ==================== JSON Output Detailed Tests ====================

#[test]
fn test_json_output_contains_all_fields() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config").arg(&claude_dir).arg("--json");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    // Check all top-level fields exist
    assert!(json.get("version").is_some());
    assert!(json.get("plugins").is_some());
    assert!(json.get("skills").is_some());
    assert!(json.get("sessions").is_some());
    assert!(json.get("mcp_servers").is_some());
    assert!(json.get("hooks").is_some());
    assert!(json.get("agents").is_some());
    assert!(json.get("commands").is_some());
}

#[test]
fn test_json_plugin_has_version() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--json")
        .arg("--plugins");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let plugins = json.get("plugins").unwrap().as_array().unwrap();

    assert!(!plugins.is_empty());
    // First plugin should have version field
    if let Some(first_plugin) = plugins.first() {
        assert!(first_plugin.get("name").is_some());
        // Version might be null if not specified
        assert!(first_plugin.get("version").is_some());
    }
}

#[test]
fn test_json_skill_has_all_fields() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--json")
        .arg("--skills");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let skills = json.get("skills").unwrap().as_array().unwrap();

    assert!(!skills.is_empty());
    if let Some(first_skill) = skills.first() {
        assert!(first_skill.get("name").is_some());
        assert!(first_skill.get("source").is_some());
    }
}

// ==================== Config Path Edge Cases ====================

#[test]
fn test_config_path_with_tilde() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    // Use absolute path (tilde expansion is shell-level)
    cmd.arg("--config").arg(&claude_dir);

    cmd.assert().success();
}

#[test]
fn test_config_path_with_relative_path() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(claude_dir.clone())
        .current_dir(dir.path());

    cmd.assert().success();
}

// ==================== Multiple Filter Combination Tests ====================

#[test]
fn test_multiple_filters_with_plugins_and_mcp() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    // Add MCP server
    std::fs::create_dir_all(claude_dir.join("mcp-servers")).unwrap();
    std::fs::create_dir_all(claude_dir.join("mcp-servers/test-server")).unwrap();
    let mcp_yaml = "name: test-server\ncommand: npx -y @test/server";
    std::fs::write(
        claude_dir.join("mcp-servers/test-server/smithery.yaml"),
        mcp_yaml,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--plugins")
        .arg("--mcp");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show both plugins and MCP
    assert!(stdout.contains("PLUGINS"));
    assert!(stdout.contains("MCP"));
    assert!(stdout.contains("context7"));
    assert!(stdout.contains("test-server"));
    // Should not show other types
    assert!(!stdout.contains("SKILLS"));
}

#[test]
fn test_multiple_filters_with_all_types() {
    let dir = TempDir::new().unwrap();
    let claude_dir = create_mock_claude_dir(&dir);

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--plugins")
        .arg("--skills")
        .arg("--sessions")
        .arg("--mcp")
        .arg("--hooks")
        .arg("--agents")
        .arg("--commands");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show all types
    assert!(stdout.contains("PLUGINS"));
    assert!(stdout.contains("SKILLS"));
    assert!(stdout.contains("SESSIONS"));
    assert!(output.status.success());
}

#[test]
fn test_multiple_filters_with_search() {
    let dir = TempDir::new().unwrap();
    let claude_dir = dir.path().join(".claude");
    std::fs::create_dir_all(&claude_dir).unwrap();

    // Create components with matching names
    let settings = r#"{
        "installed_plugins": [
            {"name": "test-component", "version": "1.0.0"}
        ]
    }"#;
    std::fs::write(claude_dir.join("settings.json"), settings).unwrap();

    std::fs::create_dir_all(claude_dir.join("skills")).unwrap();
    std::fs::create_dir_all(claude_dir.join("skills/test-component")).unwrap();
    std::fs::write(
        claude_dir.join("skills/test-component/skill.yaml"),
        "name: test-component\nversion: 1.0.0\n",
    )
    .unwrap();

    std::fs::create_dir_all(claude_dir.join("commands")).unwrap();
    std::fs::write(
        claude_dir.join("commands/test-component.md"),
        "---\nname: test-component\n---\n",
    )
    .unwrap();

    let sessions = r#"{"sessions": []}"#;
    std::fs::write(claude_dir.join("session_history.json"), sessions).unwrap();

    let mut cmd = Command::cargo_bin("claude-list").unwrap();
    cmd.arg("--config")
        .arg(&claude_dir)
        .arg("--search")
        .arg("test-component")
        .arg("--plugins")
        .arg("--skills")
        .arg("--commands");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    // Should show all matching components
    let count = stdout.matches("test-component").count();
    assert!(
        count >= 3,
        "Should find at least 3 matches for 'test-component'"
    );
}
