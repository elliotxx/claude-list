---
hook: pre-submit
---
# Pre-Submit Check

在提交代码前，必须确保 `make check` 成功通过。

## 检查内容

- 代码格式 (`cargo fmt --check`)
- Clippy 静态分析 (`cargo clippy`)
- 所有测试 (`cargo test`)

## 运行方式

此 hook 会自动在提交前运行。如果任何检查失败，提交将被阻止。

## 手动运行

```bash
make check
```

## 跳过检查

如需跳过检查强制提交，可以使用 `--no-verify`（不推荐）：

```bash
git commit --no-verify -m "message"
```
