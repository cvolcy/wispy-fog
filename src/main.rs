use std::env;
use crate::{agents::BasicAgent, config::Args, tools::{EchoTool, ToolRegistry}};
use clap::Parser;

mod agents;
mod config;
mod tools;

#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv::dotenv().ok();
    let args = Args::parse();

    println!("Starting agent with model: {}", args.model.unwrap_or_default());
    
    let api_key = env::var("GEMINI_API_KEY")
        .map_err(|_| "GEMINI_API_KEY environment variable not set".to_string())?;

    println!("API Key length: {}", api_key.len());

    let config = config::Config::from_env();
    println!("config {}", config.model);

    let mut registry = ToolRegistry::new();
    println!("Registering Tools...");
    registry.register_tool(Box::new(EchoTool));
    for tool in registry.get_tools() {
        println!("Registered tool: {} - {}", tool.name(), tool.description());
    }

    let agent: BasicAgent = BasicAgent::new(config, registry);

    let result = agent.run("Hello, world!".to_string()).await;
    println!("Agent result: {}", result.unwrap_or_else(|e| format!("Error: {}", e)));

    Ok(())
}

