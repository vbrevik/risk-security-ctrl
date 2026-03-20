use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::path::Path;
use tracing::{info, warn};
use uuid::Uuid;

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

/// Guidance file structure (*-guidance.json companion files)
#[derive(Debug, Deserialize)]
pub struct GuidanceFile {
    pub framework_id: String,
    pub source_pdf: String,
    pub guidance: Vec<GuidanceEntry>,
}

/// One guidance entry per concept
#[derive(Debug, Deserialize)]
pub struct GuidanceEntry {
    pub concept_id: String,
    pub source_page: i64,
    pub about_en: Option<String>,
    pub about_nb: Option<String>,
    pub suggested_actions_en: Option<Vec<String>>,
    pub suggested_actions_nb: Option<Vec<String>>,
    pub transparency_questions_en: Option<Vec<String>>,
    pub transparency_questions_nb: Option<Vec<String>>,
    pub resources: Option<Vec<ResourceEntry>>,
    pub references: Option<Vec<ReferenceEntry>>,
}

/// Transparency resource entry
#[derive(Debug, Deserialize)]
pub struct ResourceEntry {
    pub title: String,
    pub url: Option<String>,
    #[serde(rename = "type")]
    pub resource_type: Option<String>,
}

/// Academic reference entry
#[derive(Debug, Deserialize)]
pub struct ReferenceEntry {
    pub title: String,
    pub authors: Option<String>,
    pub year: Option<i64>,
    pub venue: Option<String>,
    pub url: Option<String>,
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

/// Import guidance data from a *-guidance.json companion file
pub async fn import_guidance_file(
    db: &SqlitePool,
    file_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Importing guidance from: {}", file_path.display());

    let content = tokio::fs::read_to_string(file_path).await?;
    let guidance_file: GuidanceFile = serde_json::from_str(&content)?;

    info!(
        "Processing {} guidance entries for framework {}",
        guidance_file.guidance.len(),
        guidance_file.framework_id
    );

    for entry in &guidance_file.guidance {
        // Validate concept exists (STIG V-222606)
        let exists: Option<(String,)> =
            sqlx::query_as("SELECT id FROM concepts WHERE id = ?")
                .bind(&entry.concept_id)
                .fetch_optional(db)
                .await?;

        if exists.is_none() {
            warn!(
                "Concept {} not found, skipping guidance entry",
                entry.concept_id
            );
            continue;
        }

        // Wrap per-entry import so one failure doesn't abort the whole file
        if let Err(e) = import_guidance_entry(db, &guidance_file.source_pdf, entry).await {
            warn!(
                "Failed to import guidance for concept {}: {}. Skipping.",
                entry.concept_id, e
            );
        }
    }

    // Rebuild FTS5 index
    sqlx::query("INSERT INTO concept_guidance_fts(concept_guidance_fts) VALUES('rebuild')")
        .execute(db)
        .await?;

    info!(
        "Successfully imported guidance from {}",
        file_path.display()
    );
    Ok(())
}

/// Import a single guidance entry within a transaction
async fn import_guidance_entry(
    db: &SqlitePool,
    source_pdf: &str,
    entry: &GuidanceEntry,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tx = db.begin().await?;

    // Upsert concept_guidance
    let guidance_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO concept_guidance (id, concept_id, source_pdf, source_page, about_en, about_nb) \
         VALUES (?, ?, ?, ?, ?, ?) \
         ON CONFLICT(concept_id) DO UPDATE SET \
         source_pdf = excluded.source_pdf, \
         source_page = excluded.source_page, \
         about_en = excluded.about_en, \
         about_nb = excluded.about_nb, \
         updated_at = datetime('now')",
    )
    .bind(&guidance_id)
    .bind(&entry.concept_id)
    .bind(source_pdf)
    .bind(entry.source_page)
    .bind(&entry.about_en)
    .bind(&entry.about_nb)
    .execute(&mut *tx)
    .await?;

    // Delete existing child rows before reinserting
    sqlx::query("DELETE FROM concept_actions WHERE concept_id = ?")
        .bind(&entry.concept_id)
        .execute(&mut *tx)
        .await?;
    sqlx::query("DELETE FROM concept_transparency_questions WHERE concept_id = ?")
        .bind(&entry.concept_id)
        .execute(&mut *tx)
        .await?;
    sqlx::query("DELETE FROM concept_references WHERE concept_id = ?")
        .bind(&entry.concept_id)
        .execute(&mut *tx)
        .await?;

    // Insert suggested actions — iterate over the longer of en/nb arrays
    let empty_vec = vec![];
    let actions_en = entry.suggested_actions_en.as_deref().unwrap_or(&empty_vec);
    let actions_nb = entry.suggested_actions_nb.as_deref().unwrap_or(&empty_vec);
    let action_count = actions_en.len().max(actions_nb.len());
    for i in 0..action_count {
        let text_en = actions_en.get(i).map(|s| s.as_str());
        let text_nb = actions_nb.get(i).map(|s| s.as_str());
        // action_text_en is NOT NULL in schema, use empty string if only nb present
        let text_en_val = text_en.unwrap_or("");
        sqlx::query(
            "INSERT INTO concept_actions (id, concept_id, action_text_en, action_text_nb, sort_order) \
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(&entry.concept_id)
        .bind(text_en_val)
        .bind(text_nb)
        .bind((i + 1) as i64)
        .execute(&mut *tx)
        .await?;
    }

    // Insert transparency questions — iterate over the longer of en/nb arrays
    let questions_en = entry.transparency_questions_en.as_deref().unwrap_or(&empty_vec);
    let questions_nb = entry.transparency_questions_nb.as_deref().unwrap_or(&empty_vec);
    let question_count = questions_en.len().max(questions_nb.len());
    for i in 0..question_count {
        let text_en = questions_en.get(i).map(|s| s.as_str());
        let text_nb = questions_nb.get(i).map(|s| s.as_str());
        let text_en_val = text_en.unwrap_or("");
        sqlx::query(
            "INSERT INTO concept_transparency_questions (id, concept_id, question_text_en, question_text_nb, sort_order) \
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(&entry.concept_id)
        .bind(text_en_val)
        .bind(text_nb)
        .bind((i + 1) as i64)
        .execute(&mut *tx)
        .await?;
    }

    // Insert references (resources first, then academic)
    let mut sort_order: i64 = 1;

    if let Some(resources) = &entry.resources {
        for resource in resources {
            sqlx::query(
                "INSERT INTO concept_references (id, concept_id, reference_type, title, url, sort_order) \
                 VALUES (?, ?, 'transparency_resource', ?, ?, ?)",
            )
            .bind(Uuid::new_v4().to_string())
            .bind(&entry.concept_id)
            .bind(&resource.title)
            .bind(&resource.url)
            .bind(sort_order)
            .execute(&mut *tx)
            .await?;
            sort_order += 1;
        }
    }

    if let Some(references) = &entry.references {
        for reference in references {
            sqlx::query(
                "INSERT INTO concept_references (id, concept_id, reference_type, title, authors, year, venue, url, sort_order) \
                 VALUES (?, ?, 'academic', ?, ?, ?, ?, ?, ?)",
            )
            .bind(Uuid::new_v4().to_string())
            .bind(&entry.concept_id)
            .bind(&reference.title)
            .bind(&reference.authors)
            .bind(reference.year)
            .bind(&reference.venue)
            .bind(&reference.url)
            .bind(sort_order)
            .execute(&mut *tx)
            .await?;
            sort_order += 1;
        }
    }

    tx.commit().await?;
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
        "nist-sp-800-53.json",
        "nist-rmf.json",
        "eu-ai-act.json",
        "nist-ai-rmf.json",
        "google-saif.json",
        "mitre-atlas.json",
        "nis2.json",
        "dora.json",
        "gdpr.json",
        "cer-directive.json",
        "iso42001.json",
        "iso42005.json",
        "iso23894.json",
        "iso9000.json",
        "iso27000.json",
        "fmn.json",
        "data-centric.json",
        "zero-trust.json",
        "cisa-ztmm.json",
        "nist-ai-genai-profile.json",
        "xai-dataops.json",
        "mitre-attack.json",
        "cve-cwe.json",
    ];

    for file_name in &framework_files {
        let file_path = data_dir.join(file_name);
        if file_path.exists() {
            import_ontology_file(db, &file_path).await?;
        } else {
            warn!("Ontology file not found: {}", file_path.display());
        }
    }

    // Import relationships from all relationships-*.json files
    let mut rel_files: Vec<_> = std::fs::read_dir(data_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name.starts_with("relationships-") && name.ends_with(".json")
        })
        .map(|e| e.path())
        .collect();
    rel_files.sort();

    if rel_files.is_empty() {
        // Fall back to monolithic relationships.json for backwards compatibility
        let relationships_path = data_dir.join("relationships.json");
        if relationships_path.exists() {
            import_relationships(db, &relationships_path).await?;
        } else {
            warn!("No relationship files found in {}", data_dir.display());
        }
    } else {
        info!("Found {} relationship files to import", rel_files.len());
        for rel_path in &rel_files {
            import_relationships(db, rel_path).await?;
        }
    }

    // Scan for *-guidance.json companion files (after frameworks + relationships)
    let mut guidance_entries = tokio::fs::read_dir(data_dir).await?;
    while let Some(dir_entry) = guidance_entries.next_entry().await? {
        let name = dir_entry.file_name().to_string_lossy().to_string();
        if name.ends_with("-guidance.json") {
            if let Err(e) = import_guidance_file(db, &dir_entry.path()).await {
                warn!("Failed to import guidance file {}: {}", name, e);
            }
        }
    }

    info!("Full ontology import completed");
    Ok(())
}
