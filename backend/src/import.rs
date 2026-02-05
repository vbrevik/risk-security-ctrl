use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::path::Path;
use tracing::{info, warn};

/// Framework definition from JSON
#[derive(Debug, Deserialize, Serialize)]
pub struct FrameworkData {
    pub id: String,
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub source_url: Option<String>,
}

/// Concept definition from JSON
#[derive(Debug, Deserialize, Serialize)]
pub struct ConceptData {
    pub id: String,
    pub framework_id: String,
    pub parent_id: Option<String>,
    pub concept_type: String,
    pub code: Option<String>,
    pub name_en: String,
    pub name_nb: Option<String>,
    pub definition_en: Option<String>,
    pub definition_nb: Option<String>,
    pub source_reference: Option<String>,
    pub sort_order: Option<i64>,
}

/// Relationship definition from JSON
#[derive(Debug, Deserialize, Serialize)]
pub struct RelationshipData {
    pub id: String,
    pub source_concept_id: String,
    pub target_concept_id: String,
    pub relationship_type: String,
    pub description: Option<String>,
}

/// Ontology file structure
#[derive(Debug, Deserialize)]
pub struct OntologyFile {
    pub framework: FrameworkData,
    pub concepts: Vec<ConceptData>,
}

/// Relationships file structure
#[derive(Debug, Deserialize)]
pub struct RelationshipsFile {
    pub relationships: Vec<RelationshipData>,
}

/// Import a framework and its concepts from a JSON file
pub async fn import_ontology_file(
    db: &SqlitePool,
    file_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Importing ontology from: {}", file_path.display());

    // Read and parse JSON file
    let content = tokio::fs::read_to_string(file_path).await?;
    let ontology: OntologyFile = serde_json::from_str(&content)?;

    // Insert framework
    info!(
        "Inserting framework: {} ({})",
        ontology.framework.name, ontology.framework.id
    );
    sqlx::query!(
        r#"
        INSERT INTO frameworks (id, name, version, description, source_url)
        VALUES (?, ?, ?, ?, ?)
        ON CONFLICT(id) DO UPDATE SET
            name = excluded.name,
            version = excluded.version,
            description = excluded.description,
            source_url = excluded.source_url,
            updated_at = datetime('now')
        "#,
        ontology.framework.id,
        ontology.framework.name,
        ontology.framework.version,
        ontology.framework.description,
        ontology.framework.source_url
    )
    .execute(db)
    .await?;

    // Insert concepts
    info!("Inserting {} concepts", ontology.concepts.len());
    for concept in &ontology.concepts {
        sqlx::query!(
            r#"
            INSERT INTO concepts (
                id, framework_id, parent_id, concept_type, code,
                name_en, name_nb, definition_en, definition_nb,
                source_reference, sort_order
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                framework_id = excluded.framework_id,
                parent_id = excluded.parent_id,
                concept_type = excluded.concept_type,
                code = excluded.code,
                name_en = excluded.name_en,
                name_nb = excluded.name_nb,
                definition_en = excluded.definition_en,
                definition_nb = excluded.definition_nb,
                source_reference = excluded.source_reference,
                sort_order = excluded.sort_order,
                updated_at = datetime('now')
            "#,
            concept.id,
            concept.framework_id,
            concept.parent_id,
            concept.concept_type,
            concept.code,
            concept.name_en,
            concept.name_nb,
            concept.definition_en,
            concept.definition_nb,
            concept.source_reference,
            concept.sort_order
        )
        .execute(db)
        .await?;
    }

    info!(
        "Successfully imported {} concepts from {}",
        ontology.concepts.len(),
        ontology.framework.name
    );
    Ok(())
}

/// Import relationships from the relationships.json file
pub async fn import_relationships(
    db: &SqlitePool,
    file_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Importing relationships from: {}", file_path.display());

    // Read and parse JSON file
    let content = tokio::fs::read_to_string(file_path).await?;
    let relationships_file: RelationshipsFile = serde_json::from_str(&content)?;

    // Insert relationships
    info!(
        "Inserting {} relationships",
        relationships_file.relationships.len()
    );
    for relationship in &relationships_file.relationships {
        let result = sqlx::query!(
            r#"
            INSERT INTO relationships (id, source_concept_id, target_concept_id, relationship_type, description)
            VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                source_concept_id = excluded.source_concept_id,
                target_concept_id = excluded.target_concept_id,
                relationship_type = excluded.relationship_type,
                description = excluded.description
            "#,
            relationship.id,
            relationship.source_concept_id,
            relationship.target_concept_id,
            relationship.relationship_type,
            relationship.description
        )
        .execute(db)
        .await;

        if let Err(e) = result {
            warn!(
                "Failed to insert relationship {}: {}. Skipping (likely missing concept).",
                relationship.id, e
            );
        }
    }

    info!("Successfully imported relationships");
    Ok(())
}

/// Import all ontology data from the ontology-data directory
pub async fn import_all_ontologies(
    db: &SqlitePool,
    data_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting full ontology import from: {}", data_dir.display());

    // Import framework files
    let framework_files = [
        "iso31000.json",
        "iso31010.json",
        "nist-csf.json",
        "iso9000.json",
        "iso27000.json",
        "fmn.json",
        "data-centric.json",
        "zero-trust.json",
    ];

    for file_name in &framework_files {
        let file_path = data_dir.join(file_name);
        if file_path.exists() {
            import_ontology_file(db, &file_path).await?;
        } else {
            warn!("Ontology file not found: {}", file_path.display());
        }
    }

    // Import relationships
    let relationships_path = data_dir.join("relationships.json");
    if relationships_path.exists() {
        import_relationships(db, &relationships_path).await?;
    } else {
        warn!(
            "Relationships file not found: {}",
            relationships_path.display()
        );
    }

    info!("Full ontology import completed");
    Ok(())
}
