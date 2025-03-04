use scraper::{ElementRef, Selector};

pub fn extract_element_attribute(
    elm: &ElementRef,
    query: &str,
    attribute: &str,
) -> anyhow::Result<String> {
    elm.select(&Selector::parse(query).map_err(|_| anyhow::anyhow!("Invalid query"))?)
        .next()
        .and_then(|element| {
            element
                .value()
                .attr(attribute)
                .map(|value| value.to_string())
                .clone()
        })
        .ok_or_else(|| anyhow::anyhow!("Element not found"))
}

pub fn extract_text_content(elm: &ElementRef, query: &str) -> anyhow::Result<String> {
    elm.select(&Selector::parse(query).map_err(|_| anyhow::anyhow!("Invalid query"))?)
        .next()
        .and_then(|element| Some(element.text().collect()))
        .ok_or_else(|| anyhow::anyhow!("Element not found"))
}
