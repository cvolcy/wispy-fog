# Wispy Fog

Wispy Fog is a Rust-based AI agent framework that uses the Rig ecosystem to power interactive chat sessions with LLM providers. It provides a CLI application, tool registration, conversation history persistence, and a small extensible architecture for future tool and provider support.

## Features

- **LLM Provider Support**: Works with Gemini and Ollama providers via Rig
- **Interactive CLI Agent**: Chat with an agent using a prompt/response loop
- **Conversation History**: Saves history to `history.jsonl` in the configured output directory
- **Tool Registry**: Built-in tools include `echo`, `write_file`, `read_file`, and `terminal`
- **Skill Loader**: Dynamically loads `SKILL.md` tools from `output/skills`
- **Config via CLI and `.env`**: Supports runtime configuration with environment fallback
- **Async Runtime**: Uses Tokio for async operations and tool execution

## Architecture

```
src/
├── main.rs                # Application entry point and orchestration
├── config.rs              # CLI args, environment variables, and resolved app configuration
├── agents/
│   ├── basic.rs           # BasicAgent implementation using Rig providers
│   ├── history.rs         # JSONL history persistence implementation
│   └── mod.rs             # Agent trait and exports
└── tools/
    ├── mod.rs             # Tool registry and dynamic tool management
    ├── echo.rs            # Echo tool implementation
    ├── write_file.rs      # Write-file tool implementation
    ├── read_file.rs       # Read-file tool implementation
    ├── terminal.rs        # Terminal command tool implementation
    └── skillmd.rs         # SkillMD-based dynamic tool loader
```

## Installation

### Prerequisites

- Rust toolchain (1.70 or later recommended)
- Docker installed if using `read_file` or `terminal` tools
- A valid provider configuration (Gemini API key or Ollama base URL)

### Build from Source

```bash
git clone <repository-url>
cd wispy-fog
cargo build --release
```

The binary will be available at `target/release/wispy-fog`.

## Usage

### Configuration

The application supports both CLI arguments and environment variables.

**Environment Variables:**
- `GEMINI_API_KEY` – Gemini API key for the Gemini provider
- `OLLAMA_API_BASE_URL` – Ollama base URL for the Ollama provider
- `OUTPUT_DIR` – Output directory for history and tools (default: `output`)

**CLI Arguments:**
- `--model <MODEL>`: Model to use (e.g. `gemini-3-flash-preview`)
- `--provider <PROVIDER>`: Provider to use (`gemini` or `ollama`)
- `--output-dir <DIR>`: Output directory (overrides `OUTPUT_DIR`)
- `--history-type <TYPE>`: History storage type (`jsonl`)
- `--api-key <KEY>`: API key or provider-specific credential (overrides env value)

Example `.env`:

```env
GEMINI_API_KEY=your_gemini_key_here
OUTPUT_DIR=output
```

### Running the Application

```bash
cargo run --release
```

or:

```bash
cargo run --release -- -p ollama --model gemma4:latest --output-dir ./logs
```

### Interactive Session

The app launches a prompt-based interactive loop. Example output:

```text
starting wispy-fog
configuration: model=gemini-3-flash-preview, provider=Gemini
registered 4 tool(s)
prompt: Hello
Response: ...
prompt: exit
```

## Tools

The current tool registry includes:

- `EchoTool` – echoes the supplied message
- `WriteFileTool` – writes text files in the output directory
- `ReadFileTool` – reads file content through a sandboxed Docker command
- `TerminalTool` – executes commands inside a Docker container
- `SkillMD` loader – loads tools from `SKILL.md` files in `output/skills`

## Development

### Dependencies

- `rig` / `rig-core` – LLM provider integration framework
- `tokio` – async runtime
- `serde`, `serde_json`, `serde_yaml` – serialization and metadata
- `clap` – CLI argument parsing
- `dotenv` – environment variable loading
- `env_logger`, `log` – logging
- `anyhow` – error handling

### Running Tests

```bash
cargo test
```

### Building

```bash
cargo build
cargo build --release
cargo check
```

## History Storage

Conversation history is written to `history.jsonl` in the configured output directory. Each line is a JSON object representing a chat message.

## Provider Notes

- `Gemini` uses `GEMINI_API_KEY`
- `Ollama` uses `OLLAMA_API_BASE_URL`

If an unrecognized provider is provided, the app defaults to `Gemini`.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests where appropriate
5. Run `cargo build` and `cargo test`
6. Open a pull request

## License

See the `LICENSE` file for license details.
