use anyhow::Result;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use figlet_rs::FIGfont;
use rayon::prelude::*;
use std::process::Command;
use which::which;

#[derive(Clone, Copy)]
struct AiTool {
    name: &'static str,
    command: &'static str,
    description: &'static str,
}

const TOOLS: &[AiTool] = &[
    AiTool {
        name: "OpenCode",
        command: "opencode",
        description: "Open source AI coding agent",
    },
    AiTool {
        name: "Kilo",
        command: "kilo",
        description: "Interactive CLI coding assistant",
    },
    AiTool {
        name: "Cline",
        command: "cline",
        description: "Autonomous coding agent",
    },
    AiTool {
        name: "Gemini CLI",
        command: "gemini",
        description: "Google Gemini AI assistant",
    },
    AiTool {
        name: "Qwen",
        command: "qwen",
        description: "Alibaba Qwen AI assistant",
    },
    AiTool {
        name: "Claude CLI",
        command: "claude",
        description: "Anthropic Claude AI assistant",
    },
    AiTool {
        name: "Copilot CLI",
        command: "copilot-cli",
        description: "GitHub Copilot for terminal",
    },
    AiTool {
        name: "Ollama",
        command: "ollama",
        description: "Run LLMs locally",
    },
    AiTool {
        name: "LM Studio",
        command: "lmstudio",
        description: "Local LLM runner",
    },
    AiTool {
        name: "Mistral Vibe CLI",
        command: "vibe",
        description: "Mistral AI assistant",
    },
    AiTool {
        name: "Codex",
        command: "codex",
        description: "CLI made by OpenAI",
    },
];

fn run_tool(command: &str) -> Result<()> {
    let mut child = Command::new(command)
        .env("TERM", "xterm-256color")
        .spawn()?;
    child.wait()?;
    Ok(())
}

fn print_banner() {
    println!();
    let font = FIGfont::standard().unwrap();
    if let Some(figure) = font.convert("AIRun") {
        for line in figure.to_string().lines() {
            println!("  {}", line.cyan().bold());
        }
    }
    println!("\n  Launch any AI CLI in seconds\n");
}

fn main() -> Result<()> {
    print_banner();

    let installed: Vec<AiTool> = TOOLS
        .par_iter()
        .filter(|t| which(t.command).is_ok())
        .copied()
        .collect();

    if installed.is_empty() {
        println!("{}", "  No AI tools found on your system.".yellow());
        println!("\n  Install one of the following tools:");
        for tool in TOOLS {
            println!("    • {} - {}", tool.name.white().bold(), tool.description);
        }
        println!("\n  Check your PATH or install an AI tool first.\n");
        return Ok(());
    }

    let items: Vec<String> = installed
        .iter()
        .map(|tool| format!("{} - {}", tool.name, tool.description))
        .collect();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Type to search for an AI tool")
        .default(0)
        .items(&items)
        .interact()?;

    let tool = &installed[selection];
    println!("\n  Launching {}...\n", tool.name.cyan().bold());
    run_tool(tool.command)?;
    println!("\n  Returned from {}.\n", tool.name.cyan());
    Ok(())
}
