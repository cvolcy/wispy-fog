//! LLM Provider trait and error definitions.
//!
//! Defines the interface for LLM providers and common error types.

use async_trait::async_trait;

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
}

/// Common error types for the agent system.
#[derive(Debug)]
pub enum AgentError {
    /// A general error with a descriptive message.
    GeneralError(String),
    /// An I/O error, typically from user input.
    IoError(String),
}
