# Prompt Contract: Section 03 — Password and Session Utilities

## GOAL
Implement pure utility functions: Argon2id password hashing/verification, CSPRNG session token generation, SHA-256 session token hashing.

## CONSTRAINTS
- Use Argon2id (not Argon2i or Argon2d)
- Session tokens must be 32 bytes (256-bit entropy), hex-encoded to 64 chars
- Database stores SHA-256 hash of tokens, never raw tokens
- verify_password must return Ok(false) for wrong password, not Err

## FAILURE CONDITIONS
- SHALL NOT store raw session tokens (always hash before DB storage)
- SHALL NOT use weak RNG for token generation (must use OS CSPRNG)
- SHALL NOT panic on password verification failure (wrong password = Ok(false))
