pub mod providers;

use providers::llm_provider::LlmProvider;
use providers::gemini_adapter::GeminiAdapter;
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
enum ModelChoice {
    /// Gemini 3.0 Flash (faster, lower latency)
    Flash,
    /// Gemini 3.1 Pro (more capable)
    Pro,
}

#[derive(Parser, Debug)]
#[command(name = "Wispy Fog")]
#[command(about = "An agentic system using Gemini API", long_about = None)]
struct Args {
    /// Model to use for generation
    #[arg(short, long, value_enum, default_value = "flash")]
    model: ModelChoice,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    let args = Args::parse();
    
    let api_key = std::env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY environment variable not set");
    
    let adapter: Box<dyn LlmProvider> = match args.model {
        ModelChoice::Flash => Box::new(GeminiAdapter::new(api_key, "gemini-3-flash-preview".to_string())),
        ModelChoice::Pro => Box::new(GeminiAdapter::new(api_key, "gemini-3.1-pro-preview".to_string())),
    };
    
    println!("Using model: {:?}", args.model);

    println!("Enter a prompt (or 'exit' to quit):");

    loop {
        let mut input = String::new();
        println!("prompt: ");
        
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        let response = query_llm(&adapter, input).await;
        match response {
            Ok(ans) => println!("Response: {}", ans),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}

async fn query_llm(provider: &Box<dyn LlmProvider>, prompt: &str) -> Result<String, providers::llm_provider::AgentError> {
    provider.generate_content(prompt).await
}
