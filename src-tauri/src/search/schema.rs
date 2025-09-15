use tantivy::schema::{
    Field, IndexRecordOption, NumericOptions, Schema, TextFieldIndexing, TextOptions,
};

use super::analyzers::{TOKENIZER_BIGRAM, TOKENIZER_LANG_JA};

pub struct SlideSchema {
    pub schema: Schema,
    pub key: Field,
    pub year: Field,
    pub course: Field,
    pub lecture: Field,
    pub page: Field,
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
        let year = schema_builder.add_u64_field(
            "year",
            NumericOptions::default()
                .set_indexed()
                .set_fast()
                .set_stored(),
        );
        let course = schema_builder.add_text_field(
            "course",
            TextOptions::default()
                .set_indexing_options(
                    TextFieldIndexing::default()
                        .set_tokenizer("raw")
                        .set_index_option(IndexRecordOption::Basic),
                )
                .set_stored(),
        );
        let lecture = schema_builder.add_text_field(
            "lecture",
            TextOptions::default()
                .set_indexing_options(
                    TextFieldIndexing::default()
                        .set_tokenizer("raw")
                        .set_index_option(IndexRecordOption::Basic),
                )
                .set_stored(),
        );
        let page = schema_builder.add_text_field(
            "page",
            TextOptions::default()
                .set_indexing_options(
                    TextFieldIndexing::default()
                        .set_tokenizer("raw")
                        .set_index_option(IndexRecordOption::Basic),
                )
                .set_stored(),
        );
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
            year,
            course,
            lecture,
            page,
            content_raw,
            content_ja,
            content_bi,
        }
    }
}
