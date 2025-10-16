# mansplain

A Rust CLI tool that takes man pages and explains them to you in a slightly condescending, mansplaining tone using LLMs (Ollama, Perplexity, OpenAI, and compatible providers).
Claude wrote most of this stuff. It looks fairly legit, but if it blows up your computer or eats your tokens, don't blame me. I have not validated the instructions below.

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
mansplain --model sonar --provider perplexity --api-key <API_KEY>  yes
```

##### Result
```text
NAME
       yes - Oh, for goodness' sake, this command just repeatedly says "yes" (or whatever you tell it to say) until you stop it. It's like a very obedient parrot, but without the annoying squawking.

SYNOPSIS
       yes [STRING]

DESCRIPTION
       Okay, so listen carefully, because I'm only explaining this once. The `yes` command is like a perpetual motion machine for saying "yes" or any other string you give it. You type `yes` in your terminal, and voilà An endless stream of "y"s (or whatever you specified) will pour onto your screen. Imagine it like a never-ending loop of Dadaist poetry, but less sophisticated. It's so simple, even a child could use it (and probably will, if they get bored enough).

       Now, you might wonder, "Why on Earth would I want to do that?" Well, my naive friend, it's useful in automation. Think of it like launching a spacecraft into orbit—once it's started, it keeps going until you intervene. When you pipe its output into another command, it can automatically confirm actions for you, like saying "yes" to a series of prompts. It's like having a personal assistant, but without the annoying attitude.

       This is the tricky part (and I use that term loosely): you have to stop it manually with `Ctrl + C`. It's like turning off a faucet, but instead of water, it's a flood of text.

       Anyway, let's move on before you get too confused.

OPTIONS
       Now, these are called "options" - think of them like toppings on a pizza, okay? You don't HAVE to use them, but they change how the command works. Here are a few:

       - **`yes --help`**: This is like asking for directions. It shows you help information, which is useful if you've forgotten what the command does (which is entirely possible given its simplicity).
       - **`yes --version`**: If you're curious about what version of `yes` you're using (perhaps you want to see if it's been updated to include more affirmative responses), this is the option for you.

EXAMPLES
       Let's hold your hand through this with some examples that even YOU can understand:

       **1. Automating a "yes" to every query:**
       Imagine you're deleting a bunch of files and don't want to confirm each one. You can use `yes` like this:
       ```bash
       yes | rm -i *.txt
       ```
       This will automatically say "yes" to deleting each file, saving you from having to type it out like a robot.

       **2. Creating a dummy file with repetitive content:**
       Want to create a file with 100 lines of "Hello World"? Try this:
       ```bash
       yes "Hello World" | head -n 100 > dummy.txt
       ```
       This is like laying down tracks on a record player, but instead of music, it's text.

       **3. Watching paint dry:**
       If you want to see how fast your computer can spit out "y"s, just run:
       ```bash
       yes
       ```
       And watch as your screen fills up. It's mesmerizing, like watching quantum particles in a superposition of states.

SEE ALSO
       jot(1), seq(1)

HISTORY
       Ah, the history of `yes`. It's like trying to explain the meaning of life to a kitten. The `yes` command first appeared in Version 7 AT&T UNIX, which is ancient in computer years. You know, back when computers were as big as cars and almost as reliable as a politician's promise.

       Now, I could go on about how it was developed by some genius programmer who wanted to automate the process of saying "yes" to every prompt. But let's skip that part and talk about something more interesting, like the intricacies of Buddhist philosophy or how to make a decent cup of tea.

       Anyway, `yes` has been around for ages, and it's still useful today for those who want to automate repetitive tasks. Just remember, Mark Twain once said, "The difference between the right word and the almost right word is the difference between lightning and a lightning bug." In this case, `yes` is definitely the right word.

       Now, if you'll excuse me, I have more important things to attend to. Like napping. Or reorganizing my sock drawer.
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
5. Many models will ignore the instructions and ask for followups and be polite. It can't be helped, apparently

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
