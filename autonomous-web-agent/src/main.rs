use clap::{Parser, ValueEnum};
use colored::Colorize;
use regex::Regex;
use std::collections::HashMap;
use std::time::Instant;
use url::Url;

use autonomous_web_agent::browser::Browser;
use autonomous_web_agent::config::{AgentConfig, SafetyConfig};

use autonomous_web_agent::guardrails::Guardrails;
use autonomous_web_agent::planner::Planner;
use autonomous_web_agent::report::Report;
use autonomous_web_agent::task::{AgentAction, ExtractionGoal, Task};

#[derive(Debug, Clone, ValueEnum)]
enum GoalType {
    Links,
    Text,
    Structured,
}

#[derive(Parser, Debug)]
#[command(name = "autonomous-web-agent")]
#[command(about = "An autonomous web agent with safety guardrails")]
struct Cli {
    /// Seed URL to start browsing from
    #[arg(short, long)]
    url: Option<String>,

    /// Allowed domains (comma-separated). If empty, all domains allowed.
    #[arg(short, long, value_delimiter = ',')]
    domains: Vec<String>,

    /// Maximum crawl depth
    #[arg(short = 'D', long, default_value = "2")]
    max_depth: usize,

    /// Maximum number of HTTP requests
    #[arg(short = 'R', long, default_value = "15")]
    max_requests: usize,

    /// Rate limit in milliseconds between requests
    #[arg(long, default_value = "500")]
    rate_limit: u64,

    /// Extraction goal type
    #[arg(short, long, value_enum, default_value = "text")]
    goal: GoalType,

    /// CSS selectors for extraction (comma-separated)
    #[arg(short, long, value_delimiter = ',')]
    selectors: Vec<String>,

    /// Regex patterns for links to follow (comma-separated)
    #[arg(short, long, value_delimiter = ',')]
    patterns: Vec<String>,

    /// Output results as JSON
    #[arg(long)]
    json: bool,

    /// Run built-in demo mode
    #[arg(long)]
    demo: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    println!("{}", "🤖 Autonomous Web Agent".cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━".cyan());

    if cli.demo {
        run_demo().await;
        return;
    }

    let seed_url = match &cli.url {
        Some(u) => match Url::parse(u) {
            Ok(url) => url,
            Err(e) => {
                eprintln!("{} Invalid URL: {}", "Error:".red().bold(), e);
                std::process::exit(1);
            }
        },
        None => {
            eprintln!(
                "{} Provide --url or use --demo mode",
                "Error:".red().bold()
            );
            std::process::exit(1);
        }
    };

    let safety = SafetyConfig {
        allowed_domains: cli.domains,
        max_depth: cli.max_depth,
        max_requests: cli.max_requests,
        rate_limit_ms: cli.rate_limit,
        blocked_content_patterns: vec![],
    };

    let config = AgentConfig {
        safety: safety.clone(),
        ..Default::default()
    };

    let goal = match cli.goal {
        GoalType::Links => ExtractionGoal::CollectLinks,
        GoalType::Text => {
            let selectors = if cli.selectors.is_empty() {
                vec![
                    "h1".to_string(),
                    "h2".to_string(),
                    "h3".to_string(),
                    "p".to_string(),
                ]
            } else {
                cli.selectors
            };
            ExtractionGoal::ExtractText { selectors }
        }
        GoalType::Structured => {
            let fields: HashMap<String, String> = cli
                .selectors
                .iter()
                .filter_map(|s| {
                    let parts: Vec<&str> = s.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        Some((parts[0].to_string(), parts[1].to_string()))
                    } else {
                        None
                    }
                })
                .collect();
            ExtractionGoal::ExtractStructured { fields }
        }
    };

    let link_patterns: Vec<Regex> = cli
        .patterns
        .iter()
        .filter_map(|p| Regex::new(p).ok())
        .collect();

    let task = Task {
        name: format!("Crawl {}", seed_url.host_str().unwrap_or("unknown")),
        seed_urls: vec![seed_url],
        goal,
        link_follow_patterns: link_patterns,
    };

    let report = run_agent(task, config, safety).await;

    if cli.json {
        println!("{}", report.to_json());
    } else {
        report.print_summary();
    }
}

async fn run_agent(task: Task, config: AgentConfig, safety: SafetyConfig) -> Report {
    let task_name = task.name.clone();
    let start = Instant::now();

    let browser = match Browser::new(&config) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{} Failed to create browser: {}", "Error:".red().bold(), e);
            return Report {
                task_name,
                items: vec![],
                pages_visited: 0,
                pages_skipped: 0,
                safety_blocks: vec![format!("Browser init failed: {}", e)],
                duration: start.elapsed(),
            };
        }
    };

    let mut guardrails = Guardrails::new(safety);
    let mut planner = Planner::new(task);

    loop {
        let action = planner.next_action();

        match action {
            AgentAction::Visit(url, depth) => {
                // Check all guardrails before proceeding
                if let Err(e) = guardrails.check_budget() {
                    println!("  {} {}", "⛔".red(), e);
                    break;
                }

                if let Err(e) = guardrails.check_depth(depth) {
                    println!("  {} {}", "⏭️ ".yellow(), e);
                    continue;
                }

                if let Err(e) = guardrails.check_url(&url) {
                    println!("  {} {}", "🚫".yellow(), e);
                    continue;
                }

                // Rate limit
                guardrails.enforce_rate_limit().await;

                println!(
                    "  {} [depth={}] {}",
                    "→".blue(),
                    depth,
                    truncate_url(&url, 80)
                );

                // Fetch page
                let page = match browser.fetch(&url).await {
                    Ok(p) => p,
                    Err(e) => {
                        println!("    {} Fetch failed: {}", "✗".red(), e);
                        guardrails.record_request();
                        continue;
                    }
                };

                guardrails.record_request();

                // Check content safety
                if let Err(e) = guardrails.check_content(&page.body) {
                    println!("    {} {}", "🚫".red(), e);
                    continue;
                }

                if page.status != 200 {
                    println!("    {} HTTP {}", "⚠".yellow(), page.status);
                    continue;
                }

                // Parse and process
                let parsed = Browser::parse(&page);
                let link_count = parsed.links.len();
                let title = parsed
                    .title
                    .as_deref()
                    .unwrap_or("(no title)")
                    .to_string();

                planner.process_page(&parsed, depth);

                let new_items = planner.results.len();
                println!(
                    "    {} \"{}\" | {} links | {} items total",
                    "✓".green(),
                    truncate_str(&title, 50),
                    link_count,
                    new_items
                );
            }
            AgentAction::Done => {
                println!("\n  {} Agent finished - frontier exhausted", "✓".green().bold());
                break;
            }
        }
    }

    Report {
        task_name,
        items: planner.results,
        pages_visited: planner.pages_visited,
        pages_skipped: planner.pages_skipped,
        safety_blocks: guardrails.safety_blocks,
        duration: start.elapsed(),
    }
}

async fn run_demo() {
    println!("\n{}", "Running demo: Extract headings from example.com".yellow());
    println!(
        "{}",
        "Safety: domain-locked to example.com, max 5 requests, depth 1\n".dimmed()
    );

    let safety = SafetyConfig {
        allowed_domains: vec!["example.com".to_string()],
        max_depth: 1,
        max_requests: 5,
        rate_limit_ms: 500,
        blocked_content_patterns: vec![],
    };

    let config = AgentConfig {
        safety: safety.clone(),
        ..Default::default()
    };

    let task = Task {
        name: "Demo: example.com headings".to_string(),
        seed_urls: vec![Url::parse("https://example.com").unwrap()],
        goal: ExtractionGoal::ExtractText {
            selectors: vec!["h1".to_string(), "h2".to_string(), "p".to_string()],
        },
        link_follow_patterns: vec![],
    };

    let report = run_agent(task, config, safety).await;
    report.print_summary();
}

fn truncate_url(url: &Url, max_len: usize) -> String {
    let s = url.as_str();
    truncate_str(s, max_len)
}

fn truncate_str(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}
