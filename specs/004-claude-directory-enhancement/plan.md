# Implementation Plan: Claude Directory Enhancement

**Branch**: `004-claude-directory-enhancement` | **Date**: 2026-02-07 | **Spec**: [link](spec.md)
**Input**: Feature specification from `/specs/004-claude-directory-enhancement/spec.md`

## Summary

增强 claude-list 工具，使其成为 Claude Code 配置目录（.claude）的全方位管理工具。核心目标：

1. **修复现有问题**：提取插件描述、正确识别插件来源、真实 MCP 状态
2. **新增目录覆盖**：`--teams`、`--tasks`、`--plans`、`--projects`、`--stats`
3. **会话详情**：`--sessions-detail --last N`
4. **健康检查**：`--health` 实时监控组件状态

## Technical Context

**Language/Version**: Rust 1.75+ (现有项目技术栈)
**Primary Dependencies**: clap (CLI), serde (JSON), anyhow (error handling), anstyle (ANSI 颜色), unicode-width (ANSI 宽度计算)
**Storage**: Filesystem (`.claude` 目录，JSON/YAML/Markdown 文件)
**Testing**: cargo test, 集成测试
**Target Platform**: Linux, macOS (现有支持平台)
**Project Type**: CLI 单项目工具
**Performance Goals**: 会话详情 < 2s (1000条记录)，健康检查 < 5s (全组件)
**Constraints**: 子命令优雅降级，组件不存在时返回空列表而非错误
**Scale**: 100+ 同类型组件场景

## Constitution Check

| Gate | Status | Notes |
|------|--------|-------|
| 规范驱动开发 | ✅ 通过 | spec.md 已完成并澄清 |
| 测试优先 (TDD) | ⚠️ 需注意 | 需为每个用户故事编写契约测试 |
| 开源协作 | ✅ 通过 | 提交信息清晰，文档同步更新 |
| 项目结构简洁 | ✅ 通过 | 复用现有 src/ 结构 |
| 本地 linting | ✅ 通过 | cargo fmt + cargo clippy |

## Project Structure

### Documentation (this feature)

```text
specs/004-claude-directory-enhancement/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
src/
├── main.rs              # CLI 入口点
├── cli.rs               # CLI 参数定义 (扩展新命令)
├── lib.rs               # 模块声明
├── info.rs              # 数据结构定义 (新增实体)
├── error.rs             # 错误类型
├── parsers/             # 新增解析器
│   ├── plugins.rs       # 修复：提取描述、来源
│   ├── mcp.rs           # 修复：真实连接状态
│   ├── teams.rs         # 新增：团队解析
│   ├── tasks.rs         # 新增：任务解析
│   ├── plans.rs         # 新增：计划解析
│   └── stats.rs         # 新增：统计解析
├── formatters/          # 扩展格式化器
│   ├── compact.rs       # 支持新实体
│   ├── detailed.rs      # 支持新实体
│   └── json.rs          # 支持新实体
└── health/              # 新增：健康检查模块
    ├── mod.rs
    ├── checker.rs
    └── models.rs

tests/
├── cli_test.rs          # 集成测试
├── fixtures/.claude/    # 测试数据
└── health_test.rs       # 健康检查测试
```

**Structure Decision**: 复用现有项目结构，新增解析器在 `parsers/` 目录，健康检查模块在 `health/` 目录。

## Complexity Tracking

无复杂度违规需要记录。
