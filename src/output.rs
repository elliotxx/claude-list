//! Color output utilities for terminal display.
//!
//! Provides ANSI color styling for different component types,
//! with support for TTY detection and environment-based control.

use anstyle::{AnsiColor, Style};
use std::io::{IsTerminal, Write as IoWrite};
use unicode_width::UnicodeWidthChar;

/// Component type enumeration for color mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentType {
    Plugin,
    Skill,
    Mcp,
    Hook,
    Agent,
    Command,
    Version,
}

/// Color scheme mapping component types to ANSI colors.
#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub plugins: Option<Style>,
    pub skills: Option<Style>,
    pub mcp: Option<Style>,
    pub hooks: Option<Style>,
    pub agents: Option<Style>,
    pub commands: Option<Style>,
    pub version: Option<Style>,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            plugins: Some(AnsiColor::Blue.on_default()),
            skills: Some(AnsiColor::Green.on_default()),
            mcp: Some(AnsiColor::Yellow.on_default()),
            hooks: Some(AnsiColor::Magenta.on_default()),
            agents: Some(AnsiColor::Red.on_default()),
            commands: Some(AnsiColor::BrightYellow.on_default()),
            version: Some(AnsiColor::BrightBlack.on_default()),
        }
    }
}

impl ColorScheme {
    /// Get color style for a component type.
    pub fn for_component(&self, component_type: ComponentType) -> Option<Style> {
        match component_type {
            ComponentType::Plugin => self.plugins,
            ComponentType::Skill => self.skills,
            ComponentType::Mcp => self.mcp,
            ComponentType::Hook => self.hooks,
            ComponentType::Agent => self.agents,
            ComponentType::Command => self.commands,
            ComponentType::Version => self.version,
        }
    }
}

/// Settings for color output control.
#[derive(Debug, Clone, Copy)]
pub struct ColorSettings {
    /// Whether colors are enabled.
    pub enabled: bool,
    /// Force colors even in non-TTY (for testing).
    pub force_colors: bool,
}

impl ColorSettings {
    /// Create settings from environment and context.
    pub fn from_env() -> Self {
        // Check NO_COLOR environment variable first
        if std::env::var("NO_COLOR").is_ok() {
            return Self {
                enabled: false,
                force_colors: false,
            };
        }

        // Check if stdout is a TTY
        let is_tty = std::io::stdout().is_terminal();

        Self {
            enabled: is_tty,
            force_colors: false,
        }
    }

    /// Determine if colors should be used.
    pub fn should_use_colors(&self) -> bool {
        self.force_colors || self.enabled
    }

    /// Create settings with forced colors (for testing).
    pub fn force() -> Self {
        Self {
            enabled: true,
            force_colors: true,
        }
    }
}

/// Render a string with color for a component type.
pub fn colored_string(
    text: &str,
    component_type: ComponentType,
    scheme: &ColorScheme,
    settings: &ColorSettings,
) -> String {
    if !settings.should_use_colors() {
        return text.to_string();
    }

    if let Some(style) = scheme.for_component(component_type) {
        format!("{}{}{}", style, text, anstyle::Reset)
    } else {
        text.to_string()
    }
}

/// Calculate the visible width of a string, ignoring ANSI escape sequences.
pub fn visible_width(s: &str) -> usize {
    // Strip ANSI escape codes and calculate width
    let mut width = 0;
    let mut in_escape = false;
    for c in s.chars() {
        if in_escape {
            if c == 'm' || c == 'K' {
                in_escape = false;
            }
        } else if c == '\x1b' {
            in_escape = true;
        } else {
            width += c.width().unwrap_or(0);
        }
    }
    width
}

