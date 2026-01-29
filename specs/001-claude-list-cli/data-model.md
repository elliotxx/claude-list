# Data Model: CLAUDE-LIST CLI

## Overview

Core data structures for representing Claude Code environment information.

## Entity Definitions

### ClaudeInfo

Aggregated information about the Claude Code environment.

| Field | Type | Description |
|-------|------|-------------|
| `version` | String | CLAUDE-LIST version |
| `config_dir` | PathBuf | Path to `.claude` directory |
| `plugins` | Vec\<PluginInfo\> | List of installed plugins |
| `skills` | Vec\<SkillInfo\> | List of installed skills |
| `sessions` | SessionInfo | Session count and metadata |
| `mcp_servers` | Vec\<McpInfo\> | List of MCP servers |
| `hooks` | Vec\<HookInfo\> | List of installed hooks |
| `agents` | Vec\<AgentInfo\> | List of installed agents |

### Item (Base Type)

Unified representation for any installable component.

| Field | Type | Description |
|-------|------|-------------|
| `name` | String | Component name (unique within type) |
| `version` | Option\<String\> | Version string if available |
| `source` | Source | Component source (Official/ThirdParty) |
| `path` | PathBuf | Path to component files |
| `description` | Option\<String\> | Human-readable description |

### ItemKind

Enumeration of component types.

```rust
enum ItemKind {
    Plugin,
    Skill,
    Session,
    Mcp,
    Hook,
    Agent,
}
```

### Source

Enumeration of component sources.

```rust
enum Source {
    Official,    // Built-in or official distribution
    ThirdParty,  // Community or third-party
}
```

## Component-Specific Types

### PluginInfo

| Field | Type | Description |
|-------|------|-------------|
| `name` | String | Plugin identifier |
| `version` | Option\<String\> | Plugin version |
| `source` | Source | Official or third-party |
| `path` | PathBuf | Path to plugin config |

### SkillInfo

| Field | Type | Description |
|-------|------|-------------|
| `name` | String | Skill name |
| `version` | Option\<String\> | Version from skill.yaml |
| `source` | Source | Official or third-party |
| `path` | PathBuf | Path to skill directory |
| `description` | Option\<String\> | From skill.yaml |

### SessionInfo

| Field | Type | Description |
|-------|------|-------------|
| `count` | usize | Total number of recorded sessions |
| `last_session` | Option\<DateTime\<Utc\>\> | Timestamp of most recent session |

### McpInfo

| Field | Type | Description |
|-------|------|-------------|
| `name` | String | MCP server name |
| `status` | String | Connection status |
| `command` | Option\<String\> | Startup command |
| `path` | PathBuf | Path to mcp.json |

### HookInfo

| Field | Type | Description |
|-------|------|-------------|
| `name` | String | Hook filename (without extension) |
| `hook_type` | String | Type from frontmatter |
| `path` | PathBuf | Path to hook file |

### AgentInfo

| Field | Type | Description |
|-------|------|-------------|
| `name` | String | Agent name from frontmatter |
| `description` | Option\<String\> | From frontmatter |
| `path` | PathBuf | Path to agent file |

## Relationships

```
ClaudeInfo
├── plugins: Vec<PluginInfo>
├── skills: Vec<SkillInfo>
├── sessions: SessionInfo
├── mcp_servers: Vec<McpInfo>
├── hooks: Vec<HookInfo>
└── agents: Vec<AgentInfo>
```

All components are independent lists - no cross-references between component types.

## Validation Rules

- Component names must be non-empty
- Paths must be absolute or relative to `.claude` directory
- Version strings, if present, should match typical version formats (semver or similar)
- Source must be one of: Official, ThirdParty
