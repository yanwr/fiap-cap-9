use axum::{routing::{post, put, get}, Router};

use crate::{app::AppRoutes, feature::{auth::{sing_in::SingInUseCase, sing_up::SingUpUseCase}, biometrics::{create::CreateUseCase, get_by::GetByUseCase, update::UpdateUseCase}}, state::AppState};

impl AppRoutes {
    pub fn auth_routes() -> Router<AppState> {
        Router::new()
            .route("/singin", post(SingInUseCase::sing_in))
            .route("/singup", post(SingUpUseCase::sing_up))
    }

    pub fn biometrics_routes() -> Router<AppState> {
        Router::new()
            .route("/actions/create", post(CreateUseCase::create))
            .route("/actions/get/:customer_id", get(GetByUseCase::get_by))
            .route("/actions/update", put(UpdateUseCase::update))
    }
}
