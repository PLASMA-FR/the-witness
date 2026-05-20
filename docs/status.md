# Project Status

Who this is for: judges, users, and maintainers who want a quick, honest snapshot of what is available.

What you will do: confirm the supported surfaces, see which pieces require local setup, and find the detailed test report.

The Witness is a local-first Gemma 4 reliability firewall for AI endpoints.

| Feature | Status | Notes |
|---|---|---|
| CLI | Working | Setup, doctor, model, endpoint, dashboard, service, replay, export, and logs command surfaces are present. |
| TUI | Working | First-run setup and operator screens run through `the-witness start`. |
| Web UI | Working | Local mission-control dashboard at `http://127.0.0.1:8790`. |
| Dashboard API | Working | Localhost control API covers health, config/settings, models, endpoints, requests, logs, doctor, and proxy controls. |
| Local proxy | Working | Non-streaming OpenAI-compatible chat completions are supported for the MVP. |
| Prompt repair | Working | Blocked responses produce a repaired retry prompt and retry chain. |
| JSONL audit logs | Working | Verdicts, retries, repairs, and endpoint events are recorded. |
| Demo mode | Working | Reviewers can test the approval loop without external API keys. |
| Linux installer | Working | `scripts/install.sh` builds the release binary and prints next steps. |
| Windows installer | Available | `scripts/install.ps1` builds the release binary and prints PowerShell-specific next steps. |
| macOS support | Available path | Uses the Rust binary and shell installer; run `the-witness doctor` on the target Mac before live use. |
| Service support | Available | User service helpers cover systemd, launchd, and Windows Scheduled Task flows. |
| Ollama backend | Supported | Default local judge path using `gemma4:e2b`; `gemma4:e4b` is the stronger/high-risk option. |
| llama.cpp backend | Supported | Configure a local server URL or model path, then run doctor/model tests. |
| LiteRT prefilter | Optional | Requires a LiteRT-compatible model and runtime. |
| Unsloth fine-tuned judge | Available | Public adapter: `https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge`. |
| Blackbox endpoint | Example integration | Uses `BLACKBOX_API_KEY` from the environment and never stores the key value in config. |
| Streaming chat | Later work | The MVP focuses on non-streaming OpenAI-compatible chat completions. |
| Cactus | Not claimed | This submission focuses on Ollama, llama.cpp, LiteRT, and Unsloth. |

## Responsible boundaries

The Witness reduces risk; it does not guarantee truth. High-risk medical, legal, financial, emergency, and safety-critical outputs should stay reviewable by qualified people.

For detailed evidence, see [`docs/final_full_test_report.md`](final_full_test_report.md).
