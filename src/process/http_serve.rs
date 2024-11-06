use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", path, addr);

    let shared_state = Arc::new(HttpServeState { path: path.clone() });

    let app = Router::new()
        .nest_service("/tower", ServeDir::new(path))
        .route("/*path", get(file_handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Read file {:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            Html(format!("File {} does not exist", p.display())),
        )
    } else {
        // TODO: test p is a directory
        // if it is a directory, list all files/subdirectories
        // as <li><a href="/path/to/file">file name</a></li>
        // <html><body><ul>.....</ul></body></html>
        if p.is_dir() {
            match tokio::fs::read_dir(p).await {
                Ok(mut entries) => {
                    let mut content = "<ul>".to_string();
                    while let Ok(Some(entry)) = entries.next_entry().await {
                        let path = entry.path();
                        let name = path.file_name().unwrap();
                        let name = name.to_str().unwrap();
                        let href = format!("/{}", path.to_str().unwrap());

                        content.push_str(&format!("<li><a href=\"{href}\">{name}</a></li>"));
                    }

                    content.push_str("</ul>");
                    (StatusCode::OK, Html(content))
                }
                Err(e) => {
                    warn!("Error reading directory: {:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Html(e.to_string()))
                }
            }
        } else {
            match tokio::fs::read_to_string(p).await {
                Ok(content) => (StatusCode::OK, Html(content)),
                Err(e) => {
                    warn!("Error reading file: {:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Html(e.to_string()))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::StreamExt;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState { path: ".".into() });
        let response = file_handler(State(state), Path("Cargo.toml".into()))
            .await
            .into_response();
        assert_eq!(response.status(), StatusCode::OK);
        let mut body_stream = response.into_body().into_data_stream();
        let mut content = String::new();
        while let Some(Ok(chunk)) = body_stream.next().await {
            content.push_str(core::str::from_utf8(&chunk).unwrap());
        }
        assert!(content.trim().starts_with("[package]"));
    }
}
