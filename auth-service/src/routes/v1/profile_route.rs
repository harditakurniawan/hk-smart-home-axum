use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use std::sync::LazyLock;

static ROUTER: LazyLock<Router> = LazyLock::new(|| {
    Router::new()
        .route("/", get(hello))
        // .route("/login", get(hello))
        // .route("/register", get(hello))
        // .route("/refresh-token", get(hello))
        // .route("/logout", get(hello))
});
pub static PROFILE: LazyLock<Router> = LazyLock::new(|| Router::new().nest("/profiles", ROUTER.clone()));

async fn hello() -> impl IntoResponse {
    (StatusCode::OK, "hello").into_response()
}