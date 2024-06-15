use hyper::StatusCode;
use uuid::Uuid;

use crate::infra::errors::{AppError, AppErrorData};

use super::domain::BiometricDtoRequest;

pub struct Validator;
impl Validator {
    pub fn customer_id_and_image_not_empty(request: &BiometricDtoRequest) -> Result<(), AppError> {
        if request.customer_id.to_string().is_empty() || request.image_path.is_empty() {
            return Err(AppErrorData::new(
                StatusCode::BAD_REQUEST,
                "customer_id and/or image_path is expected".to_string(),
                None,
            )
            .to_business_error());
        }
        Ok(())
    }
    pub fn customer_id_not_empty(customer_id: &Uuid) -> Result<(), AppError> {
        if customer_id.is_nil() || customer_id.to_string().is_empty() {
            return Err(AppErrorData::new(
                StatusCode::BAD_REQUEST,
                "customer_id is expected".to_string(),
                None,
            )
            .to_business_error());
        }
        Ok(())
    }
}   
