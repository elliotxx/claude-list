# Quickstart Guide: Detailed Output with Description

**Feature**: 003-detailed-description
**Date**: 2026-02-01

## Overview

This guide covers the enhanced `--long` (`-l`) output format with description column.

## New Feature: Description Column

### Before

```bash
$ claude-list --long
PLUGINS    2 installed
  NAME                            VERSION          SOURCE         PATH
  ...
```

### After

```bash
$ claude-list --long
PLUGINS    2 installed
  NAME                            SOURCE         DESCRIPTION
  ...
  context7                        official       Official plugin
```

## Usage

### Basic Usage

```bash
# Show detailed output with descriptions
claude-list --long

# Short flag also works
claude-list -l

# Combine with search
claude-list --long --search context

# Combine with filters
claude-list --long --plugins
```

### Output Columns

| Column | Description |
|--------|-------------|
| NAME | Component name (30 chars, colored) |
| SOURCE | "official" or "third-party" (15 chars) |
| DESCRIPTION | Component description (50 chars, truncated with "...") |

### Description Sources

| Component | Description |
|-----------|-------------|
| Plugin | "Official plugin" or "Third-party plugin" |
| Skill | From skill.yaml description field |
| MCP | "{status} MCP server" |
| Hook | "{type} hook" |
| Agent | From agent markdown description |
| Command | From command markdown description |

### Truncation Example

```bash
# Long descriptions are truncated
$ claude-list --long
SKILLS     1 available
  NAME                            SOURCE         DESCRIPTION
  ...
  my-skill                        official       This is a very long description that exceed...
```

## Comparison with Other Modes

### Compact Mode

```
CLAUDE-LIST v0.1.4
CONFIG: /Users/test/.claude

PLUGINS    2 installed
  context7
  my-plugin

SKILLS     3 available
  test-skill
  ...
```

### Detailed Mode (New)

```
CLAUDE-LIST v0.1.4
CONFIG: /Users/test/.claude

PLUGINS    2 installed
  NAME                            SOURCE         DESCRIPTION
  context7                        official       Official plugin
  my-plugin                       third-party    Third-party plugin
```

### JSON Mode

```bash
$ claude-list --json
{
  "version": "0.1.4",
  "plugins": [
    {
      "name": "context7",
      "source": "official",
      "description": "Official plugin"
    }
  ],
  ...
}
```

## Tips

1. Use `--no-color` to disable colors in scripts
2. Pipe to `head` to see first N lines: `claude-list --long | head -20`
3. Use `--search` with `--long` to filter and show descriptions

## Related

- See `contracts/detailed-output-format.md` for full specification
- See `data-model.md` for data structure details
