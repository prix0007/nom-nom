use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde_json::Value;

pub enum ApiResponse {
    Ok,
    JsonData(Value),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::Ok => (StatusCode::OK).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}

pub enum ApiError {
    BadRequest,
    Forbidden,
    Unauthorised,
    InternalServerError,
    NotFound(Option<Value>),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest => (StatusCode::BAD_REQUEST).into_response(),
            Self::Forbidden => (StatusCode::FORBIDDEN).into_response(),
            Self::Unauthorised => (StatusCode::UNAUTHORIZED).into_response(),
            Self::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
            Self::NotFound(None) => (StatusCode::NOT_FOUND).into_response(),
            Self::NotFound(Some(err)) => (StatusCode::NOT_FOUND, err.to_string()).into_response(),
        }
    }
}
