# Tasks: Claude Directory Enhancement

**Feature**: 004-claude-directory-enhancement | **Total Tasks**: 62 | **Date**: 2026-02-07

## Dependencies Graph

```
Phase 1 (Setup) ──────────────────────────────────────────────►
                │                                                     │
                ▼                                                     ▼
Phase 2 (Foundational) ───────────────────────────────────────────►
                │                                                     │
                ▼                                                     ▼
        ┌───────────────┬───────────────┬───────────────┬───────────────┐
        │               │               │               │               │
        ▼               ▼               ▼               ▼               ▼
    [US1-Fix]     [US2-AddDirs]   [US3-Sessions]   [US4-Health]       │
        │               │               │               │               │
        └───────────────┴───────────────┴───────────────┘               │
                                    │                                     │
                                    ▼                                     │
                          Final Phase (Polish) ─────────────────────────►
```

## Parallel Execution

| Phase | Tasks | Parallelizable |
|-------|-------|----------------|
| Phase 1 | T001-T002 | Yes (T001, T002) |
| Phase 2 | T003-T005 | Yes (T003, T004, T005) |
| US1-Fix | T006-T020 | Models → Tests → Parsers → Tests |
| US2-AddDirs | T021-T044 | Teams/Tasks/Plans 可并行 |
| US3-Sessions | T045-T051 | Models → Tests → Parsers → Tests |
| US4-Health | T052-T064 | Models → Tests → Health Module → Tests |

---

## Phase 1: Setup

- [ ] T001 Create test fixtures directory structure in tests/fixtures/.claude/
- [ ] T002 Add serde-yaml and chrono dependencies to Cargo.toml

---

## Phase 2: Foundational

- [ ] T003 [P] Define extended PluginInfo struct with description and source fields in src/info.rs
- [ ] T004 [P] Define TeamInfo, TaskInfo, PlanInfo structs in src/info.rs
- [ ] T005 [P] Define HealthStatus, ComponentHealth, HealthLevel, ProjectStats, UsageStats, SessionDetail types in src/info.rs

---

## Phase 3: US1-Fix - 修复已知问题

**Goal**: 修复插件描述、来源识别、MCP 状态问题

### Models (TDD - Write Tests First)

- [ ] T006 [P] [US1] Add unit tests for PluginSource enum in tests/
- [ ] T007 [P] [US1] Add unit tests for extended PluginInfo in tests/
- [ ] T008 [P] [US1] Add unit tests for MCP connection status in tests/

### Models

- [ ] T009 [US1] Implement PluginSource enum (Official, ThirdParty, Community) in src/info.rs
- [ ] T010 [US1] Extend PluginInfo with description and source fields in src/info.rs
- [ ] T011 [US1] Extend McpServerInfo with real_connection_status field in src/info.rs

### Parsers

- [ ] T012 [US1] Implement plugin description extraction in src/parsers/plugins.rs
- [ ] T013 [US1] Implement plugin source identification in src/parsers/plugins.rs
- [ ] T014 [US1] Implement MCP connection status check in src/parsers/mcp.rs

### Tests (Integration)

- [ ] T015 [US1] Add integration test for --plugins with description and source in tests/cli_test.rs
- [ ] T016 [US1] Add integration test for --mcp with real connection status in tests/cli_test.rs

### Formatters

- [ ] T017 [US1] Update compact formatter for plugins with description and source in src/formatters/compact.rs
- [ ] T018 [US1] Update detailed formatter for plugins with description and source in src/formatters/detailed.rs
- [ ] T019 [US1] Update JSON formatter for plugins with new fields in src/formatters/json.rs
- [ ] T020 [US1] Update compact and detailed formatters for MCP with connection status in src/formatters/

---

## Phase 4: US2-AddDirs - 新增目录覆盖

**Goal**: 新增 teams, tasks, plans, projects, stats 目录解析

### Models (TDD - Write Tests First)

- [ ] T021 [P] [US2] Add unit tests for teams parser in tests/
- [ ] T022 [P] [US2] Add unit tests for tasks parser in tests/
- [ ] T023 [P] [US2] Add unit tests for plans parser in tests/
- [ ] T024 [P] [US2] Add unit tests for projects parser in tests/
- [ ] T025 [P] [US2] Add unit tests for stats parser in tests/

### Models

- [ ] T026 [US2] Define ProjectStats struct in src/info.rs
- [ ] T027 [US2] Define UsageStats struct in src/info.rs
- [ ] T028 [US2] Define TaskStatus enum in src/info.rs
- [ ] T029 [US2] Define PlanStatus enum in src/info.rs

