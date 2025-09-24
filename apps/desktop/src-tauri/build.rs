use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use sha2::{Digest, Sha256};

fn main() {
    generate_migrations().expect("failed to generate migrations");
    tauri_build::build();
}

#[derive(Deserialize)]
struct Journal {
    entries: Vec<JournalEntry>,
}

#[derive(Deserialize)]
struct JournalEntry {
    when: i64,
    tag: String,
}

struct MigrationData {
    folder_millis: i64,
    tag: String,
    hash: String,
    statements: Vec<String>,
}

fn generate_migrations() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let migrations_dir = manifest_dir.join("migrations");
    let journal_path = migrations_dir.join("meta").join("_journal.json");

    println!("cargo:rerun-if-changed={}", journal_path.display());

    let journal_content = fs::read_to_string(&journal_path)?;
    let journal: Journal = serde_json::from_str(&journal_content)?;

    let mut migrations = Vec::with_capacity(journal.entries.len());

    for entry in journal.entries {
        let sql_path = migrations_dir.join(format!("{}.sql", entry.tag));
        println!("cargo:rerun-if-changed={}", sql_path.display());
        let sql_content = fs::read_to_string(&sql_path)?;
        let statements = split_statements(&sql_content);
        let hash = compute_hash(&sql_content);
        migrations.push(MigrationData {
            folder_millis: entry.when,
            tag: entry.tag,
            hash,
            statements,
        });
    }

    migrations.sort_by_key(|migration| migration.folder_millis);

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let out_path = out_dir.join("migrations.rs");
    write_migrations_file(&out_path, &migrations)?;

    Ok(())
}

fn split_statements(sql: &str) -> Vec<String> {
    sql.split("--> statement-breakpoint")
        .map(|segment| segment.trim())
        .filter(|segment| !segment.is_empty())
        .map(|segment| segment.trim_end_matches(';').trim().to_owned())
        .collect()
}

fn compute_hash(sql: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(sql.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn write_migrations_file(
    path: &Path,
    migrations: &[MigrationData],
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    writeln!(file, "pub(crate) static MIGRATIONS: &[Migration] = &[")?;
    for migration in migrations {
        writeln!(file, "    Migration {{")?;
        writeln!(file, "        tag: {},", quote(&migration.tag))?;
        writeln!(file, "        folder_millis: {},", migration.folder_millis)?;
        writeln!(file, "        hash: {},", quote(&migration.hash))?;
        writeln!(file, "        statements: &[")?;
        for statement in &migration.statements {
            writeln!(file, "            {},", quote(statement))?;
        }
        writeln!(file, "        ],")?;
        writeln!(file, "    }},")?;
    }
    writeln!(file, "];")?;

    Ok(())
}

fn quote(input: &str) -> String {
    serde_json::to_string(input).expect("failed to serialize string literal")
}
