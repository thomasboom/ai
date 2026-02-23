# AIRun

Launch any AI CLI in seconds.

AIRun detects installed AI CLIs on your system and lets you pick one from an interactive fuzzy search menu. No more remembering different commands or typing long CLI names.

## Features

- **Auto-detection** - Scans your PATH for supported AI tools
- **Fuzzy search** - Quickly find and select tools with fuzzy matching
- **Parallel scanning** - Fast detection using parallel processing
- **Colorful UI** - Clean, colored terminal interface
- **Zero config** - Works out of the box, no setup required

## Supported Tools

| Tool | Command | Description |
|------|---------|-------------|
| OpenCode | `opencode` | Open source AI coding agent |
| Kilo | `kilo` | Interactive CLI coding assistant |
| Cline | `cline` | Autonomous coding agent |
| Cursor CLI | `agent` | Cursor AI agent CLI |
| Gemini CLI | `gemini` | Google Gemini AI assistant |
| Qwen | `qwen` | Alibaba Qwen AI assistant |
| Claude CLI | `claude` | Anthropic Claude AI assistant |
| Copilot CLI | `copilot-cli` | GitHub Copilot for terminal |
| Ollama | `ollama` | Run LLMs locally |
| LM Studio | `lmstudio` | Local LLM runner |
| Mistral Vibe CLI | `vibe` | Mistral AI assistant |
| Codex | `codex` | CLI made by OpenAI |

## Installation

### From crates.io

```bash
cargo install airun
```

### From source

```bash
git clone https://github.com/ThomasNowProductions/AIRun
cd AIRun
cargo install --path .
```

## Usage

Simply run:

```bash
airun
```

AIRun will display an interactive menu with all detected AI tools. Use arrow keys or type to fuzzy search, then press Enter to launch.

If no AI tools are found, AIRun will display a list of supported tools you can install.

## Building

```bash
cargo build --release
```

The binary will be available at `target/release/airun`.

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests.

## License

AGPL-3.0-only
