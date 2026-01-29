//! Parse hooks from hooks/ directory

use crate::error::Result;
use crate::info::HookInfo;
use std::fs;
use std::path::Path;

pub fn parse_hooks(base_path: &Path) -> Result<Vec<HookInfo>> {
    let hooks_dir = base_path.join("hooks");

    if !hooks_dir.exists() || !hooks_dir.is_dir() {
        return Ok(vec![]);
    }

    let mut hooks = Vec::new();

    for entry in fs::read_dir(&hooks_dir)? {
        let entry = entry?;
        let hook_path = entry.path();

        if !hook_path.is_file() || hook_path.extension().map(|e| e.to_str()) != Some(Some("md")) {
            continue;
        }

        let name = hook_path
            .file_stem()
            .and_then(|n| n.to_str())
            .map(String::from)
            .unwrap_or_default();

        let content = fs::read_to_string(&hook_path)?;

        // Parse frontmatter
        let hook_type = if content.starts_with("---") {
            content
                .trim_start_matches("---")
                .split("---")
                .next()
                .and_then(|s| {
                    s.lines()
                        .find(|l| l.trim_start().starts_with("hook:"))
                        .map(|l| l.split(":").nth(1).unwrap_or("").trim().to_string())
                })
                .unwrap_or_else(|| "unknown".to_string())
        } else {
            "unknown".to_string()
        };

        hooks.push(HookInfo {
            name,
            hook_type,
            path: hook_path,
        });
    }

    Ok(hooks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_parse_hooks() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        create_hooks_dir(path, "test-hook", "pre-commit");
        create_hooks_dir(path, "another-hook", "post-commit");

        let hooks = parse_hooks(path).unwrap();
        assert_eq!(hooks.len(), 2);
    }

    #[test]
    fn test_missing_hooks_dir() {
        let dir = TempDir::new().unwrap();
        let hooks = parse_hooks(dir.path()).unwrap();
        assert!(hooks.is_empty());
    }

    fn create_hooks_dir(base: &Path, name: &str, hook_type: &str) {
        let hook_dir = base.join("hooks");
        std::fs::create_dir_all(&hook_dir).unwrap();

        let content = format!(
            r#"---
hook: {}
---
# {}
"#,
            hook_type, name
        );

        File::create(hook_dir.join(format!("{}.md", name)))
            .unwrap()
            .write_all(content.as_bytes())
            .unwrap();
    }
}
