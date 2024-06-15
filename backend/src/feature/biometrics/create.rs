use axum::extract::State;

use crate::{infra::{axum::{AppJsonRequest, AppJsonResponse}, errors::AppError}, state::AppState};

use super::{domain::{BiometricDtoRequest, BiometricDtoResponse, Biometrics}, validators::Validator};

pub struct CreateUseCase;
impl CreateUseCase {
    pub async fn create(
        State(app_state): State<AppState>,
        AppJsonRequest(request): AppJsonRequest<BiometricDtoRequest>
    ) -> Result<AppJsonResponse<BiometricDtoResponse>, AppError> {
        Validator::customer_id_and_image_not_empty(&request)?;
        let mut transaction = app_state.begin_transaction().await?;
        let mut biometric = Biometrics::new(request.customer_id, request.image_path);
        biometric = Biometrics::insert(&mut transaction, biometric).await?;
        app_state.commit_transaction(transaction).await?;
        Ok(AppJsonResponse::new(BiometricDtoResponse::from(biometric)))
    }
}
