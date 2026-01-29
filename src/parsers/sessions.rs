//! Parse session history from history.jsonl

use crate::error::Result;
use crate::info::SessionInfo;
use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn parse_sessions(base_path: &Path) -> Result<SessionInfo> {
    // Try new format: history.jsonl (JSON Lines)
    let history_path = base_path.join("history.jsonl");

    if history_path.exists() {
        return parse_sessions_from_jsonl(&history_path);
    }

    // Fallback to old format: session_history.json
    let sessions_path = base_path.join("session_history.json");

    if !sessions_path.exists() {
        return Ok(SessionInfo {
            count: 0,
            last_session: None,
        });
    }

    let content = fs::read_to_string(&sessions_path)?;
    let json: Value = serde_json::from_str(&content)?;

    let sessions = json
        .get("sessions")
        .and_then(|v| v.as_array())
        .map(|arr| arr.len())
        .unwrap_or(0);

    let last_session = json
        .get("sessions")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.last())
        .and_then(|s| s.get("id").or(s.get("timestamp")).or(s.get("time")))
        .and_then(|v| v.as_str())
        .map(String::from);

    Ok(SessionInfo {
        count: sessions,
        last_session,
    })
}

/// Parse history.jsonl (JSON Lines format)
fn parse_sessions_from_jsonl(history_path: &Path) -> Result<SessionInfo> {
    let content = fs::read_to_string(history_path)?;

    let mut count = 0;
    let mut last_timestamp: Option<String> = None;

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if let Ok(json) = serde_json::from_str::<Value>(line) {
            count += 1;

            // Get timestamp (can be u64 or string)
            if let Some(ts) = json.get("timestamp") {
                let ts_str = match ts {
                    Value::Number(n) => n.as_u64().map(|v| v.to_string()),
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                };
                if let Some(ref ts_val) = ts_str {
                    if last_timestamp.is_none() || ts_val > &last_timestamp.clone().unwrap() {
                        last_timestamp = Some(ts_val.clone());
                    }
                }
            }
        }
    }

    Ok(SessionInfo {
        count,
        last_session: last_timestamp,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_parse_sessions() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        let sessions = r#"{
            "sessions": [
                {"id": "1", "timestamp": "2025-01-28T10:00:00Z"},
                {"id": "2", "timestamp": "2025-01-29T10:00:00Z"}
            ]
        }"#;

        File::create(path.join("session_history.json"))
            .unwrap()
            .write_all(sessions.as_bytes())
            .unwrap();

        let result = parse_sessions(path).unwrap();
        assert_eq!(result.count, 2);
        assert!(result.last_session.is_some());
    }

    #[test]
    fn test_parse_sessions_from_jsonl() {
        // Test new format: history.jsonl (JSON Lines)
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        // Create history.jsonl
        let jsonl_content = r#"{"display":"/help","pastedContents":{},"timestamp":1766567598086,"project":"/test"}
{"display":"/model","pastedContents":{},"timestamp":1766567616338,"project":"/test"}
{"display":"/agents","pastedContents":{},"timestamp":1766567624402,"project":"/test"}
"#;

        File::create(path.join("history.jsonl"))
            .unwrap()
            .write_all(jsonl_content.as_bytes())
            .unwrap();

        let result = parse_sessions(path).unwrap();
        assert_eq!(result.count, 3);
        // Last timestamp should be 1766567624402
        assert_eq!(result.last_session, Some("1766567624402".to_string()));
    }

    #[test]
    fn test_empty_sessions() {
        let dir = TempDir::new().unwrap();
        let result = parse_sessions(dir.path()).unwrap();
        assert_eq!(result.count, 0);
        assert!(result.last_session.is_none());
    }

    #[test]
    fn test_sessions_malformed_jsonl() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        // Valid line followed by invalid line
        let jsonl_content = r#"{"timestamp":"2025-01-01T00:00:00Z"}
invalid json
{"timestamp":"2025-01-02T00:00:00Z"}
"#;
        File::create(path.join("history.jsonl"))
            .unwrap()
            .write_all(jsonl_content.as_bytes())
            .unwrap();

        let result = parse_sessions(path).unwrap();
        // Should count valid lines only
        assert_eq!(result.count, 2);
    }

    #[test]
    fn test_sessions_empty_file() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        File::create(path.join("history.jsonl"))
            .unwrap()
            .write_all(b"")
            .unwrap();

        let result = parse_sessions(path).unwrap();
        assert_eq!(result.count, 0);
        assert!(result.last_session.is_none());
    }

    #[test]
    fn test_sessions_single_line() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        let jsonl_content = r#"{"timestamp":"2025-01-15T10:30:00Z"}"#;
        File::create(path.join("history.jsonl"))
            .unwrap()
            .write_all(jsonl_content.as_bytes())
            .unwrap();

        let result = parse_sessions(path).unwrap();
        assert_eq!(result.count, 1);
        assert_eq!(result.last_session, Some("2025-01-15T10:30:00Z".to_string()));
    }

    #[test]
    fn test_session_history_json_fallback() {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        // Create old format session_history.json
        let sessions_json = r#"{
            "sessions": [
                {"id": "1", "timestamp": "2025-01-01T10:00:00Z"},
                {"id": "2", "timestamp": "2025-01-02T10:00:00Z"}
            ]
        }"#;
        File::create(path.join("session_history.json"))
            .unwrap()
            .write_all(sessions_json.as_bytes())
            .unwrap();

        let result = parse_sessions(path).unwrap();
        assert_eq!(result.count, 2);
    }
}
