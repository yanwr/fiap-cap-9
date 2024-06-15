use chrono::{DateTime, Utc};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Postgres, Transaction};
use uuid::Uuid;

use crate::{infra::errors::{AppError, AppErrorData, ToBusinessError}, support::hash::hash_bcrypt::HashBCrypt};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerDtoResponse {
    pub id: Uuid,
    pub email: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerDtoRequest {
    pub email: String,
    pub password: String
}

#[derive(Clone, Debug, PartialEq, FromRow)]
pub struct Customer {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>
}
impl Customer {
    pub fn new(
        email: String,
        password: String
    ) -> Result<Self, AppError> {
        Ok(Self {
            id: Uuid::now_v7(),
            email,
            password: HashBCrypt::encode(password)?,
            created_at: Utc::now()
        })
    }

    pub async fn insert(transaction: &mut Transaction<'_, Postgres>, customer: Self) -> Result<Self, AppError> {
        let query = r#"
        INSERT INTO customer
            (id, email, password)
        VALUES
            ($1, $2, $3)
        RETURNING *
        "#;
        let stored_customer: Self = sqlx::query_as(query)
            .bind(customer.id)
            .bind(customer.email)
            .bind(customer.password)
            .fetch_one(&mut **transaction)
            .await
            .map_err(|err| err.to_business_error("insert", None))?;
        Ok(stored_customer)
    }

    pub async fn get_by(
        transaction: &mut Transaction<'_, Postgres>, 
        email: String
    ) -> Result<Self, AppError> {
        let query = r#"
            SELECT * FROM customer
            WHERE email = $1
        "#;
        let result = sqlx::query_as(query)
            .bind(email)
            .fetch_optional(&mut **transaction)
            .await;
        match result {
            Ok(customer) => {
                if let Some(customer) = customer {
                    Ok(customer)
                } else {
                    Err(AppErrorData::new(
                        StatusCode::NOT_FOUND,
                        String::from("Customer not found"),
                        None,
                    )
                    .to_business_error())
                }
            }
            Err(err) => Err(err.to_business_error("get by", None)),
        }
    }
}
