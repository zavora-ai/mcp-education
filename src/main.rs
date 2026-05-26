mod server;

use rmcp::{ServiceExt, transport::stdio};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let manifest = adk_mcp_sdk::ServerManifest::from_file(std::path::Path::new("mcp-server.toml"))?;
    let errors = manifest.validate();
    if !errors.is_empty() {
        for e in &errors { eprintln!("  - {e}"); }
    }

    let base_url = std::env::var("CANVAS_BASE_URL").unwrap_or_else(|_| "https://canvas.instructure.com".into());
    let token = std::env::var("CANVAS_TOKEN").unwrap_or_default();

    if token.is_empty() {
        eprintln!("Warning: CANVAS_TOKEN not set. Set it to use Canvas LMS tools.");
    }

    let server = server::EducationServer {
        client: reqwest::Client::new(),
        base_url,
        token,
    };
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
