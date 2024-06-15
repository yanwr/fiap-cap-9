use std::string::FromUtf8Error;

use crate::infra::observability::Tags;
use axum::extract::rejection::{JsonDataError, JsonRejection};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use bcrypt::BcryptError;
use data_encoding::DecodeError;
use hex::FromHexError;
use hyper::header::InvalidHeaderValue;
use rsa::pkcs8::spki;
use serde_json::json;
use tracing::{error, info};

/// Our app's top level error type.
#[derive(Debug)]
pub enum AppError {
    Reqwest(reqwest::Error),
    ReqwestMiddleware(reqwest_middleware::Error),
    Business(Box<AppErrorData>),
    AxumJsonRejection(JsonRejection),
    AxumJsonDataError(JsonDataError),
}

pub trait ToBusinessError {
    fn to_business_error(&self, message: &str, tags: Option<Tags>) -> AppError;
}

#[derive(Debug)]
pub struct AppErrorData {
    pub status: StatusCode,
    pub message: String,
    tags: Option<Tags>,
}

impl AppErrorData {
    pub fn new(status: StatusCode, message: String, tags: Option<Tags>) -> Self {
        AppErrorData {
            status,
            message,
            tags,
        }
    }

    pub fn to_business_error(self) -> AppError {
        AppError::Business(Box::new(self))
    }

    fn get_tags(&self) -> Tags {
        if let Some(t) = self.tags.clone() {
            t
        } else {
            Tags::default()
        }
    }
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let app_error_data = to_app_error_data(self);

        let body = Json(json!({
            "status": app_error_data.status.as_u16(),
            "message": app_error_data.message,
        }));

        let tags = app_error_data.get_tags();
        let status_code = app_error_data.status;

        if status_code.is_server_error() {
            error!(
                user_id = tags.user_id,
                "{}",
                app_error_data.message.clone()
            );
        } else {
            info!(
                user_id = tags.user_id,
                "{}",
                app_error_data.message.clone()
            );
        }

        let response = (app_error_data.status, body).into_response();

        let (mut parts, body) = response.into_parts();
        parts.extensions.insert(app_error_data.get_tags());

        Response::from_parts(parts, body)
    }
}

fn to_app_error_data(app_error: AppError) -> Box<AppErrorData> {
    match app_error {
        AppError::Business(data) => data,
        AppError::Reqwest(err) => Box::new(AppErrorData::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Reqwest client error :: {}", err),
            None,
        )),
        AppError::ReqwestMiddleware(err) => Box::new(AppErrorData::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Reqwest client error :: {}", err),
            None,
        )),
        AppError::AxumJsonRejection(rejection) => Box::new(AppErrorData::new(
            rejection.status(),
            rejection.body_text(),
            None,
        )),
        AppError::AxumJsonDataError(rejection) => Box::new(AppErrorData::new(
            rejection.status(),
            rejection.body_text(),
            None,
        )),
    }
}

impl From<reqwest::Error> for AppError {
    fn from(inner: reqwest::Error) -> Self {
        AppError::Reqwest(inner)
    }
}

impl From<reqwest_middleware::Error> for AppError {
    fn from(inner: reqwest_middleware::Error) -> Self {
        AppError::ReqwestMiddleware(inner)
    }
}

impl From<JsonRejection> for AppError {
    fn from(inner: JsonRejection) -> Self {
        AppError::AxumJsonRejection(inner)
    }
}

impl From<JsonDataError> for AppError {
    fn from(inner: JsonDataError) -> Self {
        AppError::AxumJsonDataError(inner)
    }
}

impl ToBusinessError for std::fmt::Error {
    fn to_business_error(&self, message: &str, tags: Option<Tags>) -> AppError {
        let message = format!("General Error: {} :: {}", message, self);
        AppErrorData::new(StatusCode::INTERNAL_SERVER_ERROR, message, tags).to_business_error()
    }
}

impl ToBusinessError for serde_json::Error {
    fn to_business_error(&self, message: &str, tags: Option<Tags>) -> AppError {
        let message = format!("Serde json error: {} :: {}", message, self);
        AppErrorData::new(StatusCode::INTERNAL_SERVER_ERROR, message, tags).to_business_error()
    }
}

impl ToBusinessError for reqwest_middleware::Error {
    fn to_business_error(&self, message: &str, tags: Option<Tags>) -> AppError {
        let message = format!("Error when call external service: {} :: {}", message, self);
        AppErrorData::new(StatusCode::INTERNAL_SERVER_ERROR, message, tags).to_business_error()
    }
}

impl ToBusinessError for InvalidHeaderValue {
    fn to_business_error(&self, message: &str, tags: Option<Tags>) -> AppError {
        let message = format!("Error when call external service: {} :: {}", message, self);
        AppErrorData::new(StatusCode::INTERNAL_SERVER_ERROR, message, tags).to_business_error()
    }
}

impl ToBusinessError for sqlx::Error {
    fn to_business_error(&self, message: &str, tags: Option<Tags>) -> AppError {
        let message = format!("SQL Error: {} :: {}", message, self);
        AppErrorData::new(StatusCode::INTERNAL_SERVER_ERROR, message, tags).to_business_error()
    }
}

impl ToBusinessError for rsa::Error {
    fn to_business_error(&self, message: &str, tags: Option<Tags>) -> AppError {
        let message = format!("RSA Error: {} :: {}", message, self);
        AppErrorData::new(StatusCode::INTERNAL_SERVER_ERROR, message, tags).to_business_error()
    }
}

impl ToBusinessError for aes_gcm::Error {
    fn to_business_error(&self, message: &str, tags: Option<Tags>) -> AppError {
        let message = format!("AES Error: {} :: {}", message, self);
        AppErrorData::new(StatusCode::INTERNAL_SERVER_ERROR, message, tags).to_business_error()
    }
}

impl ToBusinessError for FromUtf8Error {
    fn to_business_error(&self, message: &str, tags: Option<Tags>) -> AppError {
        let message = format!("UTF8 Error: {} :: {}", message, self);
        AppErrorData::new(StatusCode::INTERNAL_SERVER_ERROR, message, tags).to_business_error()
    }
}

impl ToBusinessError for rsa::pkcs1::Error {
    fn to_business_error(&self, message: &str, tags: Option<Tags>) -> AppError {
        let message = format!("RSA PKCS1 Error: {} :: {}", message, self);
        AppErrorData::new(StatusCode::INTERNAL_SERVER_ERROR, message, tags).to_business_error()
    }
}

impl ToBusinessError for FromHexError {
    fn to_business_error(&self, message: &str, tags: Option<Tags>) -> AppError {
        let message = format!("Hex Error: {} :: {}", message, self);
        AppErrorData::new(StatusCode::INTERNAL_SERVER_ERROR, message, tags).to_business_error()
    }
}

impl ToBusinessError for spki::Error {
    fn to_business_error(&self, message: &str, tags: Option<Tags>) -> AppError {
        let message = format!("SPKI Error: {} :: {}", message, self);
        AppErrorData::new(StatusCode::INTERNAL_SERVER_ERROR, message, tags).to_business_error()
    }
}

impl ToBusinessError for DecodeError {
    fn to_business_error(&self, message: &str, tags: Option<Tags>) -> AppError {
        let message = format!("DecodeError error: {} :: {}", message, self);
        AppErrorData::new(StatusCode::INTERNAL_SERVER_ERROR, message, tags).to_business_error()
    }
}

impl ToBusinessError for BcryptError {
    fn to_business_error(&self, message: &str, tags: Option<Tags>) -> AppError {
        let message = format!("BcryptError error: {} :: {}", message, self);
        AppErrorData::new(StatusCode::INTERNAL_SERVER_ERROR, message, tags).to_business_error()
    }
}

impl ToBusinessError for jsonwebtoken::errors::Error {
    fn to_business_error(&self, message: &str, tags: Option<Tags>) -> AppError {
        let message = format!("JWT Error: {} :: {}", message, self);
        AppErrorData::new(StatusCode::INTERNAL_SERVER_ERROR, message, tags).to_business_error()
    }
}
