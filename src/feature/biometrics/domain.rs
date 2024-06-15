use chrono::{DateTime, SecondsFormat, Utc};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Postgres, Transaction};
use strum_macros::Display;
use uuid::Uuid;

use crate::infra::errors::{AppError, AppErrorData, ToBusinessError};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BiometricDtoResponse {
    pub customer_id: Uuid,
    pub image_path: String,
    pub status: BiometricsStatus,
    pub created_at: String,
    pub updated_at: String
}
impl BiometricDtoResponse {
    pub fn from(domain: Biometrics) -> Self {
        Self {
            customer_id: domain.customer_id,
            image_path: domain.image_path,
            status: domain.status,
            created_at: domain.created_at.to_rfc3339_opts(SecondsFormat::Secs, false),
            updated_at: domain.updated_at.to_rfc3339_opts(SecondsFormat::Secs, false),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BiometricDtoRequest {
    pub customer_id: Uuid,
    pub image_path: String,
}

#[derive(
    sqlx::Type, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Display,
)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "biometrics_status", rename_all = "snake_case")]
pub enum BiometricsStatus {
    #[strum(serialize = "in_analysis")]
    InAnalysis,
    #[strum(serialize = "reproved")]
    Reproved,
    #[strum(serialize = "take_again")]
    TakeAgain,
    #[strum(serialize = "conclued")]
    Conclued,
}

#[derive(Clone, Debug, PartialEq, FromRow)]
pub struct Biometrics {
    pub customer_id: Uuid,
    pub image_path: String,
    pub status: BiometricsStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}
impl Biometrics {
    pub fn new(
        customer_id: Uuid,
        image_path: String,
    ) -> Self {
        Self {
            customer_id,
            image_path,
            status: BiometricsStatus::InAnalysis,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub async fn insert(transaction: &mut Transaction<'_, Postgres>, biometric: Self) -> Result<Self, AppError> {
        let query = r#"
            INSERT INTO biometrics
                (customer_id, image_path, status, updated_at)
            VALUES
                ($1, $2, $3, $4)
            RETURNING *
        "#;
        let stored_biometric: Self = sqlx::query_as(query)
            .bind(biometric.customer_id)
            .bind(biometric.image_path)
            .bind(biometric.status)
            .bind(biometric.updated_at)
            .fetch_one(&mut **transaction)
            .await
            .map_err(|err| err.to_business_error("insert", None))?;
        Ok(stored_biometric)
    }

    pub async fn update(
        transaction: &mut Transaction<'_, Postgres>, 
        biometric: Self
    ) -> Result<Self, AppError> {
        let query = r#"
            UPDATE biometrics
            SET image_path = $2,
                status = $3
                updated_at = $4
            WHERE customer_id = $1
            RETURNING *
        "#;
        let updated_biometric: Self = sqlx::query_as(query)
            .bind(biometric.customer_id)
            .bind(biometric.image_path)
            .bind(biometric.status)
            .bind(Utc::now())
            .fetch_one(&mut **transaction)
            .await
            .map_err(|err| err.to_business_error("update", None))?;
        Ok(updated_biometric)
    }

    pub async fn get_by(
        transaction: &mut Transaction<'_, Postgres>, 
        customer_id: Uuid
    ) -> Result<Self, AppError> {
        let query = r#"
            SELECT * FROM biometrics
            WHERE customer_id = $1
        "#;
        let result = sqlx::query_as(query)
            .bind(customer_id)
            .fetch_optional(&mut **transaction)
            .await;
        match result {
            Ok(biometric) => {
                if let Some(biometric) = biometric {
                    Ok(biometric)
                } else {
                    Err(AppErrorData::new(
                        StatusCode::NOT_FOUND,
                        String::from("Biometric not found"),
                        None,
                    )
                    .to_business_error())
                }
            }
            Err(err) => Err(err.to_business_error("get by", None)),
        }
    }
}
