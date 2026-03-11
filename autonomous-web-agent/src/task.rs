use regex::Regex;
use std::collections::HashMap;
use url::Url;

#[derive(Debug, Clone)]
pub enum ExtractionGoal {
    CollectLinks,
    ExtractText { selectors: Vec<String> },
    ExtractStructured { fields: HashMap<String, String> },
}

#[derive(Debug)]
pub struct Task {
    pub name: String,
    pub seed_urls: Vec<Url>,
    pub goal: ExtractionGoal,
    pub link_follow_patterns: Vec<Regex>,
}

#[derive(Debug)]
pub enum AgentAction {
    Visit(Url, usize),
    Done,
}
