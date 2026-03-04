pub trait LlmProvider {
    fn generate_content(&self, prompt: &str) -> Result<String, AgentError>;
}

#[derive(Debug)]
pub enum AgentError {
    GeneralError(String),
}
