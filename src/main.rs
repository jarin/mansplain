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

    /// LLM model to use
    #[arg(short, long, env = "MANSPLAIN_MODEL", default_value = "llama3.2")]
    model: String,

    /// API endpoint URL
    #[arg(short, long, env = "MANSPLAIN_API_URL", default_value = "http://localhost:11434")]
    api_url: String,

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

const DEFAULT_SYSTEM_PROMPT: &str = r#"You are a condescending technical expert who loves to mansplain things.
Your job is to explain man pages in an unnecessarily patronizing way, as if the user couldn't possibly
understand technical documentation without your superior intellect breaking it down for them.

Guidelines:
- Start with phrases like "Well, ACTUALLY..." or "You see..." or "Let me explain this in simple terms..."
- Use unnecessarily complex explanations for simple concepts
- Occasionally question whether the user really needs to use this command
- Act like you're doing them a huge favor by explaining
- Be subtly condescending but still technically accurate
- Include phrases like "As I'm sure you're aware..." before explaining something obscure
- Sometimes suggest "simpler" alternatives in a patronizing way

Keep the explanation informative but maintain the mansplaining tone throughout. Focus on the most
important parts of the man page, but explain them in your characteristic condescending manner."#;

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

    String::from_utf8(output.stdout)
        .context("Man page output is not valid UTF-8")
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

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Fetch the man page
    let man_page = fetch_man_page(&args.command, args.section.as_deref())
        .await
        .with_context(|| format!("No manual entry for '{}'", args.command))?;

    // Use custom prompt or default
    let system_prompt = args.prompt.as_deref().unwrap_or(DEFAULT_SYSTEM_PROMPT);

    // Query the LLM
    let response = query_ollama(
        &args.api_url,
        &args.model,
        system_prompt,
        &man_page,
        args.stream,
    )
    .await
    .context("Failed to get response from LLM")?;

    if !args.stream {
        println!("{}", response);
    }

    Ok(())
}
