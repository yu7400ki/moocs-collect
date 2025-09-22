use crate::error::{CollectError, Result};
use scraper::{ElementRef, Selector};

pub fn extract_element_attribute(elm: &ElementRef, query: &str, attribute: &str) -> Result<String> {
    let selector = Selector::parse(query)
        .map_err(|_| CollectError::parse("Invalid CSS selector", Some(query.to_string())))?;

    elm.select(&selector)
        .next()
        .and_then(|element| {
            element
                .value()
                .attr(attribute)
                .map(|value| value.to_string())
        })
        .ok_or_else(|| {
            CollectError::parse(
                "Element or attribute not found",
                Some(format!("query: {query}, attribute: {attribute}")),
            )
        })
}

pub fn extract_text_content(elm: &ElementRef, query: &str) -> Result<String> {
    let selector = Selector::parse(query)
        .map_err(|_| CollectError::parse("Invalid CSS selector", Some(query.to_string())))?;

    elm.select(&selector)
        .next()
        .map(|element| element.text().collect())
        .ok_or_else(|| CollectError::parse("Element not found", Some(format!("query: {query}"))))
}

pub fn parse_selector(query: &str) -> Result<Selector> {
    Selector::parse(query)
        .map_err(|_| CollectError::parse("Invalid CSS selector", Some(query.to_string())))
}
