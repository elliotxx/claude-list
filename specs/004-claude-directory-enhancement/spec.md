# Feature Specification: Claude Directory Enhancement

**Feature Branch**: `004-claude-directory-enhancement`
**Created**: 2026-02-07
**Status**: Draft
**Input**: User description: "我想实现刚才的需求，帮我规划"

## Overview

增强 claude-list 工具，使其成为 Claude Code 配置目录（.claude）的全方位管理工具。当前工具已覆盖核心组件（skills、plugins、MCP、commands、agents、hooks），但仍有大量高价值目录和数据未被覆盖，且部分已覆盖组件的信息不完整或不准确。

## User Scenarios & Testing

### User Story 1 - 修复已知问题 (Priority: P1)

**用户作为 Claude Code 用户**，运行 claude-list 查看环境配置时，希望能够看到准确的组件信息，包括插件描述、正确的来源标识和真实的 MCP 状态。

**Why this priority**: 这些是影响用户体验的核心问题，修复后立即可提升工具可信度。

**Independent Test**: 可通过运行现有命令并验证输出信息来独立测试。

**Acceptance Scenarios**:

1. **Given** 用户安装了多个插件，**When** 运行 `claude-list --plugins`，**Then** 每个插件显示正确的描述信息（而非空）
2. **Given** 用户同时有官方和第三方插件，**When** 查看插件列表，**Then** 来源列正确区分 "official"、"third-party" 和 "community"
3. **Given** 用户配置了 MCP 服务器，**When** 运行 `claude-list --mcp`，**Then** 显示的连接状态反映实际连接情况（非仅配置文件状态）

---

### User Story 2 - 新增目录覆盖 (Priority: P1)

**用户作为 Claude Code 高级用户**，希望能够全面了解 .claude 目录中的所有重要信息，包括团队配置、任务列表、计划文档和项目统计。

**Why this priority**: 这些目录包含用户日常工作的关键信息，覆盖后工具价值大幅提升。

**Independent Test**: 可通过新增的 CLI 选项独立测试每个目录的解析功能。

**Acceptance Scenarios**:

1. **Given** 用户配置了 agent teams，**When** 运行 `claude-list --teams`，**Then** 显示所有团队及其成员信息
2. **Given** 用户创建了任务列表，**When** 运行 `claude-list --tasks`，**Then** 显示所有任务及其状态
3. **Given** 用户有计划文档，**When** 运行 `claude-list --plans`，**Then** 列出所有计划及其标题
4. **Given** 用户有多个项目会话，**When** 运行 `claude-list --projects`，**Then** 显示项目统计信息（项目数、会话数、活跃度）

---

### User Story 3 - 会话详情查看 (Priority: P2)

**用户作为需要排查问题的开发者**，希望能够查看具体的会话历史详情，而不仅仅是会话计数。

**Why this priority**: 当前只显示统计信息，用户无法了解具体会话内容，影响问题排查效率。

**Independent Test**: 可通过新增的 `--sessions-detail` 选项独立测试。

**Acceptance Scenarios**:

1. **Given** 用户运行 `claude-list --sessions-detail`，**When** 命令执行，**Then** 显示最近 N 个会话的详细信息（时间、项目、消息数）
2. **Given** 用户运行 `claude-list --sessions-detail --last 5`，**When** 命令执行，**Then** 只显示最近 5 个会话

---

### User Story 4 - 实时状态监控 (Priority: P2)

**用户作为需要监控环境的系统管理员**，希望能够实时检查组件健康状态，快速识别问题组件。

**Why this priority**: 健康检查功能可大幅提升问题诊断效率。

**Independent Test**: 可通过新增的 `--health` 选项独立测试。

**Acceptance Scenarios**:

1. **Given** 用户运行 `claude-list --health`，**When** 命令执行，**Then** 显示所有组件的健康状态（正常/警告/错误）
2. **Given** 某个组件配置文件损坏，**When** 运行健康检查，**Then** 该组件显示为错误状态并说明原因

---

### Edge Cases

- 组件目录不存在或为空时，应返回空列表而非错误
- 配置文件格式错误时，应优雅降级并显示警告信息
- 大量组件时（如超过 100 个技能），输出应保持可读性
- Unicode 字符和特殊字符在组件名称中应正确显示
- 权限不足无法读取某些文件时，应显示警告而非中断

