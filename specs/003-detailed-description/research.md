# Research: Detailed Output with Description Feature

## Decision Summary

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Description truncation width | 50 characters | 标准终端宽度下合理的信息量 |
| Unicode handling | `unicode-width` crate | Rust 生态标准库，正确处理中文等宽字符 |
| No-description placeholder | "-" | 与现有空值表示一致 |
| Column widths | NAME=30, SOURCE=15, DESCRIPTION=50 | 平衡信息密度和可读性 |

## Details

### Unicode Width Handling

**Decision**: Use `unicode-width` crate for correct character width calculation.

**Rationale**:
- Rust 生态中处理 Unicode 字符显示宽度的标准解决方案
- 正确处理 CJK 字符（中文、日文、韩文）为 2 个宽度单位
- 与现有 `unicode-width` 依赖保持一致（已用于 `anstyle`）

**Implementation approach**:
```rust
use unicode_width::UnicodeWidthStr;

fn truncate_with_ellipsis(text: &str, max_width: usize) -> String {
    if text.width() <= max_width {
        return text.to_string();
    }
    // 计算可容纳的字符数，减去省略号的宽度
    let ellipsis_width = "...".width();
    let mut width = 0;
    let mut result = String::new();
    for c in text.chars() {
        let char_width = c.width().unwrap_or(0);
        if width + char_width + ellipsis_width > max_width {
            break;
        }
        width += char_width;
        result.push(c);
    }
    result + "..."
}
```

### Column Width Configuration

**Decision**: Fixed column widths optimized for standard terminal (80 columns)

| Column | Width | Rationale |
|--------|-------|-----------|
| NAME | 30 | 保持插件/技能名称识别度 |
| SOURCE | 15 | "official" 或 "third-party" 足够 |
| DESCRIPTION | 50 | 描述信息的主要空间 |

### Description Source Strategy

**Decision**: 为没有描述的组件类型添加推断描述

| Component | Has Description | Strategy |
|-----------|-----------------|----------|
| Plugin | No | 使用固定模板: "Official/Third-party plugin" |
| Skill | Yes | 直接使用 `description` 字段 |
| MCP | No | 使用模板: "{status} MCP server" |
| Hook | No | 使用模板: "{hook_type} hook" |
| Agent | Yes | 直接使用 `description` 字段 |
| Command | Yes | 直接使用 `description` 字段 |

### Comparison: Existing vs New Format

**Before (current detailed format)**:
```
PLUGINS    2 installed
  NAME                            VERSION          SOURCE         PATH
  ------------------------------  ----------------  -------------  ----------------------------------------
  context7                        2.1.0            official       /Users/test/.claude/settings.json

SKILLS     1 available
  NAME                            VERSION          SOURCE         PATH
  ...
```

**After (new detailed format)**:
```
PLUGINS    2 installed
  NAME                            SOURCE         DESCRIPTION
  ------------------------------  -------------  --------------------------------------------------
  context7                        official       Official plugin

SKILLS     1 available
  NAME                            SOURCE         DESCRIPTION
  ...
```

## Alternatives Considered

1. **Dynamic terminal width detection**:
   - Rejected: 增加了复杂性，标准终端宽度（80列）足够满足需求

2. **Always show full description**:
   - Rejected: 长描述会破坏终端输出格式

3. **Description only when available**:
   - Rejected: 不一致的用户体验

## Open Questions Resolved

- ✅ Q1: Description truncation width? → 50 characters
- ✅ Q2: Unicode handling? → `unicode-width` crate
- ✅ Q3: No-description placeholder? → "-"
