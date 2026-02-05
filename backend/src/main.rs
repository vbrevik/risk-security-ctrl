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

    // Create database connection pool
    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&db).await?;

    tracing::info!("Database migrations completed");

    // Import ontology data (if not already imported)
    let ontology_data_dir = std::path::Path::new("../ontology-data");
    if ontology_data_dir.exists() {
        // Check if data already exists
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concepts")
            .fetch_one(&db)
            .await?;

        if count.0 == 0 {
            tracing::info!("No ontology data found, importing...");
            if let Err(e) = import::import_all_ontologies(&db, ontology_data_dir).await {
                tracing::error!("Failed to import ontology data: {}", e);
                tracing::warn!("Continuing without ontology data. Import manually with CLI.");
            }
        } else {
            tracing::info!("Ontology data already loaded ({} concepts)", count.0);
        }
    } else {
        tracing::warn!(
            "Ontology data directory not found at {:?}",
            ontology_data_dir
        );
    }

    // Create application state
    let state = AppState {
        db,
        config: config.clone(),
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
