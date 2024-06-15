use hyper::StatusCode;

use crate::infra::errors::{AppError, AppErrorData};

use super::domain::CustomerDtoRequest;

pub struct Validator;
impl Validator {
    pub fn email_and_password_not_empty(request: &CustomerDtoRequest) -> Result<(), AppError> {
        if request.email.is_empty() || request.password.is_empty() {
            return Err(AppErrorData::new(
                StatusCode::BAD_REQUEST,
                "email and/or password is expected".to_string(),
                None,
            )
            .to_business_error());
        }
        Ok(())
    }
}
