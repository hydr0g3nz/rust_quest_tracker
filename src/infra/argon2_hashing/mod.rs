use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash(password: String) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let bytes = password.as_bytes();
    let argon2 = Argon2::default();
    let result = argon2
        .hash_password(bytes, &salt)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    Ok(result.to_string())
}

pub fn verify(password: String, hash: String) -> Result<bool> {
    let parsed_hash = PasswordHash::new(&hash).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let bytes = password.as_bytes();

    Ok(Argon2::default()
        .verify_password(bytes, &parsed_hash)
        .is_ok())
}
