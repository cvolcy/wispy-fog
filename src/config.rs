use clap::Parser;
use std::env;

/// Command line arguments that may be set via flags or environment variables.
#[derive(Parser, Debug)]
#[command(name = "Wispy Fog")]
pub struct Args {
    #[arg(
        short,
        long,
        help = "Model to use for generation (e.g., 'gemini-flash-001' or 'gemini-pro-001')",
    )]
    pub model: Option<String>,

    #[arg(
        short,
        long,
        help = "Provider to use for generation (e.g., 'gemini')",
    )]
    pub provider: Option<String>,

    #[arg(
        short,
        long,
        help = "Directory to save output files (default: 'output')",
    )]
    pub output_dir: Option<String>,

    #[arg(
        long,
        help = "kind of history manager to use (e.g., 'jsonl')",
    )]
    pub history_manager: Option<String>,

    #[arg(
        short,
        long,
        help = "API key for the LLM provider",
    )]
    pub api_key: Option<String>,
}

/// Resolved configuration used throughout the application.
#[derive(Debug, Clone)]
pub struct Config {
    pub model: String,
    pub provider: ModelProvider,
    pub output_dir: String,
    pub history_manager: String,
    pub api_key: String,
}

#[derive(Debug, Clone)]
pub enum ModelProvider {
    Gemini,
    // future providers can be added here
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model: "gemini-3-flash-preview".to_string(),
            provider: ModelProvider::Gemini,
            output_dir: "output".to_string(),
            history_manager: "jsonl".to_string(),
            api_key: String::new(),
        }
    }
}

impl Config {
    /// Build a configuration from parsed CLI arguments and environment variables.
    pub fn from_args(args: Args) -> Self {
        let _ = dotenv::dotenv();

        let mut cfg = Config::default();

        if let Some(model) = args.model {
            cfg.model = model;
        }

        if let Some(provider) = args.provider {
            cfg.provider = match provider.as_str() {
                "gemini" => ModelProvider::Gemini,
                other => {
                    log::warn!("unknown provider '{}', defaulting to Gemini", other);
                    ModelProvider::Gemini
                }
            };
        }

        if let Some(output_dir) = args.output_dir {
            cfg.output_dir = output_dir;
        }

        cfg.history_manager = if let Some(history_manager) = args.history_manager {
            history_manager
        }
        else {
            "jsonl".to_string()
        };

        if let Some(key) = args.api_key {
            cfg.api_key = key;
        } else {
            // try environment variable fallback
            let env_var = match cfg.provider {
                ModelProvider::Gemini => "GEMINI_API_KEY",
            };
            if let Ok(env_key) = env::var(env_var) {
                cfg.api_key = env_key;
            }
        }

        cfg
    }
}
