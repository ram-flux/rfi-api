//
//  Copyright 2024 Ram Flux, LLC.
//


pub mod fun;
pub mod response;

use axum::extract::rejection::{FormRejection, JsonRejection, PathRejection, QueryRejection};
use axum::Json;
use axum::{http::StatusCode, response::IntoResponse};

// use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    // The `#[from]` attribute generates `From<JsonRejection> for ApiError`
    // implementation. See `thiserror` docs for more information
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error(transparent)]
    QueryExtractorRejection(#[from] QueryRejection),
    #[error(transparent)]
    PathExtractorRejection(#[from] PathRejection),
    #[error(transparent)]
    FormExtractorRejection(#[from] FormRejection),
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
    #[error("{0}")]
    Msg(String),
}

// We implement `IntoResponse` so ApiError can be used as a response
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (msg, code) = match self {
            ApiError::JsonExtractorRejection(_x) => (_x.body_text(), StatusCode::OK),
            ApiError::QueryExtractorRejection(_x) => (_x.body_text(), StatusCode::OK),
            ApiError::PathExtractorRejection(_x) => (_x.body_text(), StatusCode::OK),
            ApiError::FormExtractorRejection(_x) => (_x.body_text(), StatusCode::OK),
            ApiError::InternalServerError(msg) => (msg, StatusCode::INTERNAL_SERVER_ERROR),
            ApiError::Msg(msg) => (msg, StatusCode::OK),
        };
        let payload = json!({
            "code":203,
            "message": msg,
            "result": ""
        });
        (code, Json(payload)).into_response()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(error: anyhow::Error) -> Self {
        ApiError::InternalServerError(error.to_string())
    }
}

//to be renovated
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Msg(String),
}

impl Error {
    pub fn get_status_code(&self) -> axum::http::StatusCode {
        match self {
            Error::Msg(_) => axum::http::StatusCode::OK,
        }
    }
}
