use crate::providers::llm_provider::{AgentError, LlmProvider};
use serde::{Deserialize, Serialize};

pub struct GeminiAdapter {
    api_key: String,
    model_name: String,
    client: reqwest::blocking::Client,
}

impl GeminiAdapter {
    pub fn new(api_key: String) -> Self {
        GeminiAdapter {
            api_key,
            model_name: "gemini-3-flash-preview".to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl LlmProvider for GeminiAdapter {
    fn generate_content(&self, prompt: &str) -> Result<String, AgentError> {
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
            .map_err(|e| AgentError::GeneralError(format!("Failed to send request: {}", e)))?
            .json::<GeminiResponse>()
            .map_err(|e| AgentError::GeneralError(format!("Failed to parse response: {}", e)))?;

        if let Some(candidate) = response.candidates.as_ref().and_then(|c| c.iter().next()) {
            if let Some(part) = candidate.content.parts.iter().next() {
                return Ok(part.text.clone());
            }
        }

        println!("Response: {}", serde_json::to_string_pretty(&response).unwrap_or_default());
        Err(AgentError::GeneralError("No content found in Gemini response".to_string()))
    }
}

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize, Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize, Deserialize)]
struct Part {
    text: String,
}

#[derive(Deserialize, Serialize)]
struct GeminiResponse {
    candidates: Option<Vec<Candidate>>,
}

#[derive(Deserialize, Serialize)]
struct Candidate {
    content: Content,
}
