use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tracing::info;

use tower_http::services::ServeDir;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    // 拼接路径
    let p = std::path::Path::new(&state.path).join(path);

    if !p.exists() {
        let response_body = format!("File not found: {:?}", p.display());
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("Content-Type", "text/plain")
            .body::<String>(response_body)
            .unwrap()
    } else if p.is_dir() {
        let mut response_body = String::from("<html><body><ul>");

        for entry in std::fs::read_dir(p).unwrap() {
            let entry = entry.unwrap();
            // let entry_path = entry.path();
            let entry_name = entry.file_name().into_string().unwrap();

            let link = format!(
                "<li><a href=\"{}\">{}</a></li>",
                // entry_path.display(),
                entry_name,
                entry_name
            );
            response_body.push_str(&link);
        }

        response_body.push_str("</ul></body></html>");

        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/html")
            .body(response_body)
            .unwrap()
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/plain")
                .body(content)
                .unwrap(),
            Err(e) => {
                let response_body = format!("Failed to read file: {:?}", e);
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header("Content-Type", "text/plain")
                    .body(response_body)
                    .unwrap()
            }
        }
    }
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on addr {:?}", path, addr);

    let state = HttpServeState { path: path.clone() };

    let dir_service = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_gzip()
        .precompressed_zstd();

    //axum router
    let router = Router::new()
        .route("/*requestPath", get(file_handler))
        .nest_service("/tower", dir_service)
        .with_state(Arc::new(state));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = HttpServeState {
            path: PathBuf::from("."),
        };
        let state = Arc::new(state);
        let resp = file_handler(State(state), Path("Cargo.toml".to_string())).await;

        let response = resp.into_response();

        assert_eq!(response.status(), 200);

        // TODO: 读取body
        // let body = response.into_body();
        // let (body_bytes, _) = resp_body.into_body().into_bytes().await.unwrap();
        // let body = String::from_utf8(body_bytes.to_vec()).unwrap();
        // let body = body.to_vec().await.unwrap();
        // let body = String::from_utf8(body).unwrap();
        // assert!(body.contains("Cargo.toml"));
    }
}
