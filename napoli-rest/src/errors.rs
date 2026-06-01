use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

pub struct ApiError {
    status: StatusCode,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(ErrorBody {
                error: self.message,
            }),
        )
            .into_response()
    }
}

pub fn bad_request(msg: impl Into<String>) -> ApiError {
    ApiError {
        status: StatusCode::BAD_REQUEST,
        message: msg.into(),
    }
}

pub fn not_found(msg: impl Into<String>) -> ApiError {
    ApiError {
        status: StatusCode::NOT_FOUND,
        message: msg.into(),
    }
}

pub fn internal(msg: impl Into<String>) -> ApiError {
    ApiError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: msg.into(),
    }
}
