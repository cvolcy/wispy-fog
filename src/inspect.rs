use colored::*;
use serde_json::Value;
use std::fs;
use std::path::Path;

/// Reads the transcript file from the given output directory and prints
/// a human-readable, colored representation to stdout.
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

    let data: Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to parse transcript JSON: {}", e);
            return;
        }
    };

    if let Some(array) = data["contents"].as_array() {
        for entry in array {
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
    } else {
        println!("Transcript has no contents array");
    }
}
