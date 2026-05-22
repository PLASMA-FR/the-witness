# The Witness

A local-first Gemma 4 reliability firewall for AI endpoints.

The Witness watches AI endpoints, verifies responses with Gemma 4, blocks bad outputs, repairs prompts, retries, and only returns approved responses.

Gemma is a trademark of Google LLC. The Witness is an independent open-source project.

## Why it exists

Most AI apps trust the first model response. The Witness adds a local verification layer before that response reaches a user, agent, or workflow.

Run your app through The Witness as an OpenAI-compatible local proxy. The proxy forwards the request to the upstream endpoint, captures the candidate response, asks Gemma 4 for a structured verdict, and returns the approved response.

```text
AI App
  -> The Witness Proxy
  -> Upstream Endpoint
  -> Candidate Response
  -> Gemma 4 Judge
  -> Approved / Blocked / Needs Human Decision
  -> Return / Repair + Retry / Audit Log
```

The Witness is the missing local trust layer between AI generation and real-world action.

## Quick install

Linux/macOS:

```bash
curl -fsSL https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.sh | bash
```

Windows PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -Command "irm https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.ps1 | iex"
```

Prefer to inspect first?

```bash
curl -fsSL https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.sh -o install.sh
less install.sh
bash install.sh
```

## First run

```bash
the-witness setup
the-witness doctor
the-witness start
```

Run the terminal UI:

```bash
the-witness start
```

## Dashboard (Web UI)

**Note:** The dashboard Web UI is **not mandatory**—it's an optional, more organized way to interact with The Witness. It's still in beta. For the best experience, we recommend using the CLI and TUI. If you'd like to try the dashboard:

```bash
the-witness dashboard
```

Open the Web UI at:

```text
http://127.0.0.1:8790
```

## Pull Gemma models

The model names are configurable. These are the project defaults used by the setup wizard and examples:

```bash
ollama pull gemma4:e2b
ollama pull gemma4:e4b
```

| Model | Use |
|---|---|
| `gemma4:e2b` | Fast/default local judge for everyday approval checks. |
| `gemma4:e4b` | Stronger judge for high-risk, coding, and stricter profiles. |

Test a local judge:

```bash
the-witness model test --backend ollama --model gemma4:e2b
```

## Fine-tuned Witness judge

A Witness-specific Gemma 4 E2B judge adapter is published on Hugging Face:

```text
https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge
```

Fine-tuning notebook:

```text
https://colab.research.google.com/drive/17-CgEQLNg8bpnhhWzJwpapRxQyHIqybq?usp=sharing
```

The adapter is loaded with a compatible Gemma 4 E2B base model and the selected runtime. The repository keeps Kaggle references limited to submission/competition workflow notes; model hosting for the Witness-specific judge moved to Hugging Face in the public community model path.

## Add an endpoint

The Blackbox example keeps the secret outside the repo. The config stores the environment variable name, not the key value.

```bash
export BLACKBOX_API_KEY="YOUR_KEY_HERE"
the-witness endpoint add-blackbox
```

Or add any OpenAI-compatible endpoint:

```bash
the-witness endpoint add \
  --name "Codex" \
  --upstream "https://api.openai.com/v1" \
  --local "http://localhost:8787/v1" \
  --profile coding \
  --retry-limit 4 \
  --strictness high
```

## Test through the proxy

Use the local proxy URL shown by `the-witness endpoint list` or the Web UI.

```bash
curl http://localhost:8787/Blackbox%20Grok%20Code/v1/chat/completions \
  -H 'content-type: application/json' \
  -d '{"model":"blackboxai/x-ai/grok-code-fast-1:free","messages":[{"role":"user","content":"Write a Python script that prints Hello World"}]}'
