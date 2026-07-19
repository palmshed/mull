# Mull

Bring Mull into your terminal. Fast, flicker-free CLI built for plans, subagents, and parallel work.

**[Homepage](https://palmshed.ai/cli)** | **[Documentation](https://docs.palmshed.ai/build/overview)**

## Install

```bash
curl -fsSL https://palmshed.ai/cli/install.sh | bash
```

Or install with npm:

```bash
npm i -g @palmshed/mull
```

## Get Started

```bash
# Launch the interactive TUI
mull

# Run a single task
mull -p "Explain this codebase"
```

On first launch, Mull opens your browser to authenticate. For CI or headless environments, use an API key from [console.palmshed.ai](https://console.palmshed.ai):

```bash
export PALMSHED_API_KEY="palmshed-..."
```

## Update

```bash
mull update
```

Or if installed via npm:

```bash
npm i -g @palmshed/mull@latest
```

## Supported Platforms

| Platform | Architecture |
|---|---|
| macOS | Apple Silicon (arm64) |
| Linux | x86_64, arm64 |
| Windows | x86_64 |

## Documentation

For full documentation including configuration, MCP servers, custom models, headless mode, agent mode, and more, visit [docs.palmshed.ai/build/overview](https://docs.palmshed.ai/build/overview).

## Feedback

Run `/feedback` inside Mull to report issues or send feedback directly.
