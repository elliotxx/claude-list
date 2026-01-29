# claude-list

Parse and display Claude Code `.claude` directory information: plugins, skills, agents, sessions, and more.

## Overview

A Rust CLI tool that follows Unix philosophy—do one thing well. It reads your Claude Code configuration directory and presents key information in a clean, minimal format with modern aesthetics.

## Features

- **Minimal by default**: Shows only essential info
- **Progressive disclosure**: `-l` for details, `-ll` for full info
- **Machine-friendly**: `--json` output for scripting
- **Graceful**: Handles missing files and partial data

## Installation

```bash
cargo install claude-list
```

## Usage

```bash
# Compact overview (default)
claude-list

# Detailed output
claude-list -l

# Full information with stats
claude-list -ll

# Filter by type
claude-list --plugins
claude-list --skills
claude-list --sessions
claude-list --mcp

# JSON output for scripts
claude-list --json

# Custom .claude directory
claude-list -C /path/to/.claude
```

## Example Output

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

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
```

## License

MIT
