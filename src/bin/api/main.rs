use std::env;
use std::net::SocketAddr;
use login_auth_service::app::AppRoutes;
use login_auth_service::state::AppState;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_state = AppState::create().await?;
    let app = AppRoutes::routes(app_state).await?;
    info!("starting...");
    let port = env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(9095);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    info!("Listening on {}", addr);
    Ok(())
}
