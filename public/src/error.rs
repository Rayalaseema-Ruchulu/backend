use axum::response::{IntoResponse, Response};
use http::StatusCode;

pub(crate) type ApiResult<T> = Result<T, ApiError>;

pub enum ApiError {
    WorkerError(worker::Error),
    NotFound,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::WorkerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound => StatusCode::NOT_FOUND,
        }
        .into_response()
    }
}

impl From<worker::Error> for ApiError {
    fn from(value: worker::Error) -> Self {
        Self::WorkerError(value)
    }
}
