//! Data structures for Claude Code environment information

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ====================
// Description Provider Trait
// ====================

/// Trait for providing component descriptions.
/// Used to derive or access descriptions for display in detailed output.
pub trait DescriptionProvider {
    /// Get the description for this component.
    /// Returns None if no description is available.
    fn get_description(&self) -> Option<String>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClaudeInfo {
    pub version: String,
    pub config_dir: PathBuf,
    pub plugins: Vec<PluginInfo>,
    pub skills: Vec<SkillInfo>,
    pub sessions: SessionInfo,
    pub mcp_servers: Vec<McpInfo>,
    pub hooks: Vec<HookInfo>,
    pub agents: Vec<AgentInfo>,
    pub commands: Vec<CommandInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub version: Option<String>,
    pub source: Source,
    pub path: PathBuf,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Source {
    #[serde(rename = "official")]
    Official,
    #[serde(rename = "third-party")]
    ThirdParty,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: Option<String>,
    pub source: Source,
    pub path: PathBuf,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl DescriptionProvider for PluginInfo {
    fn get_description(&self) -> Option<String> {
        if let Some(ref desc) = self.description {
            return Some(desc.clone());
        }
        Some(match self.source {
            Source::Official => "Official plugin".to_string(),
            Source::ThirdParty => "Third-party plugin".to_string(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillInfo {
    pub name: String,
    pub version: Option<String>,
    pub source: Source,
    pub path: PathBuf,
    pub description: Option<String>,
}

impl DescriptionProvider for SkillInfo {
    fn get_description(&self) -> Option<String> {
        self.description.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionInfo {
    pub count: usize,
    pub last_session: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct McpInfo {
    pub name: String,
    pub status: String,
    pub command: Option<String>,
    pub path: PathBuf,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl DescriptionProvider for McpInfo {
    fn get_description(&self) -> Option<String> {
        if let Some(ref desc) = self.description {
            return Some(desc.clone());
        }
        Some(format!("{} MCP server", self.status))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HookInfo {
    pub name: String,
    pub hook_type: String,
    pub path: PathBuf,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl DescriptionProvider for HookInfo {
    fn get_description(&self) -> Option<String> {
        if let Some(ref desc) = self.description {
            return Some(desc.clone());
        }
        Some(format!("{} hook", self.hook_type))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentInfo {
    pub name: String,
    pub description: Option<String>,
    pub path: PathBuf,
}

impl DescriptionProvider for AgentInfo {
    fn get_description(&self) -> Option<String> {
        self.description.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandInfo {
    pub name: String,
    pub description: Option<String>,
    pub allowed_tools: Option<String>,
    pub argument_hint: Option<String>,
    pub path: PathBuf,
}

impl DescriptionProvider for CommandInfo {
    fn get_description(&self) -> Option<String> {
        self.description.clone()
    }
}
