# Implementation Plan: CLAUDE-LIST CLI

**Branch**: `001-claude-list-cli` | **Date**: 2025-01-29 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-claude-list-cli/spec.md`

## Summary

A Rust CLI tool that parses the `.claude` directory structure and displays information about installed components (plugins, skills, sessions, MCP servers, hooks, agents). The tool follows Unix philosophy with minimalist output by default and progressive disclosure via flags (`-l`, `-ll`). Error handling is graceful - missing files don't crash the tool.

## Technical Context

**Language/Version**: Rust 1.75+  
**Primary Dependencies**: clap (CLI parsing), serde (JSON handling), anyhow (error handling)  
**Storage**: Filesystem only (read `.claude` directory)  
**Testing**: cargo test, assert_cmd for CLI testing, insta for output snapshots  
**Target Platform**: macOS/Linux (Unix CLI), WASM-compatible for web  
**Project Type**: Single CLI binary (no web/mobile components)  
**Performance Goals**: <2 seconds execution, <10MB memory footprint  
**Constraints**: Fixed 80-char output width, 6 component types to parse  
**Scale/Scope**: Single user local tool, small codebase (~1-2K LOC expected)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Requirement | Status | Notes |
|-----------|-------------|--------|-------|
| I. 规范驱动开发 | 遵循 spec.md 进行实现 | ✅ PASS | 规范已定义，plan 遵循规范 |
| II. 测试优先 | TDD + 契约测试 | ⚠️ PENDING | 实现后需编写测试 |
| III. 开源协作 | 清晰提交信息 + 文档同步 | ✅ PASS | 遵循提交规范 |
| 技术栈约束 | 使用 speckit 模板系统 | ✅ PASS | 正在使用中 |
| 质量门禁 | Linting + 测试通过 | ⏳ FUTURE | 将在实现阶段验证 |

**Gate Status**: ✅ PASS - No violations detected

## Project Structure

### Documentation (this feature)

```text
specs/001-claude-list-cli/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (this file includes research)
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
claude-list/
├── Cargo.toml
├── src/
│   ├── main.rs          # CLI entry point, arg parsing
│   ├── cli.rs           # CLI arguments struct definition
│   ├── lib.rs           # Library root
│   ├── info.rs          # ClaudeInfo and Item data structures
│   ├── error.rs         # Error types and handling
│   ├── parsers/         # Parser modules
│   │   ├── mod.rs
│   │   ├── plugins.rs
│   │   ├── skills.rs
│   │   ├── sessions.rs
│   │   ├── mcp.rs
│   │   ├── hooks.rs
│   │   └── agents.rs
│   └── formatters/      # Output formatting modules
│       ├── mod.rs
│       ├── compact.rs
│       ├── detailed.rs
│       └── json.rs
├── tests/
│   ├── unit/
│   ├── integration/
│   │   ├── test_cli.rs
│   │   └── snapshots/
│   └── fixtures/
│       └── .claude/     # Mock .claude directory for testing
```

**Structure Decision**: Single Rust binary project with lib.rs exposing core parsing and formatting logic. Main.rs handles CLI entry point. Tests include unit tests, integration tests with assert_cmd, and snapshot tests for output formatting.

## Phase 0: Research Findings

### Technology Decisions

| Decision | Chosen | Rationale |
|----------|--------|-----------|
| CLI parsing | clap 4.4+ | Stable, derive macros, supports all required flags |
| JSON handling | serde + serde_json | Standard Rust ecosystem, derives work well |
| Error handling | anyhow | Simple error propagation for CLI |
| Output width | Fixed 80 chars | Simpler than dynamic, avoids terminal deps |
| Testing framework | cargo test + assert_cmd | Built-in + CLI-specific testing |

### Component Parsing Strategy

| Component | Source | Parsing Approach |
|-----------|--------|------------------|
| Plugins | `settings.json` → `installed_plugins` array | JSON parsing, version extraction |
| Skills | `skills/[name]/` directories | Directory traversal, read skill.yaml |
| Sessions | `session_history.json` | JSON parsing, count + last timestamp |
| MCP | `mcp.json` | JSON parsing, server name + status |
| Hooks | `hooks/*.md` | File discovery, frontmatter parsing |
| Agents | `agents/*.md` | File discovery, frontmatter parsing |

### Error Handling Strategy

- Missing optional files: Skip gracefully, log debug message, continue
- Malformed JSON: Skip component with warning, continue
- Permission errors: Skip with warning, continue
- Missing `.claude` directory: Error message + exit code 1

## Phase 1: Design Artifacts

### Generated Files

- [x] `data-model.md` - Entity definitions and relationships
- [x] `quickstart.md` - Development setup guide
- [x] `contracts/` - Not applicable (no external API, CLI tool)

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | No violations detected | - |

## Next Steps

1. Run `/speckit.tasks` to generate implementation tasks
2. Implement parsers for each component type
3. Implement formatters (compact, detailed, JSON)
4. Write tests following TDD approach
5. Verify all success criteria are met
