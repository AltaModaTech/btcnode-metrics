use axum::extract::State;
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;

use crate::state::AppState;

pub async fn metrics_handler(State(state): State<AppState>) -> impl IntoResponse {
    let service = state.service.clone();
    match tokio::task::spawn_blocking(move || service.scrape()).await {
        Ok(body) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")],
            body,
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("metrics collection failed: {e}"),
        )
            .into_response(),
    }
}

pub async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}
