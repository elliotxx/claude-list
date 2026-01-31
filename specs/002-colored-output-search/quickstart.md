# Quickstart Guide: v0.1.2 Features

**Date**: 2026-01-30
**Feature**: 002-colored-output-search

## Overview

This guide covers the new colored output and search features in claude-list v0.1.2.

## Prerequisites

- claude-list v0.1.2 or later
- Terminal that supports ANSI colors (most modern terminals)

## Installation

```bash
# Option 1: Homebrew (macOS)
brew tap elliotxx/tap && brew install elliotxx/tap/claude-list

# Option 2: Shell script
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/elliotxx/claude-list/releases/latest/download/claude-list-installer.sh | sh

# Option 3: From crates.io
cargo install claude-list
```

## Feature 1: Colored Output

### Automatic Color Detection

Colors are automatically enabled when:
- Running in a terminal (TTY)
- `NO_COLOR` environment variable is not set
- `--no-color` flag is not used

### Color Scheme

| Component Type | Color | Example |
|----------------|-------|---------|
| Plugins | Blue | `context7` |
| Skills | Green | `brainstorming` |
| MCP Servers | Yellow | `test-mcp` |
| Hooks | Magenta | `pre-commit` |
| Agents | Red | `database-agent` |
| Commands | Orange | `analyze-code` |
| Version Numbers | Gray | `1.0.0` |

### Examples

```bash
# Normal output with colors
claude-list

# Detailed output with colors
claude-list -l

# Search results with colors
claude-list --search context
```

### Disabling Colors

```bash
# Using flag
claude-list --no-color

# Using environment variable
NO_COLOR=1 claude-list

# Pipe to file (automatically disables)
claude-list > output.txt
```

## Feature 2: Search Functionality

### Basic Search

```bash
# Single keyword (case-insensitive)
claude-list --search context

# The above matches: "context7", "context", "CONTEXT", etc.
```

### Multi-Keyword AND Search

```bash
# Multiple keywords - all must match (AND logic)
claude-list --search "context7 plugin"

# Matches: "context7-plugin-loader", "plugin-context-manager"
# Does NOT match: "context7-demo" (no "plugin")
```

### Search with Filters

```bash
# Search in plugins only
claude-list --search context --plugins

# Detailed output + search + filter
claude-list -l --search "context" --skills

# Search in MCP servers
claude-list --search test --mcp
```

### Empty Search Results

When no components match your search, you'll see:

```
No components match search: "nonexistent"

Try:
  claude-list --search <different-keyword>
  claude-list              # Show all components
```

## Common Use Cases

### Finding a Specific Plugin

```bash
claude-list --search context7 --plugins
```

### Finding Skills Related to Code Review

```bash
claude-list --search "code review"
```

### Finding All MCP Servers

```bash
claude-list --mcp
```

### Combining Search and Filters

```bash
# Find skills with "api" in the name
claude-list --search api --skills
```

## Troubleshooting

### Colors Not Showing

1. Check if running in a terminal:
   ```bash
   claude-list --no-color   # Should show plain text
   ```

2. Check `NO_COLOR` environment variable:
   ```bash
   echo $NO_COLOR   # Should be empty or not set
   ```

3. Output to file (colors disabled automatically):
   ```bash
   claude-list > output.txt && cat output.txt  # No colors
   ```

### Search Not Finding Expected Results

1. Remember: search is case-insensitive
2. Remember: multi-keyword uses AND logic
3. Use single quotes to preserve spaces:
   ```bash
   claude-list --search "context plugin"  # Correct
   claude-list --search context plugin   # Wrong - only first word used
   ```

## Performance

- Search completes in under 100ms for typical configurations
- Color rendering adds minimal overhead (<5ms)

## Compatibility

| Platform | Colors | Search |
|----------|--------|--------|
| macOS Terminal | ✅ Full support | ✅ Full support |
| iTerm2 | ✅ Full support | ✅ Full support |
| Windows Terminal | ✅ Full support | ✅ Full support |
| Linux Terminal | ✅ Full support | ✅ Full support |
| SSH | ✅ Depends on terminal | ✅ Full support |
| CI/CD Pipelines | ❌ Auto-disabled | ✅ Full support |

## Next Steps

- Run `claude-list --help` for all options
- Try `claude-list --search <your-keyword>` to find components
- Use `--no-color` for scripted environments