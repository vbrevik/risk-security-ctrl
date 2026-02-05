use axum::Router;
use sqlx::SqlitePool;

// Re-export the necessary types from the main crate
use ontology_backend::{AppState, Config};

pub async fn create_test_app() -> Router {
    // Use the existing database for tests
    // In a production setup, you'd create a separate test database
    let config = Config::from_env();

    let pool = SqlitePool::connect(&config.database_url)
        .await
        .expect("Failed to connect to test database");

    // Ensure migrations are run
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // Import test data if needed
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concepts")
        .fetch_one(&pool)
        .await
        .expect("Failed to count concepts");

    if count.0 == 0 {
        let data_dir = std::path::Path::new("../ontology-data");
        if data_dir.exists() {
            ontology_backend::import::import_all_ontologies(&pool, data_dir)
                .await
                .expect("Failed to import test data");
        }
    }

    let state = AppState {
        db: pool,
        config: config.clone(),
    };

    ontology_backend::create_router(state)
}
