# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`mansplain` is a Rust CLI tool that fetches man pages and pipes them through an LLM (Ollama, Perplexity, OpenAI, or compatible providers) to generate parodically condescending, "mansplaining" explanations. It works similarly to the `man` command but with added personality.

## Tool Management

This repository uses [mise](https://mise.jdx.dev/) for tool version management.

Tool versions (specified in `mise.toml`):
- Node.js: latest
- uv (Python package installer): latest

Note: Rust is not managed via mise in this project. Install via [rustup](https://rustup.rs/) or your preferred method.

Setup:
```bash
mise install
```

## Build Commands

Build for development:
```bash
cargo build
```

Build for release (optimized):
```bash
cargo build --release
```

Run the binary:
```bash
# Development build
cargo run -- ls

# Release build (faster)
./target/release/mansplain ls
```

Run tests:
```bash
cargo test
```

Check code without building:
```bash
cargo check
```

Format code:
```bash
cargo fmt
```

Lint with clippy:
```bash
cargo clippy
```

## Project Architecture

### Main Components

- `src/main.rs` - Single-file CLI application containing:
  - **CLI argument parsing** using `clap` with derive macros
  - **Man page fetching** via subprocess calling the `man` command
  - **Multi-provider LLM API integration**:
    - Ollama API (default)
    - OpenAI-compatible API (Perplexity, OpenAI, and others)
  - **Streaming support** for real-time output

### Dependencies

Core dependencies (see `Cargo.toml`):
- `clap` - Command-line argument parsing with environment variable support
- `tokio` - Async runtime for HTTP requests
- `reqwest` - HTTP client for LLM API calls
- `serde` / `serde_json` - JSON serialization/deserialization
- `anyhow` - Error handling

### Configuration

The tool supports multiple configuration methods (in order of precedence):

1. **Command-line arguments**:
   - `--provider` - LLM provider (ollama, perplexity, openai) (default: ollama)
   - `--model` / `-m` - LLM model name
   - `--api-url` / `-a` - API endpoint URL
   - `--api-key` / `-k` - API key (for Perplexity, OpenAI, etc.)
   - `--prompt` / `-p` - Custom system prompt
   - `--stream` / `-s` - Enable streaming output

2. **Environment variables**:
   - `MANSPLAIN_PROVIDER` - LLM provider
   - `MANSPLAIN_MODEL` - LLM model name
   - `MANSPLAIN_API_URL` - API endpoint URL
   - `MANSPLAIN_API_KEY` - API key
   - `MANSPLAIN_PROMPT` - Custom system prompt

3. **Defaults** (by provider):
   - **Ollama**: Model `gemma3:12b`, API URL `http://localhost:11434`
   - **Perplexity**: Model `llama-3.1-sonar-small-128k-online`, API URL `https://api.perplexity.ai`
   - **OpenAI**: Model `gpt-4`, API URL `https://api.openai.com/v1`
   - Prompt: See `DEFAULT_SYSTEM_PROMPT` in main.rs or `prompt.txt`

### Customizing the Prompt

The default mansplaining prompt can be customized in three ways:

1. **Edit prompt.txt** - Template file for reference/sharing
2. **Use --prompt flag** - Pass custom prompt directly: `mansplain ls --prompt "Your custom prompt"`
3. **Set MANSPLAIN_PROMPT** - Export as environment variable

The prompt defines the LLM's personality and how it explains man pages.

## Usage Examples

Basic usage:
```bash
mansplain ls
```

With man section:
```bash
mansplain 1 printf
```

Using a different model:
```bash
mansplain grep --model llama3.1
```

Custom API endpoint:
```bash
mansplain curl --api-url http://localhost:8080
```

With streaming output:
```bash
mansplain tar --stream
```

Using environment variables:
```bash
export MANSPLAIN_MODEL="mistral"
export MANSPLAIN_API_URL="http://api.example.com"
mansplain vim
```

## Development Notes

### Adding New Features

- The codebase is intentionally kept as a single-file CLI for simplicity
- If the file grows too large, consider splitting into modules (e.g., `api.rs`, `cli.rs`, `man.rs`)
- All async operations use tokio's runtime

### LLM API Integration

Supports two API formats:

1. **Ollama API** (`/api/generate` endpoint) - via `query_ollama` function
2. **OpenAI-compatible API** (`/chat/completions` endpoint) - via `query_openai_compatible` function
   - Works with Perplexity, OpenAI, and other OpenAI-compatible providers

To add support for other API formats:

1. Create new request/response structs for the API format
2. Add a new query function (similar to `query_ollama` or `query_openai_compatible`)
3. Add a new provider case in the main match statement
4. Update documentation

### Error Handling

Uses `anyhow` for error propagation. All functions return `Result<T>` and use context for better error messages.

## Testing

Prerequisites:
- Ollama must be running locally: `ollama serve`
- At least one model must be available: `ollama pull llama3.2`

Test the binary:
```bash
cargo build --release
./target/release/mansplain --help
./target/release/mansplain ls
```
