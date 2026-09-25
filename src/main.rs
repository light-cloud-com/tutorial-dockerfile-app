use axum::{Json, Router, http::StatusCode, routing::{get, post}};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::signal::unix::{SignalKind, signal};

#[derive(Deserialize)]
struct CountRequest {
    text: String,
}

#[derive(Serialize)]
struct CountResponse {
    characters: usize,
    words: usize,
    lines: usize,
}

// One JSON object per line; the Logs tab reads "severity" and "message".
fn log(severity: &str, message: &str) {
    println!("{}", json!({ "severity": severity, "message": message }));
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

async fn count(Json(body): Json<CountRequest>) -> Result<Json<CountResponse>, (StatusCode, String)> {
    if body.text.len() > 100_000 {
        return Err((StatusCode::PAYLOAD_TOO_LARGE, "text is longer than 100000 bytes".into()));
    }
    Ok(Json(CountResponse {
        characters: body.text.chars().count(),
        words: body.text.split_whitespace().count(),
        lines: body.text.lines().count(),
    }))
}

#[tokio::main]
async fn main() {
    // Light Cloud passes the port in PORT; 8080 matches EXPOSE in the Dockerfile.
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
    let app = Router::new()
        .route("/health", get(health))
        .route("/count", post(count));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    log("INFO", &format!("listening on port {port}"));

    // Finish running requests when Light Cloud stops the instance with SIGTERM.
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            signal(SignalKind::terminate()).unwrap().recv().await;
            log("INFO", "SIGTERM received, shutting down");
        })
        .await
        .unwrap();
}
