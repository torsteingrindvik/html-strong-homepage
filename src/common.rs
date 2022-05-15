use axum::response::Html;
use html_strong::document_tree::Node;
use reqwest::StatusCode;

pub async fn internal_server_error(error: std::io::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled internal error: {error}"),
    )
}

pub async fn render(contents: Node) -> Result<Html<String>, (StatusCode, String)> {
    let response = contents.render_string().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Render error: {e}"),
        )
    })?;

    Ok(Html(response))
}
