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
| OpenCode | `opencode` | Made by OpenCode |
| Kilo | `kilo` | Made by Kilo |
| Cline | `cline` | Made by Rethink |
| Cursor CLI | `agent` | Made by Cursor |
| Gemini CLI | `gemini` | Made by Google |
| Qwen | `qwen` | Made by Alibaba |
| Claude CLI | `claude` | Made by Anthropic |
| Copilot CLI | `copilot-cli` | Made by GitHub/Microsoft |
| Ollama | `ollama` | Made by Ollama |
| LM Studio | `lmstudio` | Made by LM Studio |
| Mistral Vibe CLI | `vibe` | Made by Mistral AI |
| Codex | `codex` | Made by OpenAI |
| Amp | `amp` | Made by Amp |
| Auggie CLI | `auggie` | Made by Auggie |
| Autohand Code | `autohand` | Made by Autohand |
| CodeBuddy Code | `codebuddy` | Made by CodeBuddy |
| Corust Agent | `corust` | Made by Corust |
| Factory Droid | `droid` | Made by Factory AI |
| GitHub Copilot CLI | `gh copilot` | Made by Microsoft |
| Junie | `junie` | Made by JetBrains |
| Kimi CLI | `kimi` | Made by Moonshot AI |
| Qodo CLI | `qodo` | Made by Qodo |
| Stakpak | `stakpak` | Made by Stakpak |
| Goose | `goose session` | Made by Scale AI |
| Codebuff | `codebuff` | Made by Codebuff |
| Freebuff | `freebuff` | Made by Codebuff |

## Installation

### From crates.io

```bash
cargo install airun
```

### From source

```bash
git clone https://codeberg.org/thomasboom/AIRun
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
