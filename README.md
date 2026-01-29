<div align="center">

  <div>
    <img src="assets/logo.svg" alt="claude-list Logo" width="128" height="128">
  </div>

  <h1 style="margin-top: 10px;">claude-list</h1>

  CLI tool for viewing installed plugins, skills, agents, and MCP servers in Claude Code.

  <div align="center">
    <a href="https://github.com/elliotxx/claude-list/actions"><img alt="CI Status" src="https://img.shields.io/github/actions/workflow/status/elliotxx/claude-list?logo=github"/></a>
    <a href="https://crates.io/crates/claude-list"><img alt="Crates.io" src="https://img.shields.io/crates/v/claude-list"/></a>
    <a href="https://github.com/elliotxx/claude-list/blob/main/LICENSE"><img alt="License" src="https://img.shields.io/badge/License-MIT-yellow.svg"/></a>
    <a href="https://www.rust-lang.org/"><img alt="Rust" src="https://img.shields.io/badge/Rust-1.75+-orange.svg"/></a>
  </div>

  <p>
    <a href="#why-claude-list">Why?</a>
    ◆ <a href="#quick-start">Quick Start</a>
    ◆ <a href="#features">Features</a>
    ◆ <a href="#installation">Installation</a>
    ◆ <a href="#architecture">Architecture</a>
  </p>
</div>

---

## Latest News 🔥

- **[2026/01]** Added Homebrew support with cargo-dist multi-platform builds
- **[2026/01]** Released v0.1.0 with compact, detailed, and JSON output modes
- **[2026/01]** Implemented 7 component parsers (plugins, skills, sessions, mcp, hooks, agents, commands)

---

## Why claude-list?

A Rust CLI tool that follows Unix philosophy—do one thing well. It reads your Claude Code `.claude` directory and presents key information in a clean, minimal format with modern aesthetics.

- **🎨 Clean Output** - Human-readable compact format by default
- **🔍 Detailed Views** - Version, source, and path information on demand
- **🤖 Scriptable** - JSON output for automation and integration
- **⚡ Fast** - Sub-second execution (< 0.03s)
- **🔒 Safe** - Handles missing files and partial data gracefully
- **📦 Multi-Platform** - Pre-built binaries for Linux and macOS

---

## Quick Start

```bash
# Install from crates.io
cargo install claude-list

# View your Claude Code environment
claude-list
```

