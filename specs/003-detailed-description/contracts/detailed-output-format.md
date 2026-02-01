# Detailed Output Format Specification

## Overview

This document defines the detailed output format for the `--long` (`-l`) flag in `claude-list`.

## Output Structure

### Header

```
CLAUDE-LIST v{version}
CONFIG: {config_dir}
```

### Component Sections

Each component type is displayed in its own section:

```
{TYPE_NAME}    {count} {suffix}
  {header_row}
  {divider_row}
  {data_rows}
```

### Column Format

| Component Type | Column 1 | Column 2 | Column 3 |
|----------------|----------|----------|----------|
| Plugins | NAME (30) | SOURCE (15) | DESCRIPTION (50) |
| Skills | NAME (30) | SOURCE (15) | DESCRIPTION (50) |
| MCP | NAME (30) | STATUS (18) | DESCRIPTION (50) |
| Hooks | NAME (30) | TYPE (18) | DESCRIPTION (50) |
| Agents | NAME (30) | DESCRIPTION (50) | - |
| Commands | NAME (30) | DESCRIPTION (50) | - |

### Header Examples

**Plugins/Skills**:
```
NAME                            SOURCE         DESCRIPTION
```

**MCP**:
```
NAME                            STATUS         DESCRIPTION
```

**Hooks**:
```
NAME                            TYPE           DESCRIPTION
```

**Agents/Commands**:
```
NAME                            DESCRIPTION
```

## Description Truncation

### Rules

1. Maximum display width: 50 characters
2. When description exceeds max width:
   - Truncate at character boundary
   - Append "..." (3 characters)
3. Unicode characters (CJK): count as 2 width units each
4. When no description available: display "-"

### Example

Input description: "This is a very long description that exceeds fifty characters and should be truncated"

Output: "This is a very long description that exceeds fifty ch..."

## Alignment

- All columns are left-aligned except VERSION (if shown)
- Header and divider rows match column widths
- Divider uses "-" repeated for each column width

## Example Output

```
CLAUDE-LIST v0.1.4
CONFIG: /Users/test/.claude

PLUGINS    2 installed
  NAME                            SOURCE         DESCRIPTION
  ------------------------------  -------------  --------------------------------------------------
  context7                        official       Official plugin
  my-plugin                       third-party    Third-party plugin

SKILLS     3 available
  NAME                            SOURCE         DESCRIPTION
  ------------------------------  -------------  --------------------------------------------------
  test-skill                      official       A test skill for unit testing
  analyze-code                    third-party    Third-party skill for code analysis

MCP        1 servers
  NAME                            STATUS         DESCRIPTION
  ------------------------------  -------------  --------------------------------------------------
  test-mcp                        connected      connected MCP server

HOOKS      2 configured
  NAME                            TYPE           DESCRIPTION
  ------------------------------  -------------  --------------------------------------------------
  pre-commit                      pre-commit     pre-commit hook

AGENTS     1 defined
  NAME                            DESCRIPTION
  ------------------------------  --------------------------------------------------
  database-agent                  Agent for database operations

COMMANDS   1 available
  NAME                            DESCRIPTION
  ------------------------------  --------------------------------------------------
  /analyze-code                   Analyze code quality
```

## Error Cases

### Empty Component List

```
PLUGINS    0 installed
```

(No table header or rows shown)

### All Empty

```
CLAUDE-LIST v0.1.4
CONFIG: /Users/test/.claude

(No component sections displayed)
```
