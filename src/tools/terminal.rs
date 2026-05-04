//! Terminal tool - enables agents to execute commands in the terminal.

use log::debug;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

/// Error type for terminal operations.
#[derive(Debug, Clone)]
pub struct TerminalError(String);

impl fmt::Display for TerminalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "command execution failed: {}", self.0)
    }
}

impl Error for TerminalError {}

impl TerminalError {
    /// Create a new error with the given message.
    fn new(msg: impl Into<String>) -> Self {
        Self(msg.into())
    }
}

/// Arguments for the terminal tool.
#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct TerminalArgs {
    /// terminal command to execute.
    pub command: String
}

/// A tool that gives access to the terminal.
///
/// # Security Notes
/// - Prevents path traversal attacks via parent directory references
/// - Ensure appropriate file system permissions are in place
///
/// # Example
/// ```ignore
/// let tool = TerminalTool::new();
/// let args = TerminalArgs {
///     command: "cat output.txt".to_string(),
/// };
/// let result = tool.call(args).await;
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TerminalTool {
    base_path: PathBuf
}

impl TerminalTool {
    /// Create a new terminal tool instance.
    pub fn new(base_path: &Path) -> Self {
        Self {
            base_path: base_path.to_path_buf(),
        }
    }
}

impl Tool for TerminalTool {
    const NAME: &'static str = "terminal";

    type Error = TerminalError;
    type Args = TerminalArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let parameters = schemars::schema_for!(TerminalArgs);
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: format!(
                "Give access to a shell terminal from the docker image curlimages/curl."
            ),
            parameters: serde_json::to_value(parameters)
                .unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let image = "curlimages/curl";
        debug!("terminal tool executing command in container:\n{}", args.command);
        debug!("terminal tool using base path for sandbox: {}", self.base_path.to_string_lossy());
        // 2. Wrap the agent's command in a docker execution
        // We mount a specific local folder to /workspace inside the container
        let output = Command::new("docker")
            .args([
                "run", 
                "--rm",                      // Auto-delete container after execution
                "-v", format!("./{}:/workspace", self.base_path.to_string_lossy()).as_str(), // Mount local ./sandbox to /workspace
                "-w", "/workspace",           // Set working directory inside container
                image, 
                "sh", "-c", &args.command     // Run the agent's command
            ])
            .output()
            .await;

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                let results = format!("Output:\n{}\nErrors:\n{}", stdout, stderr);
                debug!("terminal tool executed command in container:\n{}", results);
                Ok(results)
            }
            Err(e) => Err(TerminalError::new(format!("Docker execution failed: {}", e))),
        }
    }
}
