//! LLM Provider trait and error definitions.
//!
//! Defines the interface for LLM providers and common error types.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use async_trait::async_trait;
use crate::agents::event::LlmMessage;

/// Trait for LLM providers.
///
/// Implementors of this trait can generate content based on a given prompt.
#[async_trait]
pub trait LlmProvider {
    /// Generates content based on the provided prompt.
    ///
    /// # Errors
    ///
    /// Returns an `AgentError` if content generation fails.
    async fn generate_content(&self, prompt: &str) -> Result<String, AgentError>;

    /// Logs the prompt and response to a transcript file in JSONL format.
    /// Each call appends two separate JSON objects (user and model) as new lines.
    /// The file is created if it does not already exist.
    /// # Errors
    ///
    /// Returns an `AgentError` if writing to the transcript file fails.
    async fn transcript(&self, prompt: LlmMessage, response: LlmMessage, output_dir: String) -> Result<(), AgentError> {
        let path = format!("{}/transcript.jsonl", output_dir);

        // open file for append, create if needed
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| AgentError::IoError(format!("Failed to open transcript file: {}", e)))?;

        // write user entry
        let user_line = serde_json::to_string(&serde_json::json!({
            "role": "user",
            "parts": [{"text": prompt.text}],
            "timestamp": prompt.timestamp,
        }))
        .map_err(|e| AgentError::GeneralError(format!("Failed to serialize transcript entry: {}", e)))?;
        file.write_all(user_line.as_bytes())
            .map_err(|e| AgentError::IoError(format!("Failed to write transcript entry: {}", e)))?;
        file.write_all(b"\n")
            .map_err(|e| AgentError::IoError(format!("Failed to write newline: {}", e)))?;

        // write model entry
        let model_line = serde_json::to_string(&serde_json::json!({
            "role": "model",
            "parts": [{"text": response.text}],
            "timestamp": response.timestamp,
        }))
        .map_err(|e| AgentError::GeneralError(format!("Failed to serialize transcript entry: {}", e)))?;
        file.write_all(model_line.as_bytes())
            .map_err(|e| AgentError::IoError(format!("Failed to write transcript entry: {}", e)))?;
        file.write_all(b"\n")
            .map_err(|e| AgentError::IoError(format!("Failed to write newline: {}", e)))?;

        Ok(())
    }

    /// Reads the last `count` entries from a JSONL transcript file, returning
    /// them as `LlmMessage` structures. Each line of the file is parsed as a
    /// separate JSON object. The messages are prefixed with their role (e.g.
    /// "User:" or "Model:") to maintain compatibility with how the history is
    /// later used.
    async fn get_transcript_history(&self, count: usize, output_dir: String) -> Result<Vec<LlmMessage>, AgentError> {
        let path = format!("{}/transcript.jsonl", output_dir);

        if !Path::new(&path).exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&path)
            .map_err(|e| AgentError::IoError(format!("Failed to read transcript file: {}", e)))?;

        let mut messages: Vec<LlmMessage> = Vec::new();
        for line in content.lines().rev() {
            if line.trim().is_empty() {
                continue;
            }
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
                let role = v["role"].as_str().unwrap_or("");
                let text = v["parts"]
                    .get(0)
                    .and_then(|p| p["text"].as_str())
                    .unwrap_or("")
                    .to_string();
                let timestamp = v["timestamp"].as_u64().unwrap_or(0);
                let prefixed = match role {
                    "user" => format!("User: {}", text),
                    "model" => format!("Model: {}", text),
                    _ => text,
                };
                messages.push(LlmMessage { text: prefixed, timestamp });

                if messages.len() >= count {
                    break;
                }
            }
        }

        Ok(messages)
    }
}

/// Common error types for the agent system.
#[derive(Debug)]
pub enum AgentError {
    /// A general error with a descriptive message.
    GeneralError(String),
    /// An I/O error, typically from user input.
    IoError(String),
}
