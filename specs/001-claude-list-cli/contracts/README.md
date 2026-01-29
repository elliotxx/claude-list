# Contracts: CLAUDE-LIST CLI

This directory contains API contracts and data schemas.

## Not Applicable

CLAUDE-LIST is a local CLI tool that reads from the filesystem. There is no external API contract.

### Input Contract: `.claude` Directory Structure

The tool reads from the `.claude` directory following Claude Code's official structure:

```
.claude/
├── settings.json        # Contains installed plugins array
├── session_history.json # Session records
├── mcp.json            # MCP server configuration
├── hooks/              # Hook markdown files
│   └── *.md
├── skills/             # Skill directories
│   └── [skill-name]/
│       └── skill.yaml
├── agents/             # Agent markdown files
│   └── *.md
└── project_constitutions/
    └── *.md
```

### Output Contract: CLI Interface

See `/specs/001-claude-list-cli/plan.md` for CLI specification.

### Output Contract: JSON Format

```json
{
  "version": "1.0.0",
  "config_dir": "/path/to/.claude",
  "plugins": [
    {
      "name": "context7",
      "version": "2.1.0",
      "source": "official"
    }
  ],
  "skills": [...],
  "sessions": {
    "count": 47,
    "last_session": "2025-01-29T10:00:00Z"
  },
  "mcp_servers": [...],
  "hooks": [...],
  "agents": [...]
}
```
