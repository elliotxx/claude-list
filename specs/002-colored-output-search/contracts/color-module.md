# Module Interface: Color Output Module

**Date**: 2026-01-30
**Feature**: 002-colored-output-search
**Module**: src/output.rs (NEW)

## Public API

### Types

```rust
pub struct ColorScheme {
    pub plugins: Option<Color>,
    pub skills: Option<Color>,
    pub mcp: Option<Color>,
    pub hooks: Option<Color>,
    pub agents: Option<Color>,
    pub commands: Option<Color>,
    pub version: Option<Color>,
}

pub struct ColorSettings {
    pub enabled: bool,
    pub force_colors: bool,
}
```

### Functions

```rust
/// Create default color scheme for this project
impl ColorScheme {
    pub fn default() -> Self;
}

/// Check if colors should be enabled
impl ColorSettings {
    pub fn from_env() -> Self;
    pub fn should_use_colors(&self) -> bool;
}

/// Get color for component type
pub fn color_for_component(
    component_type: &str,
    scheme: &ColorScheme
) -> Option<Color>;
```

### Constants

```rust
// Component colors from spec
pub const PLUGIN_COLOR: RgbColor = RgbColor(99, 179, 237);   // #63b3ed Blue
pub const SKILL_COLOR: RgbColor = RgbColor(104, 211, 145);  // #68d391 Green
pub const MCP_COLOR: RgbColor = RgbColor(246, 224, 94);     // #f6e05e Yellow
pub const HOOK_COLOR: RgbColor = RgbColor(183, 148, 244);   // #b794f4 Magenta
pub const AGENT_COLOR: RgbColor = RgbColor(252, 129, 129);  // #fc8181 Red
pub const COMMAND_COLOR: RgbColor = RgbColor(237, 137, 54); // #ed8936 Orange
pub const VERSION_COLOR: RgbColor = RgbColor(160, 174, 192);// #a0aec0 Gray
```

## Dependencies

- `anstyle` - ANSI styling
- Standard library only (no additional dependencies)

## Usage Example

```rust
use crate::output::{ColorScheme, ColorSettings};

fn display_component(name: &str, comp_type: &str, colors: &ColorScheme, settings: &ColorSettings) {
    if settings.should_use_colors() {
        if let Some(color) = color_for_component(comp_type, colors) {
            println!("{}", color.render().string(name));
            return;
        }
    }
    println!("{}", name);
}
```