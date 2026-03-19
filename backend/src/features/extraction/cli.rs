use std::io::Write;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand, ValueEnum};

use super::extractor::{ExtractionConfig, ExtractionError, OutputFormat, PdfExtractor};
use super::playbook::PlaybookExtractor;
use super::validation;

/// Top-level CLI definition.
#[derive(Parser, Debug)]
#[command(name = "ontology-backend", about = "Risk Management Framework Explorer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Extract structured data from a NIST reference PDF
    ExtractPdf(ExtractPdfArgs),
}

#[derive(clap::Args, Debug)]
pub struct ExtractPdfArgs {
    /// Path to the PDF file to extract
    pub pdf_path: PathBuf,

    /// Extractor type (auto-detected from PDF content if omitted)
    #[arg(long = "type", value_enum)]
    pub extractor_type: Option<ExtractorType>,

    /// Manual page offset override
    #[arg(long)]
    pub page_offset: Option<i32>,

    /// Output file path (stdout if omitted)
    #[arg(long, short)]
    pub output: Option<PathBuf>,

    /// Output format
    #[arg(long, value_enum, default_value = "json")]
    pub format: CliOutputFormat,

    /// Path to ontology JSON for validation
    #[arg(long)]
    pub validate: Option<PathBuf>,

    /// Show detailed extraction progress
    #[arg(long)]
    pub verbose: bool,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ExtractorType {
    Playbook,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum CliOutputFormat {
    Json,
    Markdown,
    Raw,
}

/// Validate that the given path points to a readable PDF file.
/// Returns the canonicalized path on success.
pub fn validate_pdf_path(path: &Path) -> Result<PathBuf, ExtractionError> {
    let canonical = std::fs::canonicalize(path).map_err(|_| {
        ExtractionError::FileNotFound(path.display().to_string())
    })?;

    let meta = std::fs::metadata(&canonical).map_err(|_| {
        ExtractionError::FileNotFound(canonical.display().to_string())
    })?;

    if !meta.is_file() {
        return Err(ExtractionError::InvalidPdf(format!(
            "{} is not a regular file",
            canonical.display()
        )));
    }

    if canonical.extension().and_then(|e| e.to_str()) != Some("pdf") {
        return Err(ExtractionError::InvalidPdf(format!(
            "{} does not have .pdf extension",
            canonical.display()
        )));
    }

    // Check %PDF magic bytes
    let mut file = std::fs::File::open(&canonical)?;
    let mut magic = [0u8; 4];
    std::io::Read::read_exact(&mut file, &mut magic)?;
    if &magic != b"%PDF" {
        return Err(ExtractionError::InvalidPdf(format!(
            "{} does not have PDF magic bytes",
            canonical.display()
        )));
    }

    Ok(canonical)
}

/// Execute the extract-pdf subcommand.
pub fn run_extract(args: ExtractPdfArgs) -> Result<(), ExtractionError> {
    // 1. Validate input path
    let pdf_path = validate_pdf_path(&args.pdf_path)?;

    // 2. Select extractor
    let extractor: Box<dyn PdfExtractor> = match args.extractor_type {
        Some(ExtractorType::Playbook) => Box::new(PlaybookExtractor),
        None => {
            // Auto-detect from PDF content
            let pages = super::extractor::read_pdf_pages(&pdf_path)?;
            let first_pages_text: String = pages
                .iter()
                .take(5)
                .map(|(_, text)| text.as_str())
                .collect::<Vec<_>>()
                .join(" ");

            if first_pages_text.contains("AI Risk Management Framework")
                || first_pages_text.contains("Playbook")
            {
                Box::new(PlaybookExtractor)
            } else {
                return Err(ExtractionError::InvalidPdf(
                    "Could not auto-detect PDF type. Use --type to specify.".to_string(),
                ));
            }
        }
    };

    // 3. Build config
    let output_format = match args.format {
        CliOutputFormat::Json => OutputFormat::Json,
        CliOutputFormat::Markdown => OutputFormat::Markdown,
        CliOutputFormat::Raw => OutputFormat::Raw,
    };

    let ontology_path = args
        .validate
        .as_ref()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "ontology-data/nist-ai-rmf.json".to_string());

    let config = ExtractionConfig {
        page_offset_override: args.page_offset,
        output_format,
        ontology_path,
    };

    // 4. Extract
    if args.verbose {
        eprintln!("Extracting from: {}", pdf_path.display());
        eprintln!("Extractor: {}", extractor.name());
    }

    let result = extractor.extract(&pdf_path, &config)?;

    if args.verbose {
        eprintln!(
            "Extracted {} sections (offset: {}, source: {:?})",
            result.sections.len(),
            result.page_offset_detected,
            result.page_offset_source
        );
    }

    // 5. Optional validation
    let validation_report = if let Some(ref ontology_path) = args.validate {
        let total_pages = super::extractor::read_pdf_pages(&pdf_path)
            .map(|p| p.len())
            .unwrap_or(0);
        Some(validation::validate(&result, ontology_path, total_pages))
    } else {
        None
    };

    // 6. Format output
    let output_text = format_output(&result, &validation_report, &args.format)?;

    // 7. Write output
    if let Some(output_path) = &args.output {
        let parent = output_path.parent().unwrap_or(Path::new("."));
        let mut temp = tempfile::NamedTempFile::new_in(parent)
            .map_err(|e| ExtractionError::IoError(e))?;
        temp.write_all(output_text.as_bytes())?;
        temp.persist(output_path)
            .map_err(|e| ExtractionError::IoError(e.error))?;

        if args.verbose {
            eprintln!("Output written to: {}", output_path.display());
        }
    } else {
        print!("{output_text}");
    }

    Ok(())
}

fn format_output(
    result: &super::extractor::ExtractionResult,
    validation: &Option<validation::ValidationReport>,
    format: &CliOutputFormat,
) -> Result<String, ExtractionError> {
    match format {
        CliOutputFormat::Json => {
            if let Some(report) = validation {
                let combined = serde_json::json!({
                    "extraction": result,
                    "validation": report,
                });
                serde_json::to_string_pretty(&combined)
                    .map_err(|e| ExtractionError::InvalidPdf(e.to_string()))
            } else {
                serde_json::to_string_pretty(result)
                    .map_err(|e| ExtractionError::InvalidPdf(e.to_string()))
            }
        }
        CliOutputFormat::Markdown => {
            let mut md = String::new();
            md.push_str(&format!("# Extraction: {}\n\n", result.framework_id));
            md.push_str(&format!(
                "Source: {} | Sections: {} | Offset: {}\n\n",
                result.source_pdf,
                result.sections.len(),
                result.page_offset_detected
            ));
            for section in &result.sections {
                md.push_str(&format!(
                    "## {} (page {})\n\n{}\n\n---\n\n",
                    section.concept_code, section.logical_page, section.raw_text
                ));
            }
            Ok(md)
        }
        CliOutputFormat::Raw => {
            let mut raw = String::new();
            for section in &result.sections {
                raw.push_str(&format!(
                    "=== {} ===\n{}\n\n",
                    section.concept_code, section.raw_text
                ));
            }
            Ok(raw)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use std::io::Write;

    // -- Argument parsing --

    #[test]
    fn parse_extract_pdf_defaults() {
        let cli = Cli::parse_from(["ontology-backend", "extract-pdf", "/path/to/file.pdf"]);
        match cli.command {
            Some(Commands::ExtractPdf(args)) => {
                assert_eq!(args.pdf_path, PathBuf::from("/path/to/file.pdf"));
                assert!(args.extractor_type.is_none());
                assert!(args.page_offset.is_none());
                assert!(args.output.is_none());
            }
            None => panic!("Expected ExtractPdf command"),
        }
    }

    #[test]
    fn parse_extract_pdf_all_options() {
        let cli = Cli::parse_from([
            "ontology-backend",
            "extract-pdf",
            "/path/to/file.pdf",
            "--type",
            "playbook",
            "--page-offset",
            "4",
            "--output",
            "out.json",
        ]);
        match cli.command {
            Some(Commands::ExtractPdf(args)) => {
                assert!(matches!(args.extractor_type, Some(ExtractorType::Playbook)));
                assert_eq!(args.page_offset, Some(4));
                assert_eq!(args.output, Some(PathBuf::from("out.json")));
            }
            None => panic!("Expected ExtractPdf command"),
        }
    }

    #[test]
    fn parse_extract_pdf_with_validate() {
        let cli = Cli::parse_from([
            "ontology-backend",
            "extract-pdf",
            "/path/to/file.pdf",
            "--validate",
            "/path/to/ontology.json",
        ]);
        match cli.command {
            Some(Commands::ExtractPdf(args)) => {
                assert_eq!(
                    args.validate,
                    Some(PathBuf::from("/path/to/ontology.json"))
                );
            }
            None => panic!("Expected ExtractPdf command"),
        }
    }

    #[test]
    fn no_subcommand_defaults_to_serve() {
        let cli = Cli::parse_from(["ontology-backend"]);
        assert!(cli.command.is_none());
    }

    // -- Input validation --

    #[test]
    fn rejects_nonexistent_path() {
        let result = validate_pdf_path(Path::new("/tmp/nonexistent_test_file_xzy.pdf"));
        assert!(matches!(result, Err(ExtractionError::FileNotFound(_))));
    }

    #[test]
    fn rejects_directory_path() {
        let result = validate_pdf_path(Path::new("/tmp"));
        assert!(matches!(result, Err(ExtractionError::InvalidPdf(_))));
    }

    #[test]
    fn rejects_non_pdf_extension() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        // Rename to .txt
        let txt_path = tmp.path().with_extension("txt");
        std::fs::rename(tmp.path(), &txt_path).unwrap();
        let result = validate_pdf_path(&txt_path);
        assert!(matches!(result, Err(ExtractionError::InvalidPdf(_))));
        let _ = std::fs::remove_file(&txt_path);
    }

    #[test]
    fn rejects_missing_magic_bytes() {
        let dir = tempfile::tempdir().unwrap();
        let pdf_path = dir.path().join("test.pdf");
        let mut f = std::fs::File::create(&pdf_path).unwrap();
        f.write_all(b"not a pdf file").unwrap();
        drop(f);

        let result = validate_pdf_path(&pdf_path);
        assert!(matches!(result, Err(ExtractionError::InvalidPdf(_))));
    }

    #[test]
    fn accepts_valid_pdf_path() {
        let dir = tempfile::tempdir().unwrap();
        let pdf_path = dir.path().join("test.pdf");
        let mut f = std::fs::File::create(&pdf_path).unwrap();
        f.write_all(b"%PDF-1.4 fake content").unwrap();
        drop(f);

        let result = validate_pdf_path(&pdf_path);
        assert!(result.is_ok());
    }

    // -- Error handling --

    #[test]
    fn extraction_error_produces_nonzero_exit() {
        // run_extract with invalid path returns Err
        let args = ExtractPdfArgs {
            pdf_path: PathBuf::from("/tmp/nonexistent.pdf"),
            extractor_type: Some(ExtractorType::Playbook),
            page_offset: None,
            output: None,
            format: CliOutputFormat::Json,
            validate: None,
            verbose: false,
        };
        let result = run_extract(args);
        assert!(result.is_err());
    }

    #[test]
    fn no_partial_output_on_failure() {
        let dir = tempfile::tempdir().unwrap();
        let output_path = dir.path().join("test_output.json");

        let args = ExtractPdfArgs {
            pdf_path: PathBuf::from("/tmp/nonexistent.pdf"),
            extractor_type: Some(ExtractorType::Playbook),
            page_offset: None,
            output: Some(output_path.clone()),
            format: CliOutputFormat::Json,
            validate: None,
            verbose: false,
        };
        let _ = run_extract(args);
        assert!(!output_path.exists(), "Partial output file should not exist");
    }
}
