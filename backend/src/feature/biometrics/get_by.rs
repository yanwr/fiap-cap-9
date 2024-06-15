use axum::extract::{Path, State};
use uuid::Uuid;

use crate::{infra::{axum::AppJsonResponse, errors::AppError}, state::AppState};

use super::{domain::{BiometricDtoResponse, Biometrics}, validators::Validator};

pub struct GetByUseCase;
impl GetByUseCase {
    pub async fn get_by(
        State(app_state): State<AppState>,
        Path(customer_id): Path<Uuid>,
    ) -> Result<AppJsonResponse<BiometricDtoResponse>, AppError> {
        Validator::customer_id_not_empty(&customer_id)?;
        let mut transaction = app_state.begin_transaction().await?;
        let biometric = Biometrics::get_by(&mut transaction, customer_id).await?;
        app_state.commit_transaction(transaction).await?;
        Ok(AppJsonResponse::new(BiometricDtoResponse::from(biometric)))
    }
}
