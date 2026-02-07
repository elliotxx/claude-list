# CLI Contracts: Claude Directory Enhancement

## 新增命令

### `--teams`

```bash
claude-list --teams          # 显示所有团队
claude-list --teams -l       # 详细模式
claude-list --teams --json   # JSON 格式
```

**输出示例 (compact)**:

```
TEAMS    2 configured
  development-team
    members: 3 (lead + 2 contributors)
  review-team
    members: 2 (reviewers)
```

**输出示例 (detailed)**:

```
TEAMS    2 configured

TEAM: development-team
  Description: Main development team
  Members:
    NAME             ROLE
    alice            lead
    bob              contributor
    charlie         contributor
  Created: 2025-01-15T10:30:00Z

TEAM: review-team
  Description: Code review team
  Members:
    NAME             ROLE
    david           reviewer
    eve             reviewer
  Created: 2025-02-01T14:20:00Z
```

---

### `--tasks`

```bash
claude-list --tasks              # 显示所有任务
claude-list --tasks --status pending    # 按状态筛选
claude-list --tasks --json       # JSON 格式
```

**输出示例 (compact)**:

```
TASKS   5 total (2 pending, 1 in_progress, 2 completed)
  [PENDING]   setup-ci           #123
  [IN PROGRESS] refactor-auth    #124
  [COMPLETED]  add-tests          #125
```

**输出示例 (detailed)**:

```
TASKS   5 total

TASK: setup-ci (#123)
  Status: pending
  Title: Set up CI pipeline
  Assignee: alice
  Dependencies: none
  Created: 2025-02-01T10:00:00Z

TASK: refactor-auth (#124)
  Status: in_progress
  Title: Refactor authentication
  Assignee: bob
  Dependencies: #123
  Created: 2025-02-02T09:00:00Z
```

---

### `--plans`

```bash
claude-list --plans             # 显示所有计划
claude-list --plans --status active  # 按状态筛选
claude-list --plans --json      # JSON 格式
```

**输出示例 (compact)**:

```
PLANS   3 documents
  [DRAFT]    q1-2025-roadmap
  [ACTIVE]    security-review
  [COMPLETED] migration-guide
```

**输出示例 (detailed)**:

```
PLANS   3 documents

PLAN: q1-2025-roadmap
  Status: draft
  Created: 2025-01-01T00:00:00Z
  Path: .claude/plans/q1-2025-roadmap/plan.md

PLAN: security-review
  Status: active
  Created: 2025-01-15T12:00:00Z
  Path: .claude/plans/security-review/plan.md
```

---

### `--projects`

```bash
claude-list --projects          # 显示项目统计
claude-list --projects -l      # 详细模式
claude-list --projects --json   # JSON 格式
```

**输出示例 (compact)**:

```
PROJECTS  2 active
  /Users/alice/project-a       47 sessions, 2h ago
  /Users/alice/project-b       23 sessions, 1d ago
```

**输出示例 (detailed)**:

```
PROJECTS  2 active

PROJECT: /Users/alice/project-a
  Sessions:      47
  Last Active:   2 hours ago
  Total Messages: 1,234
  Components:    5 plugins, 12 skills, 3 MCP

PROJECT: /Users/alice/project-b
  Sessions:      23
  Last Active:   1 day ago
  Total Messages: 567
  Components:    3 plugins, 8 skills, 1 MCP
```

---

### `--stats`

```bash
claude-list --stats             # 显示使用统计
claude-list --stats --json     # JSON 格式
```

**输出示例 (compact)**:

```
STATS   Last 30 days
  Total Sessions:     127
  Total Messages:     3,891
  Top Model:          sonnet-4 (67%)
  Most Active Day:    Tuesday (avg 15 sessions)
```

**输出示例 (detailed)**:

```
STATS   Last 30 days (2025-02-01 to 2025-02-28)

ACTIVITY
  Daily Average: 4.2 sessions, 130 messages
  Peak Day: Tuesday (avg 15 sessions)
  Slowest Day: Sunday (avg 1 session)

MODEL USAGE
  sonnet-4        2,607 (67%)
  haiku            1,040 (27%)
  opus-4           244 (6%)

HOURLY DISTRIBUTION
  09:00 ████████████ 45
  10:00 ████████████████ 62
  11:00 ██████████████ 55
  ...
```

---

### `--sessions-detail`

```bash
claude-list --sessions-detail              # 显示最近 10 个会话
claude-list --sessions-detail --last 5    # 只显示最近 5 个
claude-list --sessions-detail -l          # 详细模式
claude-list --sessions-detail --json      # JSON 格式
```

**输出示例 (compact)**:

```
SESSIONS   5 recent
  Today    3 sessions, 42 messages  /project-a
  Yesterday 5 sessions, 89 messages /project-b
  2 days   2 sessions, 15 messages /project-c
```

**输出示例 (detailed)**:

```
SESSIONS   5 recent

SESSION #abc123
  Project:   /project-a
  Model:     sonnet-4
  Messages:  23
  Started:   2025-02-07T10:30:00Z
  Duration:  45 minutes
  Summary:   Working on feature-X implementation

SESSION #def456
  Project:   /project-a
  Model:     haiku
  Messages:  12
  Started:   2025-02-07T09:00:00Z
  Duration:  15 minutes
  Summary:   Quick bug fix
```

---

### `--health`

```bash
claude-list --health              # 检查所有组件健康状态
claude-list --health --verbose    # 详细输出
claude-list --health --json      # JSON 格式
```

**输出示例 (compact)**:

```
HEALTH   Overall: ✓ Healthy

✓ Plugins      3 installed, all OK
✓ Skills       12 available, all OK
⚠ MCP Servers  2 configured, 1 warning
✓ Hooks        5 configured, all OK
✓ Agents       3 configured, all OK
```

**输出示例 (verbose)**:

```
HEALTH   Overall: ⚠ Warning

PLUGINS    ✓ Healthy
  - context7 v2.1.0 (official)
  - plugin_playwright v1.0.0 (third-party)
  - plugin_example v0.5.0 (community)

SKILLS     ✓ Healthy
  - brainstorming v1.0
  - claude-code-guide v2.0
  ...

MCP SERVERS ⚠ Warning
  - test-mcp: configured, not running
  - production-mcp: ✓ connected

HOOKS      ✓ Healthy
  - pre-commit-hook v1.0
  ...

COMPONENTS CHECK
  ✓ Config files present
  ✓ Dependencies resolved
  ⚠ MCP server test-mcp not running
```

## 扩展现有命令

### `--plugins` (扩展)

**新增输出字段**:

```
PLUGINS    3 installed
  NAME                 VERSION  SOURCE      DESCRIPTION
  -------------------  -------  ---------  ---------------------------------
  context7             2.1.0    official    Context management for Claude
  plugin_playwright    1.0.0    third-party Browser automation plugin
  plugin_example       0.5.0    community   Example plugin template
```

### `--mcp` (扩展)

**新增输出字段**:

```
MCP        2 servers
  NAME              STATUS      TYPE      PATH
  ----------------  ----------  --------  ---------------------------------
  test-mcp          configured  stdio      .claude/mcp-servers/test/
  production-mcp     connected   sse       .claude/mcp-servers/prod/
```
