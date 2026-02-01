//! Detailed output formatter (for -l flag)
//!
//! Output format: NAME, SOURCE, DESCRIPTION instead of NAME, VERSION, SOURCE, PATH

use crate::info::{ClaudeInfo, DescriptionProvider};
use crate::output::{
    truncate_with_ellipsis, write_colored_padded_field, Alignment, ColorScheme, ColorSettings,
    ComponentType,
};
use std::io::Write;

const NAME_WIDTH: usize = 30;
const SOURCE_WIDTH: usize = 15;
const STATUS_WIDTH: usize = 18;
const TYPE_WIDTH: usize = 18;
const DESC_WIDTH: usize = 50;

pub fn format_detailed(
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
        writeln!(output, "  {:<30} {:<15} PATH", "NAME", "SOURCE")?;
        writeln!(
            output,
            "  {:<30} {:<15} {}",
            "-".repeat(NAME_WIDTH),
            "-".repeat(SOURCE_WIDTH),
            "-".repeat(40)
        )?;
        for plugin in &info.plugins {
            let source = match plugin.source {
                crate::info::Source::Official => "official",
                crate::info::Source::ThirdParty => "third-party",
            };
            write!(output, "  ")?;
            write_colored_padded_field(
                output,
                &plugin.name,
                ComponentType::Plugin,
                color_scheme,
                color_settings,
                NAME_WIDTH,
                Alignment::Left,
            )?;
            write_colored_padded_field(
                output,
                source,
                ComponentType::Plugin,
                color_scheme,
                color_settings,
                SOURCE_WIDTH,
                Alignment::Left,
            )?;
            write!(output, " {}", plugin.path.display())?;
            writeln!(output)?;
        }
        writeln!(output)?;
    }

    // SKILLS
    if !info.skills.is_empty() {
        writeln!(output, "SKILLS     {} available", info.skills.len())?;
        writeln!(output, "  {:<30} {:<15} DESCRIPTION", "NAME", "SOURCE")?;
        writeln!(
            output,
            "  {:<30} {:<15} {}",
            "-".repeat(NAME_WIDTH),
            "-".repeat(SOURCE_WIDTH),
            "-".repeat(DESC_WIDTH)
        )?;
        for skill in &info.skills {
            let source = match skill.source {
                crate::info::Source::Official => "official",
                crate::info::Source::ThirdParty => "third-party",
            };
            let description = skill.get_description().unwrap_or_default();
            write!(output, "  ")?;
            write_colored_padded_field(
                output,
                &skill.name,
                ComponentType::Skill,
                color_scheme,
                color_settings,
                NAME_WIDTH,
                Alignment::Left,
            )?;
            write_colored_padded_field(
                output,
                source,
                ComponentType::Skill,
                color_scheme,
                color_settings,
                SOURCE_WIDTH,
                Alignment::Left,
            )?;
            write!(output, " ")?;
            let truncated_desc = truncate_with_ellipsis(&description, DESC_WIDTH, "...");
            writeln!(output, "{}", truncated_desc)?;
        }
        writeln!(output)?;
    }

    // SESSIONS
    if info.sessions.count > 0 {
        writeln!(output, "SESSIONS   {} recorded", info.sessions.count)?;
        if let Some(ref last) = info.sessions.last_session {
            writeln!(output, "  Last session: {}", last)?;
        }
        writeln!(output)?;
    }

    // MCP
    if !info.mcp_servers.is_empty() {
        writeln!(output, "MCP        {} servers", info.mcp_servers.len())?;
        writeln!(output, "  {:<30} {:<18} DESCRIPTION", "NAME", "STATUS")?;
        writeln!(
            output,
            "  {:<30} {:<18} {}",
            "-".repeat(NAME_WIDTH),
            "-".repeat(STATUS_WIDTH),
            "-".repeat(DESC_WIDTH)
        )?;
        for mcp in &info.mcp_servers {
            let description = mcp.get_description().unwrap_or_default();
            write!(output, "  ")?;
            write_colored_padded_field(
                output,
                &mcp.name,
                ComponentType::Mcp,
                color_scheme,
                color_settings,
                NAME_WIDTH,
                Alignment::Left,
            )?;
            write_colored_padded_field(
                output,
                &mcp.status,
                ComponentType::Mcp,
                color_scheme,
                color_settings,
                STATUS_WIDTH,
                Alignment::Left,
            )?;
            write!(output, " ")?;
            let truncated_desc = truncate_with_ellipsis(&description, DESC_WIDTH, "...");
            writeln!(output, "{}", truncated_desc)?;
        }
        writeln!(output)?;
    }

    // HOOKS
    if !info.hooks.is_empty() {
        writeln!(output, "HOOKS      {} configured", info.hooks.len())?;
        writeln!(output, "  {:<30} {:<18} DESCRIPTION", "NAME", "TYPE")?;
        writeln!(
            output,
            "  {:<30} {:<18} {}",
            "-".repeat(NAME_WIDTH),
            "-".repeat(TYPE_WIDTH),
            "-".repeat(DESC_WIDTH)
        )?;
        for hook in &info.hooks {
            let description = hook.get_description().unwrap_or_default();
            write!(output, "  ")?;
            write_colored_padded_field(
                output,
                &hook.name,
                ComponentType::Hook,
                color_scheme,
                color_settings,
                NAME_WIDTH,
                Alignment::Left,
            )?;
            write_colored_padded_field(
                output,
                &hook.hook_type,
                ComponentType::Hook,
                color_scheme,
                color_settings,
                TYPE_WIDTH,
                Alignment::Left,
            )?;
            write!(output, " ")?;
            let truncated_desc = truncate_with_ellipsis(&description, DESC_WIDTH, "...");
            writeln!(output, "{}", truncated_desc)?;
        }
        writeln!(output)?;
    }

    // AGENTS
    if !info.agents.is_empty() {
        writeln!(output, "AGENTS     {} defined", info.agents.len())?;
        writeln!(output, "  {:<30} DESCRIPTION", "NAME")?;
        writeln!(
            output,
            "  {:<30} {}",
            "-".repeat(NAME_WIDTH),
            "-".repeat(DESC_WIDTH)
        )?;
        for agent in &info.agents {
            let description = agent.get_description().unwrap_or_default();
            write!(output, "  ")?;
            write_colored_padded_field(
                output,
                &agent.name,
                ComponentType::Agent,
                color_scheme,
                color_settings,
                NAME_WIDTH,
                Alignment::Left,
            )?;
            write!(output, " ")?;
            let truncated_desc = truncate_with_ellipsis(&description, DESC_WIDTH, "...");
            writeln!(output, "{}", truncated_desc)?;
        }
        writeln!(output)?;
    }

    // COMMANDS
    if !info.commands.is_empty() {
        writeln!(output, "COMMANDS   {} available", info.commands.len())?;
        writeln!(output, "  {:<30} DESCRIPTION", "NAME")?;
        writeln!(
            output,
            "  {:<30} {}",
            "-".repeat(NAME_WIDTH),
            "-".repeat(DESC_WIDTH)
        )?;
        for cmd in &info.commands {
            let description = cmd.get_description().unwrap_or_default();
            write!(output, "  ")?;
            write_colored_padded_field(
                output,
                &cmd.name,
                ComponentType::Command,
                color_scheme,
                color_settings,
                NAME_WIDTH,
                Alignment::Left,
            )?;
            write!(output, " ")?;
            let truncated_desc = truncate_with_ellipsis(&description, DESC_WIDTH, "...");
            writeln!(output, "{}", truncated_desc)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::info::{
        AgentInfo, CommandInfo, HookInfo, McpInfo, PluginInfo, SessionInfo, SkillInfo, Source,
    };
    use std::path::PathBuf;

    #[test]
    fn test_format_detailed_plugins_output() {
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

        let color_scheme = ColorScheme::default();
        let color_settings = ColorSettings::force();

        let mut buffer = Vec::new();
        format_detailed(&info, &color_scheme, &color_settings, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        // Verify plugins format: NAME, SOURCE, PATH (no VERSION, no DESCRIPTION)
        assert!(output.contains("NAME"));
        assert!(output.contains("SOURCE"));
        assert!(output.contains("PATH"));
        assert!(output.contains("context7"));
        assert!(output.contains("official"));
        assert!(output.contains("settings.json")); // PATH should be shown
        assert!(!output.contains("2.1.0")); // VERSION should not appear
        assert!(!output.contains("Official plugin")); // DESCRIPTION should not appear
    }

    #[test]
    fn test_format_detailed_skills_output() {
        let info = ClaudeInfo {
            version: "0.1.0".to_string(),
            config_dir: PathBuf::from("/test/.claude"),
            plugins: vec![],
            skills: vec![SkillInfo {
                name: "test-skill".to_string(),
                version: Some("1.0.0".to_string()),
                source: Source::Official,
                path: PathBuf::from("/test/.claude/skills/test-skill"),
                description: Some("A test skill".to_string()),
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

        let color_scheme = ColorScheme::default();
        let color_settings = ColorSettings::force();

        let mut buffer = Vec::new();
        format_detailed(&info, &color_scheme, &color_settings, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("NAME"));
        assert!(output.contains("SOURCE"));
        assert!(output.contains("DESCRIPTION"));
        assert!(output.contains("test-skill"));
        assert!(output.contains("official"));
        assert!(output.contains("A test skill")); // Skill description
        assert!(!output.contains("1.0.0")); // VERSION should not appear
    }

    #[test]
    fn test_format_detailed_mcp_output() {
        let info = ClaudeInfo {
            version: "0.1.0".to_string(),
            config_dir: PathBuf::from("/test/.claude"),
            plugins: vec![],
            skills: vec![],
            sessions: SessionInfo {
                count: 0,
                last_session: None,
            },
            mcp_servers: vec![McpInfo {
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
        let color_settings = ColorSettings::force();

        let mut buffer = Vec::new();
        format_detailed(&info, &color_scheme, &color_settings, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("NAME"));
        assert!(output.contains("STATUS"));
        assert!(output.contains("DESCRIPTION"));
        assert!(output.contains("test-mcp"));
        assert!(output.contains("connected"));
        assert!(output.contains("connected MCP server")); // Derived description
        assert!(!output.contains("PATH"));
    }

    #[test]
    fn test_format_detailed_hooks_output() {
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
            hooks: vec![HookInfo {
                name: "pre-commit".to_string(),
                hook_type: "pre-commit".to_string(),
                path: PathBuf::from("/test/.claude/hooks/pre-commit.md"),
                description: None,
            }],
            agents: vec![],
            commands: vec![],
        };

        let color_scheme = ColorScheme::default();
        let color_settings = ColorSettings::force();

        let mut buffer = Vec::new();
        format_detailed(&info, &color_scheme, &color_settings, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("NAME"));
        assert!(output.contains("TYPE"));
        assert!(output.contains("DESCRIPTION"));
        assert!(output.contains("pre-commit"));
        assert!(output.contains("pre-commit hook")); // Derived description
        assert!(!output.contains("PATH"));
    }

    #[test]
    fn test_format_detailed_agents_output() {
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
            agents: vec![AgentInfo {
                name: "database-agent".to_string(),
                description: Some("Agent for database operations".to_string()),
                path: PathBuf::from("/test/.claude/agents/database-agent.md"),
            }],
            commands: vec![],
        };

        let color_scheme = ColorScheme::default();
        let color_settings = ColorSettings::force();

        let mut buffer = Vec::new();
        format_detailed(&info, &color_scheme, &color_settings, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("NAME"));
        assert!(output.contains("DESCRIPTION"));
        assert!(output.contains("database-agent"));
        assert!(output.contains("Agent for database operations"));
    }

    #[test]
    fn test_format_detailed_commands_output() {
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
            commands: vec![CommandInfo {
                name: "analyze-code".to_string(),
                description: Some("Analyze code quality".to_string()),
                allowed_tools: None,
                argument_hint: None,
                path: PathBuf::from("/test/.claude/commands/analyze-code.md"),
            }],
        };

        let color_scheme = ColorScheme::default();
        let color_settings = ColorSettings::force();

        let mut buffer = Vec::new();
        format_detailed(&info, &color_scheme, &color_settings, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("NAME"));
        assert!(output.contains("DESCRIPTION"));
        assert!(output.contains("analyze-code"));
        assert!(output.contains("Analyze code quality"));
    }

    #[test]
    fn test_format_detailed_truncates_long_description() {
        // Use SkillInfo for truncation test since plugins show PATH, not description
        let info = ClaudeInfo {
            version: "0.1.0".to_string(),
            config_dir: PathBuf::from("/test/.claude"),
            plugins: vec![],
            skills: vec![SkillInfo {
                name: "test-skill".to_string(),
                version: Some("1.0.0".to_string()),
                source: Source::Official,
                path: PathBuf::from("/test/.claude/skills/test-skill"),
                description: Some("This is a very long description that definitely exceeds fifty characters and should be truncated".to_string()),
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

        let color_scheme = ColorScheme::default();
        let color_settings = ColorSettings::force();

        let mut buffer = Vec::new();
        format_detailed(&info, &color_scheme, &color_settings, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        // Should contain truncated description with "..."
        assert!(output.contains("..."));
        // The full description should not appear
        assert!(!output.contains("definitely exceeds fifty characters and should be truncated"));
    }

    #[test]
    fn test_format_detailed_empty_sections() {
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
        let color_settings = ColorSettings::force();

        let mut buffer = Vec::new();
        format_detailed(&info, &color_scheme, &color_settings, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        // Should only show header, no section headers
        assert!(output.contains("CLAUDE-LIST"));
        assert!(output.contains("CONFIG:"));
    }
}