## Requirements

### Functional Requirements

#### 修复类需求

- **FR-001**: 系统 MUST 从插件的 package.json 或类似元数据文件中提取插件描述信息
- **FR-002**: 系统 MUST 根据 marketplace 来源正确区分官方插件、第三方插件和社区插件
- **FR-003**: 系统 MUST 提供真实的 MCP 服务器连接状态（而非仅从配置文件读取）

#### 新增目录覆盖

- **FR-004**: 系统 MUST 支持通过 `--teams` 选项显示团队配置信息
- **FR-005**: 系统 MUST 支持通过 `--tasks` 选项显示任务列表及其状态
- **FR-006**: 系统 MUST 支持通过 `--plans` 选项显示计划文档列表
- **FR-007**: 系统 MUST 支持通过 `--projects` 选项显示项目统计信息
- **FR-008**: 系统 MUST 支持通过 `--stats` 选项显示使用统计信息

#### 会话详情

- **FR-009**: 系统 MUST 支持 `--sessions-detail` 选项显示会话详细信息
- **FR-010**: 系统 MUST 支持 `--last N` 参数限制显示的会话数量

#### 健康检查

- **FR-011**: 系统 MUST 支持 `--health` 选项执行组件健康检查
- **FR-012**: 系统 MUST 检查配置文件完整性（文件是否存在、格式是否正确）
- **FR-013**: 系统 MUST 检查组件版本一致性（依赖冲突检测）
- **FR-014**: 系统 MUST 以视觉化方式显示健康状态（✓ 正常、⚠ 警告、✗ 错误）

#### 输出格式

- **FR-015**: 新增目录的信息 MUST 支持所有现有输出格式（compact、detailed、json）
- **FR-016**: JSON 输出 MUST 包含完整的组件信息结构

### Key Entities

- **TeamInfo**: 团队配置信息（名称、描述、成员列表、角色、创建时间）
- **TaskInfo**: 任务信息（ID、描述、状态、依赖关系、所有者、创建时间）
- **PlanInfo**: 计划文档信息（ID、标题、创建时间、状态、文件路径）
- **ProjectStats**: 项目统计信息（项目路径、会话数、最后活跃时间、消息总数）
- **UsageStats**: 使用统计信息（每日活动、模型使用分布、总会话数、小时分布）
- **HealthStatus**: 健康状态（状态枚举、检查项列表、错误消息、检查时间）

## Success Criteria

### Measurable Outcomes

- **SC-001**: 所有插件显示正确的描述信息（100% 覆盖，非空）
- **SC-002**: 插件来源正确标识率 100%（官方、第三方、社区正确区分）
- **SC-003**: MCP 服务器状态反映实际连接情况（非配置文件状态）
- **SC-004**: 新增 5 个目录覆盖选项（teams、tasks、plans、projects、stats）
- **SC-005**: 会话详情查询响应时间在 2 秒以内（1000 条记录以内）
- **SC-006**: 健康检查完成时间在 5 秒以内（全组件检查）
- **SC-007**: 命令行参数扩展不超过 20% 的认知负担（新选项命名直观易懂）

## Assumptions

1. 用户使用标准 Claude Code 安装，.claude 目录结构符合官方规范
2. 配置文件格式遵循官方文档（JSON、YAML、Markdown frontmatter）
3. 用户有权限读取 .claude 目录下的所有文件
4. 系统环境支持基本的文件系统操作和正则表达式
5. MCP 服务器状态检查通过尝试连接实现，超时时间为 3 秒
6. 健康检查不会修改任何配置文件或数据（只读操作）
7. 大量组件场景定义为超过 100 个同类型组件

## Dependencies

- Claude Code 配置目录结构保持稳定
- 各组件的配置文件格式不发生破坏性变更
- 用户的 .claude 目录可访问且具有读取权限

## Clarifications

### Session 2026-02-07

- Q: Team/Tasks/Plans 目录路径 → A: Teams: `.claude/agents/teams/`（复用 agents 目录结构）
- Q: Tasks 和 Plans 目录路径 → A: Tasks: `.claude/tasks/`, Plans: `.claude/plans/`
- Q: JSON 输出结构定义 → A: 规划阶段再定义
- Q: 可扩展性指标 → A: 不需要具体指标，当前 100 个组件足够
