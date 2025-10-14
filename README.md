# mansplain

A Rust CLI tool that takes man pages and explains them to you in a slightly condescending, mansplaining tone using LLMs (Ollama, Perplexity, OpenAI, and compatible providers).

## Installation

### Prerequisites

1. Install Rust (via [rustup](https://rustup.rs/) or [mise](https://mise.jdx.dev/))
2. Choose your LLM provider:
   - **Ollama** (default): Install [Ollama](https://ollama.ai/) and pull a model: `ollama pull llama3.2`
   - **Perplexity**: Get an API key from [Perplexity](https://www.perplexity.ai/)
   - **OpenAI**: Get an API key from [OpenAI](https://openai.com/)

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
      --provider <PROVIDER>  LLM provider to use (ollama, perplexity, openai) [default: ollama]
  -m, --model <MODEL>        LLM model to use
  -a, --api-url <API_URL>    API endpoint URL (for Ollama or custom OpenAI-compatible endpoints)
  -k, --api-key <API_KEY>    API key (for Perplexity, OpenAI, etc.)
  -p, --prompt <PROMPT>      Custom system prompt
  -s, --stream               Use streaming output (shows text as it's generated)
  -h, --help                 Print help
```

### Examples

#### Using Ollama (default)

```bash
mansplain grep --model mistral
mansplain vim --stream
```

#### Using Perplexity

```bash
export MANSPLAIN_API_KEY="your-perplexity-api-key"
mansplain awk --provider perplexity
mansplain grep --provider perplexity --model sonar
```

#### Using OpenAI

```bash
export MANSPLAIN_API_KEY="your-openai-api-key"
mansplain curl --provider openai
mansplain tar --provider openai --model gpt-4o
```

#### Custom API endpoint

```bash
mansplain sed --api-url http://my-llm-server:8080
```

### Environment Variables

You can set defaults using environment variables:

```bash
export MANSPLAIN_PROVIDER="perplexity"       # LLM provider (ollama, perplexity, openai)
export MANSPLAIN_MODEL="llama3.1"            # Model name
export MANSPLAIN_API_URL="http://..."       # API endpoint URL
export MANSPLAIN_API_KEY="your-api-key"     # API key (for Perplexity, OpenAI)
export MANSPLAIN_PROMPT="Your custom prompt" # Custom system prompt
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
2. Sends the man page content to your chosen LLM provider (Ollama, Perplexity, or OpenAI)
3. The LLM explains it in a condescending, mansplaining tone
4. Displays the explanation to you

## Requirements

- Rust 1.86.0 or later
- System `man` command available
- One of the following:
  - **Ollama**: Running instance (default)
  - **Perplexity**: API key
  - **OpenAI**: API key

## Provider Setup

### Ollama (Default)

If you get a connection error, make sure Ollama is running:

```bash
# Start Ollama
ollama serve

# In another terminal, pull a model if you haven't already
ollama pull llama3.2

# Now run mansplain
mansplain awk
```

### Perplexity

Get an API key from [Perplexity](https://www.perplexity.ai/):

```bash
export MANSPLAIN_API_KEY="your-perplexity-api-key"
mansplain awk --provider perplexity
```

Available models: `sonar` and stuff documented at [perplexity](https://docs.perplexity.ai/getting-started/models) 

### OpenAI

Get an API key from [OpenAI](https://platform.openai.com/):

```bash
export MANSPLAIN_API_KEY="your-openai-api-key"
mansplain awk --provider openai --model gpt-4o
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
