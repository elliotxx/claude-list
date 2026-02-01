# Implementation Tasks: Detailed Output with Description Feature

**Feature Branch**: `003-detailed-description`
**Generated**: 2026-02-01
**Spec**: [spec.md](spec.md)
**Plan**: [plan.md](plan.md)

---

## MVP Scope

**Minimum Viable Product**: User Story 1 + User Story 2 (core functionality)

- `--long` output shows NAME, SOURCE, DESCRIPTION columns
- Version and Path columns removed
- Description truncation works correctly
- P2 (US3) can be deferred if needed

---

## Phases

### Phase 1: Setup (No dependencies)

**Purpose**: Project initialization - create utilities needed by all user stories

- [x] T001 [P] Create: TruncationConfig struct in src/output.rs @src/output.rs
- [x] T002 [P] Create: truncate_with_ellipsis function using unicode-width @src/output.rs
- [x] T003 [P] Create: DescriptionProvider trait for all component types @src/info.rs

**Checkpoint**: Truncation utility ready for all stories

---

### Phase 2: Data Model (Blocking - Required for all User Stories)

**Purpose**: Add description fields to data structures used by all components

- [x] T010 Add: description field to PluginInfo struct @src/info.rs
- [x] T011 Add: description field to McpInfo struct @src/info.rs
- [x] T012 Add: description field to HookInfo struct @src/info.rs
- [x] T013 Add: DescriptionProvider impl for PluginInfo @src/info.rs
- [x] T014 Add: DescriptionProvider impl for McpInfo @src/info.rs
- [x] T015 Add: DescriptionProvider impl for HookInfo @src/info.rs
- [x] T016 Add: DescriptionProvider impl for SkillInfo @src/info.rs
- [x] T017 Add: DescriptionProvider impl for AgentInfo @src/info.rs
- [x] T018 Add: DescriptionProvider impl for CommandInfo @src/info.rs

**Checkpoint**: All components can provide descriptions

---

### Phase 3: User Story 1 - Enhanced Detailed Output (Priority: P1)

**Goal**: Users see NAME, SOURCE, DESCRIPTION columns instead of VERSION and PATH

**Independent Test**: Run `claude-list --long` and verify output has only NAME, SOURCE, DESCRIPTION columns

#### Implementation

- [x] T020 [US1] Modify: Rewrite PLUGINS section in format_detailed @src/formatters/detailed.rs
- [x] T021 [US1] Modify: Rewrite SKILLS section in format_detailed @src/formatters/detailed.rs
- [x] T022 [US1] Modify: Rewrite MCP section to show STATUS and DESCRIPTION @src/formatters/detailed.rs
- [x] T023 [US1] Modify: Rewrite HOOKS section to show TYPE and DESCRIPTION @src/formatters/detailed.rs
- [x] T024 [US1] Modify: Keep AGENTS section (already has DESCRIPTION) @src/formatters/detailed.rs
- [x] T025 [US1] Modify: Keep COMMANDS section (already has DESCRIPTION) @src/formatters/detailed.rs
- [x] T026 [US1] Fix: Use installPath for plugin PATH (actual cache location) @src/parsers/plugins.rs

**Note**: Plugin section shows PATH instead of DESCRIPTION (to avoid redundancy with SOURCE column).
PATH shows actual plugin installation location: installPath > projectPath > config file path.

#### Tests

- [x] T030 [US1] Test: Unit test for PLUGINS output format @src/formatters/detailed.rs
- [x] T031 [US1] Test: Unit test for SKILLS output format @src/formatters/detailed.rs
- [x] T032 [US1] Test: Integration test: --long shows correct columns @tests/cli_test.rs
- [x] T033 [US1] Test: Integration test: --long with plugins filter @tests/cli_test.rs

**Checkpoint**: US1 complete - `claude-list --long` shows NAME, SOURCE, DESCRIPTION

---

### Phase 4: User Story 2 - Description Truncation (Priority: P1)

**Goal**: Long descriptions are truncated with "..." and handle Unicode correctly

**Independent Test**: Run `claude-list --long` and verify descriptions over 50 chars are truncated

#### Implementation

- [x] T040 [US2] Verify: truncate_with_ellipsis handles Unicode correctly @src/output.rs
- [x] T041 [US2] Modify: Apply truncation to all description fields in detailed formatter @src/formatters/detailed.rs

#### Tests

- [x] T050 [US2] Test: Unit test for truncation with ASCII text @src/output.rs
- [x] T051 [US2] Test: Unit test for truncation with Unicode/CJK characters @src/output.rs
- [x] T052 [US2] Test: Integration test: long description truncated @tests/cli_test.rs
- [x] T053 [US2] Test: Integration test: short description not truncated @tests/cli_test.rs

