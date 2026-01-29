# claude-list Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-01-29

## Active Technologies

- Rust 1.75+ + clap (CLI parsing), serde (JSON handling), anyhow (error handling) (001-claude-list-cli)

## Project Structure

```text
src/
├── main.rs           # CLI 入口点
├── cli.rs            # CLI 参数定义
├── lib.rs            # 模块声明和重导出
├── info.rs           # 数据结构定义
├── error.rs          # 错误类型
├── parsers/          # 各组件解析器
│   ├── plugins.rs    # 插件解析
│   ├── skills.rs     # 技能解析
│   ├── sessions.rs   # 会话解析
│   ├── mcp.rs        # MCP 服务器解析
│   ├── hooks.rs      # Hook 解析
│   ├── agents.rs     # Agent 解析
│   └── commands.rs   # Command 解析
└── formatters/       # 输出格式化器
    ├── compact.rs    # 紧凑格式
    ├── detailed.rs   # 详细格式
    └── json.rs       # JSON 格式

tests/
├── cli_test.rs       # 集成测试
└── fixtures/.claude/ # 测试数据
```

## Commands

```bash
# 开发测试
cargo test              # 运行所有测试
cargo test --release    # 优化后测试

# 代码检查
cargo fmt               # 代码格式化
cargo clippy            # 代码质量检查
cargo clippy -- -D warnings  # 严格模式

# 构建
cargo build             # 调试构建
cargo build --release   # 发布构建

# 运行
claude-list --config ~/.claude                    # 查看所有组件
claude-list --config ~/.claude -l                 # 详细模式（显示版本、来源、路径）
claude-list --config ~/.claude --output detailed  # 同上
claude-list --config ~/.claude --json             # JSON 格式输出
claude-list --config ~/.claude --plugins          # 仅显示插件
claude-list --config ~/.claude --skills           # 仅显示技能
claude-list --config ~/.claude --sessions         # 仅显示会话
claude-list --config ~/.claude --mcp              # 仅显示 MCP 服务器
claude-list --config ~/.claude --hooks            # 仅显示 Hooks
claude-list --config ~/.claude --agents           # 仅显示 Agents
claude-list --config ~/.claude --commands         # 仅显示 Commands
```

## Code Style

Rust 1.75+: Follow standard conventions

## Git Workflow

**必须使用 `gitacp` SKILL 进行 git 提交**。执行 `/gitacp` 命令来自动生成提交信息并完成提交。

## Recent Changes

- 001-claude-list-cli: Added Rust 1.75+ + clap (CLI parsing), serde (JSON handling), anyhow (error handling)
- 001-claude-list-cli: Implemented User Story 1 - Compact output mode
- 001-claude-list-cli: Implemented User Story 2 - Detailed output mode with version/source/path
- 001-claude-list-cli: Implemented User Story 3 - Filter flags and JSON output
- 001-claude-list-cli: All 35 tests passing, performance validated (<2s)

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
