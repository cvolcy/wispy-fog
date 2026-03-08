use serde::{Deserialize, Serialize};
use rig::tool::Tool;
use rig::completion::ToolDefinition;

#[derive(Debug)]
pub struct EchoError;

impl std::fmt::Display for EchoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Echo tool error")
    }
}

impl std::error::Error for EchoError {}

#[derive(Serialize, Deserialize)]
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
        ToolDefinition {
            name: "echo".to_string(),
            description: "A simple tool that echoes back the input".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "message": {
                        "type": "string",
                        "description": "The message to echo"
                    }
                },
                "required": ["message"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(format!("Echo: {}", args.message))
    }
}

#[derive(Debug)]
pub struct WriteFileError(String);

impl std::fmt::Display for WriteFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Write file error: {}", self.0)
    }
}

impl std::error::Error for WriteFileError {}

#[derive(Serialize, Deserialize)]
pub struct WriteFileArgs {
    pub filename: String,
    pub content: String,
}

/// A tool that writes content to text or markdown files
#[derive(Serialize, Deserialize, Clone)]
pub struct WriteFileTool;

impl WriteFileTool {
    pub fn new() -> Self {
        WriteFileTool
    }
}

impl Tool for WriteFileTool {
    const NAME: &'static str = "write_file";

    type Error = WriteFileError;
    type Args = WriteFileArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "write_file".to_string(),
            description: "Write content to a text file (.txt or .md)".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "filename": {
                        "type": "string",
                        "description": "The filename to write to (e.g., output.txt or notes.md)"
                    },
                    "content": {
                        "type": "string",
                        "description": "The content to write to the file"
                    }
                },
                "required": ["filename", "content"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Validate filename extension
        let lower = args.filename.to_lowercase();
        if !lower.ends_with(".txt") && !lower.ends_with(".md") {
            return Err(WriteFileError(
                "Only .txt and .md files are allowed".to_string(),
            ));
        }

        // Write to file
        std::fs::write(&args.filename, &args.content)
            .map_err(|e| WriteFileError(e.to_string()))?;

        Ok(format!("Successfully wrote to {}", args.filename))
    }
}

/// Internal trait to allow storing heterogeneous tools
trait AnyTool: Send + Sync {
    fn to_dyn(&self) -> Box<dyn rig::tool::ToolDyn>;
}

impl<T: Tool + Clone + Send + Sync + 'static> AnyTool for T {
    fn to_dyn(&self) -> Box<dyn rig::tool::ToolDyn> {
        Box::new(self.clone()) as Box<dyn rig::tool::ToolDyn>
    }
}

pub struct ToolRegistry {
    tools: Vec<Box<dyn AnyTool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }

    pub fn register_tool<T: Tool + Clone + Send + Sync + 'static>(&mut self, tool: T) {
        self.tools.push(Box::new(tool));
    }

    /// Returns the registered tools for use by the agent.
    pub fn tools(&self) -> Vec<Box<dyn rig::tool::ToolDyn>> {
        self.tools.iter().map(|t| t.to_dyn()).collect()
    }
}