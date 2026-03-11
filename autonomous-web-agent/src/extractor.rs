use crate::browser::ParsedPage;
use crate::task::ExtractionGoal;
use scraper::Selector;
use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct ExtractedItem {
    pub source_url: String,
    pub field: String,
    pub value: String,
}

pub fn extract(page: &ParsedPage, goal: &ExtractionGoal) -> Vec<ExtractedItem> {
    match goal {
        ExtractionGoal::CollectLinks => extract_links(page),
        ExtractionGoal::ExtractText { selectors } => extract_text(page, selectors),
        ExtractionGoal::ExtractStructured { fields } => extract_structured(page, fields),
    }
}

fn extract_links(page: &ParsedPage) -> Vec<ExtractedItem> {
    page.links
        .iter()
        .map(|link| ExtractedItem {
            source_url: page.url.to_string(),
            field: "link".to_string(),
            value: link.to_string(),
        })
        .collect()
}

fn extract_text(page: &ParsedPage, selectors: &[String]) -> Vec<ExtractedItem> {
    let mut items = Vec::new();

    for selector_str in selectors {
        let selector = match Selector::parse(selector_str) {
            Ok(s) => s,
            Err(_) => continue,
        };

        for element in page.document.select(&selector) {
            let text: String = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
            if !text.is_empty() {
                items.push(ExtractedItem {
                    source_url: page.url.to_string(),
                    field: selector_str.clone(),
                    value: text,
                });
            }
        }
    }

    items
}

fn extract_structured(
    page: &ParsedPage,
    fields: &std::collections::HashMap<String, String>,
) -> Vec<ExtractedItem> {
    let mut items = Vec::new();

    for (field_name, selector_str) in fields {
        let selector = match Selector::parse(selector_str) {
            Ok(s) => s,
            Err(_) => continue,
        };

        for element in page.document.select(&selector) {
            let text: String = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
            if !text.is_empty() {
                items.push(ExtractedItem {
                    source_url: page.url.to_string(),
                    field: field_name.clone(),
                    value: text,
                });
            }
        }
    }

    items
}
