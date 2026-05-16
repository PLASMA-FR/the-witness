# The Witness docs

Start here if you want more detail than the top-level README.

## Main user docs

- [Command and operations guide](commands.md): full command reference with install, setup, models, endpoints, proxy usage, Blackbox, logs, fine-tuning, troubleshooting, and development commands.
- [Feature guide](features.md): what The Witness can do, how features map to TUI/CLI, and current MVP status.
- [Setup guide](setup.md): first-run setup, config location, and health checks.
- [Architecture](architecture.md): proxy, judge, repair, storage, and TUI data flow.

## Hackathon/docs

- [Technology tracks](tracks.md): Ollama, llama.cpp, LiteRT, and Unsloth mapping.
- [Demo script](demo_script.md): step-by-step demo flow.
- [Hackathon writeup](hackathon_writeup.md): project positioning and story.

## Fine-tuning and Kaggle

- [Kaggle CLI guide](kaggle_cli.md): local Kaggle auth/upload/download notes.
- [User completion guide](user_completion_guide.md): what remains user-side for training, upload, and final model testing.

## Quick commands

```bash
# Install
curl -fsSL https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.sh | bash

# Setup and TUI
the-witness setup
the-witness doctor
the-witness start

# Models
ollama pull gemma4:e2b
ollama pull gemma4:e4b
the-witness model list
the-witness model test --backend demo --model demo-judge

# Blackbox endpoint
export BLACKBOX_API_KEY="YOUR_KEY_HERE"
the-witness endpoint add-blackbox
the-witness start --proxy-addr 127.0.0.1:8787
```

Do not commit real secrets, `.env`, `kaggle.json`, `.safetensors`, `.gguf`, or generated model outputs.
