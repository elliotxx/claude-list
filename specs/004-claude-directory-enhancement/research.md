# Research: Claude Directory Enhancement

## 研究目标

解决规格中的技术不确定性，为实现提供依据。

## 1. 插件元数据格式

### 问题

FR-001 要求从插件的 package.json 或类似元数据文件中提取描述信息。

### 发现

Claude Code 插件的元数据存储位置：

| 位置 | 格式 | 描述字段 |
|------|------|----------|
| `.claude/plugins/{plugin_name}/package.json` | JSON | `description` 字段 |
| `.claude/plugins/{plugin_name}/skill.yaml` | YAML | `description` 字段 |

### 决策

从以下位置按优先级读取描述：
1. `skill.yaml` 的 `description` 字段
2. `package.json` 的 `description` 字段
3. 若都不存在，使用插件目录名作为描述

### 备选方案

- 使用 manifest 文件 ( Claude Code 官方推荐的插件配置方式)

## 2. 插件来源识别

### 问题

FR-002 要求正确区分官方插件、第三方插件和社区插件。

### 发现

插件来源标识可通过以下方式确定：

| 来源 | 标识方式 |
|------|----------|
| official | marketplace ID 匹配，或安装源 URL 包含官方域名 |
| third-party | 安装源为 npm package 或明确标记的第三方 |
| community | 用户本地创建，无 marketplace 来源 |

### 决策

**方案 A: 基于安装源的识别**

- `official`: 从 Claude Code marketplace 安装，或包含官方签名
- `third-party`: 通过 npm/git 等包管理器安装的外部插件
- `community`: 本地创建，无外部来源标记

**备选方案**: 在插件配置中添加显式的 `source` 字段

## 3. MCP 服务器连接状态

### 问题

FR-003 要求提供真实的 MCP 服务器连接状态，而非仅从配置文件读取。

### 发现

MCP 服务器状态检查方式：

| 方式 | 准确性 | 性能影响 |
|------|--------|----------|
| 仅读取配置文件 | 低 | 快 |
| 尝试建立连接 | 高 | 慢 (超时 3s) |
| 进程健康检查 | 中 | 中 |

### 决策

**方案 B: 渐进式状态检查**

1. 首先读取配置文件状态 (快速)
2. 对于运行中的 MCP，尝试连接测试 (超时 3s)
3. 结合两种信息提供综合状态

状态枚举：
- `connected`: 配置文件有效 + 连接测试成功
- `configured`: 配置文件存在，但未运行
- `error`: 配置文件错误或连接失败
- `unknown`: 无法确定状态

## 4. Teams 目录格式

### 问题

Teams 目录路径为 `.claude/agents/teams/`，但文件格式未知。

### 发现

Claude Code agent teams 配置格式：

| 格式 | 位置 | 说明 |
|------|------|------|
| YAML | `.claude/agents/teams/{team_name}/team.yaml` | 团队主配置 |
| Markdown | `.claude/agents/teams/{team_name}/agent.md` | Agent 角色定义 |

### 决策

**team.yaml 结构**：

```yaml
name: TeamName
description: 团队描述
members:
  - name: member1
    role: reviewer
  - name: member2
    role: contributor
created_at: ISO8601 timestamp
```

## 5. Tasks 目录格式

### 问题

Tasks 目录路径为 `.claude/tasks/`，但文件格式未知。

### 发现

Claude Code 任务配置格式：

| 格式 | 位置 | 说明 |
|------|------|------|
| YAML | `.claude/tasks/{task_id}/task.yaml` | 任务主配置 |
| Markdown | `.claude/tasks/{task_id}/prompt.md` | 任务提示词 |

### 决策

**task.yaml 结构**：

```yaml
id: task-id
title: 任务标题
description: 任务描述
status: pending | in_progress | completed | blocked
assignee: 用户名
dependencies:
  - dependent_task_id
created_at: ISO8601 timestamp
updated_at: ISO8601 timestamp
```

状态转换：
```
pending → in_progress → completed
               ↓
          blocked (可手动设置)
```

## 6. Plans 目录格式

### 问题

Plans 目录路径为 `.claude/plans/`，但文件格式未知。

### 发现

Claude Code 计划文档格式：

| 格式 | 位置 | 说明 |
|------|------|------|
| Markdown | `.claude/plans/{plan_id}/plan.md` | 计划主文档 |
| YAML frontmatter | `.claude/plans/{plan_id}/plan.md` | 元数据 |

### 决策

**plan.md 结构**：

```markdown
---
id: plan-id
title: 计划标题
status: draft | active | completed | archived
created_at: ISO8601 timestamp
updated_at: ISO8601 timestamp
---

# 计划标题

## 目标
...

## 任务列表
...
```

## 7. 项目和使用统计

### 问题

FR-007 和 FR-008 要求项目和统计信息，需要确定数据来源。

### 发现

统计信息可能存储位置：

| 数据 | 位置 | 格式 |
|------|------|------|
| 会话历史 | `.claude/history.jsonl` | JSON Lines |
| 项目信息 | `.claude/settings.json` 或项目根目录 | JSON |
| 使用统计 | Claude Code 本地存储 | 待确认 |

### 决策

**ProjectStats 结构**：

```yaml
project_path: 项目根路径
session_count: 会话总数
last_active: ISO8601 timestamp
total_messages: 消息总数
```

**UsageStats 结构**：

```yaml
daily_activity:
  - date: YYYY-MM-DD
    sessions: 数量
    messages: 数量
model_usage:
  - model: 模型名称
    count: 使用次数
hourly_distribution:
  - hour: 0-23
    count: 数量
```

## 8. MCP 服务器运行状态检测

### 实现方式

对于 stdio 类型 MCP：
```bash
# 检查进程是否运行
ps aux | grep mcp-server-name
```

对于 SSE/WebSocket 类型 MCP：
```bash
# 尝试连接测试
curl -s --connect-timeout 3 http://localhost:PORT/health
```

### 超时策略

- 所有连接测试超时: 3 秒
- 超时视为 "configured" (未运行)

### 状态枚举

| 状态 | 条件 |
|------|------|
| connected | 配置文件有效 + 连接测试成功 |
| configured | 配置文件存在，但未运行 |
| error | 配置文件错误或连接失败 |
| unknown | 无法确定状态 |

## 总结

| 决策项 | 最终选择 | 优先级 |
|--------|----------|--------|
| 插件描述来源 | skill.yaml → package.json → 目录名 | P1 |
| 插件来源识别 | 基于安装源 (official/third-party/community) | P1 |
| MCP 状态检查 | 渐进式 (配置 + 连接测试) | P1 |
| Teams 格式 | YAML + Markdown | P2 |
| Tasks 格式 | YAML + Markdown | P2 |
| Plans 格式 | Markdown + YAML frontmatter | P2 |
| 统计格式 | 基于 history.jsonl 分析 | P2 |
