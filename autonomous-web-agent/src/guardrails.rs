use crate::config::SafetyConfig;
use crate::error::{AgentError, Result};
use regex::Regex;
use std::time::Instant;
use url::Url;

pub struct Guardrails {
    config: SafetyConfig,
    request_count: usize,
    last_request_time: Option<Instant>,
    blocked_patterns: Vec<Regex>,
    pub safety_blocks: Vec<String>,
}

impl Guardrails {
    pub fn new(config: SafetyConfig) -> Self {
        let blocked_patterns = config
            .blocked_content_patterns
            .iter()
            .filter_map(|p| Regex::new(p).ok())
            .collect();

        Self {
            config,
            request_count: 0,
            last_request_time: None,
            blocked_patterns,
            safety_blocks: Vec::new(),
        }
    }

    pub fn check_url(&mut self, url: &Url) -> Result<()> {
        let domain = url.host_str().unwrap_or("");

        if self.config.allowed_domains.is_empty() {
            return Ok(());
        }

        let allowed = self.config.allowed_domains.iter().any(|d| {
            domain == d.as_str() || domain.ends_with(&format!(".{}", d))
        });

        if !allowed {
            let msg = format!("Domain '{}' not in allowlist", domain);
            self.safety_blocks.push(msg.clone());
            return Err(AgentError::SafetyViolation(msg));
        }

        Ok(())
    }

    pub fn check_budget(&mut self) -> Result<()> {
        if self.request_count >= self.config.max_requests {
            let msg = format!(
                "Request budget exhausted ({}/{})",
                self.request_count, self.config.max_requests
            );
            self.safety_blocks.push(msg.clone());
            return Err(AgentError::BudgetExhausted(msg));
        }
        Ok(())
    }

    pub fn check_depth(&mut self, depth: usize) -> Result<()> {
        if depth > self.config.max_depth {
            let msg = format!("Max depth exceeded ({}/{})", depth, self.config.max_depth);
            self.safety_blocks.push(msg.clone());
            return Err(AgentError::SafetyViolation(msg));
        }
        Ok(())
    }

    pub fn check_content(&mut self, body: &str) -> Result<()> {
        for pattern in &self.blocked_patterns {
            if pattern.is_match(body) {
                let msg = format!("Blocked content pattern matched: {}", pattern.as_str());
                self.safety_blocks.push(msg.clone());
                return Err(AgentError::SafetyViolation(msg));
            }
        }
        Ok(())
    }

    pub async fn enforce_rate_limit(&mut self) {
        if let Some(last) = self.last_request_time {
            let elapsed = last.elapsed().as_millis() as u64;
            if elapsed < self.config.rate_limit_ms {
                let delay = self.config.rate_limit_ms - elapsed;
                tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
            }
        }
    }

    pub fn record_request(&mut self) {
        self.request_count += 1;
        self.last_request_time = Some(Instant::now());
    }

    pub fn requests_made(&self) -> usize {
        self.request_count
    }
}
