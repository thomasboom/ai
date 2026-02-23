use anyhow::{bail, Result};
use clap::Parser;
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
        name: "Cursor CLI",
        command: "agent",
        description: "Cursor AI agent CLI",
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

#[derive(Parser)]
#[command(name = "airun")]
#[command(about = "Launch any AI CLI in seconds", long_about = None)]
#[command(version)]
struct Args {
    tool: Option<String>,
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

fn run_tool(command: &str, args: &[String]) -> Result<()> {
    let mut child = Command::new(command)
        .args(args)
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

fn find_tool_by_name(name: &str) -> Option<AiTool> {
    let name_lower = name.to_lowercase();
    TOOLS
        .iter()
        .find(|t| {
            t.command == name_lower
                || t.name.to_lowercase() == name_lower
                || t.name.to_lowercase().replace(' ', "") == name_lower
                || t.name.to_lowercase().replace(" cli", "") == name_lower
        })
        .copied()
}

fn main() -> Result<()> {
    let args = Args::parse();

    let installed: Vec<AiTool> = TOOLS
        .par_iter()
        .filter(|t| which(t.command).is_ok())
        .copied()
        .collect();

    if let Some(tool_name) = args.tool {
        let tool = match find_tool_by_name(&tool_name) {
            Some(t) => t,
            None => bail!("Unknown tool: {}", tool_name),
        };

        if !installed.iter().any(|t| t.command == tool.command) {
            bail!("Tool '{}' is not installed or not in PATH", tool.name);
        }

        println!("Launching {}...\n", tool.name.cyan().bold());
        run_tool(tool.command, &args.args)?;
        return Ok(());
    }

    print_banner();

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
    run_tool(tool.command, &[])?;
    println!("\n  Returned from {}.\n", tool.name.cyan());
    Ok(())
}
