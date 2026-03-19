use argon2::{
    password_hash::{
        rand_core::{OsRng, RngCore},
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};
use sha2::{Digest, Sha256};

use crate::error::AppError;

/// Hashes a plaintext password using Argon2id with OWASP-recommended defaults.
///
/// Returns the PHC-encoded string (e.g., `$argon2id$v=19$m=19456,t=2,p=1$<salt>$<hash>`).
pub fn hash_password(plain: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(plain.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("Password hashing failed: {e}")))?;
    Ok(hash.to_string())
}

/// Verifies a plaintext password against a stored PHC-encoded hash.
///
/// Returns Ok(true) on match, Ok(false) on mismatch. Never panics.
pub fn verify_password(plain: &str, phc_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(phc_hash)
        .map_err(|e| AppError::Internal(format!("Invalid password hash format: {e}")))?;
    match Argon2::default().verify_password(plain.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(e) => Err(AppError::Internal(format!("Password verification failed: {e}"))),
    }
}

/// Generates a cryptographically secure session token.
///
/// Produces 32 random bytes (256-bit entropy) from the OS CSPRNG,
/// returned as a 64-character lowercase hex string.
pub fn generate_session_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

/// Computes a SHA-256 hash of a raw session token, returned as a hex string.
///
/// The database stores this hash, not the raw token.
pub fn hash_session_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_hash_password_produces_valid_phc_string() {
        let hash = hash_password("testpassword123").unwrap();
        assert!(hash.starts_with("$argon2id$"));
        assert!(hash.split('$').count() >= 5);
    }

    #[test]
    fn test_verify_password_correct() {
        let hash = hash_password("mypassword").unwrap();
        assert!(verify_password("mypassword", &hash).unwrap());
    }

    #[test]
    fn test_verify_password_wrong() {
        let hash = hash_password("password1").unwrap();
        assert!(!verify_password("password2", &hash).unwrap());
    }

    #[test]
    fn test_generate_session_token_format() {
        let token = generate_session_token();
        assert_eq!(token.len(), 64);
        assert!(token.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_session_token_uniqueness() {
        let tokens: HashSet<String> = (0..100).map(|_| generate_session_token()).collect();
        assert_eq!(tokens.len(), 100);
    }

    #[test]
    fn test_hash_session_token_deterministic() {
        let token = "abc123";
        assert_eq!(hash_session_token(token), hash_session_token(token));
    }

    #[test]
    fn test_hash_session_token_different_inputs() {
        assert_ne!(hash_session_token("token_a"), hash_session_token("token_b"));
    }
}
