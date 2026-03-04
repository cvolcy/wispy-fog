pub mod providers;

use providers::llm_provider::LlmProvider;
use providers::gemini_adapter::GeminiAdapter;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    let api_key = std::env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY environment variable not set");
    
    let adapter = GeminiAdapter::new(api_key);

    let prompt = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "what is 10 + 9".to_string());

    println!("Prompt: {}", prompt);

    let anwser = query_llm(&adapter, &prompt).await;
    match anwser {
        Ok(response) => println!("Response: {}", response),
        Err(e) => println!("Error: {:?}", e),
    }
}

async fn query_llm(provider: &dyn LlmProvider, prompt: &str) -> Result<String, providers::llm_provider::AgentError> {
    provider.generate_content(prompt).await
}
