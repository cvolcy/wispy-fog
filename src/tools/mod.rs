//! Tool registry and management.
//!
//! This module provides the infrastructure for registering and managing tools
//! that can be used by agents. Tools must implement the `rig::tool::Tool` trait.

use crate::{config::Config, tools::skillmd::SkillMD};
use log::debug;
use rig::tool::Tool;
use tokio::fs;

pub mod echo;
pub mod read_file;
pub mod write_file;
pub mod skillmd;
pub mod terminal;

/// Internal trait to enable storing heterogeneous tool types in a single collection.
///
/// This trait bridges the gap between concrete tool implementations and the
/// dynamic trait object interface required by the rig agent framework.
trait AnyTool: Send + Sync {
    /// Convert the tool to a dynamic trait object compatible with rig agents.
    fn to_dyn(&self) -> Box<dyn rig::tool::ToolDyn>;
}

impl<T: Tool + Clone + Send + Sync + 'static> AnyTool for T {
    fn to_dyn(&self) -> Box<dyn rig::tool::ToolDyn> {
        Box::new(self.clone()) as Box<dyn rig::tool::ToolDyn>
    }
}

/// Registry for managing tools available to agents.
///
/// The `ToolRegistry` provides a unified interface for registering tools of
/// different types. It handles the conversion between concrete tool types and
/// the dynamic trait objects required by the rig framework.
///
/// # Example
/// ```ignore
/// let mut registry = ToolRegistry::new();
/// registry.register_tool(EchoTool::new());
/// registry.register_tool(WriteFileTool::new());
/// let tools = registry.tools();
/// ```
#[derive(Default)]
pub struct ToolRegistry {
    tools: Vec<Box<dyn AnyTool>>,
}

impl ToolRegistry {
    /// Create a new empty tool registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a tool with the registry.
    ///
    /// # Arguments
    /// * `tool` - A tool implementing the `rig::tool::Tool` trait
    pub fn register_tool<T: Tool + Clone + Send + Sync + 'static>(&mut self, tool: T) {
        debug!("registering tool: {}", T::NAME);
        self.tools.push(Box::new(tool));
    }

    pub async fn load_skills_from_dir(&mut self, dir_path: &str, config: Config) -> anyhow::Result<(), anyhow::Error> {
        let path = std::path::Path::new(dir_path);
        if !path.is_dir() {
            return Ok(());
        }
        let mut entries = fs::read_dir(path).await?;
        while let Ok(entry) = entries.next_entry().await {
            let entry= entry.ok_or_else(|| anyhow::anyhow!("failed to read directory entry"))?;
            let path = entry.path();
            
            if path.is_dir() {
                let skill_file_path = path.join("SKILL.md");
                if skill_file_path.exists() {
                    debug!("loading skill from file: {}", path.display());
                    match SkillMD::from_file(&skill_file_path, config.clone()) {
                        Ok(skill) => self.register_tool(skill),
                        Err(e) => debug!("failed to load skill from {}: {}", path.display(), e),
                    }
                }
            }
        }

        Ok(())
    }

    /// Get all registered tools as dynamic trait objects.
    ///
    /// Returns a vector of boxed tool trait objects compatible with rig agents.
    pub fn tools(&self) -> Vec<Box<dyn rig::tool::ToolDyn>> {
        self.tools.iter().map(|t| t.to_dyn()).collect()
    }

    /// Get the count of registered tools.
    pub fn len(&self) -> usize {
        self.tools.len()
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::ToolRegistry;
    use crate::tools::{echo::EchoTool, write_file::WriteFileTool};

    #[test]
    fn registry_tracks_registered_tools() {
        let mut registry = ToolRegistry::new();
        assert_eq!(registry.len(), 0);

        registry.register_tool(EchoTool::new());
        registry.register_tool(WriteFileTool::new(Path::new("./")));

        assert_eq!(registry.len(), 2);
        assert_eq!(registry.tools().len(), 2);
    }
}