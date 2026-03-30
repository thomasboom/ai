use anyhow::{bail, Result};
use clap::Parser;
use colored::Colorize;
use console::{Key, Term};
use dirs::config_dir;
use figlet_rs::FIGfont;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io::Write;
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
        description: "Made by Kilo AI",
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

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    pinned_tools: Vec<String>,
    show_ascii: bool,
    show_shortcuts: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            pinned_tools: Vec::new(),
            show_ascii: true,
            show_shortcuts: true,
        }
    }
}

impl Config {
    fn path() -> std::path::PathBuf {
        config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("airun")
            .join("config.json")
    }

    fn load() -> Self {
        let path = Self::path();
        if path.exists() {
            std::fs::read_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            Self::default()
        }
    }

    fn save(&self) -> Result<()> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}

#[derive(Parser)]
#[command(name = "airun")]
#[command(about = "Launch any AI CLI in seconds", long_about = None)]
#[command(version)]
struct Args {
    tool: Option<String>,
    #[arg(long, help = "Create 'ai' symlink pointing to airun")]
    remap: bool,
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

fn run_tool(command: &str, prepended_args: &[&str], user_args: &[String]) -> Result<()> {
    print!("\x1B[2J\x1B[1H\x1B[3J");
    std::io::Write::flush(&mut std::io::stdout()).ok();
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
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("AIRun").unwrap();
    let banner = figure.to_string();
    for line in banner.lines() {
        println!("{}", line.cyan().bold());
    }
    println!();
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

fn show_settings_menu(config: &Config) -> Result<()> {
    let term = Term::stdout();
    let mut selection = 0;
    let mut show_ascii = config.show_ascii;
    let mut show_shortcuts = config.show_shortcuts;

    let settings = [
        ("Show ASCII banner", &mut show_ascii),
        ("Show shortcut explainer", &mut show_shortcuts),
    ];

    loop {
        print!("\x1B[2J\x1B[H");
        std::io::stdout().flush().ok();
        println!();
        println!("  {} {} Settings", "AIRun".cyan().bold(), "›".dimmed());
        println!();
        println!("  Toggle settings with ↵, press esc to return");
        println!();

        for (i, (name, value)) in settings.iter().enumerate() {
            let is_selected = i == selection;
            let prefix = if is_selected { " > " } else { "   " };
            let checkbox = if **value { "[x]" } else { "[ ]" };
            let name_color = if is_selected {
                name.cyan().bold()
            } else {
                name.white()
            };
            println!("{}{} {}", prefix, checkbox.dimmed(), name_color);
        }

        println!();
        println!("  {}  {}", "↑↓ navigate".dimmed(), "↵ toggle".dimmed());
        println!("  {}  {}", "esc save & exit".dimmed(), "".dimmed());

        let key = term.read_key().ok();

        match key {
            Some(Key::ArrowUp) => {
                if selection > 0 {
                    selection -= 1;
                }
            }
            Some(Key::ArrowDown) => {
                if selection < settings.len() - 1 {
                    selection += 1;
                }
            }
            Some(Key::Enter) => {
                *settings[selection].1 = !*settings[selection].1;
            }
            Some(Key::Escape) => {
                let mut new_config = config.clone();
                new_config.show_ascii = show_ascii;
                new_config.show_shortcuts = show_shortcuts;
                new_config.save()?;
                break;
            }
            _ => {}
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.remap {
        let exe_path = std::env::current_exe()?;
        let exe_dir = exe_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Cannot get executable directory"))?;
        let target_path = exe_dir.join("ai");

        if target_path.exists() {
            println!("{} already exists", target_path.display());
        } else {
            #[cfg(unix)]
            {
                std::os::unix::fs::symlink(&exe_path, &target_path)?;
                println!(
                    "Created symlink: {} -> {}",
                    target_path.display(),
                    exe_path.display()
                );
            }
            #[cfg(not(unix))]
            {
                bail!("Symlinks are not supported on this platform");
            }
        }
        return Ok(());
    }

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

    let config = Config::load();

    if installed.is_empty() {
        print_banner();
        println!("{}", "  No AI tools found on your system.".yellow());
        println!("{}", "  No AI tools found on your system.".yellow());
        println!("\n  Install one of the following tools:");
        for tool in TOOLS {
            println!("    • {} - {}", tool.name.white().bold(), tool.description);
        }
        println!("\n  Check your PATH or install an AI tool first.\n");
        return Ok(());
    }
    let pinned: HashSet<String> = config.pinned_tools.iter().cloned().collect();

    let mut sorted_installed: Vec<AiTool> = installed.clone();
    sorted_installed.sort_by(|a, b| {
        let a_pinned = pinned.contains(a.command);
        let b_pinned = pinned.contains(b.command);
        if a_pinned && !b_pinned {
            std::cmp::Ordering::Less
        } else if !a_pinned && b_pinned {
            std::cmp::Ordering::Greater
        } else {
            a.name.cmp(b.name)
        }
    });

    let mut selection = 0;
    let term = Term::stdout();
    let mut filter = String::new();
    loop {
        let filtered: Vec<(usize, &AiTool)> = sorted_installed
            .iter()
            .enumerate()
            .filter(|(_, t)| {
                if filter.is_empty() {
                    true
                } else {
                    let search = format!("{} {}", t.name, t.description).to_lowercase();
                    search.contains(&filter.to_lowercase())
                }
            })
            .collect();

        if filtered.is_empty() {
            println!("{}", "  No matches found.".yellow());
            return Ok(());
        }

        if selection >= filtered.len() {
            selection = filtered.len() - 1;
        }

        let (original_idx, current_tool) = filtered[selection];

        print!("\x1B[2J\x1B[H");
        std::io::stdout().flush().ok();

        if config.show_ascii {
            print_banner();
        }

        println!("  {}  {}", "Type to search:".white().bold(), filter.cyan());
        println!();

        for (i, (_, tool)) in filtered.iter().enumerate() {
            let is_selected = i == selection;
            let is_pinned = pinned.contains(tool.command);
            let marker = if is_pinned { "📌" } else { "  " };
            let prefix = if is_selected { " > " } else { "   " };
            let name_color = if is_selected {
                tool.name.cyan().bold()
            } else {
                tool.name.white().bold()
            };
            let desc_color = if is_selected {
                tool.description.cyan()
            } else {
                tool.description.white()
            };
            println!("{}{} {} - {}", prefix, marker, name_color, desc_color);
        }

        if config.show_shortcuts {
            println!();
            println!("  {}  {}", "↑↓ navigate".dimmed(), "↵ select".dimmed());
            println!("  {}  {}", "p pin/unpin".dimmed(), "s settings".dimmed());
            println!("  {}  {}", "esc exit".dimmed(), "".dimmed());
        }

        let key = term.read_key().ok();

        match key {
            Some(Key::ArrowUp) => {
                if selection > 0 {
                    selection -= 1;
                }
            }
            Some(Key::ArrowDown) => {
                if selection < filtered.len() - 1 {
                    selection += 1;
                }
            }
            Some(Key::Char('p')) | Some(Key::Char('P')) => {
                let tool_cmd = current_tool.command;
                let mut new_config = config.clone();
                if pinned.contains(tool_cmd) {
                    new_config.pinned_tools.retain(|t| t != tool_cmd);
                } else {
                    new_config.pinned_tools.push(tool_cmd.to_string());
                }
                new_config.save()?;
                return main();
            }
            Some(Key::Char('s')) | Some(Key::Char('S')) => {
                show_settings_menu(&config)?;
                return main();
            }
            Some(Key::Enter) => {
                let tool = sorted_installed[original_idx];
                let prepended_args: Vec<&str> = tool.args.map(|a| a.to_vec()).unwrap_or_default();
                println!("\n  Launching {}...", tool.name.cyan().bold());
                run_tool(tool.command, &prepended_args, &[])?;
                return Ok(());
            }
            Some(Key::Escape) => {
                return Ok(());
            }
            Some(Key::Backspace) => {
                filter.pop();
            }
            Some(Key::Char(c)) => {
                filter.push(c);
                selection = 0;
            }
            _ => {}
        }
    }
}