**Need more details?** See [Installation](#installation) below for all installation options.

---

## Features

### Output Modes

| Mode | Command | Description |
|------|---------|-------------|
| Compact | `claude-list` | Summary with counts |
| Detailed | `claude-list -l` | Full info with version, source, path |
| JSON | `claude-list --json` | Machine-readable output |

### Filtering

Filter to show specific component types:

```bash
claude-list --plugins    # Only plugins
claude-list --skills     # Only skills
claude-list --sessions   # Only sessions
claude-list --mcp        # Only MCP servers
claude-list --hooks      # Only hooks
claude-list --agents     # Only agents
claude-list --commands   # Only commands

# Combine filters
claude-list --plugins --skills
```

### Custom Configuration Directory

```bash
claude-list --config /path/to/.claude
```

---

## Demo

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

---

## Installation

### Option 1: From crates.io (Recommended)

```bash
cargo install claude-list
```

### Option 2: From Homebrew (macOS)

```bash
brew install elliotxx/claude-list/claude-list
```

### Option 3: From GitHub Releases

Download pre-built binaries from [GitHub Releases](https://github.com/elliotxx/claude-list/releases):

```bash
# Linux (x86_64)
wget https://github.com/elliotxx/claude-list/releases/latest/download/claude-list-x86_64-unknown-linux-gnu.tar.gz
tar -xzf claude-list-x86_64-unknown-linux-gnu.tar.gz
./claude-list

# macOS (Apple Silicon)
wget https://github.com/elliotxx/claude-list/releases/latest/download/claude-list-aarch64-apple-darwin.tar.gz
tar -xzf claude-list-aarch64-apple-darwin.tar.gz
./claude-list
```

### Option 4: From Source

```bash
git clone https://github.com/elliotxx/claude-list.git
cd claude-list
cargo build --release
cargo install --path .
```

---

## Architecture

### System Overview

<div align="center">
  <img src="https://via.placeholder.com/750x400?text=claude-list+Architecture" alt="Architecture Diagram" width="750">
</div>

### Component Architecture

```
┌─────────────────────────────────────────────────────────┐
│                      User Input                          │
│                    (CLI Arguments)                       │
└──────────────────────┬──────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────┐
│  Main Entry Point (src/main.rs)                         │
│  • Parse CLI arguments                                  │
│  • Dispatch to formatters                               │
└──────────────────────┬──────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────┐
│  Parser Layer (src/parsers/)                            │
│  • plugins.rs    → Parse installed plugins              │
│  • skills.rs     → Parse skills                         │
│  • sessions.rs   → Parse session history                │
│  • mcp.rs        → Parse MCP servers                    │
│  • hooks.rs      → Parse hooks                          │
│  • agents.rs     → Parse agents                         │
│  • commands.rs   → Parse commands                       │
└──────────────────────┬──────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────┐
│  Formatter Layer (src/formatters/)                      │
│  • compact.rs    → Human-readable summary               │
│  • detailed.rs   → Full info with version/source/path   │
│  • json.rs       → Machine-readable JSON output         │
└─────────────────────────────────────────────────────────┘
```

### Key Design Decisions

- **Pattern Used**: Unix philosophy—single responsibility, compose simple tools
- **Technology Stack**: Rust 1.75+, clap (CLI), serde (JSON), anyhow (error handling)
- **Scalability**: Each parser is independent, easy to extend
- **Error Handling**: Graceful degradation for missing files

### Supported Data Sources

| Component | Format | Location |
|-----------|--------|----------|
| Plugins | JSON | `.claude/plugins/installed_plugins.json` |
| Skills | YAML | `.claude/skills/*/skill.yaml` |
| MCP Servers | Directory/YAML | `.claude/mcp-servers/*/` |
| Sessions | JSON Lines | `.claude/history.jsonl` |
| Commands | Markdown | `.claude/commands/*.md` |
| Agents | Markdown | `.claude/agents/*.md` |
| Hooks | Markdown | `.claude/hooks/*.md` |

---

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

# Run integration tests
cargo test --test cli_test
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Check format
cargo fmt --check

# Run clippy
cargo clippy --all-features -- -D warnings
```

### Testing

```bash
# Run all tests
cargo test --all-features

# Run with verbose output
cargo test --all-features --verbose

# Run specific test
cargo test test_name
```

---

## Publishing

Releases are automated via [cargo-dist](https://dist.clap.rs/):

1. Push a git tag matching `x.y.z` (e.g., `0.1.1`)
2. CI pipeline triggers automatically:
   - **Plan**: Generate build manifest
   - **Build**: Multi-platform builds (x86_64 Linux, x86_64/aarch64 macOS)
   - **Publish**: GitHub Release + crates.io + Homebrew

### Release Process

```bash
# 1. Update version in Cargo.toml
# Edit Cargo.toml: version = "0.1.0" → "0.1.1"

# 2. Commit version change
git add -A && git commit -m "chore: bump version to 0.1.1"

# 3. Create git tag
git tag 0.1.1

# 4. Push to GitHub (including tags)
git push && git push --tags
```

### CI Tokens Required

| Secret | Purpose |获取位置|
|--------|---------|--------|
| `CARGO_REGISTRY_TOKEN` | Publish to crates.io | [crates.io/settings/tokens](https://crates.io/settings/tokens) |
| `HOMEBREW_TAP_TOKEN` | Publish to Homebrew | [GitHub Settings](https://github.com/settings/tokens) |

---

## Contributing

We welcome contributions! Feel free to submit issues and pull requests.

### Contribution Areas

- **Feature Development**: Add new parsers or formatters
- **Bug Fixes**: Fix issues and improve stability
- **Documentation**: Improve guides and examples
- **Testing**: Add tests and improve coverage

### Quick Start for Contributors

```bash
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/claude-list.git
cd claude-list

# Follow installation steps above

# Create feature branch
git checkout -b feature/your-feature-name

# Make changes, test, then commit and push
git add .
git commit -m "feat: description"
git push origin feature/your-feature-name
```

---

## License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

- [Claude Code](https://claude.com/claude-code) for the inspiration
- [clap](https://github.com/clap-rs/clap) for CLI argument parsing
- [cargo-dist](https://github.com/axodotdev/cargo-dist) for automated releases

---

<div align="center">
  <p>
    <strong>Built with ❤️ for the Claude Code community</strong><br>
    <sub>Parse and display your Claude Code environment</sub>
  </p>
</div>
