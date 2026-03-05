//! Gemini API adapter implementation.
//!
//! Provides an implementation of the `LlmProvider` trait for Google's Gemini API.

use crate::providers::llm_provider::{AgentError, LlmProvider};
use crate::agents::event::LlmMessage;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use std::time::{SystemTime, UNIX_EPOCH};

/// Adapter for interacting with the Gemini API.
pub struct GeminiAdapter {
    api_key: String,
    model_name: String,
    output_dir: String,
    client: reqwest::Client,
}

impl GeminiAdapter {
    /// Creates a new GeminiAdapter instance.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key for accessing the Gemini API.
    /// * `model_name` - The name of the Gemini model to use.
    /// * `output_dir` - The directory where output files will be saved.
    pub fn new(api_key: String, model_name: String, output_dir: String) -> Self {
        GeminiAdapter {
            api_key,
            model_name,
            output_dir,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmProvider for GeminiAdapter {
    async fn generate_content(&self, prompt: &str) -> Result<String, AgentError> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model_name, self.api_key
        );

        let history = self.get_transcript_history(10, self.output_dir.clone()).await?;

        let mut contents = history.iter().map(|msg| Content {
            role: if msg.text.starts_with("User:") { "user" } else { "model" }.to_string(),
            parts: vec![Part {
                text: msg.text.clone(),
            }],
        }).collect::<Vec<_>>();

        contents.push(Content {
            role: "user".to_string(),
            parts: vec![Part {
                text: prompt.to_string(),
            }],
        });

        let request_body = GeminiRequest {
            contents: contents,
        };

        let response = self.client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| AgentError::GeneralError(format!("Failed to send request: {}", e)))?
            .json::<GeminiResponse>()
            .await
            .map_err(|e| AgentError::GeneralError(format!("Failed to parse response: {}", e)))?;

        if let Some(candidate) = response.candidates.as_ref().and_then(|c| c.iter().next()) {
            if let Some(part) = candidate.content.parts.first() {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let user_msg = LlmMessage {
                    text: prompt.to_string(),
                    timestamp,
                };
                let model_resp = LlmMessage {
                    text: part.text.clone(),
                    timestamp,
                };
                self.transcript(user_msg, model_resp, self.output_dir.clone()).await?;
                return Ok(part.text.clone());
            }
        }

        println!("Response: {}", serde_json::to_string_pretty(&response).unwrap_or_default());
        Err(AgentError::GeneralError("No content found in Gemini response".to_string()))
    }
}

/// Request structure for the Gemini API.
#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

/// Content structure for the request.
#[derive(Serialize, Deserialize)]
struct Content {
    parts: Vec<Part>,
    role: String,
}

/// Part structure containing the text.
#[derive(Serialize, Deserialize)]
struct Part {
    text: String,
}

/// Response structure from the Gemini API.
#[derive(Deserialize, Serialize)]
struct GeminiResponse {
    candidates: Option<Vec<Candidate>>,
}

/// Candidate structure in the response.
#[derive(Deserialize, Serialize)]
struct Candidate {
    content: Content,
}