/// Write a colored, padded field to output with correct alignment.
/// This handles ANSI escape codes correctly for width calculation.
pub fn write_colored_padded_field(
    output: &mut dyn IoWrite,
    text: &str,
    component_type: ComponentType,
    scheme: &ColorScheme,
    settings: &ColorSettings,
    width: usize,
    align: Alignment,
) -> std::io::Result<()> {
    if !settings.should_use_colors() {
        // No colors, use standard formatting
        match align {
            Alignment::Left => write!(output, "{:<width$}", text, width = width)?,
            Alignment::Right => write!(output, "{:>width$}", text, width = width)?,
        }
    } else if let Some(style) = scheme.for_component(component_type) {
        // With colors, we need to manually pad to account for ANSI codes
        let visible = visible_width(text);
        let padding_needed = width.saturating_sub(visible);

        match align {
            Alignment::Left => {
                write!(output, "{}{}{}", style, text, anstyle::Reset)?;
                for _ in 0..padding_needed {
                    write!(output, " ")?;
                }
            }
            Alignment::Right => {
                for _ in 0..padding_needed {
                    write!(output, " ")?;
                }
                write!(output, "{}{}{}", style, text, anstyle::Reset)?;
            }
        }
    } else {
        // No style, just pad
        match align {
            Alignment::Left => write!(output, "{:<width$}", text, width = width)?,
            Alignment::Right => write!(output, "{:>width$}", text, width = width)?,
        }
    }
    Ok(())
}

/// Alignment for padded fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Right,
}

/// Parse a component type from string.
pub fn parse_component_type(s: &str) -> Option<ComponentType> {
    match s.to_lowercase().as_str() {
        "plugin" | "plugins" => Some(ComponentType::Plugin),
        "skill" | "skills" => Some(ComponentType::Skill),
        "mcp" => Some(ComponentType::Mcp),
        "hook" | "hooks" => Some(ComponentType::Hook),
        "agent" | "agents" => Some(ComponentType::Agent),
        "command" | "commands" => Some(ComponentType::Command),
        _ => None,
    }
}

// ====================
// Description Truncation
// ====================

/// Configuration for description truncation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TruncationConfig {
    /// Maximum display width for description column
    pub max_description_width: usize,
    /// Ellipsis string to append when truncating
    pub ellipsis: &'static str,
    /// Placeholder when no description available
    pub no_description_placeholder: &'static str,
}

impl Default for TruncationConfig {
    fn default() -> Self {
        Self {
            max_description_width: 50,
            ellipsis: "...",
            no_description_placeholder: "-",
        }
    }
}

/// Truncate a string to fit within the specified width, appending an ellipsis.
/// Handles Unicode characters correctly using unicode-width.
pub fn truncate_with_ellipsis(text: &str, max_width: usize, ellipsis: &str) -> String {
    if text.is_empty() {
        return text.to_string();
    }

    let ellipsis_width = visible_width(ellipsis);

    // If ellipsis itself exceeds max width, return empty string
    if ellipsis_width >= max_width {
        return String::new();
    }

    let available_width = max_width - ellipsis_width;
    let mut current_width = 0;
    let mut result = String::new();

    for c in text.chars() {
        let char_width = c.width().unwrap_or(0);
        if current_width + char_width <= available_width {
            result.push(c);
            current_width += char_width;
        } else {
            break;
        }
    }

    // Only add ellipsis if we actually truncated
    if current_width < visible_width(text) {
        result.push_str(ellipsis);
    }

    result
}

