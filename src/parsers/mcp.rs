//! Parse MCP servers from mcp-servers/ directory

use crate::error::Result;
use crate::info::McpInfo;
use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn parse_mcp(base_path: &Path) -> Result<Vec<McpInfo>> {
    // Try new format: mcp-servers/ directory
    let mcp_servers_dir = base_path.join("mcp-servers");

    if mcp_servers_dir.exists() && mcp_servers_dir.is_dir() {
        return parse_mcp_from_directory(&mcp_servers_dir);
    }

    // Fallback to old format: mcp.json
    let mcp_path = base_path.join("mcp.json");

    if !mcp_path.exists() {
        return Ok(vec![]);
    }

    let content = match fs::read_to_string(&mcp_path) {
        Ok(c) => c,
        Err(_) => return Ok(vec![]),
    };
    let json: Value = match serde_json::from_str(&content) {
        Ok(j) => j,
        Err(_) => return Ok(vec![]),
    };

    let mut servers = Vec::new();

    if let Some(mcp_servers) = json.get("mcpServers").and_then(|v| v.as_object()) {
        for (name, server) in mcp_servers {
            let command = server
                .get("command")
                .and_then(|v| v.as_str())
                .map(String::from);
            let args = server
                .get("args")
                .and_then(|v| v.as_str())
                .map(String::from);

            servers.push(McpInfo {
                name: name.clone(),
                status: "connected".to_string(),
                command: command.or(args),
                path: mcp_path.clone(),
                description: None,
            });
        }
    }

    Ok(servers)
}

/// Parse MCP servers from mcp-servers/ directory
fn parse_mcp_from_directory(mcp_servers_dir: &Path) -> Result<Vec<McpInfo>> {
    let mut servers = Vec::new();

    for entry in fs::read_dir(mcp_servers_dir)? {
        let entry = entry?;
        let server_path = entry.path();

        if !server_path.is_dir() {
            continue;
        }

        let name = server_path
            .file_name()
            .and_then(|n| n.to_str())
            .map(String::from)
            .unwrap_or_default();

        // Look for smithery.yaml or package.json to get command info
        let smithery_path = server_path.join("smithery.yaml");
        let package_path = server_path.join("package.json");

        let command = if smithery_path.exists() {
            Some("smithery.yaml".to_string())
        } else if package_path.exists() {
            if let Ok(content) = fs::read_to_string(&package_path) {
                if let Ok(json) = serde_json::from_str::<Value>(&content) {
                    json.get("name").and_then(|v| v.as_str()).map(String::from)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        servers.push(McpInfo {
            name,
            status: "installed".to_string(),
            command,
            path: server_path,
            description: None,
        });
    }

    Ok(servers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_parse_mcp() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        let mcp_config = r#"{
            "mcpServers": {
                "MiniMax": {
                    "command": "npx",
                    "args": ["-y", "@anthropic-ai/minimax-mcp"]
                }
            }
        }"#;

        File::create(path.join("mcp.json"))
            .unwrap()
            .write_all(mcp_config.as_bytes())
            .unwrap();

        let servers = parse_mcp(path).unwrap();
        assert_eq!(servers.len(), 1);
        assert_eq!(servers[0].name, "MiniMax");
        assert!(servers[0].command.is_some());
    }

    #[test]
    fn test_parse_mcp_from_directory() {
        // Test new format: mcp-servers/ directory
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        // Create mcp-servers directory structure
        let mcp_servers_dir = path.join("mcp-servers");
        std::fs::create_dir_all(&mcp_servers_dir).unwrap();
        std::fs::create_dir_all(mcp_servers_dir.join("unsplash-mcp-server")).unwrap();
        std::fs::create_dir_all(mcp_servers_dir.join("custom-mcp")).unwrap();

        // Create smithery.yaml for unsplash
        let smithery = r#"# Smithery configuration
startCommand:
  type: stdio
  commandFunction: |
    (config) => ({
      command: 'fastmcp',
      args: ['run', 'server.py']
    })
"#;

        File::create(mcp_servers_dir.join("unsplash-mcp-server/smithery.yaml"))
            .unwrap()
            .write_all(smithery.as_bytes())
            .unwrap();

        // Create package.json for custom-mcp
        let package_json = r#"{
            "name": "custom-mcp-server",
            "version": "1.0.0"
        }"#;

        File::create(mcp_servers_dir.join("custom-mcp/package.json"))
            .unwrap()
            .write_all(package_json.as_bytes())
            .unwrap();

        let servers = parse_mcp(path).unwrap();

        // Should parse 2 servers
        assert_eq!(servers.len(), 2);

        let unsplash = servers
            .iter()
            .find(|s| s.name == "unsplash-mcp-server")
            .unwrap();
        assert_eq!(unsplash.status, "installed");
        assert!(unsplash.command.is_some());

        let custom = servers.iter().find(|s| s.name == "custom-mcp").unwrap();
        assert_eq!(custom.status, "installed");
    }

    #[test]
    fn test_missing_mcp() {
        let dir = TempDir::new().unwrap();
        let servers = parse_mcp(dir.path()).unwrap();
        assert!(servers.is_empty());
    }

    #[test]
    fn test_mcp_multiple_servers() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        let mcp_config = r#"{
            "mcpServers": {
                "server1": {"command": "npx", "args": ["-y", "server1"]},
                "server2": {"command": "python", "args": ["server2.py"]},
                "server3": {"command": "node", "args": ["server3.js"]}
            }
        }"#;

        File::create(path.join("mcp.json"))
            .unwrap()
            .write_all(mcp_config.as_bytes())
            .unwrap();

        let servers = parse_mcp(path).unwrap();
        assert_eq!(servers.len(), 3);
        assert!(servers.iter().any(|s| s.name == "server1"));
        assert!(servers.iter().any(|s| s.name == "server2"));
        assert!(servers.iter().any(|s| s.name == "server3"));
    }

    #[test]
    fn test_mcp_malformed_json() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        File::create(path.join("mcp.json"))
            .unwrap()
            .write_all(b"{ invalid json }")
            .unwrap();

        let servers = parse_mcp(path).unwrap();
        assert!(servers.is_empty());
    }

    #[test]
    fn test_mcp_empty_config() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        let mcp_config = r#"{"mcpServers": {}}"#;
        File::create(path.join("mcp.json"))
            .unwrap()
            .write_all(mcp_config.as_bytes())
            .unwrap();

        let servers = parse_mcp(path).unwrap();
        assert!(servers.is_empty());
    }

    #[test]
    fn test_mcp_directory_without_config() {
        // mcp-servers directory exists but no mcp.json
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        let mcp_servers_dir = path.join("mcp-servers");
        std::fs::create_dir_all(&mcp_servers_dir).unwrap();
        std::fs::create_dir_all(mcp_servers_dir.join("test-server")).unwrap();

        File::create(mcp_servers_dir.join("test-server/package.json"))
            .unwrap()
            .write_all(b"{}")
            .unwrap();

        let servers = parse_mcp(path).unwrap();
        assert_eq!(servers.len(), 1);
        assert_eq!(servers[0].name, "test-server");
    }
}
