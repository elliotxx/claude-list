# CLI Contract: claude-list Command Interface

**Date**: 2026-01-30
**Feature**: 002-colored-output-search

## Command Summary

```
CLI Tool: claude-list
Purpose: Display Claude Code configuration components

USAGE:
    claude-list [OPTIONS]

OPTIONS:
    -c, --config <PATH>     Custom .claude directory path
    -l, --long              Show detailed output with version/source/path
        --no-color          Disable colored output
        --search <QUERY>    Search component names (supports multiple keywords with AND)
        --plugins           Show only plugins
        --skills            Show only skills
        --sessions          Show only sessions
        --mcp                Show only MCP servers
        --hooks              Show only hooks
        --agents             Show only agents
        --commands           Show only commands
        --json               Output in JSON format
    -h, --help              Print help
    -V, --version           Print version
```

## New Options (v0.1.2)

### `--no-color`

**Description**: Disable colored output

**Behavior**:
- Overrides automatic TTY detection
- Respected even when `NO_COLOR` is not set
- If set, output contains no ANSI color codes

**Examples**:
```bash
claude-list --no-color                    # No colors
claude-list -l --no-color                 # Detailed, no colors
NO_COLOR=1 claude-list                    # Also no colors (standard)
```

### `--search <QUERY>`

**Description**: Search component names by keyword

**Behavior**:
- Case-insensitive matching
- Multi-keyword with AND logic (all keywords must match)
- Searches across all component types
- Combines with type filters (--plugins, --skills, etc.)

**Examples**:
```bash
claude-list --search context              # Single keyword
claude-list --search "context7"           # Same as above
claude-list --search "context plugin"     # Multi-keyword AND
claude-list -l --search "context" --plugins  # Search + filter
claude-list --search nonexistent          # Shows empty result
```

## Output Modes

### Compact (Default)

```
CLAUDE-LIST v0.1.1

CONFIG: /Users/user/.claude

PLUGINS    3 installed
  context7            # Blue
  plugin_playwright   # Blue
  plugin_example      # Blue

SKILLS     12 available
  brainstorming       # Green
  claude-code-guide   # Green
  ...
```

### Detailed (`-l`)

```
CLAUDE-LIST v0.1.1

CONFIG: /Users/user/.claude

PLUGINS    3 installed
  NAME                 VERSION  SOURCE     PATH
  -------------------  -------  ---------  ---------------------------------
  context7             2.1.0    official   /Users/user/.claude/plugins/...
  plugin_playwright    1.0.0    third-party /Users/user/.claude/plugins/...

# Version column displayed in gray
```

### JSON (`--json`)

**No changes** - JSON output remains plain text (no colors in JSON)

## Color Behavior

| Condition | Colors Enabled |
|-----------|---------------|
| TTY + no `--no-color` + no `NO_COLOR` | ✅ Yes |
| Non-TTY (piped) | ❌ No |
| `--no-color` flag | ❌ No |
| `NO_COLOR=1` env | ❌ No |

## Error Messages

### Empty Search Result

```
CLAUDE-LIST v0.1.1

CONFIG: /Users/user/.claude

No components match search: "nonexistent"

Try:
  claude-list --search <different-keyword>
  claude-list                    # Show all components
```