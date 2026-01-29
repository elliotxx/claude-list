//! Compact output formatter

use crate::info::ClaudeInfo;
use std::io::Write;

pub fn format_compact(info: &ClaudeInfo, output: &mut dyn Write) -> std::io::Result<()> {
    writeln!(output, "CLAUDE-LIST v{}", info.version)?;
    writeln!(output)?;
    writeln!(output, "CONFIG: {}", info.config_dir.display())?;
    writeln!(output)?;

    // PLUGINS
    if !info.plugins.is_empty() {
        writeln!(output, "PLUGINS    {} installed", info.plugins.len())?;
        for plugin in &info.plugins {
            writeln!(output, "  {}", plugin.name)?;
        }
        writeln!(output)?;
    }

    // SKILLS
    if !info.skills.is_empty() {
        writeln!(output, "SKILLS     {} available", info.skills.len())?;
        for skill in &info.skills {
            writeln!(output, "  {}", skill.name)?;
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
            writeln!(output, " {}", mcp.name)?;
        }
        writeln!(output)?;
    }

    // HOOKS
    if !info.hooks.is_empty() {
        writeln!(output, "HOOKS      {} configured", info.hooks.len())?;
        for hook in &info.hooks {
            writeln!(output, "  {}", hook.name)?;
        }
        writeln!(output)?;
    }

    // AGENTS
    if !info.agents.is_empty() {
        writeln!(output, "AGENTS     {} defined", info.agents.len())?;
        for agent in &info.agents {
            writeln!(output, "  {}", agent.name)?;
        }
        writeln!(output)?;
    }

    // COMMANDS
    if !info.commands.is_empty() {
        writeln!(output, "COMMANDS   {} available", info.commands.len())?;
        for cmd in &info.commands {
            writeln!(output, "  /{}", cmd.name)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::info::{PluginInfo, SessionInfo, SkillInfo, Source};
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
            }],
            skills: vec![SkillInfo {
                name: "test-skill".to_string(),
                version: Some("1.0.0".to_string()),
                source: Source::Official,
                path: PathBuf::from("/test/.claude/skills/test-skill"),
                description: Some("A test skill".to_string()),
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

        let mut buffer = Vec::new();
        format_compact(&info, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("CLAUDE-LIST v0.1.0"));
        assert!(output.contains("PLUGINS"));
        assert!(output.contains("context7"));
        assert!(output.contains("SKILLS"));
        assert!(output.contains("test-skill"));
        assert!(output.contains("SESSIONS"));
        assert!(output.contains("42"));
    }
}
