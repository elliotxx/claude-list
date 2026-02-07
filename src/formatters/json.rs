//! JSON output formatter

use crate::formatters::Formatter;
use crate::info::ClaudeInfo;
use std::io::Write;

pub struct JsonFormatter;

impl Formatter for JsonFormatter {
    fn format(&self, info: &ClaudeInfo, output: &mut dyn Write) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(info)?;
        writeln!(output, "{}", json)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::info::{
        AgentInfo, CommandInfo, HookInfo, McpInfo, PluginInfo, SessionInfo, SkillInfo,
        SkillLocation, Source,
    };
    use std::path::PathBuf;

    #[test]
    fn test_json_formatter_basic() {
        let info = ClaudeInfo {
            version: "0.1.0".to_string(),
            config_dir: PathBuf::from("/test/.claude"),
            plugins: vec![PluginInfo {
                name: "context7".to_string(),
                version: Some("2.1.0".to_string()),
                source: Source::Official,
                path: PathBuf::from("/test/.claude/settings.json"),
                description: None,
            }],
            skills: vec![],
            sessions: SessionInfo {
                count: 0,
                last_session: None,
            },
            mcp_servers: vec![],
            hooks: vec![],
            agents: vec![],
            commands: vec![],
        };

        let formatter = JsonFormatter;
        let mut buffer = Vec::new();
        formatter.format(&info, &mut buffer).unwrap();

        let output = String::from_utf8(buffer).unwrap();
        let json: serde_json::Value = serde_json::from_str(&output).unwrap();

        assert_eq!(json["version"], "0.1.0");
        assert_eq!(json["plugins"].as_array().unwrap().len(), 1);
        assert_eq!(json["plugins"][0]["name"], "context7");
        assert_eq!(json["plugins"][0]["version"], "2.1.0");
        assert_eq!(json["plugins"][0]["source"], "official");
    }

    #[test]
    fn test_json_formatter_full_info() {
        let info = ClaudeInfo {
            version: "0.1.0".to_string(),
            config_dir: PathBuf::from("/test/.claude"),
            plugins: vec![PluginInfo {
                name: "test-plugin".to_string(),
                version: Some("1.0.0".to_string()),
                source: Source::Official,
                path: PathBuf::from("/test/plugins"),
                description: Some("A test plugin".to_string()),
            }],
            skills: vec![SkillInfo {
                name: "test-skill".to_string(),
                version: Some("1.0.0".to_string()),
                source: Source::Official,
                path: PathBuf::from("/test/skills/test-skill"),
                description: Some("A test skill".to_string()),
                location_type: SkillLocation::Global,
            }],
            sessions: SessionInfo {
                count: 42,
                last_session: Some("2025-01-29T10:00:00Z".to_string()),
            },
            mcp_servers: vec![McpInfo {
                name: "test-mcp".to_string(),
                status: "connected".to_string(),
                command: Some("npx".to_string()),
                path: PathBuf::from("/test/mcp.json"),
                description: Some("A test MCP server".to_string()),
            }],
            hooks: vec![HookInfo {
                name: "pre-commit".to_string(),
                hook_type: "pre-commit".to_string(),
                path: PathBuf::from("/test/hooks/pre-commit.md"),
                description: Some("A pre-commit hook".to_string()),
            }],
            agents: vec![AgentInfo {
                name: "test-agent".to_string(),
                description: Some("A test agent".to_string()),
                path: PathBuf::from("/test/agents/test-agent.md"),
            }],
            commands: vec![CommandInfo {
                name: "test-command".to_string(),
                description: Some("A test command".to_string()),
                allowed_tools: Some("Bash, Read".to_string()),
                argument_hint: Some("[test]".to_string()),
                path: PathBuf::from("/test/commands/test-command.md"),
            }],
        };

        let formatter = JsonFormatter;
        let mut buffer = Vec::new();
        formatter.format(&info, &mut buffer).unwrap();

        let output = String::from_utf8(buffer).unwrap();
        let json: serde_json::Value = serde_json::from_str(&output).unwrap();

        // Verify all sections are present
        assert!(json.get("version").is_some());
        assert!(json.get("config_dir").is_some());
        assert!(json.get("plugins").is_some());
        assert!(json.get("skills").is_some());
        assert!(json.get("sessions").is_some());
        assert!(json.get("mcp_servers").is_some());
        assert!(json.get("hooks").is_some());
        assert!(json.get("agents").is_some());
        assert!(json.get("commands").is_some());

        // Verify counts
        assert_eq!(json["plugins"].as_array().unwrap().len(), 1);
        assert_eq!(json["skills"].as_array().unwrap().len(), 1);
        assert_eq!(json["mcp_servers"].as_array().unwrap().len(), 1);
        assert_eq!(json["hooks"].as_array().unwrap().len(), 1);
        assert_eq!(json["agents"].as_array().unwrap().len(), 1);
        assert_eq!(json["commands"].as_array().unwrap().len(), 1);

        // Verify session info
        assert_eq!(json["sessions"]["count"], 42);
        assert_eq!(json["sessions"]["last_session"], "2025-01-29T10:00:00Z");

        // Verify skill location serialization
        assert_eq!(json["skills"][0]["location_type"]["type"], "Global");

        // Verify plugin description
        assert_eq!(json["plugins"][0]["description"], "A test plugin");

        // Verify command fields
        assert_eq!(json["commands"][0]["allowed_tools"], "Bash, Read");
        assert_eq!(json["commands"][0]["argument_hint"], "[test]");
    }

