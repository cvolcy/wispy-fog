# Wispy Fog

An agentic system using Google's Gemini API for content generation. This CLI application allows users to interact with Gemini models through a simple command-line interface.

## Features

- **Multiple Model Support**: Choose between Gemini 3.0 Flash (fast, low latency) and Gemini 3.1 Pro (more capable)
- **Interactive CLI**: Simple prompt-response interface
- **Conversation Logging**: Requests and responses are saved to `transcript.jsonl` in JSONL format for later inspection
- **Inspect Mode**: `--inspect` command shows a colored, human-readable view of the transcript file
- **Modular Architecture**: Extensible provider system for easy addition of new LLM backends
- **Configuration Management**: Environment-based configuration with dotenv support, including custom `OUTPUT_DIR` for transcripts
- **Async Processing**: Built with Tokio for efficient asynchronous operations

## Architecture

The application follows a clean, modular architecture and retains a flat project structure:

```
src/
├── main.rs              # Application entry point
├── config.rs            # Configuration management
├── app.rs               # Main application logic and CLI
└── providers/
    ├── mod.rs           # Provider module exports and constants
    ├── llm_provider.rs  # LLM provider trait and error types
    └── gemini_adapter.rs # Gemini API implementation
```

### Key Components

- **Config Module**: Handles environment variable loading and configuration
- **App Module**: Manages the CLI interface and user interaction loop
- **Providers Module**: Defines the `LlmProvider` trait and implements Gemini integration
- **Error Handling**: Comprehensive error types for different failure scenarios

## Installation

### Prerequisites

- Rust 1.70 or later
- A Google Gemini API key

### Build from Source

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd wispy-fog
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. The binary will be available at `target/release/wispy-fog`

## Usage

### Configuration

The application supports two environment variables:

* `GEMINI_API_KEY` – your Gemini API key
* `OUTPUT_DIR` – directory where transcripts and other output files are stored (defaults to project root)

Create a `.env` file in the project root with the settings:

```env
GEMINI_API_KEY=your_api_key_here
OUTPUT_DIR=.
```

### Running the Application

```bash
# Use default model (Flash) and record conversation
./wispy-fog

# Specify model explicitly
./wispy-fog --model pro
./wispy-fog --model flash

# Inspect the transcript file in human-readable, colored format
./wispy-fog --inspect

# Get help
./wispy-fog --help
```

### Interactive Session

Once running, the application will prompt for input:

```
Using model: Flash
Enter a prompt (or 'exit' to quit):
prompt: Hello, how are you?
Response: I'm doing well, thank you for asking! How can I help you today?
prompt: exit
```

## Development

### Dependencies

- `reqwest`: HTTP client for API calls
- `serde`/`serde_json`: Serialization/deserialization
- `tokio`: Async runtime
- `dotenv`: Environment variable management
- `async-trait`: Async trait support
- `clap`: Command-line argument parsing
- `colored`: ANSI color output for transcript inspection

### Adding New Providers

To add support for a new LLM provider:

1. Implement the `LlmProvider` trait in a new module under `providers/`
2. Add the provider to the model selection logic in `app.rs`
3. Update the CLI arguments if needed

Example:

```rust
use async_trait::async_trait;
use crate::providers::llm_provider::{LlmProvider, AgentError};

pub struct NewProvider {
    // provider-specific fields
}

#[async_trait]
impl LlmProvider for NewProvider {
    async fn generate_content(&self, prompt: &str) -> Result<String, AgentError> {
        // implementation
    }
}
```

### Testing

Run tests with:
```bash
cargo test
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Check for issues
cargo check
```

## Error Handling

Errors in transcript I/O are surfaced via the same `AgentError` enum; `IoError` covers problems reading or writing the JSONL log.

The application uses a comprehensive error system:

- `GeneralError`: API and processing errors
- `IoError`: Input/output operation failures

Errors are propagated up and displayed to the user with context.

## License

This project is licensed under the terms specified in the LICENSE file.

## Transcript Storage

Logs are written to `transcript.jsonl` within `OUTPUT_DIR`. Each entry is a standalone JSON object containing `role`, `parts`, and `timestamp`. The log can be appended to or processed by external tools.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## API Reference

For Gemini API documentation, see: https://ai.google.dev/docs