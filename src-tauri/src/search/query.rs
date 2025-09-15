use tantivy::query::{BooleanQuery, Occur, QueryParser, TermQuery};
use tantivy::schema::IndexRecordOption;
use tantivy::{Index, Term};

use super::schema::SlideSchema;

use super::types::SearchOptions;

pub fn build_query(
    index: &Index,
    schema: &SlideSchema,
    query_str: &str,
    opts: &SearchOptions,
) -> Result<Box<dyn tantivy::query::Query>, tantivy::query::QueryParserError> {
    let mut parser = QueryParser::for_index(index, vec![schema.content_ja, schema.content_bi]);
    parser.set_field_boost(schema.content_ja, 5.0);
    parser.set_field_boost(schema.content_bi, 1.0);
    parser.set_conjunction_by_default();

    let base_query = parser.parse_query(query_str)?;

    let mut clauses: Vec<(Occur, Box<dyn tantivy::query::Query>)> = vec![(Occur::Must, base_query)];

    if let Some(year) = opts.year {
        let term = Term::from_field_u64(schema.year, year as u64);
        let query: Box<dyn tantivy::query::Query> =
            Box::new(TermQuery::new(term, IndexRecordOption::Basic));
        clauses.push((Occur::Must, query));
    }

    if !opts.courses.is_empty() {
        let shoulds: Vec<(Occur, Box<dyn tantivy::query::Query>)> = opts
            .courses
            .iter()
            .map(|c| {
                let term = Term::from_field_text(schema.course, c);
                let tq: Box<dyn tantivy::query::Query> =
                    Box::new(TermQuery::new(term, IndexRecordOption::Basic));
                (Occur::Should, tq)
            })
            .collect();

        let query = Box::new(BooleanQuery::new(shoulds));
        clauses.push((Occur::Must, query));
    }

    Ok(Box::new(BooleanQuery::new(clauses)))
}
