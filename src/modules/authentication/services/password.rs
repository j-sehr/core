use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use crate::modules::authentication::errors::service::AuthenticationServiceError;

pub struct PasswordService;
impl PasswordService {
    pub fn hash_password(&self, password: &str) -> Result<String, AuthenticationServiceError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(AuthenticationServiceError::from_error)?
            .to_string();

        Ok(password_hash)
    }

    pub fn verify_password(
        &self,
        hash: &str,
        password: &str,
    ) -> Result<bool, AuthenticationServiceError> {
        let parsed_hash =
            PasswordHash::new(hash).map_err(AuthenticationServiceError::from_error)?;
        let argon2 = Argon2::default();

        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(AuthenticationServiceError::from_error(e)),
        }
    }
}
