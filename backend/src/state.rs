use sqlx::{Pool, Postgres};

use crate::infra::database::DatabaseConfig;
#[derive(Clone)]
pub struct AppState {
    pub postgres_pool: Pool<Postgres>,
}

impl AppState {
    pub async fn create() -> Result<AppState, Box<dyn std::error::Error>> {
        let database_config = DatabaseConfig::from_env();
        let postgres_pool = database_config.create_db_pool().await?;
        let app_state = AppState {
            postgres_pool,
        };
        Ok(app_state)
    }
}
