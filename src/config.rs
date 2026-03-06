use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
#[command(name = "Wispy Fog")]
pub struct Args {
    #[arg(short, long,
        help = "Model to use for generation (e.g., 'gemini-flash-001' or 'gemini-pro-001')"
    )]
    pub model: Option<String>,
    #[arg(short, long,
        help = "Provider to use for generation (e.g., 'gemini')"
    )]
    pub provider: Option<String>,
    #[arg(short, long, help = "API key for the LLM provider")]
    api_key: Option<String>,
}

pub struct Config {
    pub model: String,
    pub provider: ModelProvider,
    pub api_key: String,
}

pub enum ModelProvider {
    Gemini,
    // Future providers can be added here
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model: "gemini-3-flash-preview".to_string(),
            provider: ModelProvider::Gemini,
            api_key: String::new(),
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        let _ = dotenv::dotenv();

        let mut config = Config::default();

        if let Ok(model) = env::var("MODEL") {
            config.model = model;
        }

        if let Ok(provider) = env::var("PROVIDER") {
            config.provider = match provider.as_str() {
                "gemini" => ModelProvider::Gemini,
                _ => ModelProvider::Gemini,
            };
        }

        let env_api_key = match config.provider {
            ModelProvider::Gemini => "GEMINI_API_KEY",
            // _ => "API_KEY"
        };

        if let Ok(api_key) = env::var(env_api_key) {
            config.api_key = api_key;
        }

        config
    }
}