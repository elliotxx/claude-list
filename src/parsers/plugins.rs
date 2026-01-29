//! Parse installed plugins from plugins/installed_plugins.json or settings.json

use crate::error::Result;
use crate::info::{PluginInfo, Source};
use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn parse_plugins(base_path: &Path) -> Result<Vec<PluginInfo>> {
    // Try new format: plugins/installed_plugins.json
    let installed_path = base_path.join("plugins").join("installed_plugins.json");

    if installed_path.exists() {
        return parse_plugins_v2(&installed_path);
    }

    // Fallback to old format: settings.json
    let settings_path = base_path.join("settings.json");

    if !settings_path.exists() {
        return Ok(vec![]);
    }

    let content = match fs::read_to_string(&settings_path) {
        Ok(c) => c,
        Err(_) => return Ok(vec![]),
    };

    // Gracefully handle malformed JSON
    let json: Value = match serde_json::from_str(&content) {
        Ok(j) => j,
        Err(_) => return Ok(vec![]),
    };

    let plugins = json
        .get("installed_plugins")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|p| {
                    let name = p.get("name")?.as_str()?.to_string();
                    let version = p.get("version").and_then(|v| v.as_str()).map(String::from);

                    let source = if name.starts_with("plugin_") {
                        Source::ThirdParty
                    } else {
                        Source::Official
                    };

                    Some(PluginInfo {
                        name,
                        version,
                        source,
                        path: settings_path.clone(),
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(plugins)
}

/// Parse new format: plugins/installed_plugins.json (version 2)
fn parse_plugins_v2(installed_path: &Path) -> Result<Vec<PluginInfo>> {
    let content = match fs::read_to_string(installed_path) {
        Ok(c) => c,
        Err(_) => return Ok(vec![]),
    };

    // Gracefully handle malformed JSON
    let json: Value = match serde_json::from_str(&content) {
        Ok(j) => j,
        Err(_) => return Ok(vec![]),
    };

    let mut plugins = Vec::new();

    if let Some(plugins_obj) = json.get("plugins").and_then(|v| v.as_object()) {
        for (full_name, plugin_array) in plugins_obj {
            // Parse "name@source" format
            let parts: Vec<&str> = full_name.split('@').collect();
            let name = parts[0].to_string();
            let source = if parts.len() > 1
                && parts[1] != "claude-plugins-official"
                && parts[1] != "claude-code-workflows"
            {
                Source::ThirdParty
            } else {
                Source::Official
            };

            if let Some(arr) = plugin_array.as_array() {
                if let Some(first) = arr.first() {
                    let version = first
                        .get("version")
                        .and_then(|v| v.as_str())
                        .map(String::from);

                    plugins.push(PluginInfo {
                        name,
                        version,
                        source,
                        path: installed_path.to_path_buf(),
                    });
                }
            }
        }
    }

    Ok(plugins)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_parse_plugins() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        let settings = r#"{
            "installed_plugins": [
                {"name": "context7", "version": "2.1.0"},
                {"name": "plugin_custom", "version": "1.0.0"}
            ]
        }"#;

        File::create(path.join("settings.json"))
            .unwrap()
            .write_all(settings.as_bytes())
            .unwrap();

        let plugins = parse_plugins(path).unwrap();

        assert_eq!(plugins.len(), 2);
        assert_eq!(plugins[0].name, "context7");
        assert_eq!(plugins[0].version, Some("2.1.0".to_string()));
        assert_eq!(plugins[0].source, Source::Official);

        assert_eq!(plugins[1].name, "plugin_custom");
        assert_eq!(plugins[1].source, Source::ThirdParty);
    }

    #[test]
    fn test_parse_plugins_from_installed_json() {
        // Test new format: plugins/installed_plugins.json
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        // Create plugins directory and installed_plugins.json
        let plugins_dir = path.join("plugins");
        std::fs::create_dir_all(&plugins_dir).unwrap();

        let installed_json = r#"{
            "version": 2,
            "plugins": {
                "context7@claude-plugins-official": [
                    {
                        "scope": "user",
                        "version": "d49ad3558669",
                        "installedAt": "2026-01-14T04:28:57.441Z"
                    }
                ],
                "custom-plugin@custom-source": [
                    {
                        "scope": "project",
                        "version": "1.0.0",
                        "projectPath": "/test"
                    }
                ]
            }
        }"#;

        File::create(plugins_dir.join("installed_plugins.json"))
            .unwrap()
            .write_all(installed_json.as_bytes())
            .unwrap();

        let plugins = parse_plugins(path).unwrap();

        // Should parse 2 plugins
        assert_eq!(plugins.len(), 2);

        // context7 should be Official
        let context7 = plugins.iter().find(|p| p.name == "context7").unwrap();
        assert_eq!(context7.source, Source::Official);

        // custom-plugin should be ThirdParty
        let custom = plugins.iter().find(|p| p.name == "custom-plugin").unwrap();
        assert_eq!(custom.source, Source::ThirdParty);
    }

    #[test]
    fn test_missing_settings() {
        let dir = TempDir::new().unwrap();
        let plugins = parse_plugins(dir.path()).unwrap();
        assert!(plugins.is_empty());
    }

    #[test]
    fn test_malformed_json() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        // Create malformed JSON
        File::create(path.join("settings.json"))
            .unwrap()
            .write_all(b"{ invalid json }")
            .unwrap();

        // Should gracefully degrade to empty
        let plugins = parse_plugins(path).unwrap();
        assert!(plugins.is_empty());
    }

    #[test]
    fn test_malformed_installed_json() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        let plugins_dir = path.join("plugins");
        std::fs::create_dir_all(&plugins_dir).unwrap();

        // Create malformed installed_plugins.json
        File::create(plugins_dir.join("installed_plugins.json"))
            .unwrap()
            .write_all(b"{ invalid }")
            .unwrap();

        // Should gracefully degrade to empty
        let plugins = parse_plugins(path).unwrap();
        assert!(plugins.is_empty());
    }

    #[test]
    fn test_plugin_name_extraction_from_key() {
        // Test that plugin name is correctly extracted from "name@source" key
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        let plugins_dir = path.join("plugins");
        std::fs::create_dir_all(&plugins_dir).unwrap();

        let installed_json = r#"{
            "version": 2,
            "plugins": {
                "my-awesome-plugin@some-source": [
                    {"scope": "user", "version": "1.0.0"}
                ]
            }
        }"#;

        File::create(plugins_dir.join("installed_plugins.json"))
            .unwrap()
            .write_all(installed_json.as_bytes())
            .unwrap();

        let plugins = parse_plugins(path).unwrap();
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].name, "my-awesome-plugin");
    }
}
