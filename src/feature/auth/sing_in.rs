use axum::{extract::State, http::HeaderValue, response::{IntoResponse, Response}};
use hyper::{HeaderMap, StatusCode};

use crate::{infra::{axum::{AppJsonRequest, AppJsonResponse}, env::Environment, errors::{AppError, AppErrorData}}, state::AppState, support::{hash::hash_bcrypt::HashBCrypt, jwt::{AuthorizationClaims, Jwt}}};

use super::{domain::{Customer, CustomerDtoRequest, CustomerDtoResponse}, validators::Validator};


pub struct SingInUseCase;
impl SingInUseCase {
    pub async fn sing_in(
        State(app_state): State<AppState>,
        AppJsonRequest(request): AppJsonRequest<CustomerDtoRequest>
    ) -> Result<Response, AppError> {
        Validator::email_and_password_not_empty(&request)?;
        let mut transaction = app_state.begin_transaction().await?;
        let customer = Customer::get_by(&mut transaction, request.email).await?;
        app_state.commit_transaction(transaction).await?;
        if HashBCrypt::verify(customer.password, request.password)? {
            let claims = AuthorizationClaims::new(customer.id, customer.email.clone(), 30);
            let jwt = AuthorizationClaims::generate_jwt(&claims, Environment::jwt_private_key())?;
            let mut header = HeaderMap::new();
            header.insert("Authorization", HeaderValue::from_str(&jwt).expect("Failed to insert header"));
            Ok((
                header,
                AppJsonResponse::new(CustomerDtoResponse {
                    id: customer.id,
                    email: customer.email,
                })
            ).into_response())
        } else {
            Err(AppErrorData::new(
                StatusCode::UNAUTHORIZED,
                String::from("No access"),
                None,
            )
            .to_business_error())
        }
    }
}
