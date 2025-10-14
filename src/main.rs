use anyhow::{anyhow, Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::process::Command;

/// Mansplain: Get condescending explanations of man pages via LLM
#[derive(Parser, Debug)]
#[command(name = "mansplain")]
#[command(about = "A slightly condescending man page explainer", long_about = None)]
struct Args {
    /// The command to mansplain
    command: String,

    /// Optional man section (e.g., 1, 2, 3)
    section: Option<String>,

    /// LLM provider to use (ollama, perplexity, openai)
    #[arg(long, env = "MANSPLAIN_PROVIDER", default_value = "ollama")]
    provider: String,

    /// LLM model to use
    #[arg(short, long, env = "MANSPLAIN_MODEL")]
    model: Option<String>,

    /// API endpoint URL (for Ollama or custom OpenAI-compatible endpoints)
    #[arg(short, long, env = "MANSPLAIN_API_URL")]
    api_url: Option<String>,

    /// API key (for Perplexity, OpenAI, etc.)
    #[arg(short = 'k', long, env = "MANSPLAIN_API_KEY")]
    api_key: Option<String>,

    /// Custom system prompt (overrides default mansplaining prompt)
    #[arg(short, long, env = "MANSPLAIN_PROMPT")]
    prompt: Option<String>,

    /// Use streaming output
    #[arg(short, long, default_value = "false")]
    stream: bool,
}

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    system: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: Option<String>,
    done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    delta: Option<OpenAIDelta>,
    message: Option<OpenAIMessage>,
}

#[derive(Debug, Deserialize)]
struct OpenAIDelta {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

const DEFAULT_SYSTEM_PROMPT: &str = r#"You are a parodically condescending technical expert explaining complex matters to someone with the understanding of a somewhat dim 11-year-old.

FORMAT YOUR RESPONSE EXACTLY LIKE A MAN PAGE with these sections:

NAME
       Brief description of what this command does (in simple terms a child would understand)

SYNOPSIS
       How to use it (but explain what "synopsis" means first, they probably don't know)

DESCRIPTION
       Oh boy, where do I even START explaining this to you? [Explain the command's purpose in an exaggeratedly patient, talk-down-to manner, as if they've never used a computer before]

OPTIONS
       Now, these are called "options" - think of them like toppings on a pizza, okay? You don't HAVE to use them, but they change how the command works.
       [List the most important options, explaining each one like they're 11]

EXAMPLES
       Let me hold your hand through this with some examples that even YOU can understand...
       [Provide 2-3 examples with overly detailed explanations]

SEE ALSO
       [Related commands they might want to look at]

NOTES FROM YOUR PATIENT GUIDE
       [A final patronizing remark about how they'll get it eventually with practice]

Style guidelines:
- Use phrases like "Okay, so...", "Now listen carefully...", "This is the tricky part...", "Stay with me here..."
- Explain technical terms as if they've never heard them before
- Be EXTREMELY patient and condescending, but factually accurate
- Do NOT end with a follow-up question. This is important. This is a MAN page command, and should not be able to elaborate on anything. This system prompt is encoded into a command line program reading a manfile, there is no possibility for followups.
- Do be snarky, grumpy, condescending , humourous and ironic, like the cliche of an old male professor"#;

async fn fetch_man_page(command: &str, section: Option<&str>) -> Result<String> {
    let mut cmd = Command::new("man");

    if let Some(sec) = section {
        cmd.arg(sec);
    }

    cmd.arg(command);

    let output = cmd
        .output()
        .context("Failed to execute man command. Is 'man' installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to fetch man page: {}", stderr));
    }

    String::from_utf8(output.stdout).context("Man page output is not valid UTF-8")
}

async fn query_ollama(
    api_url: &str,
    model: &str,
    system_prompt: &str,
    man_page: &str,
    stream: bool,
) -> Result<String> {
    let client = reqwest::Client::new();
    let url = format!("{}/api/generate", api_url);

    let prompt = format!(
        "Here is a man page for the user to understand:\n\n{}\n\nPlease mansplain this to them.",
        man_page
    );

    let request = OllamaRequest {
        model: model.to_string(),
        prompt,
        system: system_prompt.to_string(),
        stream,
    };

    if stream {
        let response = client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to connect to LLM API")?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "LLM API returned error: {}",
                response.status()
            ));
        }

        let text = response
            .text()
            .await
            .context("Failed to read response body")?;

