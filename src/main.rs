
use clap::Parser;
use log::{info, debug};

use crate::{
    agent::{basic::BasicAgent, history::JSONLHistory},
    config::{Args, Config},
    tools::{EchoTool, WriteFileTool, ToolRegistry},
};

use crate::agent::Agent;

mod agent;
mod config;
mod tools;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    dotenv::dotenv().ok();
    let args = Args::parse();

    info!("starting up");
    debug!("parsed cli args: {:?}", args);

    let config = Config::from_args(args);
    info!("using configuration: model={}, provider={:?}", config.model, config.provider);

    let mut registry = ToolRegistry::new();
    registry.register_tool(EchoTool::new());
    registry.register_tool(WriteFileTool::new());

    let tools = registry.tools();
    for tool in &tools {
        info!("registered tool: {}", tool.name());
    }

    let history_manager = JSONLHistory::new(format!("{}/history.jsonl", config.output_dir));
    let mut agent = BasicAgent::new(config, registry, history_manager);
    agent.run().await?;

    Ok(())
}

