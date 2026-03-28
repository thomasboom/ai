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
    args: Option<&'static [&'static str]>,
    description: &'static str,
}

const TOOLS: &[AiTool] = &[
    AiTool {
        name: "OpenCode",
        command: "opencode",
        args: None,
        description: "Made by OpenCode",
    },
    AiTool {
        name: "Kilo",
        command: "kilo",
        args: None,
        description: "Made by Kilo",
    },
    AiTool {
        name: "Cline",
        command: "cline",
        args: None,
        description: "Made by Rethink",
    },
    AiTool {
        name: "Cursor CLI",
        command: "agent",
        args: None,
        description: "Made by Cursor",
    },
    AiTool {
        name: "Gemini CLI",
        command: "gemini",
        args: None,
        description: "Made by Google",
    },
    AiTool {
        name: "Qwen",
        command: "qwen",
        args: None,
        description: "Made by Alibaba",
    },
    AiTool {
        name: "Claude CLI",
        command: "claude",
        args: None,
        description: "Made by Anthropic",
    },
    AiTool {
        name: "Copilot CLI",
        command: "copilot-cli",
        args: None,
        description: "Made by GitHub/Microsoft",
    },
    AiTool {
        name: "Ollama",
        command: "ollama",
        args: None,
        description: "Made by Ollama",
    },
    AiTool {
        name: "LM Studio",
        command: "lmstudio",
        args: None,
        description: "Made by LM Studio",
    },
    AiTool {
        name: "Mistral Vibe CLI",
        command: "vibe",
        args: None,
        description: "Made by Mistral AI",
    },
    AiTool {
        name: "Codex",
        command: "codex",
        args: None,
        description: "Made by OpenAI",
    },
    AiTool {
        name: "Amp",
        command: "amp",
        args: None,
        description: "Made by Amp",
    },
    AiTool {
        name: "Auggie CLI",
        command: "auggie",
        args: None,
        description: "Made by Auggie",
    },
    AiTool {
        name: "Autohand Code",
        command: "autohand",
        args: None,
        description: "Made by Autohand",
    },
    AiTool {
        name: "CodeBuddy Code",
        command: "codebuddy",
        args: None,
        description: "Made by CodeBuddy",
    },
    AiTool {
        name: "Corust Agent",
        command: "corust",
        args: None,
        description: "Made by Corust",
    },
    AiTool {
        name: "Factory Droid",
        command: "droid",
        args: None,
        description: "Made by Factory AI",
    },
    AiTool {
        name: "GitHub Copilot CLI",
        command: "gh",
        args: Some(&["copilot"]),
        description: "Made by Microsoft",
    },
    AiTool {
        name: "Junie",
        command: "junie",
        args: None,
        description: "Made by JetBrains",
    },
    AiTool {
        name: "Kimi CLI",
        command: "kimi",
        args: None,
        description: "Made by Moonshot AI",
    },
    AiTool {
        name: "Qodo CLI",
        command: "qodo",
        args: None,
        description: "Made by Qodo",
    },
    AiTool {
        name: "Stakpak",
        command: "stakpak",
        args: None,
        description: "Made by Stakpak",
    },
    AiTool {
        name: "Goose",
        command: "goose",
        args: Some(&["session"]),
        description: "Made by Scale AI",
    },
    AiTool {
        name: "Codebuff",
        command: "codebuff",
        args: None,
        description: "Made by Codebuff",
    },
    AiTool {
        name: "Freebuff",
        command: "freebuff",
        args: None,
        description: "Made by Codebuff",
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

fn run_tool(command: &str, prepended_args: &[&str], user_args: &[String]) -> Result<()> {
    let mut cmd = Command::new(command);
    cmd.args(prepended_args)
        .args(user_args)
        .env("TERM", "xterm-256color");
    let mut child = cmd.spawn()?;
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
                || (t.args.is_some()
                    && format!("{} {}", t.command, t.args.unwrap().join(" ")) == name_lower)
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

        let prepended_args: Vec<&str> = tool.args.map(|a| a.to_vec()).unwrap_or_default();
        println!("Launching {}...\n", tool.name.cyan().bold());
        run_tool(tool.command, &prepended_args, &args.args)?;
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
    let prepended_args: Vec<&str> = tool.args.map(|a| a.to_vec()).unwrap_or_default();
    println!("\n  Launching {}...\n", tool.name.cyan().bold());
    run_tool(tool.command, &prepended_args, &[])?;
    println!("\n  Returned from {}.\n", tool.name.cyan());
    Ok(())
}
