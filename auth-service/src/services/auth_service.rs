use std::borrow::Cow;

use axum::{
    body::Body, extract::Json, http::{Response, StatusCode}, response::IntoResponse, Extension
};

use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use sea_orm::{DatabaseConnection, DbErr, TransactionTrait};
use validator::ValidationErrors;

use crate::{core::{dtos::auth::{registration_dto::RegistrationDto, sign_in_dto::SignInDto}, enums::role_enum::RoleEnum}, repositories::{access_token_repository::{create_access_token, delete_access_token}, attempt_login_repository::create_attempt_login, failed_login_repository::create_failed_login, role_permission_repository::set_role_to_user, user_repository::{create_user, find_user_by_email, find_user_by_email_with_roles}}, utils::{jwt_util::{generate_token, IAuth}, response_util::GlobalResponse, util::{hash_password, verify_password}}, JwtKeys};

use super::redis_service;

pub async fn save_login_attempt(
    Extension(db): Extension<DatabaseConnection>,
    Json(sign_in_dto): Json<SignInDto>,
) {
    if let Err(e) = create_attempt_login(&db, Json(sign_in_dto)).await {
        eprintln!("Failed to save login attempt: {:?}", e);
    }
}

pub async fn registration(
    Extension(db): Extension<DatabaseConnection>, 
    Json(registration_dto): Json<RegistrationDto>
) -> Response<Body> {
    let is_email_exist = find_user_by_email(&db, &registration_dto.email).await;

    if is_email_exist.is_some() {
        return (
            GlobalResponse::<()>::error(StatusCode::BAD_REQUEST, "Validation failed")
            .with_validation_errors({
                let mut params: std::collections::HashMap<Cow<'_, str>, serde_json::Value> = std::collections::HashMap::new();
                let mut errors: ValidationErrors = ValidationErrors::new();
                let mut validation_error = validator::ValidationError::new("email");

                validation_error.message = Some(Cow::Owned("email already exists".to_string()));
                params.insert(Cow::Borrowed("email"), serde_json::Value::String(registration_dto.email.clone()));
                validation_error.params = params;
                errors.add("email", validation_error);
                errors
            })
        ).into_response();
    }

    let result = db.transaction::<_, (), DbErr>(|txn| {
        Box::pin(async move {
            let new_user = create_user(
                txn,
                registration_dto.name,
                registration_dto.email,
                hash_password(registration_dto.password).unwrap(),
            ).await;
            
            if let Ok(user) = &new_user {
                let user_clone = user.clone();

                set_role_to_user(txn, user_clone, RoleEnum::User.to_string().as_str()).await?;
            }

            Ok(())
        })
    }).await;

    match result {
        Ok(_) => (
            GlobalResponse::<serde_json::Value>::success(serde_json::json!({
                "message": "registration success"
            }))
        )
            .into_response(),
        Err(err) => (
            GlobalResponse::<String>::error(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        )
            .into_response(),
    }
}

pub async fn sign_in(
    Extension(db): Extension<DatabaseConnection>, 
    Extension(redis): Extension<Pool<RedisConnectionManager>>, 
    Json(jwt_keys): Json<JwtKeys>,
    Json(sign_in_dto): Json<SignInDto>,
) -> Response<Body> {
    let user = match find_user_by_email_with_roles(&db, &sign_in_dto.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            tokio::spawn({
                let db = db.clone();
                let sign_in_dto = sign_in_dto.clone();
                async move {
                    if let Err(e) = create_failed_login(&db, &sign_in_dto.email, Some("email not found")).await {
                        eprintln!("Failed to save login attempt: {:?}", e);
                    }
                }
            });

            return GlobalResponse::<()>::error(StatusCode::BAD_REQUEST, "Validation failed")
                .with_validation_errors({
                    let mut params = std::collections::HashMap::new();
                    let mut errors = ValidationErrors::new();
                    let mut validation_error = validator::ValidationError::new("email");

                    validation_error.message = Some(Cow::Owned("email not found".to_string()));
                    params.insert(Cow::Borrowed("email"), serde_json::Value::String(sign_in_dto.email.clone()));
                    validation_error.params = params;
                    errors.add("email", validation_error);
                    errors
                }).into_response();
        },
        Err(err) => {
            return GlobalResponse::<()>::error(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            .into_response();
        }
    };

    // let bannedLogin = 

    let is_valid_password = verify_password(&sign_in_dto.password, &user.password);

    if !is_valid_password {
        // tokio::spawn({
        //     let db = db.clone();
        //     let sign_in_dto = sign_in_dto.clone();
        //     async move {
        //         if let Err(e) = create_failed_login(&db, &sign_in_dto.email, Some("invalid password")).await {
        //             eprintln!("Failed to save login attempt: {:?}", e);
        //         }
        //     }
        // });

        // create_failed_login();

        let max_retry = redis_service::get_config(redis, "max_retry_login").await;
        println!("MAX RETRY : {:?}", max_retry);

        return GlobalResponse::<()>::error(StatusCode::BAD_REQUEST, "Validation failed")
            .with_validation_errors({
                let mut params = std::collections::HashMap::new();
                let mut errors = ValidationErrors::new();
                let mut validation_error = validator::ValidationError::new("password");

                validation_error.message = Some(Cow::Owned("invalid password".to_string()));
                params.insert(Cow::Borrowed("password"), serde_json::Value::String(sign_in_dto.password.clone()));
                validation_error.params = params;
                errors.add("password", validation_error);
                errors
            }).into_response();
    }

    let token: String = generate_token(&user, jwt_keys.private_key);
    tokio::spawn(create_access_token(db.clone(), user.clone(), token.clone()));

    return (
        GlobalResponse::<serde_json::Value>::success(serde_json::json!({
            "access_token": token
        }))
    ).into_response();
}

pub async fn sign_out(
    Extension(db): Extension<DatabaseConnection>,
    Extension(iauth): Extension<IAuth>,
) -> Response<Body> {
    tokio::spawn(delete_access_token(db, iauth.id));

    return (
        GlobalResponse::<serde_json::Value>::success(serde_json::json!({
            "message": "sign out success"
        }))
    ).into_response();
}