//! Configuration module for the Wispy Fog application.
//!
//! Handles loading and managing application configuration,
//! primarily environment variables.

use std::env;

/// Application configuration structure.
#[derive(Debug)]
pub struct Config {
    /// The API key for accessing the Gemini API.
    pub api_key: String,
    pub output_dir: String,
}

impl Config {
    /// Loads configuration from environment variables.
    ///
    /// # Errors
    ///
    /// Returns an error if the `GEMINI_API_KEY` environment variable is not set.
    pub fn from_env() -> Result<Self, String> {
        dotenv::dotenv().ok();

        let api_key = env::var("GEMINI_API_KEY")
            .map_err(|_| "GEMINI_API_KEY environment variable not set".to_string())?;

        let output_dir = env::var("OUTPUT_DIR")
            .unwrap_or_else(|_| "./output".into());

        // Ensure output directory exists
        std::fs::create_dir_all(&output_dir)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;

        Ok(Config { api_key, output_dir })
    }
}