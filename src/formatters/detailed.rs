//! Detailed output formatter (for -l flag)

use crate::info::ClaudeInfo;
use std::io::Write;

pub fn format_detailed(info: &ClaudeInfo, output: &mut dyn Write) -> std::io::Result<()> {
    writeln!(output, "CLAUDE-LIST v{}", info.version)?;
    writeln!(output)?;
    writeln!(output, "CONFIG: {}", info.config_dir.display())?;
    writeln!(output)?;

    // PLUGINS
    if !info.plugins.is_empty() {
        writeln!(output, "PLUGINS    {} installed", info.plugins.len())?;
        writeln!(
            output,
            "  {:<24} {:>10} {:>12} PATH",
            "NAME", "VERSION", "SOURCE"
        )?;
        writeln!(
            output,
            "  {:<24} {:>10} {:>12} {}",
            "-".repeat(24),
            "-".repeat(10),
            "-".repeat(12),
            "-".repeat(30)
        )?;
        for plugin in &info.plugins {
            let version = plugin.version.as_deref().unwrap_or("-");
            let source = match plugin.source {
                crate::info::Source::Official => "official",
                crate::info::Source::ThirdParty => "third-party",
            };
            writeln!(
                output,
                "  {:<24} {:>10} {:>12} {}",
                plugin.name,
                version,
                source,
                plugin.path.display()
            )?;
        }
        writeln!(output)?;
    }

    // SKILLS
    if !info.skills.is_empty() {
        writeln!(output, "SKILLS     {} available", info.skills.len())?;
        writeln!(
            output,
            "  {:<24} {:>10} {:>12} PATH",
            "NAME", "VERSION", "SOURCE"
        )?;
        writeln!(
            output,
            "  {:<24} {:>10} {:>12} {}",
            "-".repeat(24),
            "-".repeat(10),
            "-".repeat(12),
            "-".repeat(30)
        )?;
        for skill in &info.skills {
            let version = skill.version.as_deref().unwrap_or("-");
            let source = match skill.source {
                crate::info::Source::Official => "official",
                crate::info::Source::ThirdParty => "third-party",
            };
            writeln!(
                output,
                "  {:<24} {:>10} {:>12} {}",
                skill.name,
                version,
                source,
                skill.path.display()
            )?;
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
        writeln!(output, "  {:<24} {:>12} PATH", "NAME", "STATUS")?;
        writeln!(
            output,
            "  {:<24} {:>12} {}",
            "-".repeat(24),
            "-".repeat(12),
            "-".repeat(30)
        )?;
        for mcp in &info.mcp_servers {
            writeln!(
                output,
                "  {:<24} {:>12} {}",
                mcp.name,
                mcp.status,
                mcp.path.display()
            )?;
        }
        writeln!(output)?;
    }

    // HOOKS
    if !info.hooks.is_empty() {
        writeln!(output, "HOOKS      {} configured", info.hooks.len())?;
        writeln!(output, "  {:<24} {:>12} PATH", "NAME", "TYPE")?;
        writeln!(
            output,
            "  {:<24} {:>12} {}",
            "-".repeat(24),
            "-".repeat(12),
            "-".repeat(30)
        )?;
        for hook in &info.hooks {
            writeln!(
                output,
                "  {:<24} {:>12} {}",
                hook.name,
                hook.hook_type,
                hook.path.display()
            )?;
        }
        writeln!(output)?;
    }

    // AGENTS
    if !info.agents.is_empty() {
        writeln!(output, "AGENTS     {} defined", info.agents.len())?;
        writeln!(output, "  {:<24} DESCRIPTION", "NAME")?;
        writeln!(output, "  {:<24} {}", "-".repeat(24), "-".repeat(50))?;
        for agent in &info.agents {
            let desc = agent.description.as_deref().unwrap_or("-");
            writeln!(output, "  {:<24} {}", agent.name, desc)?;
        }
        writeln!(output)?;
    }

    // COMMANDS
    if !info.commands.is_empty() {
        writeln!(output, "COMMANDS   {} available", info.commands.len())?;
        writeln!(output, "  {:<24} DESCRIPTION", "NAME")?;
        writeln!(output, "  {:<24} {}", "-".repeat(24), "-".repeat(50))?;
        for cmd in &info.commands {
            let desc = cmd.description.as_deref().unwrap_or("-");
            writeln!(output, "  /{:<23} {}", cmd.name, desc)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::info::{McpInfo, PluginInfo, SessionInfo, SkillInfo, Source};
    use std::path::PathBuf;

    #[test]
    fn test_format_detailed() {
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
            mcp_servers: vec![McpInfo {
                name: "test-mcp".to_string(),
                status: "connected".to_string(),
                command: Some("npx".to_string()),
                path: PathBuf::from("/test/.claude/mcp.json"),
            }],
            hooks: vec![],
            agents: vec![],
            commands: vec![],
        };

        let mut buffer = Vec::new();
        format_detailed(&info, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        // Verify detailed format includes version and source
        assert!(output.contains("2.1.0"));
        assert!(output.contains("official"));
        assert!(output.contains("test-skill"));
        assert!(output.contains("NAME"));
        assert!(output.contains("VERSION"));
        assert!(output.contains("SOURCE"));
    }
}
