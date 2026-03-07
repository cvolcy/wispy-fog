# Wispy Fog

An AI agent system built with Rust, using the Rig framework for seamless integration with Large Language Models (LLMs). This CLI application provides an interactive chat interface with conversation history management.

## Features

- **LLM Integration**: Powered by the Rig framework, supporting multiple providers (currently Gemini)
- **Interactive Agent**: Real-time conversational interface with an AI agent
- **Conversation History**: Persistent chat history stored in JSONL format
- **Modular Architecture**: Clean separation of concerns with extensible agent and history systems
- **Configuration Management**: Environment-based configuration with dotenv support
- **Tool Registry**: Foundation for tool integration (extensible for future enhancements)
- **Async Processing**: Built with Tokio for efficient asynchronous operations

## Architecture

The application follows a modular architecture designed for maintainability and extensibility:

```
src/
├── main.rs              # Application entry point and initialization
├── config.rs            # Configuration management with CLI args and environment variables
├── tools.rs             # Tool registry and tool trait definitions
└── agent/
    ├── mod.rs           # Agent trait and module exports
    ├── basic.rs         # BasicAgent implementation using Rig for LLM interactions
    └── history.rs       # History management with JSONL storage
```

### Key Components

- **Config Module**: Handles CLI arguments, environment variables, and configuration resolution
- **Agent Module**: Defines the `Agent` trait and provides implementations like `BasicAgent`
- **History Module**: Manages conversation persistence with pluggable storage backends
- **Tools Module**: Registry for tools that can be used by agents (prepared for future LLM tool calling)
- **Main Module**: Orchestrates initialization, tool registration, and agent execution

## Installation

### Prerequisites

- Rust 1.70 or later
- A valid API key for the LLM provider (e.g., Gemini API key)

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

The application supports configuration via CLI arguments and environment variables:

**Environment Variables:**
- `GEMINI_API_KEY` – Your Gemini API key (required)
- `OUTPUT_DIR` – Directory for output files (defaults to "output")

**CLI Arguments:**
- `--model <MODEL>`: Model to use (e.g., "gemini-flash-001")
- `--provider <PROVIDER>`: Provider to use (currently "gemini")
- `--output-dir <DIR>`: Output directory (overrides OUTPUT_DIR)
- `--history-type <TYPE>`: History storage type (currently "jsonl")
- `--api-key <KEY>`: API key (overrides GEMINI_API_KEY)

Create a `.env` file in the project root:

```env
GEMINI_API_KEY=your_api_key_here
OUTPUT_DIR=output
```

### Running the Application

```bash
# Start interactive session with default configuration
./wispy-fog

# Specify model and output directory
./wispy-fog --model gemini-3-flash-preview --output-dir ./logs

# Get help
./wispy-fog --help
```

### Interactive Session

The application provides a simple prompt-response interface:

```
Starting up
Using configuration: model=gemini-3-flash-preview, provider=Gemini
Registered tool: echo - A simple tool that echoes back the input. Usage: echo <message>
Beginning interactive session
prompt: Hello, how are you?
Response: I'm doing well, thank you for asking! How can I help you today?
prompt: exit
```

## Development

### Dependencies

- `rig-core`: LLM framework for provider integrations
- `tokio`: Async runtime
- `serde`/`serde_json`: Serialization for history storage
- `clap`: Command-line argument parsing
- `dotenv`: Environment variable management
- `env_logger`/`log`: Logging
- `anyhow`: Error handling
- `async-trait`: Async trait support

### Extending the Agent System

#### Adding New History Backends

1. Implement the `History` trait in `agent/history.rs`
2. Add a new variant to `HistoryType` in `config.rs`
3. Update the history creation logic in `main.rs`

#### Adding New LLM Providers

1. Add a new variant to `ModelProvider` in `config.rs`
2. Update `BasicAgent::new` to handle the new provider
3. Ensure the Rig client supports the provider

#### Adding Tools

1. Implement the `Tool` trait for new tools in `tools.rs`
2. Register the tool in `main.rs`
3. (Future) Integrate with Rig's tool calling when available

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

The application uses `anyhow::Result` for comprehensive error handling. Errors are propagated from the agent layer and displayed to users with context.

## License

This project is licensed under the terms specified in the LICENSE file.

## History Storage

Conversations are stored in `history.jsonl` within the output directory. Each entry is a JSON object representing a message in the conversation. The JSONL format allows for easy parsing and analysis.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Ensure `cargo build` and `cargo test` pass
6. Submit a pull request

## API Reference

- [Rig Framework Documentation](https://docs.rs/rig-core/latest/rig_core/)
- [Google Gemini API](https://ai.google.dev/docs)