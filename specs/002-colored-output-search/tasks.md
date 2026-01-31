# Implementation Tasks: Colored Output and Search Functionality

**Feature Branch**: `002-colored-output-search`
**Generated**: 2026-01-30
**Spec**: [spec.md](spec.md)
**Plan**: [plan.md](plan.md)

---

## P1: Core Features (Must Have)

### User Story 1: Colored Component Output (P1)

**Purpose**: 为不同组件类型添加不同颜色的输出，提升可读性

**Dependencies**: 02, 03

#### Research

- [ ] T001 [P1] [US1] Research: anstyle crate usage and API surface @research.md
- [ ] T002 [P1] [US1] Research: TTY detection with std::io::IsTerminal @research.md

#### Dependencies

- [ ] T003 [P1] [US1] Dependency: Add anstyle to Cargo.toml @Cargo.toml

#### Implementation

- [ ] T004 [P1] [US1] Create: Color utilities module with constants @src/output.rs
- [ ] T005 [P1] [US1] Create: ColorSettings struct for TTY/NO_COLOR detection @src/output.rs
- [ ] T006 [P1] [US1] Create: ColorScheme struct mapping types to colors @src/output.rs
- [ ] T007 [P1] [US1] Modify: Add --no-color flag to CLI @src/cli.rs
- [ ] T008 [P1] [US1] Modify: Integrate color settings into main.rs @src/main.rs
- [ ] T009 [P1] [US1] Modify: Add color output to compact formatter @src/formatters/compact.rs
- [ ] T010 [P1] [US1] Modify: Add color output to detailed formatter @src/formatters/detailed.rs

#### Tests

- [ ] T011 [P1] [US1] Test: Unit tests for ColorSettings (TTY detection, NO_COLOR) @src/output.rs
- [ ] T012 [P1] [US1] Test: Integration test: colored output in terminal @tests/cli_test.rs
- [ ] T013 [P1] [US1] Test: Integration test: --no-color disables colors @tests/cli_test.rs

**Checkpoint**: US1 complete - `claude-list` shows colors, `claude-list --no-color` shows plain text

---

### User Story 2: Single Keyword Search (P1)

**Purpose**: 允许用户通过关键词快速查找组件

#### Implementation

- [ ] T014 [P1] [US2] Create: SearchFilter struct in filter.rs @src/parsers/filter.rs
- [ ] T015 [P1] [US2] Create: Case-insensitive matching logic @src/parsers/filter.rs
- [ ] T016 [P1] [US2] Modify: Add --search argument to CLI @src/cli.rs

#### Integration

- [ ] T017 [P1] [US2] Modify: Connect search filter to main loop @src/main.rs
- [ ] T018 [P1] [US2] Modify: Update compact formatter for empty search results @src/formatters/compact.rs

#### Tests

- [ ] T019 [P1] [US2] Test: Unit tests for SearchFilter matching @src/parsers/filter.rs
- [ ] T020 [P1] [US2] Test: Integration test: single keyword search @tests/cli_test.rs
- [ ] T021 [P1] [US2] Test: Integration test: empty search result message @tests/cli_test.rs

**Checkpoint**: US2 complete - `claude-list --search context` only shows matching components

---

### User Story 3: Multi-Keyword AND Search (P1)

**Purpose**: 支持多个关键词组合精确查找

#### Implementation

- [ ] T022 [P1] [US3] Create: Multi-keyword parsing (split by whitespace) @src/parsers/filter.rs
- [ ] T023 [P1] [US3] Create: AND matching logic (all keywords must match) @src/parsers/filter.rs

#### Tests

- [ ] T024 [P1] [US3] Test: Unit tests for multi-keyword AND logic @src/parsers/filter.rs
- [ ] T025 [P1] [US3] Test: Integration test: multi-keyword AND search @tests/cli_test.rs

**Checkpoint**: US3 complete - `claude-list --search "context plugin"` shows only components matching both

---

### Cross-Story Integration

