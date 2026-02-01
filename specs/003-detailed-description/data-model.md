# Data Model: Detailed Output with Description Feature

## Component Info Structures (Modified)

### PluginInfo

**Change**: Add `description` field (derived)

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: Option<String>,
    pub source: Source,
    pub path: PathBuf,
    // New field - derived from source
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
```

**Description derivation**: "Official plugin" or "Third-party plugin"

### McpInfo

**Change**: Add `description` field (derived)

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct McpInfo {
    pub name: String,
    pub status: String,
    pub command: Option<String>,
    pub path: PathBuf,
    // New field - derived from status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
```

**Description derivation**: "{status} MCP server" (e.g., "connected MCP server")

### HookInfo

**Change**: Add `description` field (derived)

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HookInfo {
    pub name: String,
    pub hook_type: String,
    pub path: PathBuf,
    // New field - derived from hook_type
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
```

**Description derivation**: "{hook_type} hook" (e.g., "pre-commit hook")

## Utility Types

### TruncationConfig

Configuration for description truncation:

```rust
pub struct TruncationConfig {
    /// Maximum display width for description column
    pub max_description_width: usize,
    /// Ellipsis string to append when truncating
    pub ellipsis: &'static str,
    /// Placeholder when no description available
    pub no_description_placeholder: &'static str,
}

impl Default for TruncationConfig {
    fn default() -> Self {
        Self {
            max_description_width: 50,
            ellipsis: "...",
            no_description_placeholder: "-",
        }
    }
}
```

### DescriptionProvider

Trait for deriving component descriptions:

```rust
pub trait DescriptionProvider {
    fn get_description(&self) -> Option<&str>;
}

impl DescriptionProvider for PluginInfo {
    fn get_description(&self) -> Option<&str> {
        self.description.as_deref().or_else(|| {
            Some(match self.source {
                Source::Official => "Official plugin",
                Source::ThirdParty => "Third-party plugin",
            })
        })
    }
}

impl DescriptionProvider for SkillInfo {
    fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

impl DescriptionProvider for McpInfo {
    fn get_description(&self) -> Option<&str> {
        self.description.as_deref().or_else(|| {
            Some(&format!("{} MCP server", self.status))
        })
    }
}

impl DescriptionProvider for HookInfo {
    fn get_description(&self) -> Option<&str> {
        self.description.as_deref().or_else(|| {
            Some(&format!("{} hook", self.hook_type))
        })
    }
}

impl DescriptionProvider for AgentInfo {
    fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

impl DescriptionProvider for CommandInfo {
    fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
```

## Column Layout

### Detailed Output Format (--long)

```
CLAUDE-LIST v0.1.4
CONFIG: /Users/test/.claude

PLUGINS    2 installed
  NAME                            SOURCE         DESCRIPTION
  ------------------------------  -------------  --------------------------------------------------
  context7                        official       Official plugin

SKILLS     1 available
  NAME                            SOURCE         DESCRIPTION
  ...

MCP        1 servers
  NAME                            STATUS         DESCRIPTION
  ...

HOOKS      2 configured
  NAME                            TYPE           DESCRIPTION
  ...

AGENTS     1 defined
  NAME                            DESCRIPTION
  ...

COMMANDS   1 available
  NAME                            DESCRIPTION
  ...
```

### Column Widths

| Column | Width | Alignment | Notes |
|--------|-------|-----------|-------|
| NAME | 30 | Left | Fixed width, colored |
| SOURCE | 15 | Left | "official" / "third-party" |
| DESCRIPTION | 50 | Left | Truncated with "..." |
| STATUS | 18 | Left | MCP status |
| TYPE | 18 | Left | Hook type |