/// Format a description with truncation and placeholder handling.
pub fn format_description(description: Option<&str>, config: TruncationConfig) -> String {
    let desc = description.unwrap_or(config.no_description_placeholder);
    truncate_with_ellipsis(desc, config.max_description_width, config.ellipsis)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_scheme_default() {
        let scheme = ColorScheme::default();
        assert!(scheme.plugins.is_some());
        assert!(scheme.skills.is_some());
        assert!(scheme.mcp.is_some());
        assert!(scheme.hooks.is_some());
        assert!(scheme.agents.is_some());
        assert!(scheme.commands.is_some());
        assert!(scheme.version.is_some());
    }

    #[test]
    fn test_color_settings_from_env_no_color() {
        std::env::set_var("NO_COLOR", "1");
        let settings = ColorSettings::from_env();
        assert!(!settings.enabled);
        std::env::remove_var("NO_COLOR");
    }

    #[test]
    fn test_color_settings_force() {
        let settings = ColorSettings::force();
        assert!(settings.should_use_colors());
    }

    #[test]
    fn test_parse_component_type() {
        assert_eq!(parse_component_type("plugin"), Some(ComponentType::Plugin));
        assert_eq!(parse_component_type("Plugin"), Some(ComponentType::Plugin));
        assert_eq!(parse_component_type("PLUGINS"), Some(ComponentType::Plugin));
        assert_eq!(parse_component_type("skill"), Some(ComponentType::Skill));
        assert_eq!(parse_component_type("mcp"), Some(ComponentType::Mcp));
        assert_eq!(parse_component_type("hook"), Some(ComponentType::Hook));
        assert_eq!(parse_component_type("agent"), Some(ComponentType::Agent));
        assert_eq!(
            parse_component_type("command"),
            Some(ComponentType::Command)
        );
        assert_eq!(parse_component_type("unknown"), None);
    }

    #[test]
    fn test_colored_string_no_colors() {
        let settings = ColorSettings::from_env();
        if !settings.should_use_colors() {
            let result = colored_string(
                "test",
                ComponentType::Plugin,
                &ColorScheme::default(),
                &settings,
            );
            assert_eq!(result, "test");
        }
    }

    #[test]
    fn test_colored_string_with_colors() {
        let scheme = ColorScheme::default();
        let settings = ColorSettings::force();
        let result = colored_string("test", ComponentType::Plugin, &scheme, &settings);
        // Should contain ANSI codes when colors are enabled
        assert!(result.contains("test"));
    }

    // ====================
    // Truncation Tests
    // ====================

    #[test]
    fn test_truncation_config_default() {
        let config = TruncationConfig::default();
        assert_eq!(config.max_description_width, 50);
        assert_eq!(config.ellipsis, "...");
        assert_eq!(config.no_description_placeholder, "-");
    }

    #[test]
    fn test_truncate_with_ellipsis_short_text() {
        // Text shorter than max width should not be truncated
        let result = truncate_with_ellipsis("Hello", 50, "...");
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_truncate_with_ellipsis_long_text() {
        // Long text should be truncated
        let text =
            "This is a very long description that exceeds fifty characters and should be truncated";
        let result = truncate_with_ellipsis(text, 50, "...");
        assert!(result.len() <= 53); // 50 chars + "..."
        assert!(result.ends_with("..."));
    }

    #[test]
    fn test_truncate_with_ellipsis_empty_text() {
        let result = truncate_with_ellipsis("", 50, "...");
        assert_eq!(result, "");
    }

    #[test]
    fn test_truncate_with_ellipsis_exact_width() {
        // Text exactly at max width should not need ellipsis
        // Note: With ellipsis, the available width is 50 - 3 = 47
        // So 47 chars will fit without truncation
        let text = "12345678901234567890123456789012345678901234567"; // 47 chars
        let result = truncate_with_ellipsis(text, 50, "...");
        assert_eq!(result, text);
    }

    #[test]
    fn test_truncate_with_ellipsis_unicode_cjk() {
        // CJK characters count as 2 width units each
        let text = "你好世界这是一个很长的描述"; // Each CJK char is 2 wide
        let result = truncate_with_ellipsis(text, 10, "...");
        assert!(result.len() <= 13); // 10 chars width + "..."
        assert!(result.ends_with("..."));
    }

    #[test]
    fn test_truncate_with_ellipsis_unicode_mixed() {
        // Mixed ASCII and CJK
        let text = "Hello世界这是一个很长的描述"; // H(1) + e(1) + l(1) + l(1) + o(1) + 世(2) + 界(2) + ...
        let result = truncate_with_ellipsis(text, 15, "...");
        assert!(result.ends_with("..."));
    }

    #[test]
    fn test_truncate_with_ellipsis_single_char() {
        let result = truncate_with_ellipsis("A", 50, "...");
        assert_eq!(result, "A");
    }

    #[test]
    fn test_truncate_with_ellipsis_narrow_max_width() {
        // When max width is less than ellipsis width
        let result = truncate_with_ellipsis("Hello", 2, "...");
        assert_eq!(result, "");
    }

    #[test]
    fn test_format_description_with_some() {
        let config = TruncationConfig::default();
        let result = format_description(Some("Short description"), config);
        assert_eq!(result, "Short description");
    }

    #[test]
    fn test_format_description_with_none() {
        let config = TruncationConfig::default();
        let result = format_description(None, config);
        assert_eq!(result, "-");
    }

    #[test]
    fn test_format_description_truncates_long() {
        let config = TruncationConfig::default();
        let long_desc = "This is a very long description that definitely exceeds fifty characters";
        let result = format_description(Some(long_desc), config);
        assert!(result.ends_with("..."));
        assert!(result.len() <= 53);
    }
}
