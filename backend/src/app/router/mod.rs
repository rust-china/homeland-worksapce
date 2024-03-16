pub fn compose() -> axum::Router<()> {
    axum::Router::new().route("/health", axum::routing::get(health))
}

pub(crate) async fn health() -> impl axum::response::IntoResponse {
    let json: serde_json::Value = serde_json::from_str(r#"{"healthy":true}"#).unwrap();
    (axum::http::StatusCode::OK, axum::Json(json))
}