```

Expected flow: The Witness receives the request, forwards it, captures the response, asks Gemma 4 for a verdict, blocks or returns the response, repairs and retries when needed, and writes the audit event.

## What you get

| Feature | What it does |
|---|---|
| Local proxy | Watches OpenAI-compatible chat completion requests before responses reach the app. |
| Gemma 4 judge | Returns structured JSON verdicts: approved, blocked, or needs a human decision. |
| Prompt repair | When a response is rejected, The Witness turns the rejection reason into a better retry prompt. |
| Retry chain viewer | Shows the original request, rejected response, judge reason, repaired prompt, retry, and final response. |
| Human review | High-risk or uncertain responses can be paused until a person decides. |
| Audit logs | Records verdicts, repairs, retries, endpoint errors, and manual decisions as JSONL. |
| Privacy mode | Stores metadata instead of full prompts/responses when configured. |
| Doctor command | Checks local setup and prints exact commands to fix missing pieces. |
| Demo mode | Lets reviewers test the approval loop without external API keys. |
| TUI + Optional Web UI | Use the terminal-native operator UI, or optionally the polished local dashboard (beta). |

## CLI reference

```bash
the-witness setup
the-witness doctor
the-witness dashboard
the-witness start
the-witness model list
the-witness model install --backend ollama --model gemma4:e2b
the-witness model add-ollama --model my-local-judge:latest --display-name "My local experimental judge"
the-witness model test --backend demo --model demo-judge
the-witness endpoint add-blackbox
the-witness endpoint list
the-witness endpoint test "Codex"
the-witness replay <request-id>
the-witness export <request-id> --format markdown
```

## Track fit

Primary impact track:

| Track | Fit |
|---|---|
| Safety & Trust | The Witness makes AI outputs more transparent, explainable, auditable, and controllable before they reach users or agents. |

Technology paths:

| Path | How The Witness uses it |
|---|---|
| Ollama | Default local Gemma 4 judge path. |
| llama.cpp | Resource-constrained local inference path. |
| LiteRT | Edge prefilter path for lightweight approval classification. |
| Unsloth | Fine-tuned Witness judge workflow and Colab notebook. |
| Cactus | Not claimed in this submission. |

## Status

| Feature | Status | Notes |
|---|---|---|
| Rust CLI | Working | Setup, doctor, model, endpoint, dashboard, service, replay, export, and logs commands are present. |
| TUI | Working | First-run setup and operator screens run through `the-witness start`. |
| Web UI | Beta | Local mission-control dashboard at `http://127.0.0.1:8790`; not mandatory for operation. For best experience, use CLI/TUI. |
| Local proxy | Working | Non-streaming OpenAI-compatible chat completions are supported for the MVP. |
| Demo mode | Working | Demonstrates reject, repair, retry, approve, and audit without external keys. |
| Ollama | Supported | Default backend; requires local Ollama and model pull. |
| llama.cpp | Supported | Requires a compatible local server/model path. |
| LiteRT | Optional path | Requires a LiteRT-compatible model and runtime. |
| Unsloth judge | Available | Public adapter on Hugging Face. |
| Streaming chat | Later work | The MVP starts with non-streaming requests. |

Detailed evidence is in [`docs/final_full_test_report.md`](docs/final_full_test_report.md).

## Responsible use

The Witness reduces risk; it does not guarantee truth. The judge model can still be wrong, so high-risk outputs can be escalated to human review.

Medical, legal, financial, destructive coding, emergency, and safety-critical workflows should involve qualified professionals. Local performance depends on hardware, model choice, and backend selection. Use accordingly.

## Documentation

Start here:

- [`docs/setup.md`](docs/setup.md)
- [`docs/user_completion_guide.md`](docs/user_completion_guide.md)
- [`docs/status.md`](docs/status.md)
- [`docs/tracks.md`](docs/tracks.md)
- [`docs/final_full_test_report.md`](docs/final_full_test_report.md)
- [`docs/webui_humanization_report.md`](docs/webui_humanization_report.md)

Additional guides:

- [`docs/architecture.md`](docs/architecture.md)
- [`docs/demo_script.md`](docs/demo_script.md)
- [`docs/hackathon_writeup.md`](docs/hackathon_writeup.md)
- [`docs/services.md`](docs/services.md)
- [`docs/linux.md`](docs/linux.md)
- [`docs/macos.md`](docs/macos.md)
- [`docs/windows.md`](docs/windows.md)

## Gallery

![The Witness Cover](gallery/01_cover_the_witness.png)

![TUI Dashboard](gallery/04_tui_dashboard.png)

![Model Manager](gallery/06_model_manager_tracks.png)

More assets and captions are in [`gallery/README.md`](gallery/README.md).
