use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on addr {:?}", path, addr);

    let state = HttpServeState { path };

    //axum router
    let router = Router::new()
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);

    if !p.exists() {
        warn!("File not found: {:?}", p.display());
        (
            StatusCode::NOT_FOUND,
            format!("File not found: {:?}", p.display()),
        )
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Failed to read file: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to read file: {:?}", e),
                )
            }
        }
    }
}
