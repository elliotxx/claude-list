---
name: release
description: 发布 claude-list 新版本（更新版本号、CHANGELOG、创建 Tag）
---

# Release Workflow

## 当前状态
当前版本: !`grep '^version' Cargo.toml | sed 's/version = //' | tr -d '"'`
工作区状态: !`git status --short`

## 发布流程

### 步骤 1: 检查前提条件

```bash
make check
```

如果检查失败，修复问题后再继续。

### 步骤 2: 确定版本升级类型

请选择版本升级类型：
- **patch**: 修复 bug (0.1.1 → 0.1.2)
- **minor**: 新功能 (0.1.1 → 0.2.0)
- **major**: 重大变更 (0.1.1 → 1.0.0)

### 步骤 3: 更新版本号

读取并更新 Cargo.toml 中的版本号：
```bash
# 读取当前版本
grep '^version' Cargo.toml
```

根据选择的升级类型更新版本号。

### 步骤 4: 更新 CHANGELOG.md

更新 CHANGELOG：
1. 将 `[Unreleased]` 区块改为 `[0.x.y] - YYYY-MM-DD`
2. 在顶部添加新的 `[Unreleased]` 区块

### 步骤 5: Git 操作

```bash
# 提交更改
git add -A
git commit -m "chore: bump version to 0.x.y"

# 创建 tag
git tag v0.x.y

# 推送到远程
git push
git push --tags
```

### 步骤 6: 验证

推送后检查 GitHub Actions：
1. 访问 https://github.com/elliotxx/claude-list/actions
2. 确认 Release workflow 已触发
3. 确认构建和发布成功

### 步骤 7: 发布到 crates.io（手动）

GitHub Actions 完成且构建成功后，手动发布到 crates.io：

```bash
# 1. 先 dry-run 预览
cargo publish --dry-run

# 2. 确认无误后执行发布
cargo publish
```

> **注意**： crates.io 不支持 CI 自动发布，必须手动执行 `cargo publish`

## 快捷命令

```bash
# 交互式发布（引导你完成整个流程）
/release

# 预演模式（只显示操作，不执行）
/release --dry-run

# 直接执行 patch 升级
/release patch

# 直接执行 minor 升级
/release minor

# 直接执行 major 升级
/release major
```
