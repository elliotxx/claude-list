//! Parsers for different component types

pub mod agents;
pub mod commands;
pub mod hooks;
pub mod mcp;
pub mod plugins;
pub mod sessions;
pub mod skills;

use crate::error::Result;
use crate::info::{
    AgentInfo, ClaudeInfo, CommandInfo, HookInfo, McpInfo, PluginInfo, SessionInfo, SkillInfo,
};
use std::path::PathBuf;

/// Search filter for component name matching.
#[derive(Debug, Clone, Default)]
pub struct SearchFilter {
    keywords: Vec<String>,
}

impl SearchFilter {
    /// Create a new search filter from a query string.
    /// Parses keywords (split by whitespace) for AND matching.
    pub fn new(query: &str) -> Self {
        let keywords: Vec<String> = query
            .split_whitespace()
            .map(|k| k.to_lowercase())
            .filter(|k| !k.is_empty())
            .collect();

        Self { keywords }
    }

    /// Check if a component name matches the search criteria.
    /// All keywords must be present (AND logic), case-insensitive.
    pub fn matches(&self, name: &str) -> bool {
        if self.keywords.is_empty() {
            return true;
        }

        let name_lower = name.to_lowercase();
        self.keywords.iter().all(|keyword| name_lower.contains(keyword))
    }

    /// Returns true if the filter has active search keywords.
    pub fn is_active(&self) -> bool {
        !self.keywords.is_empty()
    }
}

/// Filter flags for selecting which component types to display
#[derive(Debug, Default)]
pub struct FilterFlags {
    pub plugins: bool,
    pub skills: bool,
    pub sessions: bool,
    pub mcp: bool,
    pub hooks: bool,
    pub agents: bool,
    pub commands: bool,
    pub search: Option<SearchFilter>,
}

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
pub fn filter_components(info: ClaudeInfo, filters: FilterFlags) -> ClaudeInfo {
    // If no filter flags, show all
    let show_all = !(filters.plugins
        || filters.skills
        || filters.sessions
        || filters.mcp
        || filters.hooks
        || filters.agents
        || filters.commands);

    // Get search filter for matching
    let search_filter = filters.search.as_ref();

    ClaudeInfo {
        plugins: filter_plugin_list(info.plugins, show_all || filters.plugins, search_filter),
        skills: filter_skill_list(info.skills, show_all || filters.skills, search_filter),
        sessions: if show_all || filters.sessions {
            info.sessions
        } else {
            SessionInfo {
                count: 0,
                last_session: None,
            }
        },
        mcp_servers: filter_mcp_list(info.mcp_servers, show_all || filters.mcp, search_filter),
        hooks: filter_hook_list(info.hooks, show_all || filters.hooks, search_filter),
        agents: filter_agent_list(info.agents, show_all || filters.agents, search_filter),
        commands: filter_command_list(info.commands, show_all || filters.commands, search_filter),
        version: info.version,
        config_dir: info.config_dir,
    }
}

fn filter_plugin_list(
    plugins: Vec<PluginInfo>,
    include: bool,
    search: Option<&SearchFilter>,
) -> Vec<PluginInfo> {
    if !include {
        return vec![];
    }
    if let Some(search) = search {
        plugins.into_iter().filter(|p| search.matches(&p.name)).collect()
    } else {
        plugins
    }
}

fn filter_skill_list(
    skills: Vec<SkillInfo>,
    include: bool,
    search: Option<&SearchFilter>,
) -> Vec<SkillInfo> {
    if !include {
        return vec![];
    }
    if let Some(search) = search {
        skills.into_iter().filter(|s| search.matches(&s.name)).collect()
    } else {
        skills
    }
}

fn filter_mcp_list(
    mcp: Vec<McpInfo>,
    include: bool,
    search: Option<&SearchFilter>,
) -> Vec<McpInfo> {
    if !include {
        return vec![];
    }
    if let Some(search) = search {
        mcp.into_iter().filter(|m| search.matches(&m.name)).collect()
    } else {
        mcp
    }
}

fn filter_hook_list(
    hooks: Vec<HookInfo>,
    include: bool,
    search: Option<&SearchFilter>,
) -> Vec<HookInfo> {
    if !include {
        return vec![];
    }
    if let Some(search) = search {
        hooks.into_iter().filter(|h| search.matches(&h.name)).collect()
    } else {
        hooks
    }
}

fn filter_agent_list(
    agents: Vec<AgentInfo>,
    include: bool,
    search: Option<&SearchFilter>,
) -> Vec<AgentInfo> {
    if !include {
        return vec![];
    }
    if let Some(search) = search {
        agents.into_iter().filter(|a| search.matches(&a.name)).collect()
    } else {
        agents
    }
}

fn filter_command_list(
    commands: Vec<CommandInfo>,
    include: bool,
    search: Option<&SearchFilter>,
) -> Vec<CommandInfo> {
    if !include {
        return vec![];
    }
    if let Some(search) = search {
        commands.into_iter().filter(|c| search.matches(&c.name)).collect()
    } else {
        commands
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_filter_single_keyword() {
        let filter = SearchFilter::new("context");
        assert!(filter.matches("context7"));
        assert!(filter.matches("Context"));
        assert!(filter.matches("CONTEXT"));
        assert!(!filter.matches("plugin"));
    }

    #[test]
    fn test_search_filter_multiple_keywords() {
        let filter = SearchFilter::new("context plugin");
        assert!(filter.matches("context7-plugin"));
        assert!(filter.matches("plugin-context-manager"));
        assert!(!filter.matches("context7"));
        assert!(!filter.matches("plugin-loader"));
    }

    #[test]
    fn test_search_filter_case_insensitive() {
        let filter = SearchFilter::new("CONTEXT");
        assert!(filter.matches("context"));
        assert!(filter.matches("Context"));
        assert!(filter.matches("CONTEXT"));
    }

    #[test]
    fn test_search_filter_empty() {
        let filter = SearchFilter::new("");
        assert!(filter.is_active() == false);
        assert!(filter.matches("anything"));
    }

    #[test]
    fn test_search_filter_whitespace_only() {
        let filter = SearchFilter::new("   ");
        assert!(filter.is_active() == false);
        assert!(filter.matches("anything"));
    }

    #[test]
    fn test_search_filter_three_keywords() {
        let filter = SearchFilter::new("a b c");
        assert!(filter.matches("a b c"));
        assert!(filter.matches("c a b"));
        assert!(!filter.matches("a b"));
        assert!(!filter.matches("a c"));
    }
}
