use tantivy::snippet::Snippet;

use super::types::HighlightedText;

pub fn extract_highlights(snippet: &Snippet) -> Vec<HighlightedText> {
    fn merge_ranges(ranges: &[std::ops::Range<usize>]) -> Vec<std::ops::Range<usize>> {
        let mut sorted: Vec<_> = ranges.to_vec();
        sorted.sort_by_key(|r| r.start);
        let mut merged: Vec<std::ops::Range<usize>> = Vec::new();
        for r in sorted {
            if let Some(last) = merged.last_mut() {
                if r.start <= last.end {
                    last.end = last.end.max(r.end);
                    continue;
                }
            }
            merged.push(r);
        }
        merged
    }

    let fragment_text = snippet.fragment();
    let mut highlighted_parts = Vec::new();
    let mut current_pos = 0;

    let merged_ranges = merge_ranges(snippet.highlighted());
    for fragment_range in merged_ranges {
        if current_pos < fragment_range.start {
            highlighted_parts.push(HighlightedText {
                text: fragment_text[current_pos..fragment_range.start].to_string(),
                is_highlighted: false,
            });
        }

        highlighted_parts.push(HighlightedText {
            text: fragment_text[fragment_range.clone()].to_string(),
            is_highlighted: true,
        });

        current_pos = fragment_range.end;
    }

    if current_pos < fragment_text.len() {
        highlighted_parts.push(HighlightedText {
            text: fragment_text[current_pos..].to_string(),
            is_highlighted: false,
        });
    }

    highlighted_parts
}
