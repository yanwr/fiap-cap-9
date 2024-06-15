use axum::extract::State;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{infra::{axum::{AppJsonRequest, AppJsonResponse}, errors::AppError}, state::AppState};

use super::{domain::{BiometricDtoRequest, BiometricDtoResponse, Biometrics, BiometricsStatus}, validators::Validator};

#[derive(Serialize, Deserialize)]
pub struct BiometricUpdateDtoRequest {
    pub customer_id: Uuid,
    pub image_path: String,
    pub status: BiometricsStatus,
}

pub struct UpdateUseCase;
impl UpdateUseCase {
    pub async fn update(
        State(app_state): State<AppState>,
        AppJsonRequest(request): AppJsonRequest<BiometricUpdateDtoRequest>
    ) -> Result<AppJsonResponse<BiometricDtoResponse>, AppError> {
        Validator::customer_id_and_image_not_empty(&BiometricDtoRequest { customer_id: request.customer_id, image_path: request.image_path.clone() })?;
        let mut transaction = app_state.begin_transaction().await?;
        let mut biometric = Biometrics::get_by(&mut transaction, request.customer_id).await?;
        biometric.image_path = request.image_path;
        biometric.status = request.status;
        biometric = Biometrics::update(&mut transaction, biometric).await?;
        app_state.commit_transaction(transaction).await?;
        Ok(AppJsonResponse::new(BiometricDtoResponse::from(biometric)))
    }
}
