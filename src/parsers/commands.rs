//! Parse commands from commands/ directory

use crate::error::Result;
use crate::info::CommandInfo;
use std::fs;
use std::path::Path;

pub fn parse_commands(base_path: &Path) -> Result<Vec<CommandInfo>> {
    let commands_dir = base_path.join("commands");

    if !commands_dir.exists() || !commands_dir.is_dir() {
        return Ok(vec![]);
    }

    let mut commands = Vec::new();

    for entry in fs::read_dir(&commands_dir)? {
        let entry = entry?;
        let command_path = entry.path();

        if !command_path.is_file()
            || command_path.extension().map(|e| e.to_str()) != Some(Some("md"))
        {
            continue;
        }

        let name = command_path
            .file_stem()
            .and_then(|n| n.to_str())
            .map(String::from)
            .unwrap_or_default();

        let content = fs::read_to_string(&command_path)?;

        let mut description = None;
        let mut allowed_tools = None;
        let mut argument_hint = None;

        if content.starts_with("---") {
            if let Some(frontmatter) = content.trim_start_matches("---").split("---").next() {
                for line in frontmatter.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("description:") {
                        description = trimmed.split(":").nth(1).map(|s| s.trim().to_string());
                    } else if trimmed.starts_with("allowed-tools:") {
                        allowed_tools = trimmed.split(":").nth(1).map(|s| s.trim().to_string());
                    } else if trimmed.starts_with("argument-hint:") {
                        argument_hint = trimmed.split(":").nth(1).map(|s| s.trim().to_string());
                    }
                }
            }
        }

        commands.push(CommandInfo {
            name,
            description,
            allowed_tools,
            argument_hint,
            path: command_path,
        });
    }

    Ok(commands)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_parse_commands() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        let commands_dir = path.join("commands");
        std::fs::create_dir_all(&commands_dir).unwrap();

        let command_md = r#"---
allowed-tools: Bash(git:*), AskUserQuestion
argument-hint: [set <branch> | show]
description: Switch to iteration branch, configure it, or show current setting
---

# Switch to Iteration Branch
"#;

        File::create(commands_dir.join("gcoiter.md"))
            .unwrap()
            .write_all(command_md.as_bytes())
            .unwrap();

        let commands = parse_commands(path).unwrap();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.name, "gcoiter");
        assert!(cmd.description.is_some());
        assert!(cmd
            .description
            .as_ref()
            .unwrap()
            .contains("iteration branch"));
        assert!(cmd.allowed_tools.is_some());
        assert!(cmd.argument_hint.is_some());
    }

    #[test]
    fn test_missing_commands_dir() {
        let dir = TempDir::new().unwrap();
        let commands = parse_commands(dir.path()).unwrap();
        assert!(commands.is_empty());
    }

    #[test]
    fn test_commands_without_frontmatter() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        let commands_dir = path.join("commands");
        std::fs::create_dir_all(&commands_dir).unwrap();

        // Command without frontmatter
        File::create(commands_dir.join("no-frontmatter.md"))
            .unwrap()
            .write_all(b"# Just a markdown file\nNo frontmatter\n")
            .unwrap();

        let commands = parse_commands(path).unwrap();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].name, "no-frontmatter");
        assert!(commands[0].description.is_none());
    }

    #[test]
    fn test_commands_partial_frontmatter() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        let commands_dir = path.join("commands");
        std::fs::create_dir_all(&commands_dir).unwrap();

        // Command with only description
        let command_md = r#"---
description: Only description provided
---
# Command
"#;
        File::create(commands_dir.join("partial.md"))
            .unwrap()
            .write_all(command_md.as_bytes())
            .unwrap();

        let commands = parse_commands(path).unwrap();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].name, "partial");
        assert_eq!(
            commands[0].description,
            Some("Only description provided".to_string())
        );
        assert!(commands[0].allowed_tools.is_none());
        assert!(commands[0].argument_hint.is_none());
    }

    #[test]
    fn test_commands_multiple() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        let commands_dir = path.join("commands");
        std::fs::create_dir_all(&commands_dir).unwrap();

        for cmd_name in ["cmd1", "cmd2", "cmd3"] {
            let content = format!(
                r#"---
description: Command {}
---
# {}
"#,
                cmd_name, cmd_name
            );
            File::create(commands_dir.join(format!("{}.md", cmd_name)))
                .unwrap()
                .write_all(content.as_bytes())
                .unwrap();
        }

        let commands = parse_commands(path).unwrap();
        assert_eq!(commands.len(), 3);
        assert!(commands.iter().any(|c| c.name == "cmd1"));
        assert!(commands.iter().any(|c| c.name == "cmd2"));
        assert!(commands.iter().any(|c| c.name == "cmd3"));
    }

    #[test]
    fn test_commands_special_chars_in_name() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        let commands_dir = path.join("commands");
        std::fs::create_dir_all(&commands_dir).unwrap();

        // Command with numbers and underscores
        let command_md = r#"---
description: Test command
---
# Test
"#;
        File::create(commands_dir.join("test_command_123.md"))
            .unwrap()
            .write_all(command_md.as_bytes())
            .unwrap();

        let commands = parse_commands(path).unwrap();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].name, "test_command_123");
    }

    #[test]
    fn test_commands_non_md_files_ignored() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        let commands_dir = path.join("commands");
        std::fs::create_dir_all(&commands_dir).unwrap();

        // Create non-markdown files
        File::create(commands_dir.join("script.sh")).unwrap();
        File::create(commands_dir.join("data.json")).unwrap();
        File::create(commands_dir.join("valid-command.md")).unwrap();

        let commands = parse_commands(path).unwrap();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].name, "valid-command");
    }

    #[test]
    fn test_commands_empty_description() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        let commands_dir = path.join("commands");
        std::fs::create_dir_all(&commands_dir).unwrap();

        // Command with empty description
        let command_md = r#"---
description:
---
# Command
"#;
        File::create(commands_dir.join("empty-desc.md"))
            .unwrap()
            .write_all(command_md.as_bytes())
            .unwrap();

        let commands = parse_commands(path).unwrap();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].name, "empty-desc");
        // Empty description should be stored as empty string
        assert_eq!(commands[0].description, Some("".to_string()));
    }
}
