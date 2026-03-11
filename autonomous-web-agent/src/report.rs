use crate::extractor::ExtractedItem;
use colored::Colorize;
use serde::Serialize;
use std::time::Duration;

#[derive(Debug, Serialize)]
pub struct Report {
    pub task_name: String,
    pub items: Vec<ExtractedItem>,
    pub pages_visited: usize,
    pub pages_skipped: usize,
    pub safety_blocks: Vec<String>,
    #[serde(skip)]
    pub duration: Duration,
}

impl Report {
    pub fn print_summary(&self) {
        println!("\n{}", "═══════════════════════════════════════".cyan());
        println!("{}", format!("  Agent Report: {}", self.task_name).cyan().bold());
        println!("{}", "═══════════════════════════════════════".cyan());

        println!(
            "\n{}  Pages visited: {}",
            "📊".white(),
            self.pages_visited.to_string().green()
        );
        println!(
            "{}  Pages skipped: {}",
            "⏭️ ".white(),
            self.pages_skipped.to_string().yellow()
        );
        println!(
            "{}  Items extracted: {}",
            "📦".white(),
            self.items.len().to_string().green()
        );
        println!(
            "{}  Safety blocks: {}",
            "🛡️ ".white(),
            self.safety_blocks.len().to_string().red()
        );
        println!(
            "{}  Duration: {:.2}s",
            "⏱️ ".white(),
            self.duration.as_secs_f64()
        );

        if !self.safety_blocks.is_empty() {
            println!("\n{}", "Safety Events:".red().bold());
            for block in &self.safety_blocks {
                println!("  {} {}", "▸".red(), block);
            }
        }

        if !self.items.is_empty() {
            println!("\n{}", "Extracted Data:".green().bold());
            let display_count = self.items.len().min(20);
            for item in self.items.iter().take(display_count) {
                println!(
                    "  {} [{}] {}",
                    "▸".green(),
                    item.field.dimmed(),
                    truncate(&item.value, 100)
                );
            }
            if self.items.len() > display_count {
                println!(
                    "  {} ... and {} more items",
                    "▸".dimmed(),
                    self.items.len() - display_count
                );
            }
        }

        println!("\n{}", "═══════════════════════════════════════".cyan());
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}
