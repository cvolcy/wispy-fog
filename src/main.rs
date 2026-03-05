//! # Wispy Fog
//!
//! An agentic system using the Gemini API for content generation.
//!
//! This application provides a command-line interface to interact with
//! Google's Gemini models, allowing users to input prompts and receive
//! generated responses.

mod config;
mod app;
pub mod providers;
pub mod agents;

use app::{App, Args};
use clap::Parser;

mod inspect;
use inspect::inspect_transcript;

/// The main entry point of the application.
///
/// Parses command-line arguments, loads configuration, and runs the application loop.
#[tokio::main]
async fn main() {
    let args = Args::parse();

    let config = config::Config::from_env()
        .expect("Failed to load configuration");

    if args.inspect {
        inspect_transcript(&config.output_dir);
        return;
    }

    println!("Using model: {:?}", args.model);

    let app = App::new(config, args.model);

    if let Err(e) = app.run().await {
        eprintln!("Application error: {:?}", e);
        std::process::exit(1);
    }
}

