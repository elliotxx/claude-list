# Feature Specification: Detailed Output with Description

**Feature Branch**: `003-detailed-description`
**Created**: 2026-02-01
**Status**: Draft
**Input**: User description: "当我执行 --long 操作时，不需要 version 这一列，也不需要 path 这一列，但是需要 description 这一列，简短地描述每一个是什么，而且要做长度的截断"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Enhanced Detailed Output (Priority: P1)

当用户使用 `--long` (或 `-l`) 选项运行 `claude-list` 时，输出应显示简化的表格格式，包含名称、来源和描述三列，不再显示版本号和路径列。

**为什么这个优先级**: 这是用户明确要求的功能改进，提高输出可读性，去除冗余信息。

**独立测试**: 运行 `claude-list --long` 并验证输出格式符合预期：只包含 NAME、SOURCE、DESCRIPTION 三列。

**接受场景**:

1. **Given** 用户运行 `claude-list --long`, **When** 输出已生成, **Then** 表格包含 NAME、SOURCE、DESCRIPTION 三列
2. **Given** 用户运行 `claude-list --long`, **When** 输出已生成, **Then** 没有 VERSION 和 PATH 列
3. **Given** 存在多个插件、技能等组件, **When** 用户运行 `claude-list --long`, **Then** 每个组件都显示描述信息

### User Story 2 - Description Truncation (Priority: P1)

当组件描述内容过长时，输出应自动截断并添加省略号，确保表格对齐和可读性。

**为什么这个优先级**: 保证输出格式一致性，防止长文本破坏终端显示。

**独立测试**: 运行 `claude-list --long` 并验证长描述被正确截断。

**接受场景**:

1. **Given** 组件描述长度超过最大宽度, **When** 输出已生成, **Then** 描述被截断并显示 "..."
2. **Given** 描述被截断, **When** 截断后计算, **Then** 总宽度不超过终端最大宽度
3. **Given** 描述较短, **When** 输出已生成, **Then** 保持完整显示不截断

### User Story 3 - Description for All Component Types (Priority: P2)

所有类型的组件（插件、技能、MCP 服务器、Hooks、Agents、Commands）在 `--long` 模式下都应显示描述信息。

**为什么这个优先级**: 确保用户对所有组件类型都能快速了解其用途。

**独立测试**: 运行 `claude-list --long` 并验证所有组件类型都显示描述列。

**接受场景**:

1. **Given** 存在插件, **When** 运行 `claude-list --long`, **Then** 插件行显示描述
2. **Given** 存在技能, **When** 运行 `claude-list --long`, **Then** 技能行显示描述
3. **Given** 存在 MCP 服务器, **When** 运行 `claude-list --long`, **Then** MCP 行显示描述
4. **Given** 存在 Hooks, **When** 运行 `claude-list --long`, **Then** Hooks 行显示描述
5. **Given** 存在 Agents, **When** 运行 `claude-list --long`, **Then** Agents 行显示描述
6. **Given** 存在 Commands, **When** 运行 `claude-list --long`, **Then** Commands 行显示描述

### Edge Cases

- 当组件没有描述信息时，显示什么占位符？（建议使用 "-"）
- 当终端宽度非常窄时，截断策略是什么？
- 当描述包含多字节字符（如中文）时，如何正确计算截断长度？

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: `--long` 输出的表格 MUST 只包含 NAME、SOURCE、DESCRIPTION 三列
- **FR-002**: `--long` 输出的表格 MUST 移除 VERSION 列
- **FR-003**: `--long` 输出的表格 MUST 移除 PATH 列
- **FR-004**: 所有组件类型 MUST 在 `--long` 模式下显示描述信息
- **FR-005**: 描述内容超过最大宽度时 MUST 自动截断并显示 "..."
- **FR-006**: 描述截断 MUST 考虑多字节字符（如中文）的正确宽度计算
- **FR-007**: 没有描述的组件 MUST 显示 "-" 作为占位符

### Key Entities

- **ComponentDescription**: 组件的简短描述信息，来源于各组件的元数据
- **TruncationStrategy**: 描述截断策略，包括最大宽度和截断标记

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 用户运行 `claude-list --long` 后，输出表格只显示 NAME、SOURCE、DESCRIPTION 三列
- **SC-002**: 所有组件类型在 `--long` 模式下都显示描述信息，覆盖率 100%
- **SC-003**: 长描述被正确截断，无截断时保持原样显示
- **SC-004**: 描述截断后表格保持对齐，不破坏终端显示格式

## Assumptions

1. 描述字段在各组件解析器中已存在或可从现有字段推断
2. 最大描述宽度设为 50 字符，超过则截断
3. 截断使用 "..." 作为后缀
4. 使用 Unicode 宽度计算来处理中文等多字节字符
5. 没有描述时使用 "-" 占位符

## Dependencies

- 现有 `--long` 输出格式实现 (`src/formatters/detailed.rs`)
- 现有各组件类型的描述字段获取逻辑
