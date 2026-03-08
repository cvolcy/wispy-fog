//! Agent abstraction and implementations.
//!
//! The `Agent` trait defines the behavior expected of any agent that can be driven
//! by the application. Concrete implementations such as `BasicAgent` live in the
//! submodules.
//!
//! # Architecture
//!
//! The agent framework is organized into:
//! - **`BasicAgent`**: A simple interactive agent with tool support and history management
//! - **`History`**: Trait for managing conversation history
//! - **`JSONLHistory`**: JSONL-based history implementation

pub mod basic;
pub mod history;

use anyhow::Result;

/// A generic agent capable of being started and interacted with.
///
/// Implementations of this trait provide the logic for running an agent
/// by implementing the `run` method. Agents communicate with users or
/// other systems through I/O and can maintain state across interactions.
#[async_trait::async_trait]
pub trait Agent {
    /// Drive the agent until completion.
    ///
    /// This is the main entry point for an agent. Implementations typically
    /// run an interactive loop, handling user input and generating responses
    /// until a termination condition is met (e.g., user types "exit").
    ///
    /// # Returns
    /// An empty result on success, or an error if the agent encounters a fatal error.
    async fn run(&mut self) -> Result<()>;
}
