use std::env;
use std::net::{SocketAddr, TcpListener};
use std::path::PathBuf;
use std::str::FromStr;

use axum::body::Body;
use axum::response::Response;
use axum::{async_trait, Router};
use chrono::SecondsFormat;
use login_auth_service::feature::auth::domain::Customer;
use login_auth_service::feature::biometrics::domain::Biometrics;
use pg_embed::pg_enums::PgAuthMethod;
use pg_embed::pg_fetch::{PgFetchSettings, PG_V15};
use pg_embed::postgres::{PgEmbed, PgSettings};
use rand::Rng;
use serde_json::{json, Value};
use sqlx::migrate::MigrateError;
use test_context::AsyncTestContext;
use uuid::Uuid;
use wiremock::MockServer;

use login_auth_service::app::AppRoutes;
use login_auth_service::state::AppState;
use http_body_util::BodyExt;

pub struct TestContext {
    pub database: Option<PgEmbed>,
    pub mock_server: MockServer,
    pub app_state: AppState,
    pub app: Router,
}

pub struct DatabaseConfigTest;
impl DatabaseConfigTest {
    pub async fn embed_postgres() -> Option<PgEmbed> {
        let running_on_ci = env::var("CI")
            .ok()
            .map(|env| env.parse::<bool>().expect("Failed to parse to bool"))
            .unwrap_or(false);
        env::set_var("DB_HOST", "localhost");
        env::set_var("DB_USER", "test");
        env::set_var("DB_PASS", "test");
        if running_on_ci {
            env::set_var("DB_NAME", "test");
            env::set_var("DB_PORT", "5432");
            return None;
        }
        for _ in 1..10 {
            if let Ok(postgres) = Self::start_embed_postgres().await {
                return postgres;
            }
        }
        panic!("Failed to start postgres");
    }

    async fn start_embed_postgres() -> Result<Option<PgEmbed>, Box<dyn std::error::Error>> {
        let port = rand::thread_rng().gen_range(48000..51000);
        let settings = PgSettings {
            database_dir: PathBuf::from(format!("target/pg_embed_{}", port)),
            port,
            user: "test".to_string(),
            password: "test".to_string(),
            auth_method: PgAuthMethod::Plain,
            persistent: false,
            timeout: None,
            migration_dir: Some(PathBuf::from("./target/database/migrations")),
        };
        let fetch_settings = PgFetchSettings {
            version: PG_V15,
            ..Default::default()
        };
        let mut pg = PgEmbed::new(settings, fetch_settings).await?;
        let dn_name = format!("test_{}", port);
        pg.setup().await?;
        pg.start_db().await?;
        if pg.database_exists(&dn_name).await? {
            pg.drop_database(&dn_name).await?;
        }
        pg.create_database(&dn_name).await?;
        env::set_var("DB_NAME", &dn_name);
        env::set_var("DB_PORT", format!("{}", port));
        Ok(Some(pg))
    }
}

#[async_trait]
impl AsyncTestContext for TestContext {
    async fn setup() -> TestContext {
        let database = DatabaseConfigTest::embed_postgres().await;
        let mock_server = create_mock_server().await;
        let app_state = AppState::create()
            .await
            .expect("Failed to create app state");
        let app = AppRoutes::routes(app_state.clone())
            .await
            .expect("Failed to create app");

        run_migrations(&app_state)
            .await
            .expect("Failed to run migrations");

        TestContext {
            database,
            mock_server,
            app_state,
            app,
        }
    }

    async fn teardown(mut self) {
        if let Some(mut database) = self.database {
            database
                .stop_db()
                .await
                .expect("Failed to stop postgres database");
        };
    }
}

#[allow(dead_code)]
pub async fn create_mock_server() -> MockServer {
    for _ in 1..10 {
        let port = rand::thread_rng().gen_range(51000..54000);
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        if let Ok(listener) = TcpListener::bind(addr) {
            let mock_server = MockServer::builder().listener(listener).start().await;
            return mock_server;
        }
    }
    panic!("Failed to create mock server");
}

#[allow(dead_code)]
async fn run_migrations(app_state: &AppState) -> Result<(), MigrateError> {
    sqlx::migrate!("./database/migrations")
        .run(&app_state.postgres_pool)
        .await
}

#[allow(clippy::all)]
pub fn get_json_path(json: &Value, path: &str) -> Value {
    let divided_path = path.split('.').collect::<Vec<&str>>();
    let mut value = json.clone();
    for path_part in divided_path {
        if path_part.contains('[') {
            let start = path_part.find('[').expect("Failed to get '['");
            let end = path_part.find(']').expect("Failed to get ']'");
            let index = path_part[start + 1..end]
                .parse::<usize>()
                .expect("Failed to parse index");
            let path_array_name = &path_part[0..start];
            value = value
                .get(path_array_name)
                .unwrap_or_else(|| panic!("Expected path array name '{}'", path_array_name))
                .to_owned();
            let array = value
                .as_array()
                .expect("Failed to convert value to array")
                .to_owned();
            value = array[index].clone();
        } else {
            value = value
                .get(path_part)
                .unwrap_or_else(|| panic!("Expected path part '{}'", path_part))
                .to_owned();
        }
    }
    value
}

#[allow(dead_code)]
pub fn validate_json_path(json: Value, path: &str, expected_value: &str) {
    let value = get_json_path(&json, path);
    if value.is_string() {
        assert_eq!(
            value.as_str().expect("Failed to convert value to str"),
            expected_value
        );
    } else {
        assert_eq!(value.to_string(), expected_value);
    }
}

