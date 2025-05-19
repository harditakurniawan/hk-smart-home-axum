use serde::Serialize;
use validator::ValidationErrors;
use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

#[derive(Debug, Serialize)]
pub struct Meta {
    pub status_code: u16,
    pub error: Option<String>,
    pub details: Option<ValidationErrors>,
    pub last_page: Option<u64>,
    pub per_page: Option<u64>,
    pub page: Option<u64>,
    pub total: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct GlobalResponse<T: Serialize + ?Sized> { 
    pub meta: Meta,
    pub data: Option<Box<T>>,
}

impl<T: Serialize + ?Sized> GlobalResponse<T> {
    // pub fn new() -> Self {
    //     Self {
    //         meta: Meta {
    //             status_code: 200,
    //             error: None,
    //             details: None,
    //             last_page: None,
    //             per_page: None,
    //             page: None,
    //             total: None,
    //         },
    //         data: None,
    //     }
    // }

    pub fn success(data: impl Serialize + Into<Box<T>>) -> Self {
        Self {
            meta: Meta {
                status_code: StatusCode::OK.as_u16(),
                error: None,
                details: None,
                last_page: None,
                per_page: None,
                page: None,
                total: None,
            },
            data: Some(data.into()),
        }
    }

    pub fn error(status_code: StatusCode, error: impl Into<String>) -> Self {
        Self {
            meta: Meta {
                status_code: status_code.as_u16(),
                error: Some(error.into()),
                details: None,
                last_page: None,
                per_page: None,
                page: None,
                total: None,
            },
            data: None,
        }
    }

    pub fn with_validation_errors(mut self, errors: ValidationErrors) -> Self {
        self.meta.details = Some(errors);
        self
    }

    pub fn with_pagination(
        mut self,
        page: u64,
        per_page: u64,
        total: u64,
        last_page: u64,
    ) -> Self {
        self.meta.page = Some(page);
        self.meta.per_page = Some(per_page);
        self.meta.total = Some(total);
        self.meta.last_page = Some(last_page);
        self
    }
}

impl From<JsonRejection> for GlobalResponse<()> {
    fn from(rejection: JsonRejection) -> Self {
        let (status_code, message) = match rejection {
            JsonRejection::JsonDataError(err) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("Invalid JSON data: {}", err),
            ),
            JsonRejection::JsonSyntaxError(err) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid JSON syntax: {}", err),
            ),
            JsonRejection::MissingJsonContentType(err) => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                format!("Missing Content-Type: {}", err),
            ),
            JsonRejection::BytesRejection(err) => (
                StatusCode::BAD_REQUEST,
                format!("Failed to read request body: {}", err),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unknown JSON processing error".to_string(),
            ),
        };

        GlobalResponse::error(status_code, message)
    }
}

impl<T: Serialize + ?Sized> IntoResponse for GlobalResponse<T> {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.meta.status_code)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        
        (status, Json(self)).into_response()
    }
}