# Quickstart: Claude Directory Enhancement

## 开发环境准备

```bash
# 1. 克隆仓库并切换分支
git clone https://github.com/elliotxx/claude-list.git
cd claude-list
git checkout 004-claude-directory-enhancement

# 2. 安装 Rust 1.75+
rustc --version  # 确保版本 >= 1.75.0

# 3. 构建项目
cargo build

# 4. 运行测试
cargo test
```

## 任务分解

### P1: 修复现有问题

| 任务 | 描述 | 预计复杂度 |
|------|------|-----------|
| T-001 | 提取插件描述信息 | 中 |
| T-002 | 识别插件来源 (official/third-party/community) | 中 |
| T-003 | MCP 真实连接状态检查 | 高 |

### P1: 新增目录覆盖

| 任务 | 描述 | 预计复杂度 |
|------|------|-----------|
| T-004 | 团队解析器 (`--teams`) | 中 |
| T-005 | 任务解析器 (`--tasks`) | 中 |
| T-006 | 计划解析器 (`--plans`) | 低 |
| T-007 | 项目统计解析器 (`--projects`) | 中 |
| T-008 | 使用统计解析器 (`--stats`) | 高 |

### P2: 会话详情

| 任务 | 描述 | 预计复杂度 |
|------|------|-----------|
| T-009 | 会话详情解析器 (`--sessions-detail`) | 中 |

### P2: 健康检查

| 任务 | 描述 | 预计复杂度 |
|------|------|-----------|
| T-010 | 健康检查模块 (`--health`) | 高 |

## 测试策略

### 单元测试

```bash
# 运行单元测试
cargo test --lib

# 运行特定模块测试
cargo test parsers::teams
cargo test parsers::tasks
```

### 集成测试

```bash
# 运行 CLI 集成测试
cargo test --test cli_test

# 运行健康检查测试
cargo test health
```

### 测试数据

测试fixtures位于 `tests/fixtures/.claude/`：

```
tests/fixtures/.claude/
├── plugins/
│   ├── with-description/
│   │   └── skill.yaml
│   └── with-source/
│       └── package.json
├── teams/
│   └── example-team/
│       └── team.yaml
├── tasks/
│   └── example-task/
│       └── task.yaml
├── plans/
│   └── example-plan/
│       └── plan.md
└── mcp/
    ├── running/
    └── configured/
```

## 调试技巧

### 启用详细日志

```bash
RUST_LOG=debug cargo run -- --teams
```

### 跳过格式化直接查看原始输出

```bash
cargo run -- --teams --output json | jq
```

## 常见问题

### Q: 插件描述为空？

A: 检查 `skill.yaml` 或 `package.json` 中是否存在 `description` 字段。

### Q: MCP 状态显示为 "configured" 而非 "connected"？

A: MCP 服务器未运行，使用 `claude-list --mcp -l` 查看详细信息。

### Q: 团队成员显示异常？

A: 确保 `team.yaml` 的 `members` 数组格式正确。