**Checkpoint**: US2 complete - descriptions over 50 chars show "..."

---

### Phase 5: User Story 3 - All Component Types (Priority: P2)

**Goal**: All 6 component types show descriptions in --long output

**Independent Test**: Run `claude-list --long` and verify all sections show descriptions

#### Implementation

- [x] T060 [US3] Verify: Plugin description shows "Official/Third-party plugin" @src/formatters/detailed.rs
- [x] T061 [US3] Verify: MCP description shows "{status} MCP server" @src/formatters/detailed.rs
- [x] T062 [US3] Verify: Hook description shows "{type} hook" @src/formatters/detailed.rs

#### Tests

- [x] T070 [US3] Test: Integration test: all component types show descriptions @tests/cli_test.rs
- [x] T071 [US3] Test: Integration test: empty description shows "-" placeholder @tests/cli_test.rs

**Checkpoint**: US3 complete - all 6 types show descriptions

---

### Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final validation and cleanup

- [x] T080 Run: make check (fmt, clippy, test) @project root
- [ ] T081 Update: quickstart.md with new output examples @specs/003-detailed-description/quickstart.md
- [ ] T082 Update: Integration test snapshots if using insta @tests/

---

## Summary

| Phase | User Story | Tasks | Description | Status |
|-------|------------|-------|-------------|--------|
| 1 | - | 3 | Setup (utilities) | ✅ Complete |
| 2 | - | 9 | Data model changes | ✅ Complete |
| 3 | US1 | 9 | Enhanced detailed output | ✅ Complete |
| 4 | US2 | 4 | Description truncation | ✅ Complete |
| 5 | US3 | 4 | All component types | ✅ Complete |
| 6 | - | 3 | Polish & validation | ⚠️ Partial |

**Total Tasks**: 32
**Completed**: 30
**Remaining**: 2 (documentation updates)

---

## Dependency Graph

```
Phase 1 (Setup) ──┬─> T001 (TruncationConfig)
                  ├─> T002 (truncate function)
                  └─> T003 (DescriptionProvider trait)

Phase 1 ──────────> Phase 2 (Data Model)

Phase 2 ──────────> Phase 3 (US1) ──┬─> T020-T025 (formatter sections)
                  │                  │
                  │                  └─> T030-T033 (tests)
                  │
                  ├─────────────────> Phase 4 (US2)
                  │                  ├─> T040 (verify truncation)
                  │                  └─> T041 (apply truncation)
                  │
                  └─────────────────> Phase 5 (US3)
                     (uses existing descriptions)

Phase 3 + Phase 4 ──> Phase 6 (Polish)
```

---

## Parallel Execution Examples

### Within Phase 1
T001, T002, T003 can run in parallel (different files, no dependencies)

### Within Phase 2
T010-T018 can run in parallel (different structs, no dependencies)

### Within Phase 3
T020-T025 can run in parallel (different sections, same formatter file)
T030-T033 can run in parallel (different tests)

### Between Phases
Phase 1 → Phase 2 → Phase 3 (blocking dependencies)
US1 (Phase 3) and US2 (Phase 4) can be done in parallel after Phase 2

---

## Implementation Strategy

### MVP First (Recommended Order)

1. **Week 1**: Complete Phase 1 + Phase 2 + Phase 3
   - Core functionality: NAME, SOURCE, DESCRIPTION columns
   - Test with `claude-list --long`

2. **Week 2**: Complete Phase 4 + Phase 5
   - Polish: truncation, Unicode, all component types
   - Final validation

### Alternative: Incremental Delivery

1. **Sprint 1**: Phase 1 + Phase 2 + T020 + T030
   - Just Plugins section for testing

2. **Sprint 2**: Complete Phase 3
   - All sections with new format

3. **Sprint 3**: Phase 4 + Phase 5
   - Truncation and all component types

---

## Testing Strategy

### Unit Tests (src/)
- Truncation logic (T050, T051)
- Formatter sections (T030, T031)
- Data structures (T013-T018)

### Integration Tests (tests/cli_test.rs)
- --long flag output format (T032, T033)
- Description truncation (T052, T053)
- All component types (T070, T071)
- Empty/edge cases (T071)

### Manual Testing
```bash
# Verify new format
claude-list --long

# Verify truncation
claude-list --long --plugins | head -10

# Verify all sections
claude-list --long --skills --mcp --hooks --agents --commands

# Compare with old format
claude-list --long > new_output.txt
git diff HEAD -- formatters/detailed.rs
```
