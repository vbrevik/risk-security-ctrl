I now have all the context needed. Here is the section content:

# Section 7: Seed-Admin Binary

## Overview

This section implements a standalone binary (`seed-admin`) that bootstraps the first admin user in the database. It is designed for air-gapped deployments where the initial admin must be created before the web UI is available. The binary is idempotent: running it twice with the same email is a safe no-op.

## Dependencies on Other Sections

- **Section 1 (Dependencies and AppState):** The `argon2`, `rand`, `dotenvy`, `sqlx`, and `uuid` crates must be available in `Cargo.toml`.
- **Section 3 (Password and Session Utilities):** The `hash_password` function from `password.rs` is imported from the library crate.
- **Section 4 (Auth Service):** Optionally reuses `find_user_by_email` for the existence check, or the binary can perform its own direct query.

## Tests

Write these tests in `backend/tests/seed_admin.rs`. They use an in-memory SQLite database with migrations applied, then call the core logic function directly (not the binary's `main`). This avoids needing to spawn a subprocess.

### Integration test: seed-admin creates admin user

- Set up an in-memory SQLite pool and run migrations.
- Call the seed-admin core logic function with email `"admin@test.com"`, password `"securepass123"`, and name `"Test Admin"`.
- Query the `users` table and assert a user exists with `role = "admin"`, `is_active = 1`, and the correct email and name.
- Assert the `password_hash` column starts with `$argon2id$` (verifying proper hashing).

### Integration test: seed-admin is idempotent

- Set up an in-memory SQLite pool and run migrations.
- Call the seed-admin core logic function twice with the same email.
- Assert no error on the second call.
- Query `users` and assert exactly one row with that email exists.

## Implementation Details

### Binary Registration

Add a new `[[bin]]` entry to `backend/Cargo.toml`:

```toml
[[bin]]
name = "seed-admin"
path = "src/bin/seed_admin.rs"
```

### File to Create: `backend/src/bin/seed_admin.rs`

This file contains the binary entry point and a public async function for the core logic (testable without running the binary).

#### Core Logic Function

Extract the seeding logic into a standalone async function with this signature:

```rust
/// Seed an admin user into the database. Returns Ok(()) if the user was
/// created or already existed. Prints status to stdout.
pub async fn seed_admin(
    pool: &sqlx::SqlitePool,
    email: &str,
    password: &str,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>>
```

The function performs these steps in order:

1. **Check existence:** Query `SELECT id FROM users WHERE email = ?`. If a row is returned, print `"User with email {email} already exists, skipping."` to stdout and return `Ok(())`.
2. **Hash password:** Call `ontology_backend::features::auth::password::hash_password(password)` (imported from the library crate). This produces an Argon2id PHC string.
3. **Generate UUID:** Create a new UUID v4 for the user ID via `uuid::Uuid::new_v4().to_string()`.
4. **Insert user:** Execute an INSERT into the `users` table with the following column values:
   - `id` — the generated UUID string
   - `email` — the provided email
   - `password_hash` — the Argon2id hash from step 2
   - `name` — the provided name
   - `role` — hardcoded `"admin"`
   - `is_active` — hardcoded `1`
5. **Print confirmation:** Print `"Admin user created: {email} (id: {uuid})"` to stdout.

#### `main` Function

The `#[tokio::main]` entry point:

1. Call `dotenvy::dotenv().ok()` to load `.env` file (non-fatal if missing).
2. Read `DATABASE_URL` from environment (required, panic with clear message if absent).
3. Read `ADMIN_EMAIL` from environment (required).
4. Read `ADMIN_PASSWORD` from environment (required).
5. Read `ADMIN_NAME` from environment (optional, default to `"Admin"`).
6. Connect to the SQLite database using `sqlx::SqlitePool::connect(&database_url)`.
7. Run migrations with `sqlx::migrate!().run(&pool).await` to ensure the schema is up to date.
8. Call `seed_admin(&pool, &email, &password, &name).await`.
9. On error, print the error to stderr and exit with code 1.

### Usage

The binary is invoked from the command line with environment variables:

```bash
DATABASE_URL=sqlite:data.db \
ADMIN_EMAIL=admin@example.com \
ADMIN_PASSWORD=changeme123 \
cargo run --bin seed-admin
```

Or with an optional custom name:

```bash
DATABASE_URL=sqlite:data.db \
ADMIN_EMAIL=admin@example.com \
ADMIN_PASSWORD=changeme123 \
ADMIN_NAME="System Administrator" \
cargo run --bin seed-admin
```

### Key Design Decisions

- **Library crate imports:** The binary imports `hash_password` from `ontology_backend::features::auth::password` rather than reimplementing hashing. This ensures the same Argon2id configuration is used everywhere.
- **Idempotent by design:** The existence check uses a SELECT before INSERT rather than INSERT OR IGNORE, so the binary can print a meaningful message about what happened.
- **No session creation:** The binary only creates the user row. The admin must log in through the normal web flow to get a session.
- **Exit code 0 on existing user:** Finding an existing user with the same email is not an error. This makes the binary safe to run in automated provisioning scripts.
- **Migrations run automatically:** The binary runs `sqlx::migrate!()` before seeding, so it works on a fresh database file without requiring a separate migration step.

### Database Schema Reference

The `users` table (from `migrations/001_initial_schema.sql`):

```sql
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    name TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'viewer',
    is_active INTEGER DEFAULT 1,
    last_login_at TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);
```

The binary overrides the default `role` by explicitly setting `"admin"` in the INSERT statement rather than relying on the column default of `"viewer"`.