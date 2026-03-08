
//! # Wispy Fog - AI Agent Framework
//!
//! A framework for building AI agents with tool support and conversation history management.

use clap::Parser;
use log::{info, debug};
use std::fs;

use crate::{
    agents::{basic::BasicAgent, history::JSONLHistory},
    config::{Args, Config},
    tools::{echo::EchoTool, write_file::WriteFileTool, ToolRegistry},
};

use crate::agents::Agent;

mod agents;
mod config;
mod tools;

fn initialize_tools() -> ToolRegistry {
    let mut registry = ToolRegistry::new();

    registry.register_tool(EchoTool::new());
    registry.register_tool(WriteFileTool::new());

    debug!("initialized tool registry with {} tools", registry.len());
    registry
}

fn log_registered_tools(registry: &ToolRegistry) {
    let tools = registry.tools();
    info!("registered {} tool(s)", tools.len());
    for tool in tools {
        info!("  - {}", tool.name());
    }
}

fn ensure_output_dir(output_dir: &str) -> anyhow::Result<()> {
    if !std::path::Path::new(output_dir).exists() {
        debug!("creating output directory: {}", output_dir);
        fs::create_dir_all(output_dir)?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    let args = Args::parse();
    info!("starting wispy-fog");
    debug!("parsed cli args: {:?}", args);

    let config = Config::from_args(args);
    info!(
        "configuration: model={}, provider={:?}",
        config.model, config.provider
    );

    ensure_output_dir(&config.output_dir)?;

    let registry = initialize_tools();
    log_registered_tools(&registry);

    let history_path = format!("{}/history.jsonl", config.output_dir);
    let history_manager = JSONLHistory::new(history_path);
    debug!("initialized history manager");

    let mut agent = BasicAgent::new(config, registry, history_manager);
    agent.run().await?;

    info!("shutdown complete");
    Ok(())
}

