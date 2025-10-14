# mansplain

A Rust CLI tool that takes man pages and explains them to you in a slightly condescending, mansplaining tone using LLMs like Ollama.

## Installation

### Prerequisites

1. Install Rust (via [rustup](https://rustup.rs/) or [mise](https://mise.jdx.dev/))
2. Install [Ollama](https://ollama.ai/) (or have access to a compatible LLM API)
3. Pull a model: `ollama pull llama3.2`

### Build from source

```bash
# Clone the repository
git clone <your-repo-url>
cd mansplain

# Install dependencies (if using mise)
mise install

# Build the release binary
cargo build --release

# Optional: Install to your PATH
cargo install --path .
```

## Usage

Basic usage - just like the `man` command:

```bash
mansplain awk
mansplain grep
mansplain tar
```

With man section:

```bash
mansplain 1 printf
mansplain 3 malloc
```

### Options

```bash
mansplain [OPTIONS] <COMMAND> [SECTION]

Options:
  -m, --model <MODEL>      LLM model to use [default: llama3.2]
  -a, --api-url <API_URL>  API endpoint URL [default: http://localhost:11434]
  -p, --prompt <PROMPT>    Custom system prompt
  -s, --stream             Use streaming output (shows text as it's generated)
  -h, --help               Print help
```

### Examples

Use a different model:
```bash
mansplain grep --model mistral
```

Enable streaming for real-time output:
```bash
mansplain vim --stream
```

Custom API endpoint:
```bash
mansplain curl --api-url http://my-llm-server:8080
```

### Environment Variables

You can set defaults using environment variables:

```bash
export MANSPLAIN_MODEL="llama3.1"
export MANSPLAIN_API_URL="http://localhost:11434"
export MANSPLAIN_PROMPT="Your custom system prompt here"
```

## Customizing the Prompt

The default prompt creates a condescending "mansplaining" persona. You can customize this in three ways:

1. **Edit `prompt.txt`** - This file contains the default prompt for reference
2. **Use the `--prompt` flag** - Pass a custom prompt directly
3. **Set `MANSPLAIN_PROMPT`** - Export as an environment variable

Example with custom prompt:
```bash
mansplain ls --prompt "Explain this man page like I'm a pirate: Arr matey!"
```

## How It Works

1. Fetches the man page using the system's `man` command
2. Sends the man page content to an LLM (Ollama by default)
3. The LLM explains it in a condescending, mansplaining tone
4. Displays the explanation to you

## Requirements

- Rust 1.86.0 or later
- A running Ollama instance (or compatible LLM API)
- System `man` command available

## Starting Ollama

If you get a connection error, make sure Ollama is running:

```bash
# Start Ollama
ollama serve

# In another terminal, pull a model if you haven't already
ollama pull llama3.2

# Now run mansplain
mansplain awk
```

## Development

See [CLAUDE.md](CLAUDE.md) for detailed development documentation.

Build for development:
```bash
cargo build
```

Run tests:
```bash
cargo test
```

Run with cargo:
```bash
cargo run -- awk
```

## License

MIT (or your preferred license)
