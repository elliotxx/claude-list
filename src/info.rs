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

/// Represents where a skill is located.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(tag = "type")]
pub enum SkillLocation {
    /// Global skills from ~/.claude/skills/
    #[default]
    Global,
    /// Skills bundled within a plugin from plugins/cache/*/*/skills/
    Plugin {
        /// The name of the plugin this skill belongs to
        #[serde(skip_serializing_if = "Option::is_none")]
        plugin_name: Option<String>,
    },
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
    /// Where this skill is located (global or from plugin)
    #[serde(default)]
    pub location_type: SkillLocation,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_info_description_provider_with_description() {
        let plugin = PluginInfo {
            name: "test-plugin".to_string(),
            version: Some("1.0.0".to_string()),
            source: Source::Official,
            path: PathBuf::from("/test"),
            description: Some("A test plugin".to_string()),
        };

        assert_eq!(plugin.get_description(), Some("A test plugin".to_string()));
    }

    #[test]
    fn test_plugin_info_description_provider_without_description() {
        let plugin = PluginInfo {
            name: "test-plugin".to_string(),
            version: Some("1.0.0".to_string()),
            source: Source::Official,
            path: PathBuf::from("/test"),
            description: None,
        };

        assert_eq!(
            plugin.get_description(),
            Some("Official plugin".to_string())
        );
    }

    #[test]
    fn test_plugin_info_description_provider_third_party() {
        let plugin = PluginInfo {
            name: "test-plugin".to_string(),
            version: Some("1.0.0".to_string()),
            source: Source::ThirdParty,
            path: PathBuf::from("/test"),
            description: None,
        };

        assert_eq!(
            plugin.get_description(),
            Some("Third-party plugin".to_string())
        );
    }

    #[test]
    fn test_skill_info_description_provider_with_description() {
        let skill = SkillInfo {
            name: "test-skill".to_string(),
            version: Some("1.0.0".to_string()),
            source: Source::Official,
            path: PathBuf::from("/test"),
            description: Some("A test skill".to_string()),
            location_type: SkillLocation::Global,
        };

        assert_eq!(skill.get_description(), Some("A test skill".to_string()));
    }

    #[test]
    fn test_skill_info_description_provider_without_description() {
        let skill = SkillInfo {
            name: "test-skill".to_string(),
            version: Some("1.0.0".to_string()),
            source: Source::Official,
            path: PathBuf::from("/test"),
            description: None,
            location_type: SkillLocation::Global,
        };

        assert_eq!(skill.get_description(), None);
    }

    #[test]
    fn test_mcp_info_description_provider_with_description() {
        let mcp = McpInfo {
            name: "test-mcp".to_string(),
            status: "connected".to_string(),
            command: Some("npx".to_string()),
            path: PathBuf::from("/test"),
            description: Some("A test MCP server".to_string()),
        };

        assert_eq!(mcp.get_description(), Some("A test MCP server".to_string()));
    }

    #[test]
    fn test_mcp_info_description_provider_without_description() {
        let mcp = McpInfo {
            name: "test-mcp".to_string(),
            status: "connected".to_string(),
            command: Some("npx".to_string()),
            path: PathBuf::from("/test"),
            description: None,
        };

        assert_eq!(
            mcp.get_description(),
            Some("connected MCP server".to_string())
        );
    }

    #[test]
    fn test_hook_info_description_provider_with_description() {
        let hook = HookInfo {
            name: "pre-commit".to_string(),
            hook_type: "pre-commit".to_string(),
            path: PathBuf::from("/test"),
            description: Some("A pre-commit hook".to_string()),
        };

        assert_eq!(
            hook.get_description(),
            Some("A pre-commit hook".to_string())
        );
    }

    #[test]
    fn test_hook_info_description_provider_without_description() {
        let hook = HookInfo {
            name: "pre-commit".to_string(),
            hook_type: "pre-commit".to_string(),
            path: PathBuf::from("/test"),
            description: None,
        };

        assert_eq!(hook.get_description(), Some("pre-commit hook".to_string()));
    }