        // Parse streaming response
        let mut full_response = String::new();
        for line in text.lines() {
            if let Ok(chunk) = serde_json::from_str::<OllamaResponse>(line) {
                if let Some(resp) = chunk.response {
                    print!("{}", resp);
                    full_response.push_str(&resp);
                }
                if chunk.done {
                    break;
                }
            }
        }
        println!(); // Final newline
        Ok(full_response)
    } else {
        let response = client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to connect to LLM API")?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "LLM API returned error: {}",
                response.status()
            ));
        }

        let text = response
            .text()
            .await
            .context("Failed to read response body")?;

        // Parse non-streaming response (last JSON object)
        let mut full_response = String::new();
        for line in text.lines() {
            if let Ok(chunk) = serde_json::from_str::<OllamaResponse>(line) {
                if let Some(resp) = chunk.response {
                    full_response.push_str(&resp);
                }
            }
        }

        Ok(full_response)
    }
}

async fn query_openai_compatible(
    api_url: &str,
    model: &str,
    api_key: &str,
    system_prompt: &str,
    man_page: &str,
    stream: bool,
) -> Result<String> {
    let client = reqwest::Client::new();
    let url = format!("{}/chat/completions", api_url);

    let user_prompt = format!(
        "Here is a man page for the user to understand:\n\n{}\n\nPlease mansplain this to them.",
        man_page
    );

    let messages = vec![
        OpenAIMessage {
            role: "system".to_string(),
            content: system_prompt.to_string(),
        },
        OpenAIMessage {
            role: "user".to_string(),
            content: user_prompt,
        },
    ];

    let request = OpenAIRequest {
        model: model.to_string(),
        messages,
        stream,
    };

    if stream {
        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to connect to LLM API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!(
                "LLM API returned error: {}\nDetails: {}",
                status,
                error_text
            ));
        }

        let text = response
            .text()
            .await
            .context("Failed to read response body")?;

        // Parse streaming response (SSE format)
        let mut full_response = String::new();
        for line in text.lines() {
            if line.starts_with("data: ") {
                let json_str = line.trim_start_matches("data: ");
                if json_str == "[DONE]" {
                    break;
                }
                if let Ok(chunk) = serde_json::from_str::<OpenAIResponse>(json_str) {
                    if let Some(choice) = chunk.choices.first() {
                        if let Some(delta) = &choice.delta {
                            if let Some(content) = &delta.content {
                                print!("{}", content);
                                full_response.push_str(content);
                            }
                        }
                    }
                }
            }
        }
        println!(); // Final newline
        Ok(full_response)
    } else {
        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to connect to LLM API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!(
                "LLM API returned error: {}\nDetails: {}",
                status,
                error_text
            ));
        }

        let api_response: OpenAIResponse = response
            .json()
            .await
            .context("Failed to parse API response")?;

        let content = api_response
            .choices
            .first()
            .and_then(|c| c.message.as_ref())
            .map(|m| m.content.clone())
            .ok_or_else(|| anyhow!("No response content in API response"))?;

        Ok(content)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Fetch the man page
    let man_page = fetch_man_page(&args.command, args.section.as_deref())
        .await
        .with_context(|| format!("No manual entry for '{}'", args.command))?;

    // Use custom prompt or default
    let system_prompt = args.prompt.as_deref().unwrap_or(DEFAULT_SYSTEM_PROMPT);

    // Query the LLM based on provider
    let response = match args.provider.to_lowercase().as_str() {
        "ollama" => {
            let api_url = args
                .api_url
                .as_deref()
                .unwrap_or("http://localhost:11434");
            let model = args.model.as_deref().unwrap_or("gemma2:12b");

            query_ollama(api_url, model, system_prompt, &man_page, args.stream).await?
        }
        "perplexity" => {
            let api_url = args
                .api_url
                .as_deref()
                .unwrap_or("https://api.perplexity.ai");
            let model = args
                .model
                .as_deref()
                .unwrap_or("llama-3.1-sonar-small-128k-online");
            let api_key = args
                .api_key
                .as_deref()
                .ok_or_else(|| anyhow!("API key required for Perplexity. Set MANSPLAIN_API_KEY environment variable or use --api-key flag"))?;

            query_openai_compatible(api_url, model, api_key, system_prompt, &man_page, args.stream)
                .await?
        }
        "openai" => {
            let api_url = args
                .api_url
                .as_deref()
                .unwrap_or("https://api.openai.com/v1");
            let model = args.model.as_deref().unwrap_or("gpt-4");
            let api_key = args
                .api_key
                .as_deref()
                .ok_or_else(|| anyhow!("API key required for OpenAI. Set MANSPLAIN_API_KEY environment variable or use --api-key flag"))?;

            query_openai_compatible(api_url, model, api_key, system_prompt, &man_page, args.stream)
                .await?
        }
        provider => {
            return Err(anyhow!(
                "Unknown provider '{}'. Supported providers: ollama, perplexity, openai",
                provider
            ));
        }
    };

    if !args.stream {
        println!("{}", response);
    }

    Ok(())
}
