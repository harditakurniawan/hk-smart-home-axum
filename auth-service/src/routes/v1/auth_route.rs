use axum::{middleware::from_fn, routing::post, Router};
use std::sync::LazyLock;

use crate::{controllers::auth_controller::{registration, sign_in, sign_out}, middlewares::guard_middleware::auth_middleware};

static ROUTER: LazyLock<Router> = LazyLock::new(|| {
    Router::new()
        .route("/registration", post(registration))
        .route("/sign-in", post(sign_in))
        .route("/sign-out", post(sign_out).layer(from_fn(auth_middleware)))
});
pub static AUTH: LazyLock<Router> = LazyLock::new(|| Router::new().nest("/auth", ROUTER.clone()));