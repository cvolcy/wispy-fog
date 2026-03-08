use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct EchoError;

impl std::fmt::Display for EchoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Echo tool error")
    }
}

impl std::error::Error for EchoError {}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct EchoArgs {
    pub message: String,
}

/// A simple echo tool that returns its input
#[derive(Serialize, Deserialize, Clone)]
pub struct EchoTool;

impl EchoTool {
    pub fn new() -> Self {
        EchoTool
    }
}

impl Tool for EchoTool {
    const NAME: &'static str = "echo";

    type Error = EchoError;
    type Args = EchoArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let parameters = schemars::schema_for!(EchoArgs);
        ToolDefinition {
            name: "echo".to_string(),
            description: "A simple tool that echoes back the input".to_string(),
            parameters: serde_json::to_value(parameters).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(format!("Echo: {}", args.message))
    }
}