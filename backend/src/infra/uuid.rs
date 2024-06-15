use crate::infra::errors::{AppError, AppErrorData};
use axum::http::StatusCode;
use uuid::Uuid;

pub trait CustomUuidFns {
    fn to_uuid(&self) -> Result<Uuid, AppError>;
}

impl CustomUuidFns for String {
    fn to_uuid(&self) -> Result<Uuid, AppError> {
        Uuid::parse_str(self).map_err(|_| {
            let message = format!("Invalid UUID {}", self);
            AppErrorData::new(StatusCode::BAD_REQUEST, message, None).to_business_error()
        })
    }
}
