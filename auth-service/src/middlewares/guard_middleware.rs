use axum::{extract::Request, http::{self, StatusCode}, middleware::Next, response::{IntoResponse, Response}};
use sea_orm::DatabaseConnection;

use crate::{repositories::access_token_repository::find_token, utils::{jwt_util::decode_token, response_util::GlobalResponse}, JwtKeys};

fn extract_token_from_header(request: &Request) -> Option<String> {
    return request.headers().get(http::header::AUTHORIZATION)
    .and_then(|header_value| header_value.to_str().ok())
    .and_then(|header_value| {
        header_value.to_string().split(" ").nth(1)
        .map(|token| token.to_string())
    });
}

pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    let token = match extract_token_from_header(&request) {
        Some(token) => token,
        None => {
            return GlobalResponse::<()>::error(StatusCode::UNAUTHORIZED, StatusCode::UNAUTHORIZED.to_string()).into_response();
        }
    };

    let db_connection = match request.extensions_mut().get::<DatabaseConnection>() {
        Some(conn) => conn,
        None => {
            return GlobalResponse::<()>::error(StatusCode::INTERNAL_SERVER_ERROR, "Database connection not found".to_string()).into_response();
        }
    };

    let is_token_exist = find_token(db_connection, &token).await;
    
    if is_token_exist.is_none() {
        return GlobalResponse::<()>::error(StatusCode::UNAUTHORIZED, "Token doesn't exists".to_string()).into_response();
    }    

    let public_key = request.extensions_mut().get::<JwtKeys>().map(|jwt| jwt.public_key.to_string());
    let decoded_token = match (token.clone(), public_key) {
        (token, Some(public_key)) => match decode_token(&token, public_key) {
            Ok(decoded) => decoded,
            Err(e) => {
                return GlobalResponse::<()>::error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
        },
        _ => return GlobalResponse::<()>::error(StatusCode::INTERNAL_SERVER_ERROR, "Invalid token or keys".to_string()).into_response(),
    };

    request.extensions_mut().insert(decoded_token);

    let response = next.run(request).await;

    response
}