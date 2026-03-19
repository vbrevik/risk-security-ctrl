I now have all the context needed. Here is the section content.

# Section 3: Password and Session Utilities

## Overview

This section implements a `password` module at `backend/src/features/auth/password.rs` containing three pure utility functions: Argon2id password hashing/verification, CSPRNG session token generation, and SHA-256 session token hashing. These utilities have no database dependency and are consumed by the auth service (Section 4), route handlers (Section 6), and seed-admin binary (Section 7).

## Dependencies

**Requires completed:** Section 1 (Cargo dependencies for `argon2`, `rand`, `hex`, and `sha2` must be present in `backend/Cargo.toml`).

**Blocks:** Sections 4, 5, 6, and 7 all import from this module.

## File to Create

`/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/auth/password.rs`

## Module Registration

Update `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/auth/mod.rs` to add:

```rust
pub mod password;
```

## Tests First

All tests live in an inline `#[cfg(test)] mod tests` block at the bottom of `password.rs`. Write the tests before the implementations.

### Test: hash_password produces valid PHC string

```rust
/// Hash "testpassword123" and assert:
/// - Result is Ok
/// - Output starts with "$argon2id$"
/// - Output contains multiple '$'-separated segments (version, params, salt, hash)
#[test]
fn test_hash_password_produces_valid_phc_string()
```

### Test: verify_password returns true for correct password

```rust
/// Hash a password, then verify with the same plaintext.
/// Assert Ok(true).
#[test]
fn test_verify_password_correct()
```

### Test: verify_password returns false for wrong password

```rust
/// Hash "password1", then verify with "password2".
/// Assert Ok(false).
#[test]
fn test_verify_password_wrong()
```

### Test: generate_session_token produces 64-char hex

```rust
/// Generate a token and assert:
/// - Length is exactly 64 characters
/// - All characters are valid hex digits (0-9, a-f)
#[test]
fn test_generate_session_token_format()
```

### Test: generate_session_token produces unique values

```rust
/// Generate 100 tokens, collect into a HashSet.
/// Assert the set size is 100 (all unique).
#[test]
fn test_generate_session_token_uniqueness()
```

### Test: hash_session_token is deterministic

```rust
/// Hash the same token string twice.
/// Assert both results are identical.
#[test]
fn test_hash_session_token_deterministic()
```

### Test: hash_session_token produces different output for different input

```rust
/// Hash two different token strings.
/// Assert the outputs differ.
#[test]
fn test_hash_session_token_different_inputs()
```

## Implementation Details

### Function: hash_password

```rust
/// Hashes a plaintext password using Argon2id with OWASP-recommended defaults.
///
/// Returns the PHC-encoded string (e.g., `$argon2id$v=19$m=19456,t=2,p=1$<salt>$<hash>`).
/// The PHC format is self-describing: salt and parameters are embedded in the output,
/// so no separate column is needed for salt storage.
pub fn hash_password(plain: &str) -> Result<String, AppError>
```

Implementation approach:
- Generate a random salt via `argon2::password_hash::SaltString::generate(&mut OsRng)`
- Create `Argon2::default()` which uses Argon2id variant with OWASP defaults (19 MiB memory, 2 iterations, 1 parallelism)
- Call `argon2.hash_password(plain.as_bytes(), &salt)`
- Return the `.to_string()` of the resulting `PasswordHash`
- Map any hashing error to `AppError::Internal`

### Function: verify_password

```rust
/// Verifies a plaintext password against a stored PHC-encoded hash.
///
/// Returns Ok(true) on match, Ok(false) on mismatch. Never panics.
/// The PHC string contains the algorithm, version, parameters, and salt,
/// so Argon2 can reconstruct the exact hashing configuration.
pub fn verify_password(plain: &str, phc_hash: &str) -> Result<bool, AppError>
```

Implementation approach:
- Parse the stored hash with `argon2::password_hash::PasswordHash::new(phc_hash)`
- Call `Argon2::default().verify_password(plain.as_bytes(), &parsed_hash)`
- If verification succeeds, return `Ok(true)`
- If verification returns a `password_hash::Error::Password` (mismatch), return `Ok(false)`
- Map any other error (corrupt hash, wrong algorithm) to `AppError::Internal`

### Function: generate_session_token

```rust
/// Generates a cryptographically secure session token.
///
/// Produces 32 random bytes (256-bit entropy) from the OS CSPRNG,
/// returned as a 64-character lowercase hex string.
pub fn generate_session_token() -> String
```

Implementation approach:
- Allocate a `[u8; 32]` buffer
- Fill with `rand::rngs::OsRng` via `rand::RngCore::fill_bytes`
- Encode to lowercase hex using the `hex` crate: `hex::encode(&bytes)`

### Function: hash_session_token

```rust
/// Computes a SHA-256 hash of a raw session token, returned as a hex string.
///
/// The database stores this hash, not the raw token. On session lookup,
/// the presented token is hashed before the WHERE clause. This prevents
/// session hijacking if the database file is exposed.
pub fn hash_session_token(token: &str) -> String
```

Implementation approach:
- Use `sha2::Sha256` (already a transitive dependency of `argon2`)
- Create a `Sha256` hasher via `Digest::new()`
- Feed `token.as_bytes()` into it
- Finalize and encode the result as lowercase hex: `hex::encode(hasher.finalize())`

Note: `sha2` must be listed as an explicit dependency in `Cargo.toml` (added in Section 1) even though it is a transitive dependency, to ensure the import is stable.

## Crate Imports

The module needs these imports at the top of `password.rs`:

```rust
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use hex;
use rand::RngCore;
use sha2::{Digest, Sha256};

use crate::error::AppError;
```

Note: `OsRng` from `argon2::password_hash::rand_core` is used for salt generation in `hash_password`. A separate `OsRng` from `rand::rngs::OsRng` (or `rand::RngCore` trait on the same type) is used for session token byte generation. Both ultimately source from the OS CSPRNG. If `rand` re-exports the same `OsRng`, either import path works for `generate_session_token` -- just ensure `fill_bytes` is available via the `RngCore` trait.

## Error Mapping

Both `hash_password` and `verify_password` need to convert `argon2::password_hash::Error` into `AppError`. The mapping is:

- `password_hash::Error::Password` in `verify_password` means mismatch -- return `Ok(false)`, not an error
- All other `password_hash::Error` variants indicate corrupt data or internal failures -- map to `AppError::Internal(error.to_string())`

## Verification

After implementation, run:

```bash
cd /Users/vidarbrevik/projects/risk-security-ctrl/backend && cargo test password
```

All seven tests should pass. The tests are pure (no database, no async runtime needed) and should execute in under 2 seconds despite Argon2 being intentionally slow (test passwords are short, and default params use ~19 MiB which completes quickly on modern hardware).