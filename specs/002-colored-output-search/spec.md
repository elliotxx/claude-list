# Feature Specification: Colored Output and Search Functionality

**Feature Branch**: `002-colored-output-search`
**Created**: 2026-01-30
**Status**: Draft
**Input**: User description: "为 claude-list 添加彩色输出和搜索功能"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Colored Component Output (Priority: P1)

作为一名用户，我希望看到不同类型的组件用不同颜色显示，这样我可以更快速地识别组件类型。

**Why this priority**: 彩色输出是本次 v0.1.2 的核心用户体验改进，能显著提升可读性，是 P0 基础功能。

**Independent Test**: 运行 `claude-list` 命令，可以清晰看到插件显示为蓝色、技能显示为绿色，不同类型组件有明显颜色区分。

**Acceptance Scenarios**:

1. **Given** 用户运行 `claude-list`，**When** 输出包含多种组件类型，**Then** 不同组件类型显示不同颜色（插件蓝色、技能绿色、MCP黄色、Hooks紫色、Agents红色、Commands橙色）。

2. **Given** 用户运行 `claude-list`，**When** 在命令行输出，**Then** 版本号显示为灰色，不与组件名称混淆。

3. **Given** 用户运行 `claude-list --no-color`，**When** 纯文本输出，**Then** 不输出任何 ANSI 颜色转义码。

---

### User Story 2 - Single Keyword Search (Priority: P1)

作为一名用户，我希望能够通过关键词搜索组件，这样我可以快速找到特定的插件、技能或其他组件。

**Why this priority**: 搜索是最常用的查找功能的基础，必须优先实现。

**Independent Test**: 运行 `claude-list --search context`，只会显示名称中包含 "context" 的组件。

**Acceptance Scenarios**:

1. **Given** 用户运行 `claude-list --search context`，**When** 有插件/技能名称包含 "context"，**Then** 只显示匹配的结果。

2. **Given** 用户运行 `claude-list --search CONTEXT`，**When** 搜索不区分大小写，**Then** 显示与 "context" 匹配的结果。

3. **Given** 用户运行 `claude-list --search nonexistent`，**When** 没有任何匹配项，**Then** 显示友好的空结果提示。

---

### User Story 3 - Multi-Keyword AND Search (Priority: P1)

作为一名用户，我希望能够用多个关键词搜索，这样我可以更精确地定位需要查找的组件。

**Why this priority**: 多关键词 AND 搜索是精确查找的常用需求。

**Independent Test**: 运行 `claude-list --search "context plugin"`，只会显示同时包含 "context" 和 "plugin" 的组件。

**Acceptance Scenarios**:

1. **Given** 用户运行 `claude-list --search "context7 plugin"`，**When** 有组件同时包含两个关键词，**Then** 显示匹配两个关键词的组件。

2. **Given** 用户运行 `claude-list --search "context"`，**When** 只有单个关键词，**Then** 等同于单一关键词搜索。

3. **Given** 用户运行 `claude-list --search "a b c"`，**When** 有三个关键词，**Then** 只显示同时包含 a、b、c 三个词的组件。

---

### User Story 4 - Search Combined with Filters (Priority: P2)

作为一名用户，我希望搜索能和类型筛选组合使用，这样我可以更精确地在特定类型中查找。

**Why this priority**: 搜索 + 筛选是高级使用场景，提升功能灵活性。

**Independent Test**: 运行 `claude-list --search context --plugins`，只在插件中搜索包含 "context" 的组件。

**Acceptance Scenarios**:

1. **Given** 用户运行 `claude-list -l --search context --plugins`，**When** 搜索和筛选组合，**Then** 只在插件类型中搜索并显示详细格式。

2. **Given** 用户运行 `claude-list --search test --mcp`，**When** 搜索和 MCP 筛选组合，**Then** 只显示 MCP 服务器中匹配 test 的结果。

---

### User Story 5 - Color Control via Environment (Priority: P2)

作为一名用户，我希望能够通过环境变量控制颜色输出，这样我可以与不支持颜色的工具集成。

**Why this priority**: 支持标准环境变量是命令行工具的最佳实践。

**Acceptance Scenarios**:

1. **Given** 用户设置 `NO_COLOR=1` 环境变量，**When** 运行 `claude-list`，**Then** 输出不包含颜色转义码。

2. **Given** 用户运行 `claude-list` 并输出到管道，**When** 非TTY环境，**Then** 自动禁用颜色输出。

---

### Edge Cases

- 组件名称包含搜索关键词但在不同大小写形式时如何匹配
- 搜索关键词为空或只有空白字符时如何处理
- 当输出重定向到文件时，确保不写入 ANSI 转义码
- 组件名称为空的异常情况
- 颜色方案在深色/浅色终端背景下的可读性

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST display PLUGINS component names in blue color (#63b3ed)
- **FR-002**: System MUST display SKILLS component names in green color (#68d391)
- **FR-003**: System MUST display MCP Servers component names in yellow color (#f6e05e)
- **FR-004**: System MUST display HOOKS component names in magenta color (#b794f4)
- **FR-005**: System MUST display AGENTS component names in red color (#fc8181)
- **FR-006**: System MUST display COMMANDS component names in orange color (#ed8936)
- **FR-007**: System MUST display version numbers in gray color (#a0aec0)
- **FR-008**: System MUST provide `--no-color` flag to disable all color output
- **FR-009**: System MUST respect `NO_COLOR=1` environment variable
- **FR-010**: System MUST automatically disable colors when output is not a TTY
- **FR-011**: System MUST provide `--search` flag for component name search
- **FR-012**: Search MUST be case-insensitive matching
- **FR-013**: System MUST support single keyword search
- **FR-014**: System MUST support multiple keywords with AND logic (all keywords must match)
- **FR-015**: Search result MUST display all component types by default
- **FR-016**: Search MUST combine with existing filters (--plugins, --skills, etc.)
- **FR-017**: System MUST display empty result message when no matches found
- **FR-018**: Detailed output format (`-l`) MUST support both colors and search

### Key Entities

- **ColorScheme**: Defines color mappings for each component type
- **ComponentInfo**: Represents a parsed component with name, version, source, path, and type
- **SearchFilter**: Handles search keyword parsing and matching logic

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can identify component types by color with 95% accuracy in user testing
- **SC-002**: Search returns results in under 100ms for datasets with 100+ components
- **SC-003**: 100% of users can successfully find a component using search on first attempt
- **SC-004**: Multi-keyword search reduces time to find specific component by 50% compared to visual scanning
- **SC-005**: Color output adds less than 5ms rendering time per output operation