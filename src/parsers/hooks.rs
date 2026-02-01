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
            description: None,
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

    #[test]
    fn test_hooks_without_frontmatter() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        let hooks_dir = path.join("hooks");
        std::fs::create_dir_all(&hooks_dir).unwrap();

        // Hook without frontmatter
        File::create(hooks_dir.join("no-frontmatter.md"))
            .unwrap()
            .write_all(b"# Just a markdown file\nNo hook type defined\n")
            .unwrap();

        let hooks = parse_hooks(path).unwrap();
        assert_eq!(hooks.len(), 1);
        assert_eq!(hooks[0].name, "no-frontmatter");
        assert_eq!(hooks[0].hook_type, "unknown");
    }

    #[test]
    fn test_hooks_malformed_frontmatter() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        let hooks_dir = path.join("hooks");
        std::fs::create_dir_all(&hooks_dir).unwrap();

        // Malformed frontmatter (missing closing ---) but has valid hook line
        File::create(hooks_dir.join("bad-frontmatter.md"))
            .unwrap()
            .write_all(b"---\nhook: pre-commit\n# Missing closing\n")
            .unwrap();

        let hooks = parse_hooks(path).unwrap();
        assert_eq!(hooks.len(), 1);
        assert_eq!(hooks[0].name, "bad-frontmatter");
        // hook: line is still parsed even without closing ---
        assert_eq!(hooks[0].hook_type, "pre-commit");
    }

    #[test]
    fn test_hooks_empty_hook_type() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        let hooks_dir = path.join("hooks");
        std::fs::create_dir_all(&hooks_dir).unwrap();

        // Empty hook type
        let content = r#"---
hook:
---
# Test
"#;
        File::create(hooks_dir.join("empty-type.md"))
            .unwrap()
            .write_all(content.as_bytes())
            .unwrap();

        let hooks = parse_hooks(path).unwrap();
        assert_eq!(hooks.len(), 1);
        assert_eq!(hooks[0].hook_type, "");
    }

    #[test]
    fn test_hooks_non_md_files_ignored() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        let hooks_dir = path.join("hooks");
        std::fs::create_dir_all(&hooks_dir).unwrap();

        // Create non-markdown files
        File::create(hooks_dir.join("script.sh")).unwrap();
        File::create(hooks_dir.join("data.json")).unwrap();
        File::create(hooks_dir.join("readme.md")).unwrap();

        let hooks = parse_hooks(path).unwrap();
        assert_eq!(hooks.len(), 1);
        assert_eq!(hooks[0].name, "readme");
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
