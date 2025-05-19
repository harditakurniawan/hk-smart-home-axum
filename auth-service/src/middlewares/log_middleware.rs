use std::time::Instant;

use axum::http::StatusCode;
use axum::{extract::Request, middleware::Next, response::Response};
use axum::body::{to_bytes, Body};

use crate::utils::log_util::Log;

pub async fn logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let start_time: Instant = Instant::now();
    let path = request.uri().path().to_string();
    let method = request.method().to_string().clone();

    let (parts, body) = request.into_parts();
    let header = format!("{:?}", parts.headers);
    let bytes: axum::body::Bytes = match to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(_) => {
            let duration: std::time::Duration = start_time.elapsed();
            let response_time: u128 = duration.as_millis();
            if let Err(err) = Log::error(method, path, header, "Failed to read response body".to_string(), response_time) {
                eprintln!("Failed to log error: {:?}", err);
            }

            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Failed to read request body"))
                .unwrap();
        }
    };

    if let Ok(body_str) = String::from_utf8(bytes.to_vec()) {
        if let Err(err) = Log::request(&method, &path, &header, Some(body_str)) {
            eprintln!("Failed to log request: {:?}", err);
        }
    }

    let request: axum::http::Request<Body> = Request::from_parts(parts, Body::from(bytes.clone()));
    let response: axum::http::Response<Body> = next.run(request).await;

    let duration: std::time::Duration = start_time.elapsed();

    let status: StatusCode = response.status();
    let (parts, body) = response.into_parts();
    let bytes: axum::body::Bytes = match to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(_) => {
            let response_time: u128 = duration.as_millis();
            if let Err(err) = Log::error(method, path, header, "Failed to read response body".to_string(), response_time) {
                eprintln!("Failed to log error: {:?}", err);
            }

            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to read response body"))
                .unwrap();
        }
    };

    if let Ok(body_str) = String::from_utf8(bytes.to_vec()) {
        let response_time: u128 = duration.as_millis();
        if let Err(err) = Log::response(method, path, header, status, Some(body_str), response_time) {
            eprintln!("Failed to log response: {:?}", err);
        }
    } else {
        let response_time: u128 = duration.as_millis();
        if let Err(err) = Log::response(method, path, header, status, None, response_time) {
            eprintln!("Failed to log response: {:?}", err);
        }
    }

    Response::from_parts(parts, Body::from(bytes))
}