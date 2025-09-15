use std::path::PathBuf;

use tantivy::{Index, IndexWriter, TantivyError};

use super::analyzers::register_analyzers;
use super::schema::SlideSchema;

pub struct IndexManager {
    pub index: Index,
    pub schema: SlideSchema,
}

impl IndexManager {
    pub fn new(index_path: PathBuf) -> Result<Self, TantivyError> {
        let schema = SlideSchema::new();

        let index = if index_path.exists() {
            Index::open_in_dir(&index_path)?
        } else {
            std::fs::create_dir_all(&index_path)?;
            Index::create_in_dir(&index_path, schema.schema.clone())?
        };

        register_analyzers(&index)?;

        Ok(Self { index, schema })
    }

    pub fn writer(&self, heap_size: usize) -> Result<IndexWriter, TantivyError> {
        self.index.writer(heap_size)
    }
}
