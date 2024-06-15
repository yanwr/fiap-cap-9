use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Pool, Postgres, Transaction};

use crate::state::AppState;

use super::env::Environment;
use super::errors::{AppError, ToBusinessError};

impl AppState {
    pub async fn begin_transaction(&self) -> Result<Transaction<'_, Postgres>, AppError> {
        self.postgres_pool
            .begin()
            .await
            .map_err(|err| err.to_business_error("Failed to begin transaction", None))
    }

    pub async fn commit_transaction(
        &self,
        transaction: Transaction<'_, Postgres>,
    ) -> Result<(), AppError> {
        transaction
            .commit()
            .await
            .map_err(|err| err.to_business_error("Failed to commit transaction", None))?;
        Ok(())
    }
}

pub struct DatabaseConfig {
    pub host: String,
    pub name: String,
    pub user: String,
    pub pass: String,
    pub app_name: String,
    pub port: u16,
    pub min_pool_size: u32,
    pub max_pool_size: u32,
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        let host = Environment::as_string("DB_HOST", "localhost");
        let name = Environment::as_string("DB_NAME", "login");
        let user = Environment::as_string("DB_USER", "local");
        let pass = Environment::as_string("DB_PASS", "local");
        let port = Environment::as_u16("DB_PORT", 5432);
        let app_name = Environment::as_string("DB_APP_NAME", "login_auth_service");
        let min_pool_size = Environment::as_u32("DB_MIN_POOL_SIZE", 1);
        let max_pool_size = Environment::as_u32("DB_MAX_POOL_SIZE", 10);

        Self {
            host,
            name,
            user,
            pass,
            app_name,
            port,
            min_pool_size,
            max_pool_size,
        }
    }

    pub fn db_connection_options(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .database(&self.name)
            .username(&self.user)
            .password(&self.pass)
            .port(self.port)
            .application_name(&self.app_name)
    }

    pub async fn create_db_pool(self) -> Result<Pool<Postgres>, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .min_connections(self.min_pool_size)
            .max_connections(self.max_pool_size)
            .test_before_acquire(true)
            .connect_with(self.db_connection_options())
            .await?;
        Ok(pool)
    }
}
