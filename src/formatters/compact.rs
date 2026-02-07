//! Compact output formatter

use crate::info::ClaudeInfo;
use crate::output::{colored_string, ColorScheme, ColorSettings, ComponentType};
use std::io::Write;

pub fn format_compact(
    info: &ClaudeInfo,
    color_scheme: &ColorScheme,
    color_settings: &ColorSettings,
    output: &mut dyn Write,
) -> std::io::Result<()> {
    writeln!(output, "CLAUDE-LIST v{}", info.version)?;
    writeln!(output)?;
    writeln!(output, "CONFIG: {}", info.config_dir.display())?;
    writeln!(output)?;

    // PLUGINS
    if !info.plugins.is_empty() {
        writeln!(output, "PLUGINS    {} installed", info.plugins.len())?;
        for plugin in &info.plugins {
            let colored = colored_string(
                &plugin.name,
                ComponentType::Plugin,
                color_scheme,
                color_settings,
            );
            writeln!(output, "  {}", colored)?;
        }
        writeln!(output)?;
    }

    // SKILLS
    if !info.skills.is_empty() {
        writeln!(output, "SKILLS     {} available", info.skills.len())?;
        for skill in &info.skills {
            let colored = colored_string(
                &skill.name,
                ComponentType::Skill,
                color_scheme,
                color_settings,
            );
            writeln!(output, "  {}", colored)?;
        }
        writeln!(output)?;
    }

    // SESSIONS
    if info.sessions.count > 0 {
        writeln!(output, "SESSIONS   {} recorded", info.sessions.count)?;
        writeln!(output)?;
    }

    // MCP
    if !info.mcp_servers.is_empty() {
        writeln!(output, "MCP        {} servers", info.mcp_servers.len())?;
        for mcp in &info.mcp_servers {
            let colored =
                colored_string(&mcp.name, ComponentType::Mcp, color_scheme, color_settings);
            writeln!(output, " {}", colored)?;
        }
        writeln!(output)?;
    }

    // HOOKS
    if !info.hooks.is_empty() {
        writeln!(output, "HOOKS      {} configured", info.hooks.len())?;
        for hook in &info.hooks {
            let colored = colored_string(
                &hook.name,
                ComponentType::Hook,
                color_scheme,
                color_settings,
            );
            writeln!(output, "  {}", colored)?;
        }
        writeln!(output)?;
    }

    // AGENTS
    if !info.agents.is_empty() {
        writeln!(output, "AGENTS     {} defined", info.agents.len())?;
        for agent in &info.agents {
            let colored = colored_string(
                &agent.name,
                ComponentType::Agent,
                color_scheme,
                color_settings,
            );
            writeln!(output, "  {}", colored)?;
        }
        writeln!(output)?;
    }

    // COMMANDS
    if !info.commands.is_empty() {
        writeln!(output, "COMMANDS   {} available", info.commands.len())?;
        for cmd in &info.commands {
            let colored = colored_string(
                &cmd.name,
                ComponentType::Command,
                color_scheme,
                color_settings,
            );
            writeln!(output, "  /{}", colored)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::info::{PluginInfo, SessionInfo, SkillInfo, SkillLocation, Source};
    use std::path::PathBuf;

    #[test]
    fn test_format_compact() {
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
            skills: vec![SkillInfo {
                name: "test-skill".to_string(),
                version: Some("1.0.0".to_string()),
                source: Source::Official,
                path: PathBuf::from("/test/.claude/skills/test-skill"),
                description: Some("A test skill".to_string()),
                location_type: SkillLocation::Global,
            }],
            sessions: SessionInfo {
                count: 42,
                last_session: Some("2025-01-29T10:00:00Z".to_string()),
            },
            mcp_servers: vec![],
            hooks: vec![],
            agents: vec![],
            commands: vec![],
        };

        let color_scheme = ColorScheme::default();
        let color_settings = ColorSettings::from_env();

        let mut buffer = Vec::new();
        format_compact(&info, &color_scheme, &color_settings, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("CLAUDE-LIST v0.1.0"));
        assert!(output.contains("PLUGINS"));
        assert!(output.contains("context7"));
        assert!(output.contains("SKILLS"));
        assert!(output.contains("test-skill"));
        assert!(output.contains("SESSIONS"));
        assert!(output.contains("42"));
    }

    #[test]
    fn test_format_compact_empty() {
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

        let color_scheme = ColorScheme::default();
        let color_settings = ColorSettings::from_env();

        let mut buffer = Vec::new();
        format_compact(&info, &color_scheme, &color_settings, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("CLAUDE-LIST v0.1.0"));
        assert!(output.contains("CONFIG: /test/.claude"));
        // Should not contain any section headers when empty
        assert!(!output.contains("PLUGINS"));
        assert!(!output.contains("SKILLS"));
        assert!(!output.contains("SESSIONS"));
    }

    #[test]
    fn test_format_compact_with_mcp() {
        let info = ClaudeInfo {
            version: "0.1.0".to_string(),
            config_dir: PathBuf::from("/test/.claude"),
            plugins: vec![],
            skills: vec![],
            sessions: SessionInfo {
                count: 0,
                last_session: None,
            },
            mcp_servers: vec![crate::info::McpInfo {
                name: "test-mcp".to_string(),
                status: "connected".to_string(),
                command: Some("npx".to_string()),
                path: PathBuf::from("/test/.claude/mcp.json"),
                description: None,
            }],
            hooks: vec![],
            agents: vec![],
            commands: vec![],
        };

        let color_scheme = ColorScheme::default();
        let color_settings = ColorSettings::from_env();

        let mut buffer = Vec::new();
        format_compact(&info, &color_scheme, &color_settings, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("MCP"));
        assert!(output.contains("1 servers"));
        assert!(output.contains("test-mcp"));
    }

    #[test]
    fn test_format_compact_with_hooks() {
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
            hooks: vec![crate::info::HookInfo {
                name: "pre-commit".to_string(),
                hook_type: "pre-commit".to_string(),
                path: PathBuf::from("/test/.claude/hooks/pre-commit.md"),
                description: None,
            }],
            agents: vec![],
            commands: vec![],
        };

        let color_scheme = ColorScheme::default();
        let color_settings = ColorSettings::from_env();

        let mut buffer = Vec::new();
        format_compact(&info, &color_scheme, &color_settings, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("HOOKS"));
        assert!(output.contains("1 configured"));
        assert!(output.contains("pre-commit"));
    }

    #[test]
    fn test_format_compact_with_agents() {
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
            agents: vec![crate::info::AgentInfo {
                name: "test-agent".to_string(),
                description: Some("A test agent".to_string()),
                path: PathBuf::from("/test/.claude/agents/test-agent.md"),
            }],
            commands: vec![],
        };

        let color_scheme = ColorScheme::default();
        let color_settings = ColorSettings::from_env();

        let mut buffer = Vec::new();
        format_compact(&info, &color_scheme, &color_settings, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("AGENTS"));
        assert!(output.contains("1 defined"));
        assert!(output.contains("test-agent"));
    }

    #[test]
    fn test_format_compact_with_commands() {
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
            commands: vec![crate::info::CommandInfo {
                name: "test-command".to_string(),
                description: Some("A test command".to_string()),
                allowed_tools: None,
                argument_hint: None,
                path: PathBuf::from("/test/.claude/commands/test-command.md"),
            }],
        };

        let color_scheme = ColorScheme::default();
        let color_settings = ColorSettings::from_env();

        let mut buffer = Vec::new();
        format_compact(&info, &color_scheme, &color_settings, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("COMMANDS"));
        assert!(output.contains("1 available"));
        assert!(output.contains("/test-command")); // Commands are prefixed with /
    }

    #[test]
    fn test_format_compact_multiple_items() {
        let info = ClaudeInfo {
            version: "0.1.0".to_string(),
            config_dir: PathBuf::from("/test/.claude"),
            plugins: vec![
                PluginInfo {
                    name: "plugin1".to_string(),
                    version: Some("1.0.0".to_string()),
                    source: Source::Official,
                    path: PathBuf::from("/test"),
                    description: None,
                },
                PluginInfo {
                    name: "plugin2".to_string(),
                    version: Some("2.0.0".to_string()),
                    source: Source::Official,
                    path: PathBuf::from("/test"),
                    description: None,
                },
            ],
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

        let color_scheme = ColorScheme::default();
        let color_settings = ColorSettings::from_env();

        let mut buffer = Vec::new();
        format_compact(&info, &color_scheme, &color_settings, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("2 installed"));
        assert!(output.contains("plugin1"));
        assert!(output.contains("plugin2"));
    }

    #[test]
    fn test_format_compact_no_color() {
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

        let color_scheme = ColorScheme::default();
        let color_settings = ColorSettings {
            enabled: false,
            force_colors: false,
        };

        let mut buffer = Vec::new();
        format_compact(&info, &color_scheme, &color_settings, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        // Should not contain ANSI escape codes
        assert!(!output.contains("\x1b["));
        assert!(output.contains("test-plugin"));
    }
}
