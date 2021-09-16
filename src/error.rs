use std::io;

use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GhError {
    #[error("Page not found!")]
    PageNotFound(),
    #[error("Configuration error")]
    ConfigurationError(#[from] io::Error),
    #[error("Template error")]
    TemplateError(#[from] tinytemplate::error::Error),
}

impl IntoResponse for GhError {
    type Body = Body;
    type BodyError = <Self::Body as axum::body::HttpBody>::Error;

    fn into_response(self) -> Response<Self::Body> {
        let (status, body) = match self {
            GhError::PageNotFound() => (StatusCode::NOT_FOUND, Body::from("Page not found!")),
            GhError::ConfigurationError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Body::from("Configuration error!"),
            ),
            GhError::TemplateError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Body::from("Template error!"),
            ),
        };

        (status, body).into_response()
    }
}