    #[test]
    fn test_agent_info_description_provider_with_description() {
        let agent = AgentInfo {
            name: "test-agent".to_string(),
            description: Some("A test agent".to_string()),
            path: PathBuf::from("/test"),
        };

        assert_eq!(agent.get_description(), Some("A test agent".to_string()));
    }

    #[test]
    fn test_agent_info_description_provider_without_description() {
        let agent = AgentInfo {
            name: "test-agent".to_string(),
            description: None,
            path: PathBuf::from("/test"),
        };

        assert_eq!(agent.get_description(), None);
    }

    #[test]
    fn test_command_info_description_provider_with_description() {
        let command = CommandInfo {
            name: "test-command".to_string(),
            description: Some("A test command".to_string()),
            allowed_tools: None,
            argument_hint: None,
            path: PathBuf::from("/test"),
        };

        assert_eq!(
            command.get_description(),
            Some("A test command".to_string())
        );
    }

    #[test]
    fn test_command_info_description_provider_without_description() {
        let command = CommandInfo {
            name: "test-command".to_string(),
            description: None,
            allowed_tools: None,
            argument_hint: None,
            path: PathBuf::from("/test"),
        };

        assert_eq!(command.get_description(), None);
    }

    #[test]
    fn test_skill_location_default() {
        let location = SkillLocation::default();
        assert!(matches!(location, SkillLocation::Global));
    }

    #[test]
    fn test_skill_location_plugin_with_name() {
        let location = SkillLocation::Plugin {
            plugin_name: Some("test-plugin".to_string()),
        };
        assert!(matches!(location, SkillLocation::Plugin { .. }));
        if let SkillLocation::Plugin { plugin_name } = location {
            assert_eq!(plugin_name, Some("test-plugin".to_string()));
        }
    }

    #[test]
    fn test_skill_location_plugin_without_name() {
        let location = SkillLocation::Plugin { plugin_name: None };
        assert!(matches!(location, SkillLocation::Plugin { .. }));
        if let SkillLocation::Plugin { plugin_name } = location {
            assert!(plugin_name.is_none());
        }
    }

    #[test]
    fn test_source_serialization() {
        // Test that Source enum serializes correctly
        let official = Source::Official;
        let third_party = Source::ThirdParty;

        // The serialization format uses serde(rename)
        let official_json = serde_json::to_string(&official).unwrap();
        assert_eq!(official_json, r#""official""#);

        let third_party_json = serde_json::to_string(&third_party).unwrap();
        assert_eq!(third_party_json, r#""third-party""#);
    }

    #[test]
    fn test_source_deserialization() {
        let official: Source = serde_json::from_str(r#""official""#).unwrap();
        assert_eq!(official, Source::Official);

        let third_party: Source = serde_json::from_str(r#""third-party""#).unwrap();
        assert_eq!(third_party, Source::ThirdParty);
    }

    #[test]
    fn test_plugin_info_serialization() {
        let plugin = PluginInfo {
            name: "test-plugin".to_string(),
            version: Some("1.0.0".to_string()),
            source: Source::Official,
            path: PathBuf::from("/test"),
            description: Some("A test plugin".to_string()),
        };

        let json = serde_json::to_string(&plugin).unwrap();
        assert!(json.contains(r#""name":"test-plugin""#));
        assert!(json.contains(r#""version":"1.0.0""#));
        assert!(json.contains(r#""source":"official""#));
    }

    #[test]
    fn test_skill_location_serialization() {
        let global = SkillLocation::Global;
        let json = serde_json::to_string(&global).unwrap();
        assert!(json.contains(r#""type":"Global""#));

        let plugin = SkillLocation::Plugin {
            plugin_name: Some("test-plugin".to_string()),
        };
        let json = serde_json::to_string(&plugin).unwrap();
        assert!(json.contains(r#""type":"Plugin""#));
    }
}
