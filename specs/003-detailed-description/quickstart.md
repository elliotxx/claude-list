# Quickstart Guide: Detailed Output with Description

**Feature**: 003-detailed-description
**Date**: 2026-02-05

## Overview

This guide covers the enhanced `--long` (`-l`) output format with description column.

## New Feature: Description Column

### Before (Compact Mode)

```bash
$ claude-list
PLUGINS    2 installed
  context7
  my-plugin

SKILLS     3 available
  test-skill
```

### After (Detailed Mode)

```bash
$ claude-list --long
PLUGINS    2 installed
  NAME                           PATH
  ------------------------------ --------------------------------------------------
  context7                       /Users/test/.claude/plugins/cache/...

SKILLS     3 available
  NAME                           SOURCE          DESCRIPTION
  test-skill                     global          A test skill for demo...
```

## Usage

### Basic Usage

```bash
# Show detailed output
claude-list --long

# Short flag also works
claude-list -l

# Combine with search
claude-list --long --search context

# Combine with filters
claude-list --long --plugins
```

### Output Columns by Component

#### Plugins
| Column | Width | Description |
|--------|-------|-------------|
| NAME | 30 chars | Plugin name (colored) |
| PATH | 80 chars | Full path to plugin directory |

#### Skills
| Column | Width | Description |
|--------|-------|-------------|
| NAME | 30 chars | Skill name (colored) |
| SOURCE | 25 chars | "global" or plugin name |
| DESCRIPTION | 50 chars | From skill.yaml (truncated with "...") |

#### MCP Servers
| Column | Width | Description |
|--------|-------|-------------|
| NAME | 30 chars | MCP server name (colored) |
| STATUS | 18 chars | Server status |
| DESCRIPTION | 50 chars | Derived: "{status} MCP server" |

#### Hooks
| Column | Width | Description |
|--------|-------|-------------|
| NAME | 30 chars | Hook name (colored) |
| TYPE | 18 chars | Hook type |
| DESCRIPTION | 50 chars | Derived: "{type} hook" |

#### Agents
| Column | Width | Description |
|--------|-------|-------------|
| NAME | 30 chars | Agent name (colored) |
| DESCRIPTION | 50 chars | From agent markdown |

#### Commands
| Column | Width | Description |
|--------|-------|-------------|
| NAME | 30 chars | Command name (colored) |
| DESCRIPTION | 50 chars | From command markdown |

### Truncation

Descriptions longer than 50 characters are truncated with "...":

```bash
SKILLS     1 available
  NAME                           SOURCE          DESCRIPTION
  my-skill                       global          This is a very long description that exceed...
```

## Output Comparison

### Compact Mode (Default)

```
CLAUDE-LIST v0.1.4
CONFIG: /Users/test/.claude

PLUGINS    2 installed
  context7
  my-plugin

SKILLS     3 available
  test-skill
  my-skill
```

### Detailed Mode (--long)

```
CLAUDE-LIST v0.1.4
CONFIG: /Users/test/.claude

PLUGINS    2 installed
  NAME                           PATH
  ------------------------------ --------------------------------------------------
  context7                       /Users/test/.claude/plugins/cache/claude-plugins-official/context7/27d2b86d72da
  my-plugin                      /Users/test/.claude/plugins/cache/my-org/my-plugin/1.0.0

SKILLS     3 available
  NAME                           SOURCE          DESCRIPTION
  test-skill                     global          A test skill for demo purposes
  my-skill                       my-plugin       Custom skill from my-plugin
```

### JSON Mode

```bash
$ claude-list --json
{
  "version": "0.1.4",
  "plugins": [
    {
      "name": "context7",
      "path": "/Users/test/.claude/plugins/cache/...",
      "source": "official"
    }
  ],
  "skills": [
    {
      "name": "test-skill",
      "description": "A test skill for demo purposes",
      "location_type": "global"
    }
  ]
}
```

## Tips

1. Use `--no-color` to disable colors in scripts
2. Pipe to `head` to see first N lines: `claude-list --long | head -20`
3. Use `--search` with `--long` to filter and show descriptions
4. Plugins show PATH instead of description for better visibility

## Related

- See `contracts/detailed-output-format.md` for full specification
- See `data-model.md` for data structure details
