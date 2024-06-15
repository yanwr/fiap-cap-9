use axum::http::StatusCode;
use chrono::Duration;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infra::errors::{AppError, AppErrorData, ToBusinessError};

use super::hash::base64::HashBase64;


pub trait Jwt {
    const ALGORITHM: Algorithm;
    fn generate_jwt(claims: &Self, private_key: String) -> Result<String, AppError>;
    fn extract_jwt(jwt: String, public_key: String) -> Result<Self, AppError>
    where
        Self: Sized;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationClaims {
    pub sub: Uuid,
    pub customer_email: String,
    exp: usize,
}

impl AuthorizationClaims {
    pub fn new(customer_id: Uuid, customer_email: String, duration_in_seconds: i32) -> AuthorizationClaims {
        let expiration =
            chrono::offset::Utc::now() + Duration::seconds(i64::from(duration_in_seconds));
        AuthorizationClaims {
            sub: customer_id,
            customer_email,
            exp: usize::try_from(expiration.timestamp()).expect("Failed to convert to usize"),
        }
    }
}
impl Jwt for AuthorizationClaims {
    const ALGORITHM: Algorithm = Algorithm::RS256;

    fn generate_jwt(claims: &Self, private_key: String) -> Result<String, AppError> {
        let encoding_key = encoding_key(private_key, Self::ALGORITHM)?;
        let jwt = encode(&Header::new(Self::ALGORITHM), claims, &encoding_key).map_err(|err| {
            err.to_business_error(
                "Error while try to generate Authorization JWT",
                None,
            )
        })?;
        Ok(jwt)
    }

    fn extract_jwt(jwt: String, public_key: String) -> Result<Self, AppError> {
        extract_claims(jwt, public_key, Self::ALGORITHM)
    }
}

fn extract_claims<T: for<'de> Deserialize<'de>>(
    jwt: String,
    public_key: String,
    algorithm: Algorithm,
) -> Result<T, AppError> {
    let decoding_key = decoding_key(public_key, algorithm)?;
    let token_data = decode(&jwt, &decoding_key, &Validation::new(algorithm)).map_err(|err| {
        let message = format!("Invalid JWT: {:?}", err);
        AppErrorData::new(StatusCode::UNAUTHORIZED, message, None).to_business_error()
    })?;
    Ok(token_data.claims)
}

fn decoding_key(public_key: String, algorithm: Algorithm) -> Result<DecodingKey, AppError> {
    let normalized_public_key = HashBase64::decode(&public_key)?;
    let decoding_key = match algorithm {
        Algorithm::RS256 => {
            DecodingKey::from_rsa_pem(normalized_public_key.as_bytes()).map_err(|err| {
                let message = format!(
                    "Error while try to read Public Key {:?}: {}",
                    algorithm, err
                );
                err.to_business_error(&message, None)
            })
        }
        _ => {
            let message = format!("Invalid jwt algorithm={:?}", algorithm);
            Err(
                AppErrorData::new(StatusCode::INTERNAL_SERVER_ERROR, message, None)
                    .to_business_error(),
            )
        }
    }?;
    Ok(decoding_key)
}

fn encoding_key(private_key: String, algorithm: Algorithm) -> Result<EncodingKey, AppError> {
    let normalized_private_key = HashBase64::decode(&private_key)?;
    let encoding_key =
        match algorithm {
            Algorithm::RS256 => EncodingKey::from_rsa_pem(normalized_private_key.as_bytes())
                .map_err(|err| {
                    let message = format!(
                        "Error while try to read Private Key {:?}: {}",
                        algorithm, err
                    );
                    err.to_business_error(&message, None)
                }),
            _ => {
                let message = format!("Invalid jwt algorithm={:?}", algorithm);
                Err(
                    AppErrorData::new(StatusCode::INTERNAL_SERVER_ERROR, message, None)
                        .to_business_error(),
                )
            }
        }?;
    Ok(encoding_key)
}

#[cfg(test)]
mod test {
    use crate::infra::env::Environment;

    use super::*;

    #[test]
    fn should_return_jwt_when_generate_auth_jwt_and_extract_claims() -> Result<(), AppError> {
        let customer_id = Uuid::now_v7();
        let customer_email = String::from("user@fiap.com.br");
        let claims = AuthorizationClaims::new(customer_id, customer_email.clone(), 30);
        let jwt = AuthorizationClaims::generate_jwt(&claims, Environment::jwt_private_key())?;

        let extracted_claims = AuthorizationClaims::extract_jwt(jwt, Environment::jwt_public_key())?;

        assert_eq!(extracted_claims.sub, customer_id);
        assert_eq!(extracted_claims.customer_email, customer_email);
        Ok(())
    }

    #[test]
    fn should_return_error_when_generate_auth_jwt_with_invalid_key() -> Result<(), AppError> {
        let customer_id = Uuid::now_v7();
        let customer_email = String::from("user@fiap.com.br");
        let claims = AuthorizationClaims::new(customer_id, customer_email, 30);
        let jwt = AuthorizationClaims::generate_jwt(&claims, String::from(""));

        assert!(jwt.is_err());
        let error = jwt.err().unwrap();
        assert_eq!(
            "Business(AppErrorData { status: 500, message: \"JWT Error: Error while try to read Private Key RS256: InvalidKeyFormat :: InvalidKeyFormat\", tags: None })",
            format!("{:?}", error)
        );
        Ok(())
    }

    #[test]
    fn should_return_error_when_extract_auth_claims_with_invalid_key() -> Result<(), AppError> {
        let claims = AuthorizationClaims::extract_jwt(String::from(""), String::from(""));

        assert!(claims.is_err());
        let error = claims.err().unwrap();
        assert_eq!(
            "Business(AppErrorData { status: 500, message: \"JWT Error: Error while try to read Public Key RS256: InvalidKeyFormat :: InvalidKeyFormat\", tags: None })",
            format!("{:?}", error)
        );
        Ok(())
    }

    #[test]
    fn should_return_error_when_extract_auth_claims_with_invalid_jwt() -> Result<(), AppError> {
        let expected_claims = AuthorizationClaims::extract_jwt(String::from(""), Environment::jwt_public_key());

        assert!(expected_claims.is_err());
        let error = expected_claims.err().unwrap();
        assert_eq!(
            "Business(AppErrorData { status: 401, message: \"Invalid JWT: Error(InvalidToken)\", tags: None })",
            format!("{:?}", error)
        );
        Ok(())
    }
}
