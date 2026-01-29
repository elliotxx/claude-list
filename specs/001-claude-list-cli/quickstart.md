# Quickstart: CLAUDE-LIST CLI

## Prerequisites

- Rust 1.75 or later
- Cargo (comes with Rust)
- Git

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/claude-list.git
cd claude-list

# Build and install
cargo install --path .

# Verify installation
claude-list --help
```

### From Crates.io (when published)

```bash
cargo install claude-list
```

## Development Setup

### Initial Setup

```bash
# Clone and enter directory
git clone <repo-url>
cd claude-list

# Build in debug mode
cargo build

# Run tests
cargo test

# Run with sample data
cargo run -- --config tests/fixtures/.claude
```

### Project Structure

```
claude-list/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── cli.rs           # Arguments definition
│   ├── lib.rs           # Library root
│   ├── info.rs          # Data structures
│   ├── error.rs         # Error types
│   ├── parsers/         # Component parsers
│   └── formatters/      # Output formatters
├── tests/
│   ├── unit/            # Unit tests
│   ├── integration/     # CLI tests
│   └── fixtures/        # Test data
└── Cargo.toml
```

### Adding a New Parser

1. Create new file in `src/parsers/` (e.g., `custom.rs`)
2. Implement `Parser` trait:

```rust
use anyhow::Result;
use std::path::Path;

pub struct CustomParser;

impl CustomParser {
    pub fn parse(path: &Path) -> Result<Vec<CustomItem>> {
        // Parse logic here
        Ok(vec![])
    }
}
```

3. Add module to `src/parsers/mod.rs`

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run integration tests
cargo test --test integration
```

### Code Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Lint
cargo clippy
```

## Usage

### Basic Usage

```bash
# Show all components (compact)
claude-list

# Show detailed info
claude-list -l

# Show full info with paths
claude-list -ll

# Filter by type
claude-list --plugins
claude-list --skills
claude-list --sessions
claude-list --mcp
claude-list --hooks
claude-list --agents

# JSON output
claude-list --json

# Custom .claude directory
claude-list -C /path/to/.claude
```

### Example Output

```
CLAUDE-LIST v1.0.0

CONFIG: /Users/user/.claude

PLUGINS    3 installed
  context7  plugin_context7  plugin_playwright
SKILLS     12 available
  brainstorming  claude-code-guide  frontend-design  ...
SESSIONS   47 recorded
MCP        2 servers
```

## Common Tasks

### Adding a New Component Type

1. Add to `ItemKind` enum in `src/info.rs`
2. Create parser in `src/parsers/`
3. Add formatter method in `src/formatters/`
4. Update `ClaudeInfo` struct
5. Add tests in `tests/`
6. Update this quickstart

### Modifying Output Format

1. Edit formatter in `src/formatters/`:
   - `compact.rs` for default output
   - `detailed.rs` for `-l` output
   - `json.rs` for `--json` output
2. Run snapshot tests to update expectations
3. Commit updated snapshots

## Troubleshooting

### "No such file or directory" error

Ensure `.claude` directory exists or use `-C` flag to specify path.

### Build failures

```bash
# Update dependencies
cargo update

# Clean build
cargo clean
cargo build
```

### Test failures

```bash
# Run with verbose output
cargo test -- --verbose

# Check for snapshot drift
cargo test -- --include-ignored
```