#[allow(dead_code)]
pub async fn body_as_json_value(body: Body) -> Value {
    let bytes = body
        .collect()
        .await
        .expect("Failed to collect body bytes")
        .to_bytes();

    serde_json::from_slice(&bytes).expect("Failed to serialize json")
}

#[allow(dead_code)]
pub struct AuthCommons;
impl AuthCommons {
    pub async fn craete_customer(
        ctx: &&mut TestContext,
        email: &String,
        password: &String
    ) -> Customer {
        let mut transaction = ctx
        .app_state
        .begin_transaction()
        .await
        .expect("Failed to create transaction");

        let mut customer = Customer::new(email.clone(), password.clone()).expect("Failed to new Customer");

        customer = Customer::insert(&mut transaction, customer)
            .await
            .expect("Failed to insert customer");

        ctx.app_state
            .commit_transaction(transaction)
            .await
            .expect("Failed to commit transaction");
        customer
    }

    #[allow(dead_code)]
    pub async fn valid_customer_inserted(
        ctx: &&mut TestContext,
        response: Response
    ) {
        let (_, body) = response.into_parts();
        let bytes = body
            .collect()
            .await
            .expect("Failed to collect body bytes")
            .to_bytes();
        let json_value: Value = serde_json::from_slice(&bytes).expect("Failed to serialize json");

        assert!(json_value.get("id").is_some());
        assert!(json_value.get("email").is_some());

        let customer_id = json_value.get("id")
            .expect("Failed to read id")
            .as_str()
            .expect("Failed to parse str")
            .to_string();
        let customer_email = json_value.get("email")
            .expect("Failed to read email")
            .as_str()
            .expect("Failed to parse str")
            .to_string();

        let mut transaction = ctx
            .app_state
            .begin_transaction()
            .await
            .expect("Failed to create transaction");

        let stored_customer = Customer::get_by(&mut transaction, customer_email.clone())
            .await
            .expect("Failed to get customer");

        assert_eq!(customer_id, stored_customer.id.to_string());
        assert_eq!(customer_email, stored_customer.email);

        ctx.app_state
            .commit_transaction(transaction)
            .await
            .expect("Failed to commit transaction");
    }   
}

#[allow(dead_code)]
pub struct BiometricsCommons;
impl BiometricsCommons {
    #[allow(dead_code)]
    pub async fn craete_biometrics(
        ctx: &&mut TestContext,
        customer_email: &String,
        customer_pass: &String,
        image_path: &String
    ) -> (Customer, Biometrics) {
        let mut transaction = ctx
        .app_state
        .begin_transaction()
        .await
        .expect("Failed to create transaction");

        let customer = AuthCommons::craete_customer(&ctx, customer_email, customer_pass).await;
        let mut biometric = Biometrics::new(customer.id, image_path.to_string());
        biometric = Biometrics::insert(&mut transaction, biometric)
            .await
            .expect("Failed to insert customer");

        ctx.app_state
            .commit_transaction(transaction)
            .await
            .expect("Failed to commit transaction");
        (customer, biometric)
    }

    #[allow(dead_code)]
    pub async fn valid_biometric_inserted(
        ctx: &&mut TestContext,
        response: Response
    ) {
        let (_, body) = response.into_parts();
        let bytes = body
            .collect()
            .await
            .expect("Failed to collect body bytes")
            .to_bytes();
        let json_value: Value = serde_json::from_slice(&bytes).expect("Failed to serialize json");
      
        assert!(json_value.get("customer_id").is_some());
        assert!(json_value.get("image_path").is_some());
        assert!(json_value.get("status").is_some());
        assert!(json_value.get("created_at").is_some());
        assert!(json_value.get("updated_at").is_some());

        let customer_id = json_value.get("customer_id")
            .expect("Failed to read id")
            .as_str()
            .expect("Failed to parse str")
            .to_string();
        let image_path = json_value.get("image_path")
            .expect("Failed to read id")
            .as_str()
            .expect("Failed to parse str")
            .to_string();
        let status = json_value.get("status")
            .expect("Failed to read id")
            .as_str()
            .expect("Failed to parse str")
            .to_string();
        let created_at = json_value.get("created_at")
            .expect("Failed to read id")
            .as_str()
            .expect("Failed to parse str")
            .to_string();
        let updated_at = json_value.get("updated_at")
            .expect("Failed to read id")
            .as_str()
            .expect("Failed to parse str")
            .to_string();

        let mut transaction = ctx
            .app_state
            .begin_transaction()
            .await
            .expect("Failed to create transaction");

        let stored_biometric = Biometrics::get_by(&mut transaction, Uuid::from_str(&customer_id).expect("Failed to parse UUID"))
            .await
            .expect("Failed to get biometric");

        assert_eq!(customer_id, stored_biometric.customer_id.to_string());
        assert_eq!(image_path, stored_biometric.image_path.to_string());
        assert_eq!(status, stored_biometric.status.to_string());
        assert_eq!(created_at, stored_biometric.created_at.to_rfc3339_opts(SecondsFormat::Secs, false));
        assert_eq!(updated_at, stored_biometric.updated_at.to_rfc3339_opts(SecondsFormat::Secs, false));

        ctx.app_state
            .commit_transaction(transaction)
            .await
            .expect("Failed to commit transaction");
    }   
}
