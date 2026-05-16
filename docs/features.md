# Feature guide

This guide explains what The Witness can do and how each feature maps to commands, TUI screens, and current MVP status.

## What The Witness does

The Witness is a local-first reliability firewall for AI endpoints.

It can:

- Run a local OpenAI-compatible proxy.
- Watch configured upstream AI endpoints.
- Capture requests, prompts, models, metadata, and responses.
- Redact secrets from TUI/log output.
- Ask a local Gemma 4 judge to verify every candidate response.
- Return approved responses to the original app.
- Block disapproved responses.
- Repair prompts after rejection.
- Retry until approved or retry limit is reached.
- Escalate uncertain/high-risk responses to human review.
- Store JSONL audit logs.
- Support multiple independently configured endpoints.
- Support multiple judge backends: Ollama, llama.cpp, LiteRT, Unsloth, manual OpenAI-compatible, and demo.
- Support a setup wizard and doctor checks so the app does not assume the machine is ready.

## Core workflow

1. Install The Witness.
2. Pull or configure a Gemma judge model.
3. Run setup.
4. Add an endpoint.
5. Point your AI app at The Witness local proxy URL.
6. Start The Witness.
7. Watch requests and verdicts in the TUI.
8. Review logs/audit trails.

Commands:

```bash
curl -fsSL https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.sh | bash
ollama pull gemma4:e2b
the-witness setup
the-witness doctor
the-witness endpoint add
the-witness start
```

## First-run setup wizard

What it does:

- Explains The Witness and the local judge concept.
- Lets the user choose backend.
- Lets the user choose/edit model name or path.
- Captures judge URL.
- Shows hardware/runtime summary.
- Lets the user proceed with verified setup, demo mode, or incomplete setup.
- Saves config to the user config directory.

Command:

```bash
the-witness setup
```

Config path:

```text
${WITNESS_CONFIG_DIR:-$HOME/.config/the-witness}/witness.toml
```

Override:

```bash
the-witness --config /path/to/witness.toml setup
```

Status: implemented MVP. The wizard is terminal-driven and is also represented in the TUI screen set.

## Hardware and health checks

What it checks:

- Operating system.
- CPU architecture.
- RAM if detectable.
- Disk space if detectable.
- GPU availability if detectable.
- Ollama installation.
- Ollama daemon availability.
- Model availability.
- Proxy port availability.
- Kaggle CLI and credentials for fine-tuned model workflows.
- Logs writable.
- Setup readiness flags.

Command:

```bash
the-witness doctor
```

Status: implemented. Missing optional user-side credentials/models are reported honestly with `Fix:` lines.

## Gemma model picker and backends

Supported backends:

```text
ollama
llama.cpp
litert
unsloth
manual
demo
```

List configured models and backends:

```bash
the-witness model list
```

Install/pull model:

```bash
the-witness model install --backend ollama --model gemma4:e2b
```

Direct Ollama commands:

```bash
ollama pull gemma4:e2b
ollama pull gemma4:e4b
```

Status: implemented. Non-Ollama paths print actionable setup/test instructions and rely on user-provided model files/endpoints.

## Judge capability test

What it verifies:

- The model responds.
- The response can be parsed as JSON.
- JSON matches the required verdict schema.
- Obviously wrong answer is disapproved.
- Clearly correct answer is approved.

Command:

```bash
the-witness model test --backend demo --model demo-judge
```

Ollama:

```bash
the-witness model test --backend ollama --model gemma4:e2b --url http://localhost:11434
```

Status: implemented. Demo judge is deterministic and intended for local tests/hackathon demos.

## Local proxy

What it does:

- Receives OpenAI-compatible chat completion requests.
- Matches the request to a configured endpoint.
- Redacts secrets.
- Forwards to upstream.
- Captures candidate response.
- Runs judge verification.
- Handles approve/reject/retry/human-review fallback.
- Writes JSONL logs.

Start TUI and proxy system:

```bash
the-witness start
```

Start headless proxy/service mode:

```bash
the-witness start --proxy-addr 127.0.0.1:8787
```

Status: MVP supports non-streaming OpenAI-compatible chat completions. Streaming support is planned later.

## Endpoint watchlist

Endpoint fields:

- endpoint name
- upstream API URL
- local proxy URL
- API key/auth header or env-based auth
- model name
- validation profile
- retry limit
- timeout
- strictness level
- enabled/disabled status
- fallback behavior

Add interactively:

```bash
the-witness endpoint add
```

Add from CLI:

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

List:

```bash
the-witness endpoint list
```

Test:

```bash
the-witness endpoint test "Codex"
```

Disable/enable:

```bash
the-witness endpoint disable "Codex"
the-witness endpoint enable "Codex"
```

Status: implemented CLI and TUI MVP. Rich duplicate/copy/edit flows are represented in the design and can be expanded.

## Blackbox endpoint template

Built-in endpoint template:

```bash
export BLACKBOX_API_KEY="YOUR_KEY_HERE"
the-witness endpoint add-blackbox
```

Model:

```text
blackboxai/x-ai/grok-code-fast-1:free
```

Proxy test:

```bash
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
      {"role": "user", "content": "Write a Python script that prints Hello World"}
    ]
  }'
```

Status: implemented. Requires user-provided `BLACKBOX_API_KEY`.

## Live request stream

Shows each watched request with:

- request ID
- endpoint name
- model
- profile
- status
- retry attempt
- latency
- timestamp

Statuses:

```text
pending
forwarded
judging
approved
disapproved
retrying
human_review
failed
```

TUI command:

```bash
the-witness start
# press 3
```

Status: TUI MVP screen exists; richer live updates can be expanded as proxy integration deepens.

## Request inspector

Shows:

- endpoint name
- upstream URL
- local proxy URL
- HTTP method/path
- headers with secrets hidden
- request body
- system prompt
- user prompt
- model name
- timestamp
- token estimate

TUI command:

```bash
the-witness start
# use request stream/inspector screen
```

Status: designed and scaffolded in MVP TUI.

## Response inspector

Shows:

- candidate response
- final approved response
- rejected response history
- retry attempts
- diff between rejected and approved responses
- response latency
- token estimate

Status: designed and scaffolded in MVP TUI.

## Gemma verdict panel

Verdict schema:

```json
{
  "verdict": "APPROVED | DISAPPROVED | NEEDS_HUMAN_REVIEW",
  "confidence": 0.0,
  "safety_score": 0,
  "usefulness_score": 0,
  "prompt_alignment_score": 0,
  "correctness_risk": "low | medium | high",
  "rejection_reason": "",
  "suggested_fix": "",
  "improved_prompt_instruction": "",
  "requires_human_review": false
}
```

Model test:

```bash
the-witness model test --backend demo --model demo-judge
```

Status: schema implemented and tested.

## Prompt repair

When a response is disapproved, The Witness repairs the request by:

- preserving original user intent
- keeping original prompt
- adding hidden corrective instructions
- including rejection reason and required fix
- asking upstream to avoid previous mistake
- increasing strictness after repeated failures
- avoiding leaking judge internals to final user

Repair prompt behavior is implemented in:

```text
src/repair/prompt_repair.rs
```

Test coverage:

```bash
cargo test repair_prompt_preserves_user_intent_and_adds_fix
```

Status: implemented and tested.

## Human review queue

Human review is used when:

- judge returns `NEEDS_HUMAN_REVIEW`
- confidence is low
- correctness risk is high
- profile-specific risk trigger fires
- retry limit is reached with `human_review` fallback

Supported/planned actions:

- approve manually
- reject manually
- edit response
- retry with improved prompt
- export report
- mark unsafe
- add note

TUI command:

```bash
the-witness start
# press 4
```

Status: MVP screen exists; full interactive queue actions are next-step work.

## Profiles

Profile concepts:

```text
General Safety
Coding
Education
Medical
Finance
Legal
Scientific Research
Disaster Response
Arabic-English Multilingual
Custom
```

Current config-backed defaults include coding, education, and high-risk profile defaults. The docs and structure reserve the full profile set for expansion.

Example endpoint profile assignment:

```bash
the-witness endpoint add \
  --name "Tutor" \
  --upstream "http://localhost:8000/v1" \
  --local "http://localhost:8788/v1" \
  --profile education \
  --retry-limit 3 \
  --strictness medium \
  --model "local-tutor"
```

Status: MVP profile defaults implemented; detailed criteria can be expanded in `src/profiles`.

## Logs and audit exports

Log path command:

```bash
the-witness logs
```

Replay command:

```bash
the-witness replay <request-id>
```

Export command:

```bash
the-witness export <request-id> --format markdown
```

Planned/desired export formats:

```text
jsonl
markdown
csv
```

Status: JSONL logging implemented. Rich replay/export UX is MVP-stubbed and documented honestly.

## Privacy mode

Config field:

```toml
[defaults]
privacy_mode = false
```

When enabled in future deeper proxy flows, privacy mode should store metadata without full prompts/responses.

Status: config support exists; full privacy-mode enforcement should be verified before production claims.

## Demo mode

Use when local model setup is not ready:

```bash
the-witness setup
# choose demo at the readiness prompt

the-witness model test --backend demo --model demo-judge
the-witness start
```

Status: implemented. Useful for hackathon demos and CI-safe tests.

## Fine-tuning dataset and notebooks

Dataset files:

```text
training/dataset/witness_judge_train.jsonl
training/dataset/witness_judge_val.jsonl
```

Validate:

```bash
python3 training/scripts/validate_dataset.py
```

Regenerate:

```bash
python3 training/scripts/prepare_dataset.py
```

Notebook:

```text
training/notebooks/finetune_gemma4_e2b_unsloth.ipynb
```

Upload target:

```text
plasmafr/witness-gemma4-e2b-judge
```

Download after upload:

```bash
the-witness model download --source kaggle --model witness-gemma4-e2b-judge
```

Status: dataset, notebooks, and upload/download pipeline are ready. Training and upload remain user-side.

## Current honest MVP status

Feature | Status | Notes
--- | --- | ---
Install script | Working | Builds release binary and installs to user bin.
TUI startup | Working | Requires real TTY; non-TTY prints guidance.
Config path | Fixed | Installed default is user-writable config dir.
Setup wizard | MVP working | Terminal wizard plus TUI setup screen.
Doctor | Working | Reports actionable readiness and expected missing local deps.
Model list | Working | Shows backends and registry.
Demo model test | Working | Deterministic local judge.
Ollama path | Working when Ollama/model installed | User must pull models locally.
llama.cpp path | Config/test path | User supplies server/model.
LiteRT path | Config/test path | User supplies local model.
Unsloth path | Pipeline ready | User must train/upload/download model.
Endpoint add/list/test/enable/disable | Working CLI MVP | Rich TUI editing can expand.
Blackbox template | Working when key set | Requires `BLACKBOX_API_KEY`.
Proxy non-streaming chat | MVP | Streaming is non-goal for MVP.
Prompt repair | Implemented/tested | Used after disapproval.
Human review | MVP/scaffold | Full interactive actions can expand.
JSONL logs | Implemented | Rich export/replay is MVP-stubbed.
Secret redaction | Implemented/tested | Continue avoiding secrets in docs/logs.
Fine-tuned model | Not trained by repo | User must run notebook and upload.
