# Project Status

The Witness is a local-first Gemma 4 reliability firewall for AI endpoints. This page is a plain-language status snapshot for judges, users, and maintainers.

| Feature | Status | Notes |
|---|---|---|
| CLI | Implemented | Setup, doctor, model, endpoint, dashboard, service, replay, export, and logs command surfaces are present. |
| TUI | Implemented | First-run setup and operator screens are available through `the-witness start`. |
| Web UI | Implemented | Local dashboard at `http://127.0.0.1:8790` with dashboard, endpoints, requests, repair, review, models, logs, doctor, and settings views. |
| Dashboard API | Implemented | Localhost control API covers health, config/settings, models, endpoints, requests, logs, doctor, and proxy controls. |
| Linux installer | Implemented | `scripts/install.sh` builds the release binary and prints model/setup next steps. |
| Windows installer | Implemented | `scripts/install.ps1` builds the release binary and prints PowerShell-specific next steps. |
| macOS support | Implemented path | Uses the same Rust binary and shell installer; run `the-witness doctor` on the target Mac before live use. |
| Service support | Implemented | User service helpers cover systemd, launchd, and Windows scheduled task flows. |
| Ollama | Primary backend | Default judge path. Pull `gemma4:e2b`; use `gemma4:e4b` for stronger/high-risk profiles. |
| llama.cpp | Supported backend | Configure a local llama.cpp server URL or model path, then run doctor/model tests. |
| LiteRT | Supported backend | Configure a LiteRT model path and run the judge capability test before live use. |
| Unsloth / HF model | Supported path | Public adapter: `https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge`. |
| Blackbox endpoint | Example integration | Uses `BLACKBOX_API_KEY` from the environment and never stores the key value in config. |
| Demo mode | Implemented | Demo judge and mock upstream flows let users see reject-repair-approve without external credentials. |
| Streaming chat | Later work | MVP focuses on non-streaming OpenAI-compatible chat completions. |
| Cactus | Not claimed | Not part of this release. |

For detailed evidence, see `docs/final_full_test_report.md`.
