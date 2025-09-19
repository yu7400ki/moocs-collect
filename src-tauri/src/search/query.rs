use tantivy::query::{BooleanQuery, Occur, QueryParser};
use tantivy::Index;

use super::schema::SlideSchema;

pub fn build_query(
    index: &Index,
    schema: &SlideSchema,
    query_str: &str,
) -> Result<Box<dyn tantivy::query::Query>, tantivy::query::QueryParserError> {
    let mut parser = QueryParser::for_index(index, vec![schema.content_ja, schema.content_bi]);
    parser.set_field_boost(schema.content_ja, 5.0);
    parser.set_field_boost(schema.content_bi, 1.0);
    parser.set_conjunction_by_default();

    let base_query = parser.parse_query(query_str)?;

    let clauses: Vec<(Occur, Box<dyn tantivy::query::Query>)> = vec![(Occur::Must, base_query)];

    Ok(Box::new(BooleanQuery::new(clauses)))
}