### Parsers

- [ ] T030 [US2] Implement teams parser in src/parsers/teams.rs
- [ ] T031 [US2] Implement tasks parser in src/parsers/tasks.rs
- [ ] T032 [US2] Implement plans parser in src/parsers/plans.rs
- [ ] T033 [US2] Implement projects parser in src/parsers/projects.rs
- [ ] T034 [US2] Implement stats parser in src/parsers/stats.rs

### CLI

- [ ] T035 [US2] Add --teams CLI flag in src/cli.rs
- [ ] T036 [US2] Add --tasks CLI flag in src/cli.rs
- [ ] T037 [US2] Add --plans CLI flag in src/cli.rs
- [ ] T038 [US2] Add --projects CLI flag in src/cli.rs
- [ ] T039 [US2] Add --stats CLI flag in src/cli.rs

### Formatters

- [ ] T040 [US2] Add teams formatter for compact, detailed, JSON output in src/formatters/teams.rs
- [ ] T041 [US2] Add tasks formatter for compact, detailed, JSON output in src/formatters/tasks.rs
- [ ] T042 [US2] Add plans formatter for compact, detailed, JSON output in src/formatters/plans.rs
- [ ] T043 [US2] Add projects formatter for compact, detailed, JSON output in src/formatters/projects.rs
- [ ] T044 [US2] Add stats formatter for compact, detailed, JSON output in src/formatters/stats.rs

### Tests (Integration)

- [ ] T045 [US2] Add integration tests for all new CLI flags in tests/cli_test.rs

---

## Phase 5: US3-Sessions - 会话详情查看

**Goal**: 新增 --sessions-detail 选项显示会话详细信息

### Models (TDD - Write Tests First)

- [ ] T046 [US3] Add unit tests for sessions detail parser in tests/
- [ ] T047 [US3] Add integration test for --sessions-detail in tests/cli_test.rs
- [ ] T048 [US3] Add integration test for --last parameter limiting sessions in tests/cli_test.rs

### Models

- [ ] T049 [US3] Extend SessionInfo with detailed fields in src/info.rs

### Parsers

- [ ] T050 [US3] Implement sessions detail parser in src/parsers/sessions.rs

### CLI

- [ ] T051 [US3] Add --sessions-detail CLI flag in src/cli.rs
- [ ] T052 [US3] Add --last N parameter for limiting sessions in src/cli.rs

### Formatters

- [ ] T053 [US3] Add sessions detail formatter in src/formatters/sessions.rs

---

## Phase 6: US4-Health - 实时状态监控

**Goal**: 新增 --health 选项执行组件健康检查

### Models (TDD - Write Tests First)

- [ ] T054 [US4] Add unit tests for health checker in tests/health_test.rs
- [ ] T055 [US4] Add unit tests for compact health formatter in tests/
- [ ] T056 [US4] Add unit tests for detailed health formatter in tests/
- [ ] T057 [US4] Add integration test for --health in tests/cli_test.rs
- [ ] T058 [US4] Add integration test for --health --json output validation in tests/

### Models

- [ ] T059 [US4] Define HealthLevel enum in src/info.rs (already in T005)
- [ ] T060 [US4] Define ComponentHealth struct in src/info.rs (already in T005)
- [ ] T061 [US4] Define HealthStatus struct in src/info.rs (already in T005)

### Health Module

- [ ] T062 [US4] Create health checker module in src/health/checker.rs
- [ ] T063 [US4] Implement config integrity check in src/health/checker.rs
- [ ] T064 [US4] Implement version consistency check in src/health/checker.rs
- [ ] T065 [US4] Implement connection status check in src/health/checker.rs
- [ ] T066 [US4] Create health module exports in src/health/mod.rs

### Formatters

- [ ] T067 [US4] Add health status formatter for compact, detailed, JSON output in src/formatters/health.rs

### CLI

- [ ] T068 [US4] Add --health CLI flag in src/cli.rs
- [ ] T069 [US4] Add --verbose flag for detailed health output in src/cli.rs

---

## Final Phase: Polish & Cross-Cutting Concerns

- [ ] T070 Run cargo fmt and cargo clippy on all new code
- [ ] T071 Add CLI integration test for combined flags (--plugins --teams --health)
- [ ] T072 Update README with new CLI options documentation
- [ ] T073 Verify all acceptance scenarios pass with cargo test
