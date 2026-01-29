# Tasks: CLAUDE-LIST CLI

**Input**: Design documents from `/specs/001-claude-list-cli/`
**Prerequisites**: plan.md (required), spec.md (required), data-model.md (required)

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Source code**: `src/` at repository root
- **Tests**: `tests/` at repository root
- **Fixtures**: `tests/fixtures/.claude/` for mock data

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Initialize Rust project with Cargo and create directory structure
- [x] T002 [P] Configure Cargo.toml with dependencies (clap, serde, anyhow)
- [x] T003 [P] Configure Rust toolchain (rust-version 1.75+) and CI/CD

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 Create data structures in src/info.rs (ClaudeInfo, Item, ItemKind, Source)
- [x] T005 [P] Create error types in src/error.rs (ParseError, Display impls)
- [x] T006 Create CLI arguments struct in src/cli.rs (Args struct with clap derive)
- [x] T007 [P] Create lib.rs with module declarations and re-exports
- [x] T008 Create main.rs entry point with error handling wrapper

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - View Claude Environment Overview (Priority: P1) MVP

**Goal**: Users can run `claude-list` without arguments and see component counts and names

**Independent Test**: Run `claude-list` on mock .claude directory and verify counts are displayed

### Tests for User Story 1 ⚠️ (Required by Constitution II)

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation (TDD required)**

- [x] T009 [P] [US1] Integration test: compact output displays all component types in tests/integration/test_cli.rs
- [x] T010 [P] [US1] Snapshot test: verify compact output format matches expected in tests/integration/snapshots/

### Implementation for User Story 1

- [x] T011 [P] [US1] Create plugins parser in src/parsers/plugins.rs
- [x] T012 [P] [US1] Create skills parser in src/parsers/skills.rs
- [x] T013 [P] [US1] Create sessions parser in src/parsers/sessions.rs
- [x] T014 [P] [US1] Create mcp parser in src/parsers/mcp.rs
- [x] T015 [P] [US1] Create hooks parser in src/parsers/hooks.rs
- [x] T016 [P] [US1] Create agents parser in src/parsers/agents.rs
- [x] T017 [P] [US1] Create parsers module in src/parsers/mod.rs
- [x] T018 [P] [US1] Create compact formatter in src/formatters/compact.rs
- [x] T019 [US1] Create formatters module in src/formatters/mod.rs
- [x] T020 [US1] Implement main parsing logic (aggregate all parsers, filter empty groups)
- [x] T021 [US1] Connect CLI args to parser execution and compact formatter

**Checkpoint**: User Story 1 complete - basic `claude-list` works

---

## Phase 4: User Story 2 - View Detailed Component Information (Priority: P2)

**Goal**: Users can run `claude-list -l` to see version numbers, sources, and paths

**Independent Test**: Run `claude-list -l` and verify detailed info is displayed for each component

### Tests for User Story 2

- [x] T022 [P] [US2] Integration test: detailed output with -l flag in tests/integration/test_cli.rs
- [x] T023 [P] [US2] Snapshot test: verify detailed output format in tests/integration/snapshots/

### Implementation for User Story 2

- [x] T024 [US2] Create detailed formatter in src/formatters/detailed.rs
- [x] T025 [US2] Add version field extraction to plugins parser (src/parsers/plugins.rs)
- [x] T026 [US2] Add version field extraction to skills parser (src/parsers/skills.rs)
- [x] T027 [US2] Update compact formatter to support full info display (-ll flag)
- [x] T028 [US2] Connect -l and -ll flags to detailed formatter

**Checkpoint**: User Story 2 complete - `claude-list -l` and `claude-list -ll` work

---

## Phase 5: User Story 3 - Filter and Query Specific Component Types (Priority: P3)

**Goal**: Users can filter output to show only specific component types

**Independent Test**: Run `claude-list --plugins` and verify only plugins are displayed

### Tests for User Story 3 ⚠️ (Required by Constitution II)

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation (TDD required)**

- [x] T029 [P] [US3] Integration test: --plugins filter in tests/integration/test_cli.rs
- [x] T030 [P] [US3] Integration test: --skills filter in tests/integration/test_cli.rs
- [x] T031 [P] [US3] Integration test: --json output in tests/integration/test_cli.rs
- [x] T031b [P] [US3] Integration test: missing .claude directory error in tests/integration/test_cli.rs
- [x] T031c [P] [US3] Integration test: malformed JSON handling in tests/integration/test_cli.rs
- [x] T031d [P] [US3] Integration test: missing optional files (session_history.json) in tests/integration/test_cli.rs

### Implementation for User Story 3

- [x] T032 [US3] Create JSON formatter in src/formatters/json.rs
- [x] T033 [US3] Implement filtering logic in main parsing (respect --plugins, --skills, etc. flags)
- [x] T034 [US3] Implement JSON output mode with serde
- [x] T035 [US3] Connect filter flags to parser execution and formatter selection

**Checkpoint**: User Story 3 complete - all filtering and JSON output work

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T036 [P] Add integration tests for edge cases (missing files, malformed JSON) in tests/integration/
- [x] T037 [P] Add unit tests for parsers in tests/unit/
- [x] T038 Create test fixtures in tests/fixtures/.claude/ with mock data
- [x] T039 Add performance validation (ensure <2 seconds execution)
- [x] T040 [P] Documentation updates: update CLAUDE.md
- [x] T041 Run cargo fmt, cargo clip, add usage examplespy, and fix any warnings

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - No dependencies on other stories

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation
- Parsers before formatters
- Core data structures before parsers
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational is done, all user stories can start in parallel
- All tests for a user story marked [P] can run in parallel
- Parsers for different component types marked [P] can run in parallel

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test `claude-list` output independently
5. Demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Demo (MVP!)
3. Add User Story 2 → Test independently → Demo
4. Add User Story 3 → Test independently → Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (parsers + compact formatter)
   - Developer B: User Story 2 (detailed formatter)
   - Developer C: User Story 3 (filtering + JSON formatter)
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence

## Task Summary

| Metric | Count |
|--------|-------|
| Total Tasks | 41 |
| Setup Tasks | 3 |
| Foundational Tasks | 5 |
| User Story 1 Tasks | 13 |
| User Story 2 Tasks | 5 |
| User Story 3 Tasks | 6 |
| Polish Tasks | 6 |
| Parallelizable Tasks | 25 |
