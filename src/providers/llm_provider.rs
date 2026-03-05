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

    /// Logs the prompt and response to a transcript file.
    /// Initializes the file if it doesn't exist and appends entries to the contents array.
    /// # Errors
    ///
    /// Returns an `AgentError` if reading or writing to the transcript file fails.
    async fn transcript(&self, prompt: LlmMessage, response: LlmMessage, output_dir: String) -> Result<(), AgentError> {
        let path = format!("{}/transcript.jsonl", output_dir);

        // Read existing content or initialize with empty contents array
        let mut data = if Path::new(&path).exists() {
            let content = fs::read_to_string(&path)
                .map_err(|e| AgentError::IoError(format!("Failed to read transcript file: {}", e)))?;

            if content.is_empty() {
                serde_json::json!({ "contents": [] })
            } else {
                serde_json::from_str(&content)
                    .map_err(|e| AgentError::GeneralError(format!("Failed to parse transcript file: {}", e)))?
            }
        } else {
            serde_json::json!({ "contents": [] })
        };

        // Append user prompt to contents array
        data["contents"]
            .as_array_mut()
            .ok_or_else(|| AgentError::GeneralError("Invalid transcript structure".to_string()))?
            .push(serde_json::json!({
                "role": "user",
                "parts": [{"text": prompt.text}]
            }));

        // Append model response to contents array
        data["contents"]
            .as_array_mut()
            .ok_or_else(|| AgentError::GeneralError("Invalid transcript structure".to_string()))?
            .push(serde_json::json!({
                "role": "model",
                "parts": [{"text": response.text}]
            }));

        // Write updated content back to file
        let serialized = serde_json::to_string(&data)
            .map_err(|e| AgentError::GeneralError(format!("Failed to serialize transcript: {}", e)))?;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .map_err(|e| AgentError::IoError(format!("Failed to open transcript file: {}", e)))?;

        file.write_all(serialized.as_bytes())
            .map_err(|e| AgentError::IoError(format!("Failed to write transcript file: {}", e)))?;

        Ok(())
    }

    async fn get_transcript_history(&self, count: usize, output_dir: String) -> Result<Vec<LlmMessage>, AgentError> {
        let path = format!("{}/transcript.jsonl", output_dir);

        if !Path::new(&path).exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&path)
            .map_err(|e| AgentError::IoError(format!("Failed to read transcript file: {}", e)))?;

        let data: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| AgentError::GeneralError(format!("Failed to parse transcript file: {}", e)))?;

        let messages: Vec<LlmMessage> = data["contents"]
            .as_array()
            .ok_or_else(|| AgentError::GeneralError("Invalid transcript structure".to_string()))?
            .iter()
            .rev()
            .take(count)
            .rev()
            .filter_map(|v| {
                let text = v["parts"][0]["text"].as_str().map(|s| s.to_string());
                let timestamp = v["timestamp"].as_u64().unwrap_or(0);
                text.map(|t| LlmMessage { text: t, timestamp })
            })
            .collect();

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
