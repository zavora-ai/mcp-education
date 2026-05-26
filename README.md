# mcp-education

[![Crates.io](https://img.shields.io/crates/v/mcp-education.svg)](https://crates.io/crates/mcp-education)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

Education LMS MCP server — manage courses, assignments, grades, students, and announcements via Canvas LMS API. **12 tools** for instructors and administrators.

## Quick Start

```bash
cargo install mcp-education

CANVAS_BASE_URL=https://yourschool.instructure.com CANVAS_TOKEN=your_token mcp-education
```

## Tools (12)

### Read (8)
| Tool | Description |
|------|-------------|
| `list_courses` | All courses for authenticated user |
| `get_course` | Course details, syllabus, enrollment |
| `list_assignments` | All assignments in a course |
| `get_assignment` | Assignment details + submission summary |
| `list_students` | Enrolled students |
| `get_submissions` | All submissions for an assignment |
| `get_course_analytics` | Grade analytics summary |
| `list_modules` | Course content structure |
| `search_course_content` | Search pages, assignments, discussions |

### Write (3, gated)
| Tool | Description |
|------|-------------|
| `create_assignment` | Create new assignment (name, due date, points) |
| `grade_submission` | Grade a student's submission |
| `post_announcement` | Post announcement to course |

## Configuration

```json
{
  "mcpServers": {
    "education": {
      "command": "mcp-education",
      "env": {
        "CANVAS_BASE_URL": "https://yourschool.instructure.com",
        "CANVAS_TOKEN": "your_canvas_api_token"
      }
    }
  }
}
```

## Supported LMS

- **Canvas LMS** (Instructure) — primary backend, full API support
- Works with any Canvas instance (self-hosted or cloud)

## Getting a Canvas API Token

1. Log into Canvas → Account → Settings
2. Scroll to "Approved Integrations"
3. Click "+ New Access Token"
4. Copy the token

## License

Apache-2.0
