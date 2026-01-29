//! Parse installed skills from skills/ directory

use crate::error::Result;
use crate::info::{SkillInfo, Source};
use serde_yaml::Value;
use std::fs;
use std::path::Path;

pub fn parse_skills(base_path: &Path) -> Result<Vec<SkillInfo>> {
    let skills_dir = base_path.join("skills");

    if !skills_dir.exists() || !skills_dir.is_dir() {
        return Ok(vec![]);
    }

    let mut skills = Vec::new();

    for entry in fs::read_dir(&skills_dir)? {
        let entry = entry?;
        let skill_path = entry.path();

        if !skill_path.is_dir() {
            continue;
        }

        let name = skill_path
            .file_name()
            .and_then(|n| n.to_str())
            .map(String::from)
            .unwrap_or_default();

        // Try SKILL.md with frontmatter first (new format)
        let skill_md_path = skill_path.join("SKILL.md");
        let mut version = None;
        let mut description = None;

        if skill_md_path.exists() {
            if let Ok(content) = fs::read_to_string(&skill_md_path) {
                if content.starts_with("---") {
                    // Parse frontmatter
                    if let Some(frontmatter) = content.trim_start_matches("---")
                        .split("---")
                        .next()
                    {
                        if let Ok(yaml) = serde_yaml::from_str::<Value>(frontmatter) {
                            description = yaml
                                .get("description")
                                .and_then(|v| v.as_str())
                                .map(String::from);
                            version = yaml
                                .get("version")
                                .and_then(|v| v.as_str())
                                .map(String::from);
                        }
                    }
                }
            }
        }

        // Fallback to skill.yaml (old format)
        if description.is_none() {
            let yaml_path = skill_path.join("skill.yaml");
            if yaml_path.exists() {
                if let Ok(content) = fs::read_to_string(&yaml_path) {
                    if let Ok(yaml) = serde_yaml::from_str::<Value>(&content) {
                        version = yaml
                            .get("version")
                            .and_then(|v| v.as_str())
                            .map(String::from);
                        description = yaml
                            .get("description")
                            .and_then(|v| v.as_str())
                            .map(String::from);
                    }
                }
            }
        }

        let source = if name.starts_with('_') || name.contains('-') {
            Source::ThirdParty
        } else {
            Source::Official
        };

        skills.push(SkillInfo {
            name,
            version,
            source,
            path: skill_path,
            description,
        });
    }

    Ok(skills)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{create_dir_all, File};
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_parse_skills() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        create_dir_all(path.join("skills/official-skill")).unwrap();
        create_dir_all(path.join("skills/custom-skill")).unwrap();

        File::create(path.join("skills/official-skill/skill.yaml"))
            .unwrap()
            .write_all(b"name: official\nversion: 1.0.0\ndescription: An official skill")
            .unwrap();

        File::create(path.join("skills/custom-skill/skill.yaml"))
            .unwrap()
            .write_all(b"name: custom\nversion: 2.0.0\n")
            .unwrap();

        let skills = parse_skills(path).unwrap();
        assert_eq!(skills.len(), 2);
    }

    #[test]
    fn test_parse_skills_from_skill_md() {
        // Test new format: skills/*/SKILL.md with frontmatter
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        create_dir_all(path.join("skills/api-design-principles")).unwrap();
        create_dir_all(path.join("skills/custom-tool")).unwrap();

        // Create SKILL.md with frontmatter (actual format)
        let skill_md_1 = r#"---
name: api-design-principles
description: Master REST and GraphQL API design principles to build intuitive, scalable, and maintainable APIs that delight developers. Use when designing new APIs, reviewing API specifications, or establishing API design standards.
---

# API Design Principles
"#;

        let skill_md_2 = r#"---
name: custom-tool
description: A custom tool for special tasks
---

# Custom Tool
"#;

        File::create(path.join("skills/api-design-principles/SKILL.md"))
            .unwrap()
            .write_all(skill_md_1.as_bytes())
            .unwrap();

        File::create(path.join("skills/custom-tool/SKILL.md"))
            .unwrap()
            .write_all(skill_md_2.as_bytes())
            .unwrap();

        let skills = parse_skills(path).unwrap();

        // Should parse 2 skills
        assert_eq!(skills.len(), 2);

        // Check description is parsed from frontmatter
        let api_skill = skills.iter().find(|s| s.name == "api-design-principles").unwrap();
        assert!(api_skill.description.is_some());
        assert!(api_skill.description.as_ref().unwrap().contains("REST and GraphQL"));

        // custom-tool should be ThirdParty (has hyphen)
        let custom_skill = skills.iter().find(|s| s.name == "custom-tool").unwrap();
        assert_eq!(custom_skill.source, Source::ThirdParty);
    }

    #[test]
    fn test_missing_skills_dir() {
        let dir = TempDir::new().unwrap();
        let skills = parse_skills(dir.path()).unwrap();
        assert!(skills.is_empty());
    }
}