    #[test]
    fn test_json_formatter_empty_info() {
        let info = ClaudeInfo {
            version: "0.1.0".to_string(),
            config_dir: PathBuf::from("/test/.claude"),
            plugins: vec![],
            skills: vec![],
            sessions: SessionInfo {
                count: 0,
                last_session: None,
            },
            mcp_servers: vec![],
            hooks: vec![],
            agents: vec![],
            commands: vec![],
        };

        let formatter = JsonFormatter;
        let mut buffer = Vec::new();
        formatter.format(&info, &mut buffer).unwrap();

        let output = String::from_utf8(buffer).unwrap();
        let json: serde_json::Value = serde_json::from_str(&output).unwrap();

        // Verify empty arrays
        assert_eq!(json["plugins"].as_array().unwrap().len(), 0);
        assert_eq!(json["skills"].as_array().unwrap().len(), 0);
        assert_eq!(json["mcp_servers"].as_array().unwrap().len(), 0);
        assert_eq!(json["hooks"].as_array().unwrap().len(), 0);
        assert_eq!(json["agents"].as_array().unwrap().len(), 0);
        assert_eq!(json["commands"].as_array().unwrap().len(), 0);

        // Verify session info
        assert_eq!(json["sessions"]["count"], 0);
        assert!(json["sessions"]["last_session"].is_null());
    }

    #[test]
    fn test_json_formatter_skill_location_plugin() {
        let info = ClaudeInfo {
            version: "0.1.0".to_string(),
            config_dir: PathBuf::from("/test/.claude"),
            plugins: vec![],
            skills: vec![SkillInfo {
                name: "plugin-skill".to_string(),
                version: Some("1.0.0".to_string()),
                source: Source::Official,
                path: PathBuf::from("/test/plugin/skills/plugin-skill"),
                description: None,
                location_type: SkillLocation::Plugin {
                    plugin_name: Some("test-plugin".to_string()),
                },
            }],
            sessions: SessionInfo {
                count: 0,
                last_session: None,
            },
            mcp_servers: vec![],
            hooks: vec![],
            agents: vec![],
            commands: vec![],
        };

        let formatter = JsonFormatter;
        let mut buffer = Vec::new();
        formatter.format(&info, &mut buffer).unwrap();

        let output = String::from_utf8(buffer).unwrap();
        let json: serde_json::Value = serde_json::from_str(&output).unwrap();

        assert_eq!(json["skills"][0]["location_type"]["type"], "Plugin");
        assert_eq!(
            json["skills"][0]["location_type"]["plugin_name"],
            "test-plugin"
        );
    }

    #[test]
    fn test_json_formatter_third_party_source() {
        let info = ClaudeInfo {
            version: "0.1.0".to_string(),
            config_dir: PathBuf::from("/test/.claude"),
            plugins: vec![PluginInfo {
                name: "custom-plugin".to_string(),
                version: Some("1.0.0".to_string()),
                source: Source::ThirdParty,
                path: PathBuf::from("/test/plugins"),
                description: None,
            }],
            skills: vec![],
            sessions: SessionInfo {
                count: 0,
                last_session: None,
            },
            mcp_servers: vec![],
            hooks: vec![],
            agents: vec![],
            commands: vec![],
        };

        let formatter = JsonFormatter;
        let mut buffer = Vec::new();
        formatter.format(&info, &mut buffer).unwrap();

        let output = String::from_utf8(buffer).unwrap();
        let json: serde_json::Value = serde_json::from_str(&output).unwrap();

        assert_eq!(json["plugins"][0]["source"], "third-party");
    }

    #[test]
    fn test_json_formatter_output_is_valid_json() {
        let info = ClaudeInfo {
            version: "0.1.0".to_string(),
            config_dir: PathBuf::from("/test/.claude"),
            plugins: vec![PluginInfo {
                name: "test-plugin".to_string(),
                version: Some("1.0.0".to_string()),
                source: Source::Official,
                path: PathBuf::from("/test"),
                description: None,
            }],
            skills: vec![],
            sessions: SessionInfo {
                count: 0,
                last_session: None,
            },
            mcp_servers: vec![],
            hooks: vec![],
            agents: vec![],
            commands: vec![],
        };

        let formatter = JsonFormatter;
        let mut buffer = Vec::new();
        formatter.format(&info, &mut buffer).unwrap();

        let output = String::from_utf8(buffer).unwrap();

        // Verify the output is valid JSON
        let _: serde_json::Value = serde_json::from_str(&output).unwrap();

        // Verify it's pretty-printed (contains newlines and indentation)
        assert!(output.contains('\n'));
        assert!(output.contains("  "));
    }

    #[test]
    fn test_json_formatter_none_values_serialization() {
        let info = ClaudeInfo {
            version: "0.1.0".to_string(),
            config_dir: PathBuf::from("/test/.claude"),
            plugins: vec![PluginInfo {
                name: "test-plugin".to_string(),
                version: None,
                source: Source::Official,
                path: PathBuf::from("/test"),
                description: None,
            }],
            skills: vec![],
            sessions: SessionInfo {
                count: 0,
                last_session: None,
            },
            mcp_servers: vec![],
            hooks: vec![],
            agents: vec![],
            commands: vec![],
        };

        let formatter = JsonFormatter;
        let mut buffer = Vec::new();
        formatter.format(&info, &mut buffer).unwrap();

        let output = String::from_utf8(buffer).unwrap();
        let json: serde_json::Value = serde_json::from_str(&output).unwrap();

        // None values should be serialized as null
        assert!(json["plugins"][0]["version"].is_null());
        assert!(json["plugins"][0]["description"].is_null());
        assert!(json["sessions"]["last_session"].is_null());
    }
}
