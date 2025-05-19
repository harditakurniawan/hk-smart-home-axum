use axum::{body::Body, http::{Response, StatusCode}, response::IntoResponse, Extension, Json};
use axum_extra::extract::WithRejection;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use sea_orm::DatabaseConnection;
use validator::Validate;

use crate::{core::{dtos::auth::{registration_dto::RegistrationDto, sign_in_dto::SignInDto}}, services, utils::{jwt_util::IAuth, response_util::GlobalResponse}, JwtKeys};

pub async fn registration(
    Extension(db): Extension<DatabaseConnection>,
    WithRejection(Json(registration_dto), _): WithRejection<Json<RegistrationDto>, GlobalResponse<()>>,
) -> Response<Body> {
    if let Err(errors) = registration_dto.validate() {
        return (
            GlobalResponse::<()>::error(StatusCode::BAD_REQUEST, "Validation failed")
            .with_validation_errors(errors)
        ).into_response();
    }

    return services::auth_service::registration(Extension(db), Json(registration_dto)).await;
}

pub async fn sign_in(
    Extension(db): Extension<DatabaseConnection>, 
    Extension(redis): Extension<Pool<RedisConnectionManager>>, 
    Extension(jwt_keys): Extension<JwtKeys>, 
    WithRejection(Json(sign_in_dto), _): WithRejection<Json<SignInDto>, GlobalResponse<()>>,
) -> Response<Body> {
    // Spawn the async task using tokio
    tokio::spawn({
        let db = db.clone();
        let sign_in_dto = sign_in_dto.clone();
        async move {
            services::auth_service::save_login_attempt(
                Extension(db),
                Json(sign_in_dto),
            ).await;
        }
    });

    if let Err(errors) = sign_in_dto.validate() {
        return (
            GlobalResponse::<()>::error(StatusCode::BAD_REQUEST, "Validation failed")
            .with_validation_errors(errors)
        ).into_response();
    }

    return services::auth_service::sign_in(Extension(db), Extension(redis), Json(jwt_keys), Json(sign_in_dto)).await;
}

pub async fn sign_out(
    Extension(db): Extension<DatabaseConnection>, 
    Extension(iauth): Extension<IAuth>,
) -> Response<Body> {
    return services::auth_service::sign_out(Extension(db), Extension(iauth)).await;
}