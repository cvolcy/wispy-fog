//! Echo tool - a simple demonstration tool that echoes back input.

use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

/// Error type for the echo tool.
#[derive(Debug, Clone)]
pub struct EchoError;

impl fmt::Display for EchoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "echo tool encountered an error")
    }
}

impl Error for EchoError {}

/// Arguments for the echo tool.
#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct EchoArgs {
    /// The message to echo back.
    pub message: String,
}

/// A simple demonstration tool that echoes back the input message.
///
/// # Example
/// ```ignore
/// let tool = EchoTool::new();
/// let args = EchoArgs { message: "hello".to_string() };
/// let result = tool.call(args).await;
/// assert_eq!(result, Ok("Echo: hello".to_string()));
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EchoTool;

impl EchoTool {
    /// Create a new echo tool instance.
    pub fn new() -> Self {
        Self::default()
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
            name: Self::NAME.to_string(),
            description: "A demonstration tool that echoes back the input message".to_string(),
            parameters: serde_json::to_value(parameters)
                .expect("failed to serialize echo tool schema"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(format!("Echo: {}", args.message))
    }
}