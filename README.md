# The Witness

A local-first Gemma 4 reliability firewall for AI endpoints.

The Witness watches AI endpoints, verifies responses with Gemma 4, blocks bad outputs, repairs prompts, retries, and only returns approved responses.

Gemma is a trademark of Google LLC. The Witness is an independent open-source project.

## Why it exists

Most AI apps trust the first model response. The Witness adds a local verification layer before that response reaches a user, agent, or workflow.

Run your app through The Witness as an OpenAI-compatible local proxy. The proxy forwards the request to the upstream endpoint, captures the candidate response, asks Gemma 4 for a structured verdict, then decides what happens next:

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
the-witness dashboard
```

Open the Web UI at:

```text
http://127.0.0.1:8790
```

Run the terminal UI:

```bash
the-witness start
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

The adapter is loaded with a compatible Gemma 4 E2B base model and the selected runtime. The repository keeps Kaggle references limited to submission/competition workflow notes; model hosting for this judge is Hugging Face.

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

Expected flow: The Witness receives the request, forwards it, captures the response, asks Gemma 4 for a verdict, blocks or returns the response, repairs and retries when needed, and writes the audit trail.

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
| Web UI + TUI | Use the polished local dashboard or the terminal-native operator UI. |

## Web UI

```bash
the-witness dashboard
```

The dashboard includes:

- Mission Control
- Watched Endpoints
- Live Requests
- Request Detail
- Prompt Repair
- Needs a Human Decision
- Choose How The Witness Thinks
- Audit Logs
- System Check
- Settings

These are wired to the local control API, not static mockups: endpoints can be added, tested, edited, deleted, and copied; requests/logs load from JSONL audit state; models can be tested or selected; Settings can save config and register optional custom Ollama model tags. Demo data is shown only when no live data exists so the layout is understandable before first traffic.

The control API binds to `127.0.0.1` by default. If you expose it beyond localhost, treat it as a local operations surface and protect the host network accordingly.

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
| Web UI | Working | Local mission-control dashboard at `http://127.0.0.1:8790`. |
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

Medical, legal, financial, destructive coding, emergency, and safety-critical workflows should involve qualified professionals. Local performance depends on hardware, model choice, and backend setup. Optional backends require their corresponding local runtimes.

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
