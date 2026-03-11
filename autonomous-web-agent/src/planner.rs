use crate::browser::ParsedPage;
use crate::extractor::{self, ExtractedItem};
use crate::task::{AgentAction, Task};
use std::collections::{HashSet, VecDeque};
use url::Url;

pub struct Planner {
    task: Task,
    visited: HashSet<String>,
    frontier: VecDeque<(Url, usize)>,
    pub results: Vec<ExtractedItem>,
    pub pages_visited: usize,
    pub pages_skipped: usize,
}

impl Planner {
    pub fn new(task: Task) -> Self {
        let mut frontier = VecDeque::new();
        for url in &task.seed_urls {
            frontier.push_back((url.clone(), 0));
        }

        Self {
            task,
            visited: HashSet::new(),
            frontier,
            results: Vec::new(),
            pages_visited: 0,
            pages_skipped: 0,
        }
    }

    pub fn next_action(&mut self) -> AgentAction {
        while let Some((url, depth)) = self.frontier.pop_front() {
            let url_str = url.to_string();
            if self.visited.contains(&url_str) {
                self.pages_skipped += 1;
                continue;
            }
            self.visited.insert(url_str);
            return AgentAction::Visit(url, depth);
        }
        AgentAction::Done
    }

    pub fn process_page(&mut self, page: &ParsedPage, depth: usize) {
        self.pages_visited += 1;

        let items = extractor::extract(page, &self.task.goal);
        self.results.extend(items);

        for link in &page.links {
            let link_str = link.to_string();
            if self.visited.contains(&link_str) {
                continue;
            }

            let should_follow = self.task.link_follow_patterns.is_empty()
                || self
                    .task
                    .link_follow_patterns
                    .iter()
                    .any(|p| p.is_match(link.as_str()));

            if should_follow {
                self.frontier.push_back((link.clone(), depth + 1));
            }
        }
    }
}
