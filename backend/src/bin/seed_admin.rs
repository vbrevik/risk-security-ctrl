use sqlx::SqlitePool;

/// Seed an admin user into the database. Returns Ok(()) if the user was
/// created or already existed.
pub async fn seed_admin(
    pool: &SqlitePool,
    email: &str,
    password: &str,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if user already exists
    let existing: Option<(String,)> =
        sqlx::query_as("SELECT id FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(pool)
            .await?;

    if existing.is_some() {
        println!("User with email {email} already exists, skipping.");
        return Ok(());
    }

    // Hash password using the shared utility
    let password_hash = ontology_backend::features::auth::password::hash_password(password)
        .map_err(|e| format!("Failed to hash password: {e:?}"))?;

    let id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO users (id, email, password_hash, name, role, is_active) VALUES (?, ?, ?, ?, 'admin', 1)",
    )
    .bind(&id)
    .bind(email)
    .bind(&password_hash)
    .bind(name)
    .execute(pool)
    .await?;

    println!("Admin user created: {email} (id: {id})");
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let email =
        std::env::var("ADMIN_EMAIL").expect("ADMIN_EMAIL must be set");
    let password =
        std::env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set");
    let name =
        std::env::var("ADMIN_NAME").unwrap_or_else(|_| "Admin".to_string());

    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    if let Err(e) = seed_admin(&pool, &email, &password, &name).await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
