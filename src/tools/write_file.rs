use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct WriteFileError(String);

impl std::fmt::Display for WriteFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Write file error: {}", self.0)
    }
}

impl std::error::Error for WriteFileError {}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
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
        let parameters = schemars::schema_for!(WriteFileArgs);
        ToolDefinition {
            name: "write_file".to_string(),
            description: "Write content to a text file (.txt or .md)".to_string(),
            parameters: serde_json::to_value(parameters).unwrap(),
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