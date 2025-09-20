use tantivy::query::{BooleanQuery, Occur, QueryParser, TermSetQuery};
use tantivy::schema::Facet;
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

    let facet_filters = opts
        .facet_filters
        .iter()
        .filter(|f| !f.is_empty() && f.starts_with('/'))
        .collect::<Vec<_>>();

    if !facet_filters.is_empty() {
        let terms: Vec<Term> = facet_filters
            .iter()
            .map(|raw| Facet::from(raw.as_str())) // 入力は "/a/b" 形式を想定
            .map(|facet| Term::from_facet(schema.facet, &facet))
            .collect();
        let facet_query = Box::new(TermSetQuery::new(terms));
        clauses.push((Occur::Must, facet_query));
    }

    Ok(Box::new(BooleanQuery::new(clauses)))
}
