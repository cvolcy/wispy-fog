//! File writing tool - enables agents to write content to text files.

use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::path::Path;

/// Error type for file writing operations.
#[derive(Debug, Clone)]
pub struct WriteFileError(String);

impl fmt::Display for WriteFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "file operation failed: {}", self.0)
    }
}

impl Error for WriteFileError {}

impl WriteFileError {
    /// Create a new error with the given message.
    fn new(msg: impl Into<String>) -> Self {
        Self(msg.into())
    }
}

/// Allowed file extensions for write operations.
const ALLOWED_EXTENSIONS: &[&str] = &[".txt", ".md"];

/// Arguments for the write_file tool.
#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct WriteFileArgs {
    /// Target filename (must end with .txt or .md).
    pub filename: String,
    /// Content to write to the file.
    pub content: String,
}

/// A tool that writes content to text files (.txt or .md).
///
/// # Security Notes
/// - Validates that only text and markdown files are written to
/// - Prevents path traversal attacks via parent directory references
/// - Ensure appropriate file system permissions are in place
///
/// # Example
/// ```ignore
/// let tool = WriteFileTool::new();
/// let args = WriteFileArgs {
///     filename: "output.txt".to_string(),
///     content: "Hello, world!".to_string(),
/// };
/// let result = tool.call(args).await;
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct WriteFileTool;

impl WriteFileTool {
    /// Create a new write file tool instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate if a filename has an allowed extension.
    fn is_allowed_extension(filename: &str) -> bool {
        let lower = filename.to_lowercase();
        ALLOWED_EXTENSIONS.iter().any(|ext| lower.ends_with(ext))
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
            name: Self::NAME.to_string(),
            description: format!(
                "Write text content to a file. Supported formats: {}",
                ALLOWED_EXTENSIONS.join(", ")
            ),
            parameters: serde_json::to_value(parameters)
                .expect("failed to serialize write_file tool schema"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Validate filename extension
        if !Self::is_allowed_extension(&args.filename) {
            return Err(WriteFileError::new(format!(
                "unsupported file extension; allowed: {}",
                ALLOWED_EXTENSIONS.join(", ")
            )));
        }

        // Validate that the path doesn't escape the intended directory
        let path = Path::new(&args.filename);
        if path
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err(WriteFileError::new(
                "path traversal attacks are not allowed",
            ));
        }

        // Write to file
        std::fs::write(&args.filename, &args.content)
            .map_err(|e| WriteFileError::new(e.to_string()))?;

        Ok(format!("successfully wrote to file: {}", args.filename))
    }
}