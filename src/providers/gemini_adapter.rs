//! Gemini API adapter implementation.
//!
//! Provides an implementation of the `LlmProvider` trait for Google's Gemini API.

use crate::providers::llm_provider::{AgentError, LlmProvider};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

/// Adapter for interacting with the Gemini API.
pub struct GeminiAdapter {
    api_key: String,
    model_name: String,
    client: reqwest::Client,
}

impl GeminiAdapter {
    /// Creates a new GeminiAdapter instance.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key for accessing the Gemini API.
    /// * `model_name` - The name of the Gemini model to use.
    pub fn new(api_key: String, model_name: String) -> Self {
        GeminiAdapter {
            api_key,
            model_name,
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

        let request_body = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
            }],
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
