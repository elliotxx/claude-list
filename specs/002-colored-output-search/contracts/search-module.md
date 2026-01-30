# Module Interface: Search Filter Module

**Date**: 2026-01-30
**Feature**: 002-colored-output-search
**Module**: src/parsers/filter.rs (EXTEND)

## Public API

### Types

```rust
pub struct SearchFilter {
    keywords: Vec<String>,
}

pub struct FilterFlags {
    pub plugins: bool,
    pub skills: bool,
    pub sessions: bool,
    pub mcp: bool,
    pub hooks: bool,
    pub agents: bool,
    pub commands: bool,
    pub search: Option<SearchFilter>,  // NEW
}
```

### Functions

```rust
impl SearchFilter {
    /// Parse search query into keywords
    /// Multi-keyword AND: all keywords must match
    pub fn new(query: &str) -> Self;

    /// Check if component name matches search criteria
    pub fn matches(&self, name: &str) -> bool;

    /// Returns true if filter is active (has keywords)
    pub fn is_active(&self) -> bool;
}

/// Extend FilterFlags to include search
impl FilterFlags {
    pub fn with_search(search: Option<SearchFilter>) -> Self;
}
```

### Behavior

| Input | Behavior |
|-------|----------|
| `SearchFilter::new("context")` | Single keyword, matches "context7", "Context", etc. |
| `SearchFilter::new("context plugin")` | Two keywords, BOTH must appear in name |
| `SearchFilter::new("")` | Empty query, `is_active() = false` |
| `SearchFilter::new("  ")` | Whitespace only, `is_active() = false` |

## Dependencies

- Standard library only
- No external dependencies for simple substring matching

## Usage Example

```rust
use crate::parsers::filter::{FilterFlags, SearchFilter};

// Create filter with search
let search = SearchFilter::new("context plugin");
let filter = FilterFlags {
    plugins: false,
    skills: false,
    sessions: false,
    mcp: false,
    hooks: false,
    agents: false,
    commands: false,
    search: Some(search),
};

// Apply to component
let name = "context7-plugin-db";
if filter.matches_component(name, "plugin") {
    println!("Match: {}", name);
}
```