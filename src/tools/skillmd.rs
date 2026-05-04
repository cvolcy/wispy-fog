use log::{debug, error};
use serde::{Deserialize, Serialize};
use rig::{client::{CompletionClient, ProviderClient}, completion::{Prompt, ToolDefinition}, tool::Tool};
use core::fmt;
use std::{collections::HashMap, error::Error};

use crate::config::Config;

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct SkillMDArgs {}

#[derive(Debug, Deserialize, Clone)]
pub struct SkillMDMetadata {
    pub name: String,
    pub description: String,
    pub metadata: Option<ToolMetadata>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct ToolMetadata {
    pub parameters: Option<HashMap<String, ParameterDefinition>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ParameterDefinition {
    #[serde(rename = "type")]
    param_type: String,
    description: String,
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
    pub instructions: String,
    config: Config,
}

impl SkillMD {
    pub fn from_file(path: &std::path::Path, config: Config) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;

        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() < 3 {
            return Err(anyhow::anyhow!("invalid skillmd format: missing frontmatter"));
        }

        let yaml_str = parts[1].trim();
        let instructions = parts[2].trim().to_string();

        let metadata: SkillMDMetadata = serde_yaml::from_str(yaml_str)
            .map_err(|e| anyhow::anyhow!("failed to parse skillmd frontmatter: {}", e))?;

        Ok(Self {
            metadata,
            instructions,
            config
        })
    }
}

impl Tool for SkillMD {
    const NAME: &'static str = "skillmd";

    type Error = SkillMDError;
    type Args = serde_json::Value;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: self.metadata.name.clone(),
            description: self.metadata.description.clone(),
            parameters: match &self.metadata.metadata {
                Some(metadata) => match &metadata.parameters {
                    Some(params) => serde_json::json!({
                        "type": "object",
                        "properties": params
                    }),
                    None => serde_json::json!(schemars::schema_for!(SkillMDArgs)),
                },
                None => serde_json::json!(schemars::schema_for!(SkillMDArgs)),
            },
        }
    }

    fn name(&self) -> String {
        self.metadata.name.clone()
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let client = rig::providers::ollama::Client::from_env();

        let sub_agent = client
            .agent(self.config.model.clone())
            .preamble(&self.instructions)
            .build();

        debug!("executing skill: {} with args: {}", self.metadata.name, _args);
        let response = sub_agent
            .prompt(format!("Input parameters: {}", _args))
            .await
            .map_err(|e| {
                error!("Sub-agent error: {}", e);
                SkillMDError
            })?;

        Ok(response)
    }
}