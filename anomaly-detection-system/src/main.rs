use std::io::{self, BufRead};
use std::path::PathBuf;

use clap::Parser;

use anomaly_detection_system::detector::{Detector, DetectorConfig};
use anomaly_detection_system::parser;
use anomaly_detection_system::reporter;

#[derive(Parser)]
#[command(
    name = "anomaly-detect",
    about = "Real-time network traffic anomaly detection using Isolation Forest",
    version
)]
struct Cli {
    /// Number of isolation trees in the forest
    #[arg(long, default_value_t = 100)]
    trees: usize,

    /// Anomaly score threshold (0.0 - 1.0). Higher = fewer detections.
    #[arg(long, default_value_t = 0.65)]
    threshold: f64,

    /// Number of events to buffer before initial training
    #[arg(long, default_value_t = 256)]
    buffer_size: usize,

    /// Retrain the model every N events
    #[arg(long, default_value_t = 1000)]
    retrain_interval: usize,

    /// Path to write JSON anomaly reports
    #[arg(long, short, default_value = "anomalies.json")]
    output: PathBuf,

    /// Print status every N events (0 to disable)
    #[arg(long, default_value_t = 100)]
    status_interval: usize,
}

fn main() {
    let cli = Cli::parse();

    let config = DetectorConfig {
        n_trees: cli.trees,
        buffer_size: cli.buffer_size,
        threshold: cli.threshold,
        retrain_interval: cli.retrain_interval,
    };

    let mut detector = Detector::new(config);

    eprintln!("Anomaly Detection System — Isolation Forest");
    eprintln!("============================================");
    eprintln!("Trees: {} | Threshold: {} | Buffer: {} | Retrain every: {}",
        cli.trees, cli.threshold, cli.buffer_size, cli.retrain_interval);
    eprintln!("Output: {}", cli.output.display());
    eprintln!("Reading from stdin... (pipe network traffic data)");
    eprintln!();

    let stdin = io::stdin();
    let mut line_count = 0;

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("[WARN] Failed to read line: {}", e);
                continue;
            }
        };

        let event = match parser::parse_line(&line) {
            Some(e) => e,
            None => continue, // Skip comments, empty lines, malformed data
        };

        if let Some(report) = detector.process(&event) {
            reporter::print_anomaly(&report);
            if let Err(e) = reporter::write_json(&report, &cli.output) {
                eprintln!("[WARN] Failed to write JSON: {}", e);
            }
        }

        line_count += 1;

        // Print status update
        if cli.status_interval > 0 && line_count % cli.status_interval == 0 {
            reporter::print_status(
                detector.total_events(),
                detector.total_anomalies(),
                detector.is_trained(),
            );
        }

        // Notify when training completes
        if detector.total_events() == cli.buffer_size && detector.is_trained() {
            eprintln!("[INFO] Model trained on {} events. Now detecting anomalies.", cli.buffer_size);
        }
    }

    reporter::print_summary(detector.total_events(), detector.total_anomalies());
}
