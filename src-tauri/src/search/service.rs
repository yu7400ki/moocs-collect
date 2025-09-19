use std::path::PathBuf;

use collect::domain::models::SlideContent;
use tantivy::collector::TopDocs;
use tantivy::doc;
use tantivy::schema::{Facet, Value};
use tantivy::snippet::{Snippet, SnippetGenerator};
use tantivy::{IndexReader, ReloadPolicy, TantivyDocument, Term};

use tauri::{AppHandle, Manager};

use super::highlighter::extract_highlights;
use super::index::IndexManager;
use super::query::build_query;
use super::schema::SlideSchema;
use super::types::{HighlightedText, SearchOptions, SearchResult};

#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("Tantivy error: {0}")]
    Tantivy(#[from] tantivy::TantivyError),
    #[error("Query parser error: {0}")]
    QueryParser(#[from] tantivy::query::QueryParserError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl serde::Serialize for SearchError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub struct SearchService {
    pub index_manager: IndexManager,
    reader: IndexReader,
}

impl SearchService {
    pub fn with_index_path(index_path: PathBuf) -> Result<Self, SearchError> {
        let index_manager = IndexManager::new(index_path)?;
        let reader = index_manager
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;
        Ok(Self {
            index_manager,
            reader,
        })
    }

    pub fn from_app_handle(app: &AppHandle) -> Result<Self, SearchError> {
        let app_data_dir = app.path().app_data_dir().map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to get app data dir: {}", e),
            )
        })?;
        let index_path = app_data_dir.join("search_index");
        Self::with_index_path(index_path)
    }

    pub async fn index_slide_content(
        &self,
        slide_content: &SlideContent,
        index: usize,
    ) -> Result<(), SearchError> {
        let mut index_writer = self.index_manager.writer(50_000_000)?;
        let schema = &self.index_manager.schema;

        let text = slide_content.get_texts().join("\n");

        let page_key = &slide_content.page_key;
        let facet_path = format!("/{}/{}", page_key, index);
        let facet_value = Facet::from(&facet_path);
        index_writer.delete_term(Term::from_facet(schema.facet, &facet_value));
        let doc = doc!(
            schema.key => slide_content.page_key.to_string(),
            schema.facet => facet_value,
            schema.content_raw => text.to_string(),
            schema.content_ja => text.to_string(),
            schema.content_bi => text.to_string(),
        );
        index_writer.add_document(doc)?;
        index_writer.commit()?;
        Ok(())
    }

    pub async fn search_slides(
        &self,
        query_str: &str,
        opts: &SearchOptions,
    ) -> Result<Vec<SearchResult>, SearchError> {
        let searcher = self.reader.searcher();
        let schema: &SlideSchema = &self.index_manager.schema;

        let parsed_query = build_query(&self.index_manager.index, schema, query_str)?;

        let snippet_generator_ja =
            SnippetGenerator::create(&searcher, &*parsed_query, schema.content_ja)?;
        let snippet_generator_bi =
            SnippetGenerator::create(&searcher, &*parsed_query, schema.content_bi)?;

        let top_docs = searcher.search(&parsed_query, &TopDocs::with_limit(opts.limit))?;

        let mut results = Vec::new();
        for (score, doc_address) in top_docs {
            let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;

            let key = retrieved_doc
                .get_first(schema.key)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let facet = retrieved_doc
                .get_first(schema.facet)
                .and_then(|v| v.as_facet())
                .map(|encoded| {
                    let facet = Facet::from_encoded(encoded.into()).unwrap();
                    facet.to_path_string()
                })
                .unwrap_or_default();
            let content_raw = retrieved_doc
                .get_first(schema.content_raw)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let snippet_ja = snippet_generator_ja.snippet_from_doc(&retrieved_doc);
            let use_bi = snippet_ja.fragment().trim().is_empty();
            let snippet: Snippet = if use_bi {
                snippet_generator_bi.snippet_from_doc(&retrieved_doc)
            } else {
                snippet_ja
            };

            let highlighted_content: Vec<HighlightedText> = extract_highlights(&snippet);
            let mut content_snippet = snippet.fragment().to_string();
            if content_snippet.trim().is_empty() {
                content_snippet = if content_raw.chars().count() > 200 {
                    let truncated: String = content_raw.chars().take(200).collect();
                    format!("{}...", truncated)
                } else {
                    content_raw
                };
            }

            results.push(SearchResult {
                page_key: key,
                facet,
                content_snippet,
                highlighted_content,
                score,
            });
        }

        Ok(results)
    }
}
