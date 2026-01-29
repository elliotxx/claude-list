# CLAUDE-LIST Design Document

**Date**: 2025-01-29
**Status**: Approved

## Overview

A Rust CLI tool that parses the `.claude` directory structure (as defined by Anthropic) and displays key information: installed plugins, skills, agents, session count, and more.

**Design Philosophy**:
- Unix philosophy: do one thing well
- Minimalist output by default, expand on demand (`ls` style)
- Beautiful modern CLI aesthetic without bloat

## Architecture

```
Input → Parsers → Aggregator → Formatters → Output
```

### Core Components

| Component | Responsibility |
|-----------|---------------|
| Parsers | Read `.claude` subdirectories and config files |
| Aggregator | Collect, sort, group, and filter parsed data |
| Formatters | Transform data into requested output format |

### Data Flow

1. CLI parses arguments (compact/detailed/json, filter by type)
2. Run all configured parsers against `.claude` directory
3. Aggregator combines results, handles errors gracefully
4. Formatter renders output based on flags
5. Print to stdout

## CLI Interface

```bash
# Default: compact overview
claude-list

# Detailed output (ls -l style)
claude-list -l

# Full information
claude-list -ll

# Filter by type
claude-list --plugins
claude-list --skills
claude-list --sessions
claude-list --mcp

# JSON output for scripting
claude-list --json

# Custom .claude directory
claude-list -C /path/to/.claude
```

### Output Levels

| Level | Flag | Description |
|-------|------|-------------|
| Compact | (default) | Name + status only |
| Detailed | `-l` | Name + version + source + path |
| Full | `-ll` | Complete info + usage statistics |
| JSON | `--json` | Machine-readable output |

## Data Sources

### `.claude` Directory Structure

```
.claude/
├── CLAUDE.md              # Project rules
├── README.md              # Project readme
├── settings.json          # User settings
├── hooks/                 # User hooks
│   └── *.md
├── skills/                # Installed skills
│   └── [skill-name]/
│       ├── skill.yaml
│       └── skill.md
├── agents/                # Custom agents
│   └── [agent-name].md
├── mcp.json               # MCP server config
├── session_history.json   # Session history
└── project_constitutions/ # Project constitutions
    └── *.md
```

### Parsers

| Parser | Source | Outputs |
|--------|--------|---------|
| PluginsParser | `settings.json` → `installed_plugins` | name, version |
| SkillsParser | `skills/[name]/` | name, description, source |
| SessionsParser | `session_history.json` | count, last timestamp |
| McpParser | `mcp.json` | server name, status |
| HooksParser | `hooks/` | hook name, type |
| AgentsParser | `agents/` | agent name, description |

## Output Examples

### Default (Compact)

```
CLAUDE-LIST v1.0.0

CONFIG: /Users/yym/.claude

PLUGINS    3 installed
  context7  plugin_context7  plugin_playwright
SKILLS     12 available
  brainstorming  claude-code-guide  frontend-design  ...
SESSIONS   47 recorded
MCP        2 servers
```

### Detailed (`-l`)

```
PLUGINS    3 installed
  context7       v2.1.0  official
  plugin_context7  v1.0  local
  plugin_playwright  v1.0  local

SKILLS     12 available
  brainstorming    -      user-defined
  claude-code-guide  -    built-in
  frontend-design   v0.3  user-defined

SESSIONS   47 recorded (last: 2h ago)
MCP        2 servers
 MiniMax    connected
  ChromeDevTools  connected
```

## Error Handling

- **Partial failures**: Continue if one parser fails
- **Missing files**: Skip gracefully (e.g., no `session_history.json`)
- **Parse errors**: Show `?` or `unknown`, don't crash
- **Exit codes**: 0=success, 1=error, 2=partial data unavailable

## Project Structure

```
claude-list/
├── Cargo.toml
├── src/
│   ├── main.rs           # CLI entry point
│   ├── cli.rs            # Arg parsing
│   ├── lib.rs            # Library root
│   ├── info.rs           # ClaudeInfo struct
│   ├── formatters/
│   │   ├── mod.rs
│   │   ├── compact.rs
│   │   ├── detailed.rs
│   │   └── json.rs
│   └── parsers/
│       ├── mod.rs
│       ├── plugins.rs
│       ├── skills.rs
│       ├── sessions.rs
│       ├── mcp.rs
│       ├── hooks.rs
│       └── agents.rs
├── tests/
│   ├── integration/
│   └── fixtures/
│       └── .claude/
└── .specify/
```

## Key Types

```rust
struct ClaudeInfo {
    version: String,
    config_dir: PathBuf,
    plugins: Vec<PluginInfo>,
    skills: Vec<SkillInfo>,
    sessions: Vec<SessionInfo>,
    mcp_servers: Vec<McpInfo>,
}

struct Item {
    name: String,
    kind: ItemKind,
    version: Option<String>,
    source: Source,        // official / built-in / local / third-party
    path: PathBuf,
    description: Option<String>,
    extra: HashMap<String, Value>,
}

enum ItemKind {
    Plugin,
    Skill,
    Session,
    Mcp,
    Hook,
    Agent,
}
```

## Dependencies

```toml
[dependencies]
clap = { version = "4.4", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
```

## Testing Strategy

- **Unit tests**: Parser correctness
- **Integration tests**: CLI output verification
- **Snapshot tests**: Output format consistency
- **Fixtures**: Mock `.claude` directory structure

## Grouping & Sorting

- **Group by type**: PLUGINS / SKILLS / SESSIONS / MCP
- **Sort by source**: official → built-in → local → third-party
- **Empty groups**: Not displayed

## Design Principles

1. **Unix philosophy**: One tool, one purpose
2. **Progressive disclosure**: Default compact, expand on request
3. **Graceful degradation**: Partial failures don't crash
4. **Parse-friendly**: JSON output for scripting
5. **YAGNI**: No features not discussed in this doc
