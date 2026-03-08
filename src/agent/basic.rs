use std::io::{self, Write};

use anyhow::Result;
use log::{debug, info};
use rig::{OneOrMany, client::CompletionClient, completion::Chat, message::{AssistantContent, Message}, providers::gemini};

use crate::{agent::history::{History, JSONLHistory}, config::{Config, ModelProvider}, tools::ToolRegistry};

/// A very simple agent that wraps the rig client and maintains a chat history.
pub struct BasicAgent {
    config: Config,
    client: gemini::Client,
    tool_registry: ToolRegistry,
    history_manager: JSONLHistory,
}

impl BasicAgent {
    /// Build a new `BasicAgent` from configuration and a pre-populated tool registry.
    pub fn new(config: Config, tool_registry: ToolRegistry, history_manager: JSONLHistory) -> Self {
        let client = match config.provider {
            ModelProvider::Gemini => gemini::Client::new(config.api_key.clone())
                .expect("failed to create gemini client"),
        };

        BasicAgent {
            config,
            client,
            tool_registry,
            history_manager,
        }
    }

    /// Internal interactive loop; separated from the `Agent` trait so that it can be
    /// driven by tests or other callers if necessary.
    async fn interact(&mut self) -> Result<()> {
        info!("beginning interactive session");

        let model = self
            .client
            .agent(self.config.model.clone())
            .tools(self.tool_registry.tools())
            .build();
        
        loop {
            let chat_history: Vec<Message> = self.history_manager.get(15).await?;
            let input = read_line("prompt: ")?;
            let input = input.trim();
            if input.eq_ignore_ascii_case("exit") {
                info!("received exit command, shutting down");
                break;
            }

            debug!("sending message to model: {}", input);
            let response = model.chat(input, chat_history.clone()).await?;
            println!("Response: {}", response);

            // keep history so subsequent turns can see the conversation
            self.history_manager.add(input.into()).await?;
            self.history_manager.add( Message::Assistant {
                id: None,
                content: OneOrMany::one(AssistantContent::text(response)),
            }).await?;
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl super::Agent for BasicAgent {
    async fn run(&mut self) -> Result<()> {
        self.interact().await
    }
}

pub fn read_line(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}
