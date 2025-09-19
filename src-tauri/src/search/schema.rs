use tantivy::schema::{
    FacetOptions, Field, IndexRecordOption, Schema, TextFieldIndexing, TextOptions,
};

use super::analyzers::{TOKENIZER_BIGRAM, TOKENIZER_LANG_JA};

pub struct SlideSchema {
    pub schema: Schema,
    pub key: Field,
    pub facet: Field,
    pub content_raw: Field,
    pub content_ja: Field,
    pub content_bi: Field,
}

impl SlideSchema {
    pub fn new() -> Self {
        let mut schema_builder = Schema::builder();

        let key = schema_builder.add_text_field(
            "key",
            TextOptions::default()
                .set_indexing_options(
                    TextFieldIndexing::default()
                        .set_tokenizer("raw")
                        .set_index_option(IndexRecordOption::Basic),
                )
                .set_stored(),
        );
        let facet = schema_builder.add_facet_field("facet", FacetOptions::default().set_stored());
        let content_raw = schema_builder.add_text_field(
            "content",
            TextOptions::default()
                .set_indexing_options(
                    TextFieldIndexing::default()
                        .set_tokenizer("raw")
                        .set_index_option(IndexRecordOption::Basic),
                )
                .set_stored(),
        );
        let content_ja = schema_builder.add_text_field(
            "content_ja",
            TextOptions::default()
                .set_indexing_options(
                    TextFieldIndexing::default()
                        .set_tokenizer(TOKENIZER_LANG_JA)
                        .set_index_option(IndexRecordOption::WithFreqsAndPositions),
                )
                .set_stored(),
        );
        let content_bi = schema_builder.add_text_field(
            "content_bi",
            TextOptions::default()
                .set_indexing_options(
                    TextFieldIndexing::default()
                        .set_tokenizer(TOKENIZER_BIGRAM)
                        .set_index_option(IndexRecordOption::WithFreqsAndPositions),
                )
                .set_stored(),
        );

        let schema = schema_builder.build();

        Self {
            schema,
            key,
            facet,
            content_raw,
            content_ja,
            content_bi,
        }
    }
}

impl SlideSchema {
    pub fn from_existing(existing: &Schema) -> Option<Self> {
        let key = existing.get_field("key").ok()?;
        let facet = existing.get_field("facet").ok()?;
        let content_raw = existing.get_field("content").ok()?;
        let content_ja = existing.get_field("content_ja").ok()?;
        let content_bi = existing.get_field("content_bi").ok()?;

        Some(Self {
            schema: existing.clone(),
            key,
            facet,
            content_raw,
            content_ja,
            content_bi,
        })
    }
}
