# claude-list

CLI tool for viewing installed plugins, skills, agents, and MCP servers in Claude Code.

[![CI](https://github.com/elliotxx/claude-list/workflows/CI/badge.svg)](https://github.com/elliotxx/claude-list/actions)
[![Crates.io](https://img.shields.io/crates/v/claude-list)](https://crates.io/crates/claude-list)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

A Rust CLI tool that follows Unix philosophy—do one thing well. It reads your Claude Code configuration directory and presents key information in a clean, minimal format with modern aesthetics.

## Features

- **Minimal by default**: Shows only essential info
- **Progressive disclosure**: `-l` for details, `--output detailed` for full info
- **Machine-friendly**: `--json` output for scripting
- **Filtering**: Filter by component type (`--plugins`, `--skills`, `--mcp`, etc.)
- **Graceful**: Handles missing files and partial data

## Installation

### From source

```bash
cargo install claude-list
```

### From crates.io

```bash
cargo install claude-list
```

### From Homebrew (macOS)

```bash
brew install claude-list
```

## Usage

```bash
# Compact overview (default)
claude-list

# Detailed output with version, source, and path
claude-list -l
claude-list --output detailed

# JSON output for scripts
claude-list --json

# Filter by type
claude-list --plugins
claude-list --skills
claude-list --sessions
claude-list --mcp
claude-list --hooks
claude-list --agents
claude-list --commands

# Combine filters
claude-list --plugins --skills

# Custom .claude directory
claude-list --config /path/to/.claude
```

## Example Output

### Compact Mode (Default)

```
CLAUDE-LIST v0.1.0

CONFIG: /Users/user/.claude

PLUGINS    3 installed
  context7
  plugin_playwright
  plugin_example

SKILLS     12 available
  brainstorming
  claude-code-guide
  ...

SESSIONS   47 recorded
MCP        2 servers
  test-mcp
  another-mcp
```

### Detailed Mode (`-l`)

```
CLAUDE-LIST v0.1.0

CONFIG: /Users/user/.claude

PLUGINS    3 installed
  NAME                 VERSION  SOURCE     PATH
  -------------------  -------  ---------  ---------------------------------
  context7             2.1.0    official   /Users/user/.claude/settings.json
  plugin_playwright    1.0.0    third-party /Users/user/.claude/settings.json
  plugin_example       0.5.0    community  /Users/user/.claude/settings.json

SKILLS     12 available
  NAME                 VERSION  SOURCE     PATH
  ...
```

### JSON Mode (`--json`)

```json
{
  "version": "0.1.0",
  "config_dir": "/Users/user/.claude",
  "plugins": [...],
  "skills": [...],
  "sessions": {...},
  "mcp_servers": [...],
  "hooks": [...],
  "agents": [...],
  "commands": [...]
}
```

## Building

```bash
# Debug build
cargo build

# Release build (recommended for installation)
cargo build --release

# Install from source
cargo install --path .
```

## Development

### Prerequisites

- Rust 1.75+
- Cargo

### Quick Start

```bash
# Run all checks
cargo fmt && cargo clippy && cargo test

# Run tests only
cargo test

# Run tests with fixtures
cargo test --test cli_test
```

### Code Quality

```bash
# Check format
cargo fmt --check

# Run clippy
cargo clippy --all-features -- -D warnings

# Fix format issues
cargo fmt
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with verbose output
cargo test --all-features --verbose

# Run integration tests
cargo test --test cli_test
```

## Publishing

This section describes how to publish claude-list to crates.io.

### Version Management

claude-list follows [Semantic Versioning](https://semver.org/):

```
MAJOR.MINOR.PATCH
0.1.0, 0.1.1, 1.0.0
```

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

### Publishing Checklist

Before publishing, ensure:

- [ ] All tests pass: `cargo test --all-features`
- [ ] Clippy passes: `cargo clippy --all-features -- -D warnings`
- [ ] Format check: `cargo fmt --check`
- [ ] README is up to date
- [ ] LICENSE file exists

### Release Process

```bash
# 1. Update version in Cargo.toml
# Edit Cargo.toml: version = "0.1.0" → "0.1.1"

# 2. Commit version change
git add -A && git commit -m "chore: bump version to 0.1.1"

# 3. Create git tag (Go style: push tag triggers CI release)
git tag v0.1.1

# 4. Push to GitHub (including tags)
# CI will automatically:
#   - Validate (fmt + clippy + test)
#   - Build multi-platform binaries (Linux/macOS)
#   - Create GitHub Release with artifacts
#   - Publish to crates.io
git push && git push --tags
```

### CI Release Pipeline (cargo-dist)

Releases are automated via [cargo-dist](https://dist.clap.rs/):

1. Push a git tag matching `v*` (e.g., `v0.1.1`)
2. CI pipeline triggers automatically:
   - **Plan**: Generate build manifest
   - **Build**: Multi-platform builds (x86_64 Linux, x86_64/aarch64 macOS)
   - **Publish**: GitHub Release + crates.io

### Configuring CI Token

Before first release, add `CARGO_REGISTRY_TOKEN` to GitHub secrets:

1. Get token from https://crates.io/settings/tokens
2. Go to https://github.com/elliotxx/claude-list/settings/secrets/actions
3. Add new secret: `CARGO_REGISTRY_TOKEN`
4. CI will use this token automatically on release

### Installing Published Version

After publishing, users can install via:

```bash
# From crates.io
cargo install claude-list

# From source
cargo install --git https://github.com/elliotxx/claude-list
```

## Supported Data Sources

The parser supports both new and legacy formats for backward compatibility:

| Component | New Format | Legacy Format |
|-----------|------------|---------------|
| Plugins | `plugins/installed_plugins.json` (v2) | `settings.json` |
| Skills | `skills/*/skill.yaml` | - |
| MCP | `mcp-servers/*/` | `mcp.json` |
| Sessions | `history.jsonl` | `session_history.json` |
| Commands | `commands/*.md` | - |
| Agents | `agents/*.md` | - |
| Hooks | `hooks/*.md` | - |

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Claude Code](https://claude.com/claude-code) for the inspiration
- [clap](https://github.com/clap-rs/clap) for CLI argument parsing
- [anyhow](https://github.com/dtolnay/anyhow) for error handling
