//! Application logic module.
//!
//! Contains the main application structure and CLI interaction loop.

use crate::config::Config;
use crate::providers::llm_provider::{LlmProvider, AgentError};
use crate::providers::gemini_adapter::GeminiAdapter;
use crate::providers::{GEMINI_FLASH_MODEL, GEMINI_PRO_MODEL};
use clap::{Parser, ValueEnum};

/// Available Gemini model choices.
#[derive(ValueEnum, Clone, Debug)]
pub enum ModelChoice {
    /// Gemini 3.0 Flash (faster, lower latency)
    Flash,
    /// Gemini 3.1 Pro (more capable)
    Pro,
}

/// Command-line arguments structure.
#[derive(Parser, Debug)]
#[command(name = "Wispy Fog")]
#[command(about = "An agentic system using Gemini API", long_about = None)]
pub struct Args {
    /// Model to use for generation
    #[arg(short, long, value_enum, default_value = "flash")]
    pub model: ModelChoice,

    /// If set, print the transcript.jsonl file to stdout in a readable format and exit
    #[arg(long)]
    pub inspect: bool,
}

/// Main application structure.
pub struct App {
    provider: Box<dyn LlmProvider>,
}

impl App {
    /// Creates a new application instance with the given configuration and model choice.
    pub fn new(config: Config, model: ModelChoice) -> Self {
        let provider: Box<dyn LlmProvider> = match model {
            ModelChoice::Flash => Box::new(GeminiAdapter::new(config.api_key, GEMINI_FLASH_MODEL.to_string(), config.output_dir)),
            ModelChoice::Pro => Box::new(GeminiAdapter::new(config.api_key, GEMINI_PRO_MODEL.to_string(), config.output_dir)),
        };

        App { provider }
    }

    /// Runs the main application loop, handling user input and LLM queries.
    ///
    /// # Errors
    ///
    /// Returns an error if there's an issue with user input or LLM communication.
    pub async fn run(&self) -> Result<(), AgentError> {
        println!("Enter a prompt (or 'exit' to quit):");

        loop {
            let mut input = String::new();
            println!("prompt: ");

            std::io::stdin().read_line(&mut input)
                .map_err(|e| AgentError::IoError(e.to_string()))?;
            let input = input.trim();
            if input.eq_ignore_ascii_case("exit") {
                break;
            }

            let response = self.query_llm(input).await;
            match response {
                Ok(ans) => println!("Response: {}", ans),
                Err(e) => println!("Error: {:?}", e),
            }
        }

        Ok(())
    }

    /// Queries the LLM provider with the given prompt.
    ///
    /// # Errors
    ///
    /// Returns an error if the LLM provider fails to generate content.
    async fn query_llm(&self, prompt: &str) -> Result<String, AgentError> {
        self.provider.generate_content(prompt).await
    }
}