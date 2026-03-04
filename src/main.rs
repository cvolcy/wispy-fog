pub mod providers;

use providers::llm_provider::LlmProvider;
use providers::gemini_adapter::GeminiAdapter;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    let api_key = std::env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY environment variable not set");
    
    let adapter = GeminiAdapter::new(api_key, "gemini-3-flash-preview".to_string());
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

async fn query_llm(provider: &dyn LlmProvider, prompt: &str) -> Result<String, providers::llm_provider::AgentError> {
    provider.generate_content(prompt).await
}
