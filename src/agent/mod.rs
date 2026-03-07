//! Agent abstraction and implementations.
//!
//! The `Agent` trait defines the behavior expected of any agent that can be driven
//! by the application. Concrete implementations such as `BasicAgent` live in the
//! submodules.

pub mod basic;
pub mod history;

use anyhow::Result;

use crate::{agent::history::{JSONLHistory}, config::Config};

/// A generic agent capable of being started and interacted with.
#[async_trait::async_trait]
pub trait Agent {
    fn get_history_manager(kind: String, config: Config) -> JSONLHistory {
        let file_path = format!("{}/history.jsonl", config.output_dir);
        match kind.as_str() {
            "jsonl" => JSONLHistory::new(file_path),
            _ => panic!("unsupported history manager type: {}", kind),
        }
    }

    /// Drive the agent until completion (for example, an interactive session).
    async fn run(&mut self) -> Result<()>;
}

pub use basic::BasicAgent;
