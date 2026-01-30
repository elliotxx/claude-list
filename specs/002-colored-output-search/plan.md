# Implementation Plan: Colored Output and Search Functionality

**Branch**: `002-colored-output-search` | **Date**: 2026-01-30 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/002-colored-output-search/spec.md`

## Summary

Add colored terminal output for component type differentiation and keyword search functionality. The feature includes ANSI color support for 6 component types (plugins, skills, MCP servers, hooks, agents, commands), a `--search` flag for fuzzy/AND search, `--no-color` flag for disabling colors, and `NO_COLOR` environment variable support.

## Technical Context

**Language/Version**: Rust 1.75
**Primary Dependencies**: clap (CLI), anstyle (ANSI styling), regex (search matching)
**Storage**: N/A (no data persistence required)
**Testing**: cargo test, assert_cmd, predicates
**Target Platform**: Linux, macOS (terminal)
**Project Type**: CLI tool
**Performance Goals**: Search returns in under 100ms, color rendering adds <5ms
**Constraints**: UTF-8 output, TTY detection for auto-color, backward compatible
**Scale/Scope**: Local single-user tool, 50-100 component references

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Gate | Status | Notes |
|------|--------|-------|
| Security | ✅ PASS | No sensitive data handling, read-only |
| Complexity | ✅ PASS | Simple feature, minimal code additions |
| Performance | ✅ PASS | <100ms search, <5ms color render |
| Compatibility | ✅ PASS | CLI flags follow existing patterns |
| Testability | ✅ PASS | 36 existing tests pass |

## Project Structure

### Documentation (this feature)

```text
specs/002-colored-output-search/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output
```

### Source Code (repository root)

```text
src/
├── cli.rs               # --search, --no-color arguments
├── output.rs            # Color constants and utilities (NEW)
├── formatters/
│   ├── compact.rs       # Add color support
│   └── detailed.rs      # Add color support
└── parsers/
    └── filter.rs        # Add search matching logic (MODIFY)

tests/
└── cli_test.rs          # Add integration tests
```

**Structure Decision**: Single project structure. CLI arguments in cli.rs, color utilities in new output.rs file, formatters updated for color, filter logic updated for search. Existing test infrastructure used.

## Phase 0: Research

### Needed Research

1. **ANSI color styling in Rust**: anstyle vs colored crate vs ANSI codes
2. **TTY detection in Rust**: atty crate or std::io::IsTerminal
3. **Case-insensitive search in Rust**: regex or simple contains()

### Key Decisions (Pre-research)

| Topic | Likely Choice | Rationale |
|-------|---------------|-----------|
| Color crate | anstyle | Modern, maintained, no transitive deps |
| TTY detection | std::io::IsTerminal (Rust 1.70+) | std library, no external dep |
| Search | Simple string contains | Case-insensitive, AND logic, no regex needed for MVP |

## Phase 1: Design & Contracts

### Data Model Entities

| Entity | Fields | Purpose |
|--------|--------|---------|
| ColorScheme | ComponentType -> ANSI color | Maps component types to colors |
| ComponentInfo | name, version, source, path, type | Extended with type for coloring |
| SearchFilter | Vec<String> keywords | Parses and matches search terms |

### CLI Contracts

```
USAGE: claude-list [OPTIONS]

OPTIONS:
  -c, --config <PATH>    Custom .claude directory path
  -l, --long             Show detailed output
      --no-color         Disable colored output
      --search <QUERY>   Search component names (supports multiple keywords with AND)
      --plugins          Show only plugins
      --skills           Show only skills
      ...
      --json             Output in JSON format
  -h, --help             Print help
  -V, --version          Print version
```

## Complexity Tracking

> Not applicable - no constitution violations.

## Generated Artifacts

| Phase | File | Status |
|-------|------|--------|
| Phase 0 | research.md | TODO |
| Phase 1 | data-model.md | TODO |
| Phase 1 | contracts/ | TODO |
| Phase 1 | quickstart.md | TODO |
| Phase 2 | tasks.md | NOT CREATED (by /speckit.tasks) |

## Next Steps

- Phase 0: Execute research for color/Tty/search implementation choices
- Phase 1: Complete data model and contracts design
- Phase 2: Generate implementation tasks (/speckit.tasks)