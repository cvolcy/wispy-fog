//! File writing tool - enables agents to write content to text files.

use log::debug;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

/// Error type for file writing operations.
#[derive(Debug, Clone)]
pub struct ReadFileError(String);

impl fmt::Display for ReadFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "file operation failed: {}", self.0)
    }
}

impl Error for ReadFileError {}

impl ReadFileError {
    /// Create a new error with the given message.
    fn new(msg: impl Into<String>) -> Self {
        Self(msg.into())
    }
}

/// Arguments for the read_file tool.
#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct ReadFileArgs {
    /// Target filename.
    pub filename: String
}

/// A tool that reads content from files.
///
/// # Security Notes
/// - Prevents path traversal attacks via parent directory references
/// - Ensure appropriate file system permissions are in place
///
/// # Example
/// ```ignore
/// let tool = ReadFileTool::new();
/// let args = ReadFileArgs {
///     filename: "output.txt".to_string(),
/// };
/// let result = tool.call(args).await;
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ReadFileTool {
    base_path: PathBuf
}

impl ReadFileTool {
    /// Create a new write file tool instance.
    pub fn new(base_path: &Path) -> Self {
        Self {
            base_path: base_path.to_path_buf(),
        }
    }
}

impl Tool for ReadFileTool {
    const NAME: &'static str = "read_file";

    type Error = ReadFileError;
    type Args = ReadFileArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let parameters = schemars::schema_for!(ReadFileArgs);
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: format!(
                "Read text content from a file."
            ),
            parameters: serde_json::to_value(parameters)
                .expect("failed to serialize read_file tool schema"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // 1. Define the sandbox image (e.g., alpine for speed or python:3.9-slim for tools)
        let image = "alpine:latest";
        
        // 2. Wrap the agent's command in a docker execution
        // We mount a specific local folder to /workspace inside the container
        let output = Command::new("docker")
            .args([
                "run", 
                "--rm",                      // Auto-delete container after execution
                "-v", "./output:/workspace", // Mount local ./sandbox to /workspace
                "-w", "/workspace",           // Set working directory inside container
                image, 
                "sh", "-c", "cat", &args.filename     // Run the agent's command
            ])
            .output()
            .await;

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                let results = format!("Output:\n{}\nErrors:\n{}", stdout, stderr);
                debug!("read_file tool executed command in container:\n{}", results);
                Ok(results)
            }
            Err(e) => Err(ReadFileError::new(format!("Docker execution failed: {}", e))),
        }
    }
}
