//! Parse agents from agents/ directory

use crate::error::Result;
use crate::info::AgentInfo;
use std::fs;
use std::path::Path;

pub fn parse_agents(base_path: &Path) -> Result<Vec<AgentInfo>> {
    let agents_dir = base_path.join("agents");

    if !agents_dir.exists() || !agents_dir.is_dir() {
        return Ok(vec![]);
    }

    let mut agents = Vec::new();

    for entry in fs::read_dir(&agents_dir)? {
        let entry = entry?;
        let agent_path = entry.path();

        if !agent_path.is_file() || agent_path.extension().map(|e| e.to_str()) != Some(Some("md")) {
            continue;
        }

        let content = fs::read_to_string(&agent_path)?;

        // Parse frontmatter for name and description
        let mut name = agent_path
            .file_stem()
            .and_then(|n| n.to_str())
            .map(String::from)
            .unwrap_or_default();

        let mut description: Option<String> = None;

        if content.starts_with("---") {
            let frontmatter = content
                .trim_start_matches("---")
                .split("---")
                .next()
                .unwrap_or("");

            for line in frontmatter.lines() {
                if line.trim_start().starts_with("name:") {
                    name = line.split(":").nth(1).unwrap_or("").trim().to_string();
                } else if line.trim_start().starts_with("description:") {
                    description = Some(line.split(":").nth(1).unwrap_or("").trim().to_string());
                }
            }
        }

        agents.push(AgentInfo {
            name,
            description,
            path: agent_path,
        });
    }

    Ok(agents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_parse_agents() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        create_agents_dir(path, "test-agent", Some("A test agent"));
        create_agents_dir(path, "another-agent", None);

        let agents = parse_agents(path).unwrap();
        assert_eq!(agents.len(), 2);
    }

    #[test]
    fn test_missing_agents_dir() {
        let dir = TempDir::new().unwrap();
        let agents = parse_agents(dir.path()).unwrap();
        assert!(agents.is_empty());
    }

    fn create_agents_dir(base: &Path, name: &str, desc: Option<&str>) {
        let agent_dir = base.join("agents");
        std::fs::create_dir_all(&agent_dir).unwrap();

        let description_line = match desc {
            Some(d) => format!("description: {}", d),
            None => String::new(),
        };

        let content = format!(
            r#"---
name: {}
{}
---
# {}
"#,
            name, description_line, name
        );

        File::create(agent_dir.join(format!("{}.md", name)))
            .unwrap()
            .write_all(content.as_bytes())
            .unwrap();
    }
}
