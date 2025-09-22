use std::path::PathBuf;

use tantivy::{Index, IndexWriter, TantivyError};

use super::analyzers::register_analyzers;
use super::schema::SlideSchema;

pub struct IndexManager {
    pub index: Index,
    pub schema: SlideSchema,
    pub index_path: PathBuf,
}

impl IndexManager {
    pub fn new(index_path: PathBuf) -> Result<Self, TantivyError> {
        if index_path.exists() {
            let index = Index::open_in_dir(&index_path)?;
            if let Some(schema) = SlideSchema::from_existing(&index.schema()) {
                register_analyzers(&index)?;
                return Ok(Self {
                    index,
                    schema,
                    index_path,
                });
            }

            log::warn!(
                "Search index schema mismatch detected. Recreating index at {}",
                index_path.display()
            );
            std::fs::remove_dir_all(&index_path)?;
        }

        let schema = SlideSchema::new();
        std::fs::create_dir_all(&index_path)?;
        let index = Index::create_in_dir(&index_path, schema.schema.clone())?;

        register_analyzers(&index)?;

        Ok(Self {
            index,
            schema,
            index_path,
        })
    }

    pub fn writer(&self, heap_size: usize) -> Result<IndexWriter, TantivyError> {
        self.index.writer(heap_size)
    }

    pub fn purge_index(&self) -> Result<(), TantivyError> {
        if self.index_path.exists() {
            std::fs::remove_dir_all(&self.index_path)?;
        }
        Ok(())
    }
}