#### Integration

- [ ] T026 [P1] [US1+US2+US3] Integration: Ensure detailed output (-l) supports search and colors @src/formatters/detailed.rs
- [ ] T027 [P1] [US1+US2+US3] Integration: End-to-end test with colors + search + filters @tests/cli_test.rs

#### Performance Validation

- [ ] T028 [P1] [ALL] Benchmark: Validate search < 100ms for 100+ components @tests/
- [ ] T029 [P1] [ALL] Benchmark: Validate color rendering < 5ms @tests/

**Checkpoint**: All P1 complete - core colored output and search features working

---

## P2: Enhanced Features (Nice to Have)

### User Story 4: Search Combined with Filters (P2)

**Purpose**: 搜索与类型筛选组合使用

#### Implementation

- [ ] T030 [P2] [US4] Verify: Search already combines with existing filters via FilterFlags @src/parsers/filter.rs
- [ ] T031 [P2] [US4] Test: Integration test: search + filter combination @tests/cli_test.rs

**Checkpoint**: US4 complete - `claude-list --search context --plugins` works as expected

---

### User Story 5: Color Control via Environment (P2)

**Purpose**: 通过环境变量控制颜色输出

#### Implementation

- [ ] T032 [P2] [US5] Create: NO_COLOR environment variable handling @src/output.rs
- [ ] T033 [P2] [US5] Create: Auto-disable colors for non-TTY output @src/output.rs

#### Tests

- [ ] T034 [P2] [US5] Test: Integration test: NO_COLOR=1 disables colors @tests/cli_test.rs
- [ ] T035 [P2] [US5] Test: Integration test: piped output has no colors @tests/cli_test.rs

**Checkpoint**: US5 complete - environment-based color control working

---

## P2: Documentation

- [ ] T036 [P2] Update: Quickstart.md examples @specs/002-colored-output-search/quickstart.md
- [ ] T037 [P2] Create: CHANGELOG.md entry for v0.1.2 @CHANGELOG.md

---

## Release

- [ ] T038 [P] Version: Bump version to 0.1.2 @Cargo.toml
- [ ] T039 [P] Release: Create GitHub release @.github/workflows/release.yml

---

## Summary

| Priority | Stories | Tasks | Description |
|----------|---------|-------|-------------|
| P1 | US1, US2, US3 | T001-T029 | Core: colors + search |
| P2 | US4, US5 | T030-T037 | Enhanced: filters + env |
| Release | - | T038-T039 | Version bump + release |

**Total Tasks**: 39
**P1 Tasks**: 29
**P2 Tasks**: 8
**Release Tasks**: 2

---

## Dependency Graph

```
T003 (anstyle dep) ─┬─> T004 (Color utils)
                    ├─> T005 (ColorSettings)
                    └─> T006 (ColorScheme)

T004-T006 ──────────> T007 (--no-color CLI)
                    └─> T008 (main.rs integration)
                                            │
T007-T008 ──────────> T009 (compact color)──┤
                    └─> T010 (detailed color)─┤
                                             │
T002 (TTY research) ──────────────────────────┤
                                                 │
T009-T010 ──────────> T011-T013 (color tests)───┤
                                                 │
T014 (SearchFilter) ─┬─> T016 (--search CLI)─────┤
                    └─> T015 (case-insensitive)──┤
                                                 │
T016-CLI ────────────> T017 (main integration)──┤
                    └─> T018 (empty results)─────┤
                                                 │
T014-T018 ──────────> T019-T021 (search tests)──┤
                                                 │
T014 ───────────────> T022 (multi-keyword)──────┤
                    └─> T023 (AND logic)─────────┤
                                                 │
T022-T023 ──────────> T024-T025 (multi-key tests)┤
                                                 │
T013+T021+T025 ─────> T026 (end-to-end test)─────┤
                    └─> T027 (detailed+search)───┤
                                                 │
                    ──────> T028-T029 (benchmarks)
```