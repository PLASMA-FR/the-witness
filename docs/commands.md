# The Witness command and operations guide

This is the detailed user guide for The Witness: what it can do, how to run it, and the exact commands for setup, models, endpoints, proxy mode, logs, fine-tuning, and troubleshooting.

The Witness is a local-first Gemma 4 reliability firewall for AI endpoints. It runs as a TUI and as an OpenAI-compatible local proxy. Your AI app sends requests to The Witness instead of sending them directly to the upstream endpoint. The Witness forwards the request, captures the candidate response, asks a Gemma judge for a structured JSON verdict, and only returns responses that pass the configured validation policy.

Security rules:

- Do not put API keys in `witness.toml` unless you fully understand the risk.
- Prefer environment variables such as `BLACKBOX_API_KEY`.
- Do not commit `.env`, `kaggle.json`, model weights, or generated training outputs.
- Logs and TUI views redact configured secrets, but you should still avoid sharing logs publicly.

## Install

Quick install:

```bash
curl -fsSL https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.sh | bash
```

Safer inspect-first install:

```bash
curl -fsSL https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.sh -o install.sh
less install.sh
bash install.sh
```

Manual install:

```bash
git clone https://github.com/PLASMA-FR/the-witness.git
cd the-witness
cargo build --release
./target/release/the-witness setup
./target/release/the-witness doctor
./target/release/the-witness start
```

Install script environment variables:

```bash
WITNESS_REPO_URL="https://github.com/PLASMA-FR/the-witness.git"
WITNESS_INSTALL_DIR="$HOME/.local/bin"
WITNESS_CONFIG_DIR="$HOME/.config/the-witness"
WITNESS_DEFAULT_BACKEND="ollama"
WITNESS_DEFAULT_MODEL="gemma4:e2b"
WITNESS_STRONG_MODEL="gemma4:e4b"
WITNESS_FALLBACK="human_review"
WITNESS_SKIP_OLLAMA_PULL="1"
```

Example custom install location:

```bash
WITNESS_INSTALL_DIR="$HOME/bin" \
WITNESS_CONFIG_DIR="$HOME/.config/the-witness" \
bash install.sh
```

## Config location

Installed builds use a user-writable config path by default:

```text
${WITNESS_CONFIG_DIR:-$HOME/.config/the-witness}/witness.toml
```

Override with a specific file:

```bash
the-witness --config /path/to/witness.toml setup
the-witness --config /path/to/witness.toml start
```

Override with a config directory:

```bash
WITNESS_CONFIG_DIR=/path/to/config-dir the-witness setup
WITNESS_CONFIG_DIR=/path/to/config-dir the-witness start
```

Source checkout default config:

```text
/home/admin/Gemma/witness/witness.toml
```

That source path is useful for development, but installed users should normally use `$HOME/.config/the-witness/witness.toml`.

## First run

Run:

```bash
the-witness setup
```

The setup wizard asks for:

1. Backend: `ollama`, `llama.cpp`, `litert`, `unsloth`, or `manual`.
2. Gemma model name or local model path.
3. Judge URL or endpoint path.
4. Hardware/runtime check summary.
5. Whether to mark tests passed, use demo mode, or leave setup incomplete.

Recommended first run for a demo:

```bash
the-witness setup
# Press Enter for defaults and choose demo when prompted.
the-witness start
```

Recommended first run for Ollama:

```bash
ollama pull gemma4:e2b
the-witness setup
the-witness model test --backend ollama --model gemma4:e2b
the-witness doctor
the-witness start
```

Optional stronger/high-risk model:

```bash
ollama pull gemma4:e4b
```

## Doctor and verification

Run health checks:

```bash
the-witness doctor
```

Doctor checks:

- OS and CPU architecture.
- Available RAM and disk space when detectable.
- Selected backend is known.
- Ollama binary and daemon for Ollama mode.
- Default model `gemma4:e2b` if using Ollama.
- Optional strong model `gemma4:e4b`.
- Judge URL/path configured.
- Setup flags for judge schema test, model test, and proxy test.
- Blackbox API key if Blackbox endpoint is enabled.
- Local proxy port availability.
- Google Colab TPU for fine-tuning; GPU/Unsloth fallback; Kaggle CLI credentials only for optional Kaggle artifact upload/download.
- Model registry files.
- Log directory writability.

On a fresh machine, these can be expected incomplete items, not code failures:

- Kaggle CLI missing; only required for optional Kaggle artifact upload/download.
- Kaggle credentials missing; only required for optional Kaggle artifact upload/download.
- `BLACKBOX_API_KEY` not set.
- Fine-tuned model not downloaded.
- `gemma4:e4b` not pulled.
- Setup flags not passed until setup/tests are complete.

Run repository verification from a source checkout:

```bash
bash scripts/verify.sh
```

That runs:

```bash
cargo fmt --check
cargo test
cargo build
python3 training/scripts/validate_dataset.py
./target/debug/the-witness model list
./target/debug/the-witness model test --backend demo --model demo-judge
./target/debug/the-witness doctor
```

## TUI

Start the TUI:

```bash
the-witness start
```

If setup is incomplete, `start` opens setup first. After setup passes or demo mode is selected, it opens the dashboard.

Main TUI screens:

- Setup wizard: first-run readiness flow.
- Dashboard: watched endpoints, request counts, approval/rejection stats, latency, backend health.
- Endpoint watchlist: add/edit/enable/disable/test endpoints.
- Live request stream: request ID, endpoint, model, status, retry attempt, latency, timestamp.
- Request inspector: headers with secrets hidden, request body, prompts, model, metadata.
- Response inspector: candidate response, final approved response, retry history, diffs.
- Gemma verdict panel: verdict, confidence, safety/usefulness/alignment scores, reason, suggested fix.
- Prompt repair panel: original prompt, rejection reason, suggested fix, repaired prompt.
- Human review queue: approve, reject, edit, retry, export, mark unsafe, add note.
- Profiles: built-in validation profiles and strictness behavior.
- Logs and audit: timelines, filters, exports.
- Settings/model manager: backend and model configuration.

Current keyboard shortcuts:

```text
1  Dashboard
2  Endpoints
3  Requests
4  Human review
5  Logs
6  Settings
7  Model manager
s  Setup wizard screen
q  Quit
Esc Quit
```

On the Settings screen, backend shortcuts are:

```text
o  Ollama
l  llama.cpp
t  LiteRT
u  Unsloth
m  Manual OpenAI-compatible
d  Demo judge
```

## CLI reference

Top-level help:

```bash
the-witness --help
```

Commands:

```bash
the-witness init <PATH>
the-witness setup
the-witness doctor
the-witness start [--proxy-addr <ADDR>]
the-witness model list
the-witness model install [--backend <BACKEND>] [--model <MODEL>]
the-witness model download --source kaggle --model <MODEL>
the-witness model test [--backend <BACKEND>] [--model <MODEL>] [--url <URL>] [--model-path <PATH>]
the-witness endpoint add [OPTIONS]
the-witness endpoint add-blackbox
the-witness endpoint list
the-witness endpoint test <NAME>
the-witness endpoint disable <NAME>
the-witness endpoint enable <NAME>
the-witness logs
the-witness replay <REQUEST_ID>
the-witness export <REQUEST_ID> --format markdown
```

### `the-witness init`

Create a project/config skeleton at a path:

```bash
the-witness init /home/admin/Gemma/witness
```

For normal installed use, prefer:

```bash
the-witness setup
```

### `the-witness setup`

Run or re-run first-run setup:

```bash
the-witness setup
```

Use this when:

- You installed for the first time.
- You changed backend/model.
- Doctor says setup flags are incomplete.
- You want to switch into demo mode.
- You want to repair a broken config.

### `the-witness doctor`

Run readiness checks:

```bash
the-witness doctor
```

Use this before demos or production endpoint watching.

### `the-witness start`

Start the TUI:

```bash
the-witness start
```

Start only the proxy service on an address:

```bash
the-witness start --proxy-addr 127.0.0.1:8787
```

Proxy routes use endpoint names in path form:

```text
/<endpoint-name>/v1/chat/completions
```

For the built-in Blackbox endpoint, the documented convenience local URL is:

```text
http://localhost:8787/v1/chat/completions
```

When using custom endpoints, check the endpoint list for the local proxy URL and route style in your config.

### `the-witness model list`

List configured model/backends:

```bash
the-witness model list
```

Shows:

- Current configured backend/model/url.
- Selectable backends.
- Model registry entries.
- Installed flags.
- Local paths.

### `the-witness model install`

Interactive/default install helper:

```bash
the-witness model install
```

Ollama pull through the helper:

```bash
the-witness model install --backend ollama --model gemma4:e2b
```

Optional stronger model:

```bash
the-witness model install --backend ollama --model gemma4:e4b
```

Equivalent direct Ollama commands:

```bash
ollama pull gemma4:e2b
ollama pull gemma4:e4b
```

For llama.cpp, LiteRT, Unsloth, and manual endpoints, the install helper prints the required setup/test instructions rather than inventing local model files.

### `the-witness model test`

Test the selected model from config:

```bash
the-witness model test
```

Test demo judge:

```bash
the-witness model test --backend demo --model demo-judge
```

Test Ollama judge:

```bash
the-witness model test --backend ollama --model gemma4:e2b --url http://localhost:11434
```

Test manual OpenAI-compatible judge endpoint:

```bash
the-witness model test \
  --backend manual \
  --model local-gemma-judge \
  --url http://localhost:8000/v1
```

Test Unsloth local endpoint:

```bash
the-witness model test \
  --backend unsloth \
  --model witness-gemma4-e2b-judge \
  --url http://localhost:8000/v1
```

Model test expects the judge to return only the required JSON verdict schema.

### `the-witness model download`

Download registered Kaggle model artifacts only if you chose optional Kaggle publishing after Colab training:

```bash
the-witness model download --source kaggle --model witness-gemma4-e2b-judge
```

This requires:

- Kaggle CLI installed, only for optional Kaggle artifact download.
- Kaggle credentials configured locally, only for optional Kaggle artifact download.
- The model artifact actually uploaded and accessible.

Target slug:

```text
plasmafr/witness-gemma4-e2b-judge
```

### `the-witness endpoint add`

Interactive add:

```bash
the-witness endpoint add
```

Non-interactive add:

```bash
the-witness endpoint add \
  --name "Codex" \
  --upstream "https://api.openai.com/v1" \
  --local "http://localhost:8787/v1" \
  --profile coding \
  --retry-limit 4 \
  --strictness high \
  --model "gpt-5.5"
```

With an auth header from an environment variable, construct the header in the shell and avoid committing it:

```bash
export UPSTREAM_API_KEY="YOUR_KEY_HERE"
the-witness endpoint add \
  --name "Private API" \
  --upstream "https://api.example.com/v1" \
  --local "http://localhost:8789/v1" \
  --profile general \
  --retry-limit 3 \
  --strictness medium \
  --model "example-model" \
  --auth-header "Bearer $UPSTREAM_API_KEY"
```

Note: for long-term safer storage, prefer endpoint auth configs that reference env vars, like the built-in Blackbox endpoint does.

### `the-witness endpoint add-blackbox`

Set key in shell:

```bash
export BLACKBOX_API_KEY="YOUR_KEY_HERE"
```

Add endpoint:

```bash
the-witness endpoint add-blackbox
```

Creates:

```text
name: Blackbox Grok Code
upstream: https://api.blackbox.ai/v1
local proxy: http://localhost:8787/v1
model: blackboxai/x-ai/grok-code-fast-1:free
profile: coding
strictness: high
retry limit: 4
auth: bearer token from BLACKBOX_API_KEY
```

### `the-witness endpoint list`

List endpoints:

```bash
the-witness endpoint list
```

### `the-witness endpoint test`

Test endpoint reachability:

```bash
the-witness endpoint test "Blackbox Grok Code"
```

### `the-witness endpoint disable` / `enable`

Disable endpoint:

```bash
the-witness endpoint disable "Blackbox Grok Code"
```

Enable endpoint:

```bash
the-witness endpoint enable "Blackbox Grok Code"
```

### `the-witness logs`

Print log path:

```bash
the-witness logs
```

Default source checkout path:

```text
logs/witness.jsonl
```

Installed config-root path is under the config directory unless overridden.

### `the-witness replay`

Replay request by ID:

```bash
the-witness replay <request-id>
```

MVP status: audit data is stored for replay; rich interactive replay is a next-step feature.

### `the-witness export`

Export request report:

```bash
the-witness export <request-id> --format markdown
```

Other planned formats:

```bash
the-witness export <request-id> --format json
the-witness export <request-id> --format csv
```

MVP status: use JSONL logs until rich exporter lands.

## Proxy usage examples

### Blackbox through The Witness

```bash
export BLACKBOX_API_KEY="YOUR_KEY_HERE"
the-witness endpoint add-blackbox
the-witness start --proxy-addr 127.0.0.1:8787
```

In another terminal:

```bash
curl http://localhost:8787/v1/chat/completions \
  -H "Authorization: Bearer $BLACKBOX_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "blackboxai/x-ai/grok-code-fast-1:free",
    "messages": [
      {
        "role": "user",
        "content": "Write a Python script that prints Hello World"
      }
    ]
  }'
```

### Generic OpenAI-compatible upstream

Add endpoint:

```bash
export UPSTREAM_API_KEY="YOUR_KEY_HERE"
the-witness endpoint add \
  --name "OpenAI Compatible" \
  --upstream "https://api.example.com/v1" \
  --local "http://localhost:8790/v1" \
  --profile coding \
  --retry-limit 3 \
  --strictness medium \
  --model "example-chat-model" \
  --auth-header "Bearer $UPSTREAM_API_KEY"
```

Start proxy:

```bash
the-witness start --proxy-addr 127.0.0.1:8790
```

Send request:

```bash
curl http://localhost:8790/openai-compatible/v1/chat/completions \
  -H "Authorization: Bearer $UPSTREAM_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "example-chat-model",
    "messages": [
      {"role": "user", "content": "Explain Rust ownership in two paragraphs."}
    ]
  }'
```

Route names are normalized from endpoint names by the proxy. If a route does not match, check `the-witness endpoint list` and your configured local proxy URL.

## Judge verdict schema

The judge must return only JSON:

```json
{
  "verdict": "APPROVED",
  "confidence": 0.0,
  "safety_score": 0,
  "usefulness_score": 0,
  "prompt_alignment_score": 0,
  "correctness_risk": "low",
  "rejection_reason": "",
  "suggested_fix": "",
  "improved_prompt_instruction": "",
  "requires_human_review": false
}
```

Allowed verdicts:

```text
APPROVED
DISAPPROVED
NEEDS_HUMAN_REVIEW
```

Allowed correctness risk:

```text
low
medium
high
```

## Approval and retry behavior

For every watched request:

1. Capture request body, model name, system prompt, user prompt, and metadata.
2. Redact secrets from TUI/logs.
3. Forward to upstream endpoint.
4. Capture candidate response.
5. Ask Gemma judge for a JSON verdict.
6. If `APPROVED`, return response to original app and log event.
7. If `DISAPPROVED`, block response, repair the prompt, retry upstream, and log each step.
8. If `NEEDS_HUMAN_REVIEW`, pause/escalate based on fallback config.
9. If retry limit is reached, use configured fallback behavior.

Fallback behaviors:

```text
human_review   Pause/escalate for manual review.
safe_response  Return a safe fallback response.
error          Return an error instead of a rejected answer.
demo_judge     Explicit demo-only judge path.
```

Strictness levels:

```text
relaxed
medium
high
critical
```

## Validation profiles

Built-in/profile concepts:

- General Safety: broad safety, usefulness, prompt alignment.
- Coding: correctness, security, runnable code, no invented APIs when avoidable.
- Education: clear explanation, age-appropriate caution, no misinformation.
- Medical: high-risk health claims trigger caution/human review.
- Finance: financial claims and advice trigger caution/human review.
- Legal: legal advice risk triggers caution/human review.
- Scientific Research: evidence quality, uncertainty, hallucination risk.
- Disaster Response: high urgency and safety constraints.
- Arabic-English Multilingual: validates Arabic/English prompt alignment and mixed-language answers.
- Custom: user-defined criteria in config/code.

MVP profiles are represented in config/defaults and can be expanded with richer criteria.

## Technology tracks

### Ollama

Default and easiest local setup:

```bash
ollama serve
ollama pull gemma4:e2b
ollama pull gemma4:e4b
the-witness model test --backend ollama --model gemma4:e2b
```

### llama.cpp

Run a local OpenAI-compatible llama.cpp server, then test:

```bash
the-witness model test \
  --backend llama.cpp \
  --model /path/to/gemma-model.gguf \
  --url http://localhost:8080/v1
```

Do not commit `.gguf` files.

### LiteRT

Use a local LiteRT model path for lightweight verification/prefiltering:

```bash
the-witness model test \
  --backend litert \
  --model-path /path/to/judge.tflite
```

### Unsloth

Fine-tuned judge path:

```bash
# Train first in the Google Colab notebook.
training/notebooks/finetune_gemma4_e2b_unsloth.ipynb

# Upload target.
plasmafr/witness-gemma4-e2b-judge

# Download after upload.
the-witness model download --source kaggle --model witness-gemma4-e2b-judge

# Test after download/serve.
the-witness model test --backend unsloth --model witness-gemma4-e2b-judge
```

Honest status: Colab notebooks and dataset are ready, but the model is not trained until you run the notebook.

## Logs and audit

The Witness records audit events as JSONL.

Events include:

- setup test results
- endpoint errors
- request received
- upstream forwarded
- candidate response captured
- judge verdict
- prompt repair
- retry attempt
- final approval/failure
- human review escalation

View log location:

```bash
the-witness logs
```

Search examples:

```bash
# From source checkout.
grep 'DISAPPROVED' logs/witness.jsonl || true
grep 'NEEDS_HUMAN_REVIEW' logs/witness.jsonl || true
grep 'Blackbox Grok Code' logs/witness.jsonl || true
```

## Troubleshooting

### Permission denied during setup

Update to the latest build. This was fixed by moving installed config to a user-writable path.

```bash
curl -fsSL https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.sh | bash
rm -rf ~/.config/the-witness
the-witness setup
```

Or force a config path:

```bash
the-witness --config ~/.config/the-witness/witness.toml setup
```

### TUI does not draw in non-interactive shell

The TUI requires a real TTY. For CI or scripts, use:

```bash
the-witness doctor
the-witness start --proxy-addr 127.0.0.1:8787
```

### Ollama model missing

```bash
ollama pull gemma4:e2b
ollama list
```

### Ollama daemon not running

```bash
ollama serve
```

Or start the Ollama desktop/service for your platform.

### Blackbox endpoint fails

Check:

```bash
test -n "$BLACKBOX_API_KEY" && echo "BLACKBOX_API_KEY is set"
the-witness endpoint list
the-witness endpoint test "Blackbox Grok Code"
```

Do not paste the key into GitHub issues, docs, screenshots, or logs.

### Kaggle model download fails

Check:

```bash
python -m pip install kaggle
kaggle --version
kaggle models list 2>/dev/null || kaggle datasets list 2>/dev/null
```

Then verify credentials are configured locally and not committed.

### Doctor reports readiness failures

Read each `Fix:` line. Common fresh-machine fixes:

```bash
ollama pull gemma4:e2b
the-witness model test --backend demo --model demo-judge
the-witness setup
export BLACKBOX_API_KEY="YOUR_KEY_HERE"   # only if using Blackbox
python -m pip install kaggle                 # only if using optional Kaggle artifact flow
```

## Development commands

From a source checkout:

```bash
cargo fmt
cargo fmt --check
cargo test
cargo build
bash scripts/verify.sh
python3 training/scripts/validate_dataset.py
```

Regenerate dataset:

```bash
python3 training/scripts/prepare_dataset.py
python3 training/scripts/validate_dataset.py
wc -c training/dataset/witness_judge_train.jsonl training/dataset/witness_judge_val.jsonl
```

Check docs and secret hygiene before pushing:

```bash
grep -R "sk-" . --exclude-dir=target --exclude-dir=.git || true
grep -R "CfDJ" . --exclude-dir=target --exclude-dir=.git || true
grep -R "BLACKBOX_API_KEY=.*[A-Za-z0-9]" . --exclude-dir=target --exclude-dir=.git || true
grep -R "KAGGLE_KEY=.*[A-Za-z0-9]" . --exclude-dir=target --exclude-dir=.git || true
find . -name "kaggle.json" -o -name ".env"
git ls-files | grep -E "kaggle.json|\.env|safetensors|gguf|target/" || true
```
