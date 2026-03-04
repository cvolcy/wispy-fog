use async_trait::async_trait;

#[async_trait]
pub trait LlmProvider {
    async fn generate_content(&self, prompt: &str) -> Result<String, AgentError>;
}

#[derive(Debug)]
pub enum AgentError {
    GeneralError(String),
}
