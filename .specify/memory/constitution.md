<!--
Sync Impact Report
==================
Version change: N/A → 1.0.0 (initial creation)

Modified principles: N/A (initial creation)
Added sections: Core Principles (3), Additional Constraints, Development Workflow, Governance
Removed sections: N/A

Templates requiring updates: ✅ N/A (using defaults)
Runtime guidance docs: N/A (project has no runtime docs yet)

Deferred items: N/A
-->

# CLAUDE-LIST Constitution

## Core Principles

### I. 规范驱动开发

所有功能必须从规范的定义开始：用户故事 → 功能需求 → 验收标准 → 实现计划 → 任务分解。

**非谈判规则**：
- 未经规范（spec.md）确认，不得开始实现
- 实现必须严格遵循 plan.md 中的技术决策
- 规范变更必须经过用户确认，并同步更新所有下游文档

**基本原理**：规范驱动开发确保需求被充分理解，避免返工，并保持所有利益相关者对目标的一致性。

### II. 测试优先 (NON-NEGOTIABLE)

TDD 强制执行：测试编写 → 用户确认 → 测试失败 → 然后实现。红-绿-重构循环必须严格遵守。

**非谈判规则**：
- 每个用户故事必须有对应的契约测试（contract tests）
- 测试必须在实现之前编写并验证失败
- 未通过所有测试的功能不得提交

**基本原理**：测试优先确保代码质量，防止回归错误，并为重构提供安全网。对于开源项目，高测试覆盖率也是建立用户信任的关键。

### III. 开源协作

代码变更必须清晰记录，文档同步更新，变更对用户透明可见。

**非谈判规则**：
- 提交信息必须说明变更原因，而非仅仅描述做了什么
- PR 描述必须覆盖影响范围和迁移指南
- 文档变更必须与代码变更同步提交

**基本原理**：开源项目依赖清晰的沟通来维护社区信任。每一次变更都应该让贡献者和用户理解为什么以及如何变更。

## Additional Constraints

### 技术栈约束

- 项目必须使用 speckit 模板系统进行规范管理
- 所有功能文档存放在 `.specify/` 目录下
- 代码库根目录保持简洁，仅包含源代码和测试

### 质量门禁

- 代码必须通过本地 linting/formatting 检查
- 所有测试必须在 CI 中通过
- 复杂功能需要提交前代码审查

## Development Workflow

### 规范创建流程

1. 使用 `/speckit.specify` 从用户需求创建功能规范
2. 使用 `/speckit.plan` 从规范生成实现计划
3. 使用 `/speckit.tasks` 从计划生成任务列表
4. 逐个完成任务，实现功能

### 变更管理

- 小的修复可直接提交
- 功能变更必须遵循完整的规范流程
- 破坏性变更需要额外的迁移文档

## Governance

This constitution supersedes all other development practices. All contributors MUST comply with these principles.

**Amendment Procedure**：
- 原则变更需要用户确认
- 文档更新可由贡献者直接提交
- 版本号遵循语义化规则

**Compliance Review**：
- PR 审查者必须验证原则遵守情况
- 复杂度违规必须在 plan.md 中记录理由
- 定期审查章程有效性并迭代改进

**Version**: 1.0.0 | **Ratified**: 2025-01-29 | **Last Amended**: 2025-01-29
