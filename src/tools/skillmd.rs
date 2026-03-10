use serde::{Deserialize};
use rig::{completion::ToolDefinition, tool::Tool};
use core::fmt;
use std::error::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct SkillMDMetadata {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct SkillMDError;

impl fmt::Display for SkillMDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SkillMD encountered an error")
    }
}

impl Error for SkillMDError {}

#[derive(Clone)]
pub struct SkillMD {
    pub metadata: SkillMDMetadata,
    pub inscriptions: String
}

impl SkillMD {
    pub fn from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;

        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() < 3 {
            return Err(anyhow::anyhow!("invalid skillmd format: missing frontmatter"));
        }

        let yaml_str = parts[1];
        let inscriptions = parts[2].trim().to_string();

        let metadata: SkillMDMetadata = serde_yaml::from_str(yaml_str)
            .map_err(|e| anyhow::anyhow!("failed to parse skillmd frontmatter: {}", e))?;

        Ok(Self {
            metadata,
            inscriptions
        })
    }
}

impl Tool for SkillMD {
    const NAME: &'static str = "skillmd";

    type Error = SkillMDError;
    type Args = ();
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: self.metadata.name.clone(),
            description: self.metadata.description.clone(),
            parameters: self.metadata.parameters.clone(),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(format!(
            "Executed {} with logic: {}", 
            self.metadata.name, 
            self.inscriptions
        ))
    }
}