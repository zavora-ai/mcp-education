# mcp-education

[![Crates.io](https://img.shields.io/crates/v/mcp-education.svg)](https://crates.io/crates/mcp-education)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

Education LMS MCP server — manage courses, assignments, grades, students, and announcements via Canvas LMS. **12 tools** for instructors, TAs, and administrators. Works with any Canvas instance (Instructure cloud or self-hosted).

## Installation

```bash
cargo install mcp-education
```

## Setup

### 1. Get a Canvas API Token

1. Log into your Canvas instance
2. Go to **Account** → **Settings**
3. Scroll to **Approved Integrations**
4. Click **+ New Access Token**
5. Give it a name (e.g. "MCP Agent") and click **Generate Token**
6. Copy the token (you won't see it again)

### 2. Run

```bash
CANVAS_BASE_URL=https://yourschool.instructure.com \
CANVAS_TOKEN=your_token \
mcp-education
```

## MCP Client Configuration

### Claude Desktop / Cursor

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

## Tools (12)

### Read (9)
| Tool | Description |
|------|-------------|
| `list_courses` | All courses for the authenticated user |
| `get_course` | Course details, syllabus, enrollment count |
| `list_assignments` | All assignments in a course |
| `get_assignment` | Assignment details, rubric, submission summary |
| `list_students` | Students enrolled in a course |
| `get_submissions` | All submissions for an assignment |
| `get_course_analytics` | Grade analytics and student summaries |
| `list_modules` | Course content structure (modules + items) |
| `search_course_content` | Search pages, assignments, discussions |

### Write (3, gated)
| Tool | Approval | Description |
|------|:--------:|-------------|
| `create_assignment` | Yes | Create assignment with name, due date, points, submission types |
| `grade_submission` | Yes | Grade a student's work with score and comment |
| `post_announcement` | Yes | Post announcement to a course |

## Usage Examples

### "What courses am I teaching?"
```
→ list_courses()

Results:
  - CS 101: Intro to Programming (45 students)
  - CS 301: Data Structures (32 students)
```

### "Show me ungraded submissions for the midterm"
```
→ list_assignments(course_id="12345")
→ get_submissions(course_id="12345", assignment_id="67890")

Results:
  - Alice Johnson: submitted 2026-05-20, not graded
  - Bob Smith: submitted 2026-05-21, not graded
```

### "Create a homework assignment due Friday"
```
→ create_assignment(
    course_id="12345",
    name="Week 8 Problem Set",
    description="Complete exercises 4.1-4.5",
    due_at="2026-05-30T23:59:00Z",
    points_possible=100,
    submission_types=["online_upload"]
  )
```

### "Grade Alice's submission"
```
→ grade_submission(
    course_id="12345",
    assignment_id="67890",
    student_id="11111",
    grade="92",
    comment="Excellent work on the recursion section!"
  )
```

### "Announce the exam schedule"
```
→ post_announcement(
    course_id="12345",
    title="Final Exam Schedule",
    message="The final exam will be on June 10th at 9am in Room 301."
  )
```

## Environment Variables

| Variable | Required | Default | Description |
|----------|:--------:|---------|-------------|
| `CANVAS_BASE_URL` | Yes | `https://canvas.instructure.com` | Your Canvas instance URL |
| `CANVAS_TOKEN` | Yes | — | Canvas API access token |

## Supported Platforms

- **Canvas LMS** (Instructure) — full support
- Any Canvas instance: university, K-12, corporate training
- Cloud-hosted or self-hosted

## Security

- Write operations (`create_assignment`, `grade_submission`, `post_announcement`) are marked as requiring approval in governed environments
- The Canvas token scope determines what the agent can access
- Use a token with minimal permissions (e.g. TA role for grading only)

## License

Apache-2.0
