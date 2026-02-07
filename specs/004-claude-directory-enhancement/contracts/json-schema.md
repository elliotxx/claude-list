# JSON Output Schemas

This document defines the JSON output structures for all new entities.

## PluginInfo (Extended)

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "name": { "type": "string" },
    "version": { "type": "string" },
    "source": {
      "type": "string",
      "enum": ["official", "third-party", "community"]
    },
    "description": { "type": "string" },
    "marketplace_url": { "type": "string" }
  },
  "required": ["name", "version", "source"]
}
```

## McpServerInfo (Extended)

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "name": { "type": "string" },
    "connection_status": {
      "type": "string",
      "enum": ["connected", "configured", "error", "unknown"]
    },
    "type": { "type": "string" },
    "path": { "type": "string" }
  },
  "required": ["name", "connection_status"]
}
```

## TeamInfo

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "name": { "type": "string" },
    "description": { "type": "string" },
    "members": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "name": { "type": "string" },
          "role": {
            "type": "string",
            "enum": ["lead", "reviewer", "contributor"]
          }
        },
        "required": ["name", "role"]
      }
    },
    "created_at": { "type": "string", "format": "date-time" }
  },
  "required": ["name", "members"]
}
```

## TaskInfo

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "id": { "type": "string" },
    "title": { "type": "string" },
    "description": { "type": "string" },
    "status": {
      "type": "string",
      "enum": ["pending", "in_progress", "completed", "blocked"]
    },
    "assignee": { "type": "string" },
    "dependencies": {
      "type": "array",
      "items": { "type": "string" }
    },
    "created_at": { "type": "string", "format": "date-time" },
    "updated_at": { "type": "string", "format": "date-time" }
  },
  "required": ["id", "title", "status"]
}
```

## PlanInfo

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "id": { "type": "string" },
    "title": { "type": "string" },
    "description": { "type": "string" },
    "status": {
      "type": "string",
      "enum": ["draft", "active", "completed", "archived"]
    },
    "file_path": { "type": "string" },
    "created_at": { "type": "string", "format": "date-time" },
    "updated_at": { "type": "string", "format": "date-time" }
  },
  "required": ["id", "title", "status"]
}
```

## ProjectStats

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "project_path": { "type": "string" },
    "session_count": { "type": "integer" },
    "last_active": { "type": "string", "format": "date-time" },
    "total_messages": { "type": "integer" },
    "components_summary": {
      "type": "object",
      "properties": {
        "plugins": { "type": "integer" },
        "skills": { "type": "integer" },
        "mcp_servers": { "type": "integer" },
        "agents": { "type": "integer" },
        "commands": { "type": "integer" },
        "hooks": { "type": "integer" }
      }
    }
  },
  "required": ["project_path", "session_count"]
}
```

## UsageStats

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "period_start": { "type": "string", "format": "date-time" },
    "period_end": { "type": "string", "format": "date-time" },
    "total_sessions": { "type": "integer" },
    "total_messages": { "type": "integer" },
    "daily_activity": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "date": { "type": "string", "pattern": "^\\d{4}-\\d{2}-\\d{2}$" },
          "sessions": { "type": "integer" },
          "messages": { "type": "integer" }
        }
      }
    },
    "model_usage": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "model": { "type": "string" },
          "count": { "type": "integer" },
          "percentage": { "type": "number" }
        }
      }
    },
    "hourly_distribution": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "hour": { "type": "integer", "minimum": 0, "maximum": 23 },
          "count": { "type": "integer" }
        }
      }
    }
  },
  "required": ["period_start", "period_end", "total_sessions", "total_messages"]
}
```

## SessionDetail

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "id": { "type": "string" },
    "project_path": { "type": "string" },
    "model": { "type": "string" },
    "message_count": { "type": "integer" },
    "started_at": { "type": "string", "format": "date-time" },
    "ended_at": { "type": "string", "format": "date-time" },
    "summary": { "type": "string" }
  },
  "required": ["id", "project_path", "message_count", "started_at"]
}
```

## HealthStatus

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "overall_status": {
      "type": "string",
      "enum": ["healthy", "warning", "error"]
    },
    "components": {
      "type": "array",
      "items": {
        "$ref": "#/definitions/ComponentHealth"
      }
    },
    "checked_at": { "type": "string", "format": "date-time" },
    "duration_ms": { "type": "number" }
  },
  "required": ["overall_status", "components", "duration_ms"],
  "definitions": {
    "ComponentHealth": {
      "type": "object",
      "properties": {
        "component_type": { "type": "string" },
        "status": {
          "type": "string",
          "enum": ["healthy", "warning", "error"]
        },
        "message": { "type": "string" },
        "details": {
          "type": "object",
          "additionalProperties": { "type": "string" }
        }
      },
      "required": ["component_type", "status"]
    }
  }
}
```
