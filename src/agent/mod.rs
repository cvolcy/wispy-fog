//! Agent abstraction and implementations.
//!
//! The `Agent` trait defines the behavior expected of any agent that can be driven
//! by the application. Concrete implementations such as `BasicAgent` live in the
//! submodules.

pub mod basic;
pub mod history;

use anyhow::Result;

/// A generic agent capable of being started and interacted with.
#[async_trait::async_trait]
pub trait Agent {
    /// Drive the agent until completion (for example, an interactive session).
    async fn run(&mut self) -> Result<()>;
}
