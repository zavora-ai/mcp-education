use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};
use reqwest::Client;
use serde_json::Value;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct EmptyInput {}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CourseIdInput {
    /// Canvas course ID
    pub course_id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct AssignmentInput {
    /// Canvas course ID
    pub course_id: String,
    /// Assignment ID
    pub assignment_id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CreateAssignmentInput {
    /// Course ID
    pub course_id: String,
    /// Assignment name
    pub name: String,
    /// Description (HTML or plain text)
    pub description: Option<String>,
    /// Due date (ISO 8601)
    pub due_at: Option<String>,
    /// Points possible
    pub points_possible: Option<f64>,
    /// Submission types: online_text_entry, online_upload, online_url
    pub submission_types: Option<Vec<String>>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GradeInput {
    /// Course ID
    pub course_id: String,
    /// Assignment ID
    pub assignment_id: String,
    /// Student ID
    pub student_id: String,
    /// Grade (points or letter)
    pub grade: String,
    /// Comment (optional)
    pub comment: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct AnnouncementInput {
    /// Course ID
    pub course_id: String,
    /// Announcement title
    pub title: String,
    /// Announcement body (HTML or plain text)
    pub message: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SearchInput {
    /// Course ID
    pub course_id: String,
    /// Search term
    pub query: String,
}

#[derive(Clone)]
pub struct EducationServer {
    pub client: Client,
    pub base_url: String,
    pub token: String,
}

impl EducationServer {
    async fn get(&self, path: &str) -> Result<Value, String> {
        self.client.get(format!("{}/api/v1/{}", self.base_url, path))
            .header("Authorization", format!("Bearer {}", self.token))
            .send().await.map_err(|e| e.to_string())?
            .json::<Value>().await.map_err(|e| e.to_string())
    }

    async fn post(&self, path: &str, body: Value) -> Result<Value, String> {
        self.client.post(format!("{}/api/v1/{}", self.base_url, path))
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&body)
            .send().await.map_err(|e| e.to_string())?
            .json::<Value>().await.map_err(|e| e.to_string())
    }

    async fn put(&self, path: &str, body: Value) -> Result<Value, String> {
        self.client.put(format!("{}/api/v1/{}", self.base_url, path))
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&body)
            .send().await.map_err(|e| e.to_string())?
            .json::<Value>().await.map_err(|e| e.to_string())
    }
}

#[tool_router(server_handler)]
impl EducationServer {
    #[tool(description = "List all courses for the authenticated user")]
    async fn list_courses(&self, Parameters(_input): Parameters<EmptyInput>) -> String {
        match self.get("courses?per_page=50&include[]=total_students").await {
            Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Get course details including syllabus and enrollment count")]
    async fn get_course(&self, Parameters(input): Parameters<CourseIdInput>) -> String {
        match self.get(&format!("courses/{}?include[]=syllabus_body&include[]=total_students", input.course_id)).await {
            Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "List all assignments for a course")]
    async fn list_assignments(&self, Parameters(input): Parameters<CourseIdInput>) -> String {
        match self.get(&format!("courses/{}/assignments?per_page=50", input.course_id)).await {
            Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Get assignment details including rubric and submissions summary")]
    async fn get_assignment(&self, Parameters(input): Parameters<AssignmentInput>) -> String {
        match self.get(&format!("courses/{}/assignments/{}?include[]=submission_summary", input.course_id, input.assignment_id)).await {
            Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Create a new assignment in a course")]
    async fn create_assignment(&self, Parameters(input): Parameters<CreateAssignmentInput>) -> String {
        let mut assignment = serde_json::json!({
            "assignment": {
                "name": input.name,
                "submission_types": input.submission_types.unwrap_or_else(|| vec!["online_text_entry".into()]),
                "published": true
            }
        });
        if let Some(desc) = input.description { assignment["assignment"]["description"] = Value::String(desc); }
        if let Some(due) = input.due_at { assignment["assignment"]["due_at"] = Value::String(due); }
        if let Some(pts) = input.points_possible { assignment["assignment"]["points_possible"] = serde_json::json!(pts); }
        match self.post(&format!("courses/{}/assignments", input.course_id), assignment).await {
            Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "List students enrolled in a course")]
    async fn list_students(&self, Parameters(input): Parameters<CourseIdInput>) -> String {
        match self.get(&format!("courses/{}/users?enrollment_type[]=student&per_page=50", input.course_id)).await {
            Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Get submissions for an assignment (all students)")]
    async fn get_submissions(&self, Parameters(input): Parameters<AssignmentInput>) -> String {
        match self.get(&format!("courses/{}/assignments/{}/submissions?per_page=50&include[]=user", input.course_id, input.assignment_id)).await {
            Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Grade a student submission")]
    async fn grade_submission(&self, Parameters(input): Parameters<GradeInput>) -> String {
        let mut body = serde_json::json!({
            "submission": { "posted_grade": input.grade }
        });
        if let Some(comment) = input.comment {
            body["comment"] = serde_json::json!({"text_comment": comment});
        }
        match self.put(&format!("courses/{}/assignments/{}/submissions/{}", input.course_id, input.assignment_id, input.student_id), body).await {
            Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Post an announcement to a course")]
    async fn post_announcement(&self, Parameters(input): Parameters<AnnouncementInput>) -> String {
        let body = serde_json::json!({
            "title": input.title,
            "message": input.message,
            "is_announcement": true
        });
        match self.post(&format!("courses/{}/discussion_topics", input.course_id), body).await {
            Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Get course grades/analytics summary")]
    async fn get_course_analytics(&self, Parameters(input): Parameters<CourseIdInput>) -> String {
        match self.get(&format!("courses/{}/analytics/student_summaries?per_page=50", input.course_id)).await {
            Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "List modules (content structure) for a course")]
    async fn list_modules(&self, Parameters(input): Parameters<CourseIdInput>) -> String {
        match self.get(&format!("courses/{}/modules?include[]=items&per_page=50", input.course_id)).await {
            Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Search course content (pages, assignments, discussions)")]
    async fn search_course_content(&self, Parameters(input): Parameters<SearchInput>) -> String {
        match self.get(&format!("courses/{}/content_migrations?search_term={}", input.course_id, input.query.replace(' ', "+"))).await {
            Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }
}
