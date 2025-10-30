use axum::Json;

pub async fn test_poll_route() -> Json<String> {
    Json("Polls endpoint work!".to_string())
}
