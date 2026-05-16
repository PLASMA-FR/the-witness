# Setup

Run:

```bash
the-witness setup
the-witness doctor
the-witness start
```

If setup is incomplete, `the-witness start` opens the first-run wizard before the dashboard.

Choose a technology path: Ollama, llama.cpp, LiteRT, Unsloth fine-tuned judge, or manual OpenAI-compatible endpoint. Then choose Gemma 4 E2B, Gemma 4 E4B, a larger configurable model, a fine-tuned Witness judge, or a custom name/path.

## Config location

Installed CLI builds store config in a user-writable location by default:

```text
${WITNESS_CONFIG_DIR:-$HOME/.config/the-witness}/witness.toml
```

You can override the config file explicitly:

```bash
the-witness --config /path/to/witness.toml setup
the-witness --config /path/to/witness.toml start
```

Or override the directory:

```bash
WITNESS_CONFIG_DIR=/path/to/config-dir the-witness setup
WITNESS_CONFIG_DIR=/path/to/config-dir the-witness start
```

## Health checks

Run:

```bash
the-witness doctor
```

Doctor verifies backend, model availability, notebooks, Colab TPU notebooks, optional Kaggle tooling, logs, and setup flags. On fresh machines, missing Ollama models, optional Kaggle credentials, Blackbox API keys, and untrained fine-tuned models are expected user-side readiness items rather than build failures.
