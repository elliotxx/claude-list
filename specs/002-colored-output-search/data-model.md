# Data Model: Colored Output and Search Functionality

**Date**: 2026-01-30
**Feature**: 002-colored-output-search

## Overview

Data structures for color scheme management and search filtering.

---

## Entities

### ColorScheme

Maps component types to ANSI colors for output formatting.

**Fields**:
| Field | Type | Purpose |
|-------|------|---------|
| plugins | `Option<Color>` | Blue (#63b3ed) for PLUGINS section |
| skills | `Option<Color>` | Green (#68d391) for SKILLS section |
| mcp | `Option<Color>` | Yellow (#f6e05e) for MCP section |
| hooks | `Option<Color>` | Magenta (#b794f4) for HOOKS section |
| agents | `Option<Color>` | Red (#fc8181) for AGENTS section |
| commands | `Option<Color>` | Orange (#ed8936) for COMMANDS section |
| version | `Option<Color>` | Gray (#a0aec0) for version numbers |

**Relationships**:
- Used by `CompactFormatter` and `DetailedFormatter` to apply colors
- Can be disabled via `ColorSettings`

### ColorSettings

Controls when colors are applied to output.

**Fields**:
| Field | Type | Default | Purpose |
|-------|------|---------|---------|
| enabled | `bool` | `true` (if TTY) | Master toggle for colors |
| force_colors | `bool` | `false` | Override auto-detection |

**Rules**:
- Colors applied only when `enabled && force_colors || (is_tty && enabled)`
- Respect `NO_COLOR` environment variable

### SearchFilter

Parses search query and matches component names.

**Fields**:
| Field | Type | Purpose |
|-------|------|---------|
| keywords | `Vec<String>` | Parsed search keywords (lowercase) |
| use_and | `bool` | Always true (AND logic) |

**Methods**:
| Method | Description |
|--------|-------------|
| `new(query: &str) -> Self` | Parse query into keywords |
| `matches(name: &str) -> bool` | Case-insensitive AND match |

**Relationships**:
- Applied after type filters (--plugins, --skills, etc.)
- Combines with existing `FilterFlags`

### ComponentInfo (Extended from info.rs)

Represents a parsed component with metadata.

**Fields**:
| Field | Type | Purpose |
|-------|------|---------|
| name | `String` | Component identifier |
| version | `Option<String>` | Version string if available |
| source | `Source` | Origin (official, third-party, community) |
| path | `PathBuf` | Location of component definition |
| component_type | `ComponentType` | Type classification |

**Enumeration: Source**
```rust
enum Source {
    Official,     // First-party
    ThirdParty,   // External
    Community,    // User-contributed
}
```

**Enumeration: ComponentType**
```rust
enum ComponentType {
    Plugin,
    Skill,
    Mcp,
    Hook,
    Agent,
    Command,
}
```

---

## Validation Rules

1. **Color values** must be valid RGB (0-255) or standard ANSI colors
2. **Search keywords** must be trimmed and lowercased for matching
3. **Empty search query** should disable search filtering (show all)

---

## Persistence

**No persistence required**. All data is:
- Read from Claude Code config directory at runtime
- Computed at runtime (colors, search)
- Displayed immediately