// use std::borrow::Cow;

// use axum::{http::StatusCode, response::IntoResponse};
// use validator::ValidationErrors;

// use crate::utils::response_util::GlobalResponse;

// pub async fn handle_error(err: Box<dyn std::error::Error>) -> impl IntoResponse {
//     return (
//         GlobalResponse::<()>::error(StatusCode::INTERNAL_SERVER_ERROR, StatusCode::INTERNAL_SERVER_ERROR.to_string())
//             .with_validation_errors({
//                 let mut errors = ValidationErrors::new();
//                 let mut validation_error = validator::ValidationError::new(StatusCode::INTERNAL_SERVER_ERROR.as_str());
//                 validation_error.message = Some(Cow::Owned(err.to_string()));
//                 errors.add(StatusCode::INTERNAL_SERVER_ERROR.as_str(), validation_error);
//                 errors
//             })
//     ).into_response();
// }