use bcrypt::{hash, verify, DEFAULT_COST};

use crate::infra::errors::{AppError, ToBusinessError};

pub struct HashBCrypt;
impl HashBCrypt {
    pub fn encode(data: String) -> Result<String, AppError>{
        Ok(
            hash(&data, DEFAULT_COST)
                .map_err(|err| err.to_business_error("Error to hash", None))?
        )
    }

    pub fn verify(data_hashed: String, data: String) -> Result<bool, AppError>{
        Ok(
            verify(&data, &data_hashed)
                .map_err(|err| err.to_business_error("Error to verify", None))?
        )
    }
}
