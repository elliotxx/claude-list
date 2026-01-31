# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.3] - 2026-02-01

### Fixed

- Relaxed color rendering performance test threshold for local/CI environments

## [0.1.2] - 2026-01-31

### Added

- **Colored Output**: Components are now displayed with distinct colors for easy identification:
  - Plugins: Blue
  - Skills: Green
  - MCP Servers: Yellow
  - Hooks: Magenta
  - Agents: Red
  - Commands: Orange
  - Version Numbers: Gray

- **Search Functionality**:
  - Single keyword search (`--search keyword`)
  - Multi-keyword AND search (`--search "keyword1 keyword2"`)
  - Case-insensitive matching
  - Works with all output modes (compact, detailed, JSON)

- **Color Control**:
  - `--no-color` flag to disable colors
  - `NO_COLOR` environment variable support
  - Automatic color disabling for non-TTY output (pipes, files)

### Changed

- Improved table alignment in detailed output mode (`-l`)
- Colors automatically disabled when output is piped or redirected

### Fixed

- Fixed ANSI escape sequence handling for proper column alignment
- Fixed width calculation for Unicode characters with colors

### Performance

- Search operations complete in under 100ms for typical configurations
- Color rendering adds minimal overhead (<5ms)

## [0.1.1] - 2026-01-29

### Added

- Initial release with basic component listing
- Support for all component types: plugins, skills, sessions, MCP servers, hooks, agents, commands
- Multiple output modes: compact, detailed (`-l`), JSON
- Filter flags for each component type
- Test fixtures for all component types

### Features

- Parse `.claude/settings.json` for plugins
- Parse `.claude/skills/` directory for skills
- Parse `.claude/sessions/` for session history
- Parse `.claude/mcp.json` for MCP servers
- Parse `.claude/hooks/` for hooks
- Parse `.claude/agents/` for agents
- Parse `.claude/commands/` for commands

[Unreleased]: https://github.com/elliotxx/claude-list/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/elliotxx/claude-list/releases/tag/v0.1.3
[0.1.2]: https://github.com/elliotxx/claude-list/releases/tag/v0.1.2
[0.1.1]: https://github.com/elliotxx/claude-list/releases/tag/v0.1.1
