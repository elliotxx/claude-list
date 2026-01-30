# v0.1.2 Usability Improvements Design

## Overview

Enhance user experience with colored output and search functionality for claude-list v0.1.2.

## Goals

1. **Colored Output**: Visual differentiation of component types
2. **Search**: Find components by name using keywords

## Design

### 1. Colored Output

**Color Scheme:**

| Component Type | ANSI Color | Example Output |
|----------------|------------|----------------|
| PLUGINS | Blue (#63b3ed) | `context7` |
| SKILLS | Green (#68d391) | `brainstorming` |
| MCP Servers | Yellow (#f6e05e) | `test-mcp` |
| HOOKS | Magenta (#b794f4) | `pre-commit` |
| AGENTS | Red (#fc8181) | `database-agent` |
| COMMANDS | Orange (#ed8936) | `analyze-code` |
| Version Numbers | Gray (#a0aec0) | `1.0.0` |

**Control:**
- Default: Colors enabled in TTY
- `--no-color`: Disable colors explicitly
- `NO_COLOR=1`: Environment variable to disable colors

### 2. Search Functionality

**Usage:**

```bash
# Single keyword fuzzy search
claude-list --search context

# Multi-keyword AND search (all keywords must match)
claude-list --search "context7 plugin"

# Search with detailed output
claude-list -l --search "context"

# Search combined with filters
claude-list --search "context" --plugins
```

**Behavior:**
- Case-insensitive matching
- Each keyword must appear somewhere in the name (AND logic)
- Searches all component types
- Combines with existing `--plugins`, `--skills`, etc. filters

## Implementation

### Files to Modify

| File | Changes |
|------|---------|
| `src/cli.rs` | Add `--search` and `--no-color` arguments |
| `src/output.rs` (new) | Color utilities and constants |
| `src/formatters/compact.rs` | Add color support to compact formatter |
| `src/formatters/detailed.rs` | Add color support to detailed formatter |
| `src/main.rs` | Integrate search and color logic |
| `tests/cli_test.rs` | Add tests for new features |

### Data Flow

```
User Input (--search, --no-color)
         ↓
Args parsing
         ↓
Parse all components
         ↓
Apply filters (--plugins, etc.)
         ↓
Apply search filter (if --search given)
         ↓
Format output with colors (if not disabled)
         ↓
Print to stdout
```

## Testing

### Test Cases

1. Colored output renders correctly
2. `--no-color` disables all colors
3. `NO_COLOR=1` environment variable works
4. Single keyword search returns matching components
5. Multi-keyword AND search requires all keywords to match
6. Search combines correctly with filters
7. Empty search results handled gracefully
8. Non-TTY output has no colors

## Success Criteria

- [ ] 36 existing tests pass
- [ ] New tests cover colored output
- [ ] New tests cover search functionality
- [ ] Color output works on macOS and Linux terminals
- [ ] Performance: Search adds < 10ms overhead