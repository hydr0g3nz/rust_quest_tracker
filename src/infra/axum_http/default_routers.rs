use axum::{response::IntoResponse, http::StatusCode};
pub async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found").into_response()
}

//health check
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK").into_response()
}