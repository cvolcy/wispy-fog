use colored::*;
use serde_json::Value;
use std::fs;
use std::path::Path;

/// Reads a JSONL transcript file from the given output directory and prints
/// a human-readable, colored representation of each entry to stdout.
pub fn inspect_transcript(output_dir: &str) {
    let path = format!("{}/transcript.jsonl", output_dir);
    if !Path::new(&path).exists() {
        println!("No transcript file found at {}", path);
        return;
    }

    let content = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to read transcript file: {}", e);
            return;
        }
    };

    if content.trim().is_empty() {
        println!("Transcript file is empty");
        return;
    }

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Ok(entry) = serde_json::from_str::<Value>(line) {
            let role = entry["role"].as_str().unwrap_or("unknown");
            let text = entry["parts"]
                .get(0)
                .and_then(|p| p["text"].as_str())
                .unwrap_or("");

            let label = match role {
                "user" => "USER".blue().bold(),
                "model" => "MODEL".green().bold(),
                _ => role.normal(),
            };

            println!("{}: {}", label, text);
            println!();
        }
    }
}
