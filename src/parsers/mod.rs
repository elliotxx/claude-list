//! Parsers for different component types

pub mod agents;
pub mod commands;
pub mod hooks;
pub mod mcp;
pub mod plugins;
pub mod sessions;
pub mod skills;

use crate::error::Result;
use crate::info::{ClaudeInfo, SessionInfo};
use std::path::PathBuf;

/// Parse all components from .claude directory
pub fn parse_all(base_path: PathBuf) -> Result<ClaudeInfo> {
    Ok(ClaudeInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        config_dir: base_path.clone(),
        plugins: plugins::parse_plugins(&base_path)?,
        skills: skills::parse_skills(&base_path)?,
        sessions: sessions::parse_sessions(&base_path)?,
        mcp_servers: mcp::parse_mcp(&base_path)?,
        hooks: hooks::parse_hooks(&base_path)?,
        agents: agents::parse_agents(&base_path)?,
        commands: commands::parse_commands(&base_path)?,
    })
}

/// Filter components based on CLI flags
pub fn filter_components(
    info: ClaudeInfo,
    plugins: bool,
    skills: bool,
    sessions: bool,
    mcp: bool,
    hooks: bool,
    agents: bool,
    commands: bool,
) -> ClaudeInfo {
    // If no filter flags, show all
    let show_all = !(plugins || skills || sessions || mcp || hooks || agents || commands);

    ClaudeInfo {
        plugins: if show_all || plugins {
            info.plugins
        } else {
            vec![]
        },
        skills: if show_all || skills {
            info.skills
        } else {
            vec![]
        },
        sessions: if show_all || sessions {
            info.sessions
        } else {
            SessionInfo {
                count: 0,
                last_session: None,
            }
        },
        mcp_servers: if show_all || mcp {
            info.mcp_servers
        } else {
            vec![]
        },
        hooks: if show_all || hooks {
            info.hooks
        } else {
            vec![]
        },
        agents: if show_all || agents {
            info.agents
        } else {
            vec![]
        },
        commands: if show_all || commands {
            info.commands
        } else {
            vec![]
        },
        ..info
    }
}
