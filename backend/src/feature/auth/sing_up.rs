use axum::extract::State;
use crate::{infra::{axum::{AppJsonRequest, AppJsonResponse}, errors::AppError}, state::AppState};
use super::{domain::{Customer, CustomerDtoRequest, CustomerDtoResponse}, validators::Validator};

pub struct SingUpUseCase;
impl SingUpUseCase {
    pub async fn sing_up(
        State(app_state): State<AppState>,
        AppJsonRequest(request): AppJsonRequest<CustomerDtoRequest>,
    ) -> Result<AppJsonResponse<CustomerDtoResponse>, AppError>{
        Validator::email_and_password_not_empty(&request)?;
        let mut transaction = app_state.begin_transaction().await?;
        let mut customer = Customer::new(request.email, request.password)?;
        customer = Customer::insert(&mut transaction, customer).await?;
        app_state.commit_transaction(transaction).await?;
        Ok(
            AppJsonResponse::new(CustomerDtoResponse {
                id: customer.id,
                email: customer.email,
            })
        )
    }
}
