use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LlmMessage {
    pub text: String,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AgentEvent {
    UserMessage(LlmMessage),
    ModelResponse(LlmMessage),
    ToolCall { name: String, inputs: Vec<String>, timestamp: u64 },
}