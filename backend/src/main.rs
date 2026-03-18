use std::net::SocketAddr;

use ontology_backend::{import, AppState, Config};
use sqlx::sqlite::SqlitePoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Ontology API",
        description = "Risk Management Framework Explorer API",
        version = "0.1.0",
        contact(name = "Ontology Team")
    ),
    paths(
        ontology_backend::features::ontology::routes::health_check,
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "ontology", description = "Ontology management endpoints"),
        (name = "compliance", description = "Compliance tracking endpoints"),
        (name = "reports", description = "Reporting endpoints"),
        (name = "auth", description = "Authentication endpoints"),
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,ontology_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env();

    // Create database connection pool with foreign key enforcement
    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                sqlx::query("PRAGMA foreign_keys = ON")
                    .execute(&mut *conn)
                    .await?;
                Ok(())
            })
        })
        .connect(&config.database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&db).await?;

    tracing::info!("Database migrations completed");

    // Check for foreign key violations in existing data
    match sqlx::query_as::<_, (String, i64, Option<String>, i64)>("PRAGMA foreign_key_check")
        .fetch_all(&db)
        .await
    {
        Ok(violations) => {
            if !violations.is_empty() {
                tracing::warn!(
                    "Found {} foreign key violations in existing data",
                    violations.len()
                );
                for (table, rowid, parent, fkid) in &violations {
                    tracing::warn!(
                        "FK violation: table={}, rowid={}, parent={:?}, fkid={}",
                        table, rowid, parent, fkid
                    );
                }
            } else {
                tracing::info!("Foreign key integrity check passed");
            }
        }
        Err(e) => {
            tracing::warn!("Could not run foreign key check: {}", e);
        }
    }

    // Import ontology data (if not already imported or if new frameworks available)
    let ontology_data_dir = std::path::Path::new("../ontology-data");
    if ontology_data_dir.exists() {
        // Check how many frameworks are loaded vs available on disk
        let framework_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM frameworks")
            .fetch_one(&db)
            .await?;

        let available_files: Vec<_> = std::fs::read_dir(ontology_data_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                name.ends_with(".json") && name != "relationships.json" && name != "topic-tags.json"
            })
            .collect();

        if framework_count.0 < available_files.len() as i64 {
            tracing::info!(
                "Found {} frameworks in DB but {} data files on disk, importing new data...",
                framework_count.0,
                available_files.len()
            );
            if let Err(e) = import::import_all_ontologies(&db, ontology_data_dir).await {
                tracing::error!("Failed to import ontology data: {}", e);
                tracing::warn!("Continuing without full ontology data. Import manually with CLI.");
            }
        } else if framework_count.0 == 0 {
            tracing::info!("No ontology data found, importing...");
            if let Err(e) = import::import_all_ontologies(&db, ontology_data_dir).await {
                tracing::error!("Failed to import ontology data: {}", e);
                tracing::warn!("Continuing without ontology data. Import manually with CLI.");
            }
        } else {
            tracing::info!(
                "Ontology data already loaded ({} frameworks)",
                framework_count.0
            );
        }
    } else {
        tracing::warn!(
            "Ontology data directory not found at {:?}",
            ontology_data_dir
        );
    }

    // Load topics from ontology-data/topic-tags.json
    let topics = ontology_backend::load_topics(std::path::Path::new("../ontology-data/topic-tags.json"));
    tracing::info!("Loaded {} topics for analysis matching", topics.len());

    // Create application state
    let state = AppState {
        db,
        config: config.clone(),
        topics,
    };

    // Build router with Swagger UI
    let app = ontology_backend::create_router(state)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    tracing::info!("Starting server on {}", addr);
    tracing::info!("Swagger UI available at http://{}/swagger-ui", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
