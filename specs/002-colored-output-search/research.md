# Research: Colored Output and Search Functionality

**Date**: 2026-01-30
**Feature**: Colored Output and Search Functionality (002-colored-output-search)

## Summary

Research findings for implementing ANSI color output and keyword search in claude-list.

---

## 1. ANSI Color Library Selection

| Library | Assessment | Recommendation |
|---------|------------|----------------|
| **anstyle** | Modern, no_std, used by cargo/rustup | ✅ **SELECTED** |
| **colored** | Easy chainable API, Windows support | Alternative |
| **owo-colors** | Macro-based, zero overhead | Alternative |
| **termcolor** | Low-level writer-based | For advanced use |

**Decision**: Use `anstyle`

**Rationale**:
- No dependencies by default
- Pure Rust implementation
- Used by core Rust tooling (cargo, rustup)
- Modern API with style builder pattern
- Supports no_std environments

**Source**: [crates.io - anstyle](https://crates.io/crates/anstyle) | [Web Search - Rust ANSI Color Libraries 2024](https://websearch.toolsforge.org/search?q=Rust+ANSI+color+library+terminal+output+2024+2025+anstyle+colored+crate)

### Example Usage

```rust
use anstyle::{Color, Style};

// Define colors
const PLUGIN_COLOR: Color = Color::Raw { r: 99, g: 179, b: 237 }; // #63b3ed - Blue
const SKILL_COLOR: Color = Color::Raw { r: 104, g: 211, b: 145 }; // #68d391 - Green

// Create styled output
let style = Style::new().fg_color(Some(PLUGIN_COLOR));
println!("{}", style.render().string("context7"));
```

---

## 2. TTY Detection Strategy

| Method | Recommendation |
|--------|----------------|
| `std::io::IsTerminal` (Rust 1.70+) | ✅ **SELECTED** |
| `atty` crate | Legacy support only |

**Decision**: Use `std::io::IsTerminal`

**Rationale**:
- Part of standard library (no external dependency)
- Actively maintained by Rust team
- Cross-platform support
- Recommended by `atty` maintainers for new code

**Source**: [Web Search - Rust TTY Detection 2024](https://websearch.toolsforge.org/search?q=Rust+TTY+detection+IsTerminal+atty+crate+std%3A%3Aio+2024)

### Example Usage

```rust
use std::io::IsTerminal;

fn should_use_colors() -> bool {
    // Check if stdout is a TTY
    std::io::stdout().is_terminal() && std::env::var("NO_COLOR").is_err()
}
```

---

## 3. Search Implementation Strategy

| Approach | Complexity | Use Case |
|----------|------------|----------|
| `String::to_lowercase() + contains()` | Low | Simple case-insensitive |
| `regex::Regex` (case-insensitive) | Medium | Complex patterns |
| `unicode-case-mapping` crate | High | Full Unicode support |

**Decision**: Simple `to_lowercase() + contains()` approach

**Rationale**:
- Component names are simple ASCII identifiers
- Case-insensitive matching is sufficient
- No external dependencies needed
- AND logic implemented by filtering with all keywords

### Example Usage

```rust
fn matches_search(name: &str, keywords: &[String]) -> bool {
    let name_lower = name.to_lowercase();
    keywords.iter().all(|kw| name_lower.contains(&kw.to_lowercase()))
}

// Multi-keyword AND search
let keywords = vec!["context".to_string(), "plugin".to_string()];
matches_search("context7 plugin", &keywords); // true
matches_search("context7", &keywords); // false
```

---

## 4. NO_COLOR Environment Variable

The `NO_COLOR` environment variable is a [de facto standard](https://no-color.org/) for disabling colored output.

**Implementation**:

```rust
fn colors_enabled() -> bool {
    // Respect NO_COLOR environment variable
    std::env::var("NO_COLOR").is_err()
}

// Combined check
fn should_use_colors() -> bool {
    std::io::stdout().is_terminal() && colors_enabled()
}
```

**Source**: [no-color.org](https://no-color.org/)

---

## 5. Alternative Libraries Considered

### Not Selected: `colored` crate

**Reason**: Has `winapi` dependency on Windows, and `atty` dependency needed for TTY detection.

### Not Selected: `owo-colors` crate

**Reason**: Macro-based approach is powerful but adds compile-time complexity. `anstyle` is simpler for this project.

---

## 6. Implementation Recommendations

### File Structure

```
src/
├── output.rs           # NEW: Color scheme and utilities
├── cli.rs              # MODIFY: Add --search, --no-color args
├── formatters/
│   ├── compact.rs      # MODIFY: Add color support
│   └── detailed.rs     # MODIFY: Add color support
└── parsers/
    └── filter.rs       # MODIFY: Add search matching
```

### Dependencies to Add (Cargo.toml)

```toml
[dependencies]
anstyle = "1.0"  # Or latest version
```

### Dependencies NOT Needed

- ~~`atty`~~ - Use `std::io::IsTerminal` instead
- ~~`regex`~~ - Simple contains() is sufficient
- ~~`colored`~~ - Using anstyle instead

---

## 7. Colors Defined (from Spec)

| Component Type | HEX | RGB |
|----------------|-----|-----|
| PLUGINS | #63b3ed | (99, 179, 237) |
| SKILLS | #68d391 | (104, 211, 145) |
| MCP Servers | #f6e05e | (246, 224, 94) |
| HOOKS | #b794f4 | (183, 148, 244) |
| AGENTS | #fc8181 | (252, 129, 129) |
| COMMANDS | #ed8936 | (237, 137, 54) |
| Version | #a0aec0 | (160, 174, 192) |

---

## 8. References

- [crates.io - anstyle](https://crates.io/crates/anstyle)
- [no-color.org](https://no-color.org/)
- [Rust std::io::IsTerminal docs](https://doc.rust-lang.org/std/io/struct.IsTerminal.html)
- [crates.io - colored](https://crates.io/crates/colored)
- [crates.io - owo-colors](https://crates.io/crates/owo-colors)