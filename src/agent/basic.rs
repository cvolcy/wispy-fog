//! Basic agent implementation with conversation history and tool support.

use std::io::{self, Write};

use anyhow::Result;
use log::{debug, info, warn};
use rig::{
    completion::Chat, client::CompletionClient, message::{AssistantContent, Message},
    providers::gemini, OneOrMany,
};

use crate::{
    agent::history::{History, JSONLHistory},
    config::{Config, ModelProvider},
    tools::ToolRegistry,
};

/// A conversational agent that maintains chat history and can use tools.
///
/// `BasicAgent` provides a straightforward implementation of an AI agent
/// that maintains conversation history and can call registered tools.
/// It supports interactive sessions with configurable context windows.
pub struct BasicAgent {
    config: Config,
    client: gemini::Client,
    tool_registry: ToolRegistry,
    history_manager: JSONLHistory,
}

impl BasicAgent {
    /// Create a new agent with the given configuration and tools.
    ///
    /// # Arguments
    /// * `config` - Agent configuration (model, API key, etc.)
    /// * `tool_registry` - Registry of tools available to the agent
    /// * `history_manager` - Manager for conversation history
    ///
    /// # Panics
    /// Panics if the gemini client cannot be initialized.
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

    /// Run an interactive chat session.
    ///
    /// This method drives the agent's main interaction loop, handling user input,
    /// sending prompts to the model, and maintaining conversation history.
    async fn interact(&mut self) -> Result<()> {
        info!("beginning interactive session");

        loop {
            let chat_history: Vec<Message> = self.history_manager.get(15).await?;
            debug!("loaded {} messages from history", chat_history.len());

            let input = read_line("prompt: ")?;
            let input = input.trim();

            if input.eq_ignore_ascii_case("exit") {
                info!("received exit command, shutting down");
                break;
            }

            if input.is_empty() {
                debug!("skipping empty input");
                continue;
            }

            debug!("sending message to model: {}", input);
            let model = self
                .client
                .agent(self.config.model.clone())
                .tools(self.tool_registry.tools())
                .build();

            let response = match model.chat(input, chat_history.clone()).await {
                Ok(response) => response,
                Err(e) => {
                    warn!("model error: {}", e);
                    eprintln!("Error: Failed to get response from model");
                    continue;
                }
            };

            println!("Response: {}", response);

            self.history_manager.add(input.into()).await?;
            self.history_manager.add(Message::Assistant {
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

/// Read a line of input from stdin with a prompt.
///
/// # Arguments
/// * `prompt` - The prompt string to display
///
/// # Returns
/// The user's input as a String, or an IO error
fn read_line(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}
