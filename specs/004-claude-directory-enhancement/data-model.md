# Data Model: Claude Directory Enhancement

## 概述

本文档定义新增功能的数据模型，包括新增实体和扩展现有实体。

## 新增实体

### TeamInfo

**位置**: `.claude/agents/teams/{team_name}/team.yaml`

```rust
pub struct TeamInfo {
    pub name: String,
    pub description: Option<String>,
    pub members: Vec<TeamMember>,
    pub created_at: Option<DateTime<Utc>>,
}

pub struct TeamMember {
    pub name: String,
    pub role: TeamRole, // reviewer | contributor | lead
}
```

### TaskInfo

**位置**: `.claude/tasks/{task_id}/task.yaml`

```rust
pub struct TaskInfo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus, // pending | in_progress | completed | blocked
    pub assignee: Option<String>,
    pub dependencies: Vec<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Blocked,
}
```

### PlanInfo

**位置**: `.claude/plans/{plan_id}/plan.md` (YAML frontmatter)

```rust
pub struct PlanInfo {
    pub id: String,
    pub title: String,
    pub description: Option<String>, // 从 markdown 提取
    pub status: PlanStatus, // draft | active | completed | archived
    pub file_path: PathBuf,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub enum PlanStatus {
    Draft,
    Active,
    Completed,
    Archived,
}
```

### ProjectStats

```rust
pub struct ProjectStats {
    pub project_path: PathBuf,
    pub session_count: usize,
    pub last_active: Option<DateTime<Utc>>,
    pub total_messages: usize,
    pub components_summary: ComponentSummary,
}

pub struct ComponentSummary {
    pub plugins: usize,
    pub skills: usize,
    pub mcp_servers: usize,
    pub agents: usize,
    pub commands: usize,
    pub hooks: usize,
}
```

### UsageStats

```rust
pub struct UsageStats {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub daily_activity: Vec<DailyActivity>,
    pub model_usage: Vec<ModelUsage>,
    pub hourly_distribution: Vec<HourlyCount>,
    pub total_sessions: usize,
    pub total_messages: usize,
}

pub struct DailyActivity {
    pub date: chrono::NaiveDate,
    pub sessions: usize,
    pub messages: usize,
}

pub struct ModelUsage {
    pub model: String,
    pub count: usize,
    pub percentage: f64,
}

pub struct HourlyCount {
    pub hour: u8, // 0-23
    pub count: usize,
}
```

### HealthStatus

```rust
pub struct HealthStatus {
    pub overall_status: HealthLevel, // healthy | warning | error
    pub components: Vec<ComponentHealth>,
    pub checked_at: DateTime<Utc>,
    pub duration_ms: u64,
}

pub struct ComponentHealth {
    pub component_type: ComponentType, // plugins | skills | mcp | etc
    pub status: HealthLevel,
    pub message: Option<String>,
    pub details: Option<HashMap<String, String>>,
}

pub enum HealthLevel {
    Healthy,   // ✓
    Warning,   // ⚠
    Error,     // ✗
}
```

## 扩展现有实体

### PluginInfo (扩展)

**新增字段**:

```rust
pub struct PluginInfo {
    // ... 现有字段
    pub description: Option<String>,      // NEW: 插件描述
    pub source: PluginSource,              // NEW: 来源标识
    pub marketplace_url: Option<String>,   // NEW: marketplace 链接
}
```

### SessionDetail (新增)

```rust
pub struct SessionDetail {
    pub id: String,
    pub project_path: PathBuf,
    pub model: Option<String>,
    pub message_count: usize,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub summary: Option<String>, // 从消息提取的摘要
}
```

## 关系图

```
ProjectStats
├── ComponentSummary
│   └── 指向各组件类型计数
└── 依赖 history.jsonl

UsageStats
├── DailyActivity ← history.jsonl
├── ModelUsage ← Claude Code 日志
└── HourlyDistribution ← history.jsonl

HealthStatus
├── ComponentHealth × N
│   ├── plugins → PluginInfo
│   ├── skills → SkillInfo
│   ├── mcp → MCPConnection
│   └── ...
└── 独立检查
```

## 枚举定义

```rust
pub enum PluginSource {
    Official,
    ThirdParty,
    Community,
}

pub enum ComponentType {
    Plugins,
    Skills,
    Sessions,
    McpServers,
    Hooks,
    Agents,
    Commands,
    Teams,      // NEW
    Tasks,      // NEW
    Plans,      // NEW
    Projects,    // NEW
    Stats,      // NEW
}
```
