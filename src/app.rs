use crate::state::AppState;
use crate::support;
use axum::http::StatusCode;
use axum::middleware::from_fn;
use axum::routing::get;
use axum::{Json, Router};
use std::collections::HashMap;
use tower_http::catch_panic::CatchPanicLayer;
use tracing::info;

pub struct AppRoutes;
impl AppRoutes {
    pub async fn routes(app_state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
        let app = Router::new()
            .nest("/auth", AppRoutes::auth_routes())
            .nest("/biometrics", AppRoutes::biometrics_routes())
            .with_state(app_state)
            .layer(CatchPanicLayer::new())
            .layer(from_fn(support::axum::log_response))
            .route("/health", get(AppRoutes::health_handler));
        Ok(app)
    }

    async fn health_handler() -> Result<Json<HashMap<&'static str, &'static str>>, (StatusCode, String)> {
        info!("GET /health");
        Ok(Json(HashMap::from([("status", "up")])))
    }
}
