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
# Debug build
cargo build

# Release build
cargo build --release
```

## Development

### Quick Start

```bash
# Show available commands
make

# Run all checks (fmt + clippy + test)
make check

# Run tests only
make test

# Run with test fixtures
make run-test
```

### Makefile Commands

| Command | Description |
|---------|-------------|
| `make` | Show help (default) |
| `make check` | Run all checks (fmt + clippy + test) |
| `make test` | Run unit tests |
| `make lint` | Run code linting |
| `make clippy` | Run static analysis |
| `make fmt` | Check code format |
| `make fmt-fix` | Auto-fix format issues |
| `make build` | Debug build |
| `make build-release` | Release build |
| `make run` | Run with `~/.claude` |
| `make run-test` | Run with test fixtures |
| `make check-diff` | Show uncommitted changes |
| `make clean` | Clean build artifacts |

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with verbose output
cargo test --all-features --verbose
```

### Code Quality

```bash
# Check format
cargo fmt --check

# Run clippy
cargo clippy --all-features -- -D warnings
```

## Supported Data Sources

The parser supports both new and legacy formats for backward compatibility:

| Component | New Format | Legacy Format |
|-----------|------------|---------------|
| Plugins | `plugins/installed_plugins.json` (v2) | `settings.json` |
| Skills | `skills/*/SKILL.md` | `skills/*/skill.yaml` |
| MCP | `mcp-servers/*/` | `mcp.json` |
| Sessions | `history.jsonl` | `session_history.json` |
| Commands | `commands/*.md` | - |
| Agents | `agents/*.md` | - |
| Hooks | `hooks/*.md` | - |

## License

MIT
