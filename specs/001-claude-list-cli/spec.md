# Feature Specification: CLAUDE-LIST CLI

**Feature Branch**: `001-claude-list-cli`
**Created**: 2025-01-29
**Status**: Draft
**Input**: Create a Rust CLI tool that parses and displays Claude Code `.claude` directory information including installed plugins, skills, agents, session count, and more.

## Clarifications

### Session 2025-01-29

- Q: Source enumeration granularity → A: Only "official" vs "third-party"
- Q: Component types to display → A: All defined types (plugins, skills, sessions, mcp, hooks, agents)
- Q: Empty group handling → A: Empty groups are not displayed
- Q: Output width constraint → A: Fixed 80-character width
- Q: Missing .claude directory behavior → A: Display error message and exit with non-zero code

## User Scenarios & Testing *(mandatory)*

### User Story 1 - View Claude Environment Overview (Priority: P1)

As a Claude Code user, I want to quickly see what plugins, skills, and other extensions are installed in my environment, so I can understand my current setup and identify what I have available.

**Why this priority**: This is the core value proposition - users need to see an overview of their Claude Code environment at a glance.

**Independent Test**: Can be tested by running `claude-list` without arguments and verifying it displays counts and names for all installed components.

**Acceptance Scenarios**:

1. **Given** the user has Claude Code installed with some plugins and skills, **When** they run `claude-list`, **Then** they should see a count of installed plugins, skills, sessions, and MCP servers.
2. **Given** the user runs `claude-list` with no arguments, **When** the command executes, **Then** output should be displayed in compact format with no errors.
3. **Given** the user runs `claude-list` with the `-C` flag pointing to a valid `.claude` directory, **When** the command executes, **Then** it should display information from that directory.

---

### User Story 2 - View Detailed Component Information (Priority: P2)

As a user who wants more information about a specific component, I want to see version numbers, sources, and paths, so I can understand what is installed and where it is located.

**Why this priority**: Detailed information is useful for debugging, documentation, and understanding dependencies, but not required for basic usage.

**Independent Test**: Can be tested by running `claude-list -l` and verifying detailed information is displayed for each component.

**Acceptance Scenarios**:

1. **Given** the user runs `claude-list -l`, **When** the command executes, **Then** each component should show its name, version (if available), and source type (official/third-party).
2. **Given** the user runs `claude-list -ll`, **When** the command executes, **Then** additional information such as paths and usage statistics should be displayed.

---

### User Story 3 - Filter and Query Specific Component Types (Priority: P3)

As a user interested in only one type of component, I want to filter the output to show only plugins or only skills, so I can focus on the information I need without distraction.

**Why this priority**: Convenience feature for users who know what they are looking for and want focused output.

**Independent Test**: Can be tested by running `claude-list --plugins` and verifying only plugins are displayed.

**Acceptance Scenarios**:

1. **Given** the user runs `claude-list --plugins`, **When** the command executes, **Then** only plugins should be displayed with no other component types.
2. **Given** the user runs `claude-list --skills`, **When** the command executes, **Then** only skills should be displayed.
3. **Given** the user runs `claude-list --json`, **When** the command executes, **Then** output should be valid JSON format containing all component data.
4. **Given** the user runs `claude-list --hooks`, **When** the command executes, **Then** only hooks should be displayed.
5. **Given** the user runs `claude-list --agents`, **When** the command executes, **Then** only agents should be displayed.

---

### Edge Cases

- What happens when the `.claude` directory does not exist? → Display error and exit with non-zero code
- What happens when `session_history.json` is missing or malformed? → Skip sessions, show other components
- What happens when a skill directory is empty or missing expected files? → Skip invalid skill, continue
- How does the tool handle circular symlinks or permission errors? → Skip with warning, continue
- What happens with very large session histories (thousands of sessions)? → Only show count, not individual sessions

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The CLI MUST parse the `.claude` directory to discover installed components.
- **FR-002**: The CLI MUST support three output levels: compact (default), detailed (`-l`), and full (`-ll`).
- **FR-003**: The CLI MUST support filtering by component type using flags: `--plugins`, `--skills`, `--sessions`, `--mcp`, `--hooks`, `--agents`.
- **FR-004**: The CLI MUST output valid JSON when `--json` flag is used.
- **FR-005**: The CLI MUST accept a custom `.claude` directory path via `-C` flag.
- **FR-006**: The CLI MUST display installed plugins with name, version (if available), and source type.
- **FR-007**: The CLI MUST display installed skills with name and source type.
- **FR-008**: The CLI MUST display session count and last session timestamp.
- **FR-009**: The CLI MUST display configured MCP servers and their connection status.
- **FR-013**: The CLI MUST display installed hooks with name and hook type.
- **FR-014**: The CLI MUST display installed agents with name and description.
- **FR-010**: The CLI MUST NOT crash when optional files are missing (e.g., `session_history.json`).
- **FR-011**: The CLI MUST display a user-friendly error message and exit with non-zero code when the `.claude` directory does not exist.
- **FR-012**: The CLI MUST complete execution within 2 seconds on typical hardware.
- **FR-015**: The CLI MUST NOT display groups with zero components (empty groups are hidden).

### Key Entities

- **ClaudeInfo**: Aggregated information about the Claude Code environment containing version, config path, and lists of all components.
- **Item**: Unified representation of any installable component with name, version, source, path, and optional description.
- **ItemKind**: Enumeration of component types (Plugin, Skill, Session, Mcp, Hook, Agent).
- **Source**: Enumeration of component sources (Official, ThirdParty).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can run `claude-list` and see component counts within 2 seconds.
- **SC-002**: Output is readable and well-formatted in terminal environments with standard width.
- **SC-003**: All component types (plugins, skills, sessions, MCP, hooks, agents) are discoverable via command output.
- **SC-004**: JSON output is valid and parseable by standard JSON parsers.
- **SC-005**: The CLI handles missing optional files without error messages or crashes.
- **SC-006**: Users can filter to show only the component type they are interested in.
