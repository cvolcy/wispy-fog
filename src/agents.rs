use std::fmt::Error;
use rig::{client::CompletionClient, completion::{Chat, Prompt}, providers::gemini};

use crate::{config::{Config, ModelProvider}, tools::ToolRegistry};

pub struct BasicAgent {
    config: Config,
    tool_registry: ToolRegistry,
}

impl BasicAgent {
    pub fn new(config: Config, tool_registry: ToolRegistry) -> Self {
        BasicAgent { config, tool_registry }
    }

    pub async fn run(&self, input: String) -> Result<String, Error> {
        let client = match self.config.provider {
            ModelProvider::Gemini => gemini::Client::new(self.config.api_key.clone()).unwrap(),
        };
        println!("tools registered: {:?}", self.tool_registry.get_tools().len());
        println!("client uri: {:?}", client.base_url());

        let model = client.agent(self.config.model.clone()).build();
        let mut chat_history = Vec::new();
        
        loop {
            let mut input = String::new();
            println!("prompt: ");

            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input.eq_ignore_ascii_case("exit") {
                break;
            }

            if let Ok(response) = model.chat(input, chat_history.clone()).await {
                println!("Response: {:?}", response);
                chat_history.push(input.into());
                chat_history.push(response.into());
            }

        }
        // Placeholder for agent logic
        Ok(format!("Received input: {}", input))
    }
}