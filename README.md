# The Witness

The Witness is a local-first Gemma 4 reliability firewall for AI endpoints.

It runs as a modern TUI and OpenAI-compatible local proxy. Your app sends requests to The Witness, The Witness forwards them to the upstream AI endpoint, captures the candidate response, asks a local Gemma 4 judge for a strict JSON verdict, and only returns approved responses. Disapproved responses are blocked, repaired, retried, and logged. High-risk or uncertain responses can be paused for human review.

Security rule: never commit, print, screenshot, or log API keys or Kaggle tokens. Use environment variables such as `BLACKBOX_API_KEY`; examples reference `$BLACKBOX_API_KEY` only.

## Quick install

```bash
curl -fsSL https://raw.githubusercontent.com/<OWNER>/<REPO>/main/scripts/install.sh | bash
```

Safer inspect-first install:

```bash
curl -fsSL https://raw.githubusercontent.com/<OWNER>/<REPO>/main/scripts/install.sh -o install.sh
less install.sh
bash install.sh
```

## Manual install

```bash
git clone <REAL_REPO_URL>
cd the-witness
cargo build --release
./target/release/the-witness setup
./target/release/the-witness doctor
./target/release/the-witness start
```

Installer/default model policy:

```bash
WITNESS_DEFAULT_BACKEND="ollama"
WITNESS_DEFAULT_MODEL="gemma4:e2b"
WITNESS_STRONG_MODEL="gemma4:e4b"
WITNESS_FALLBACK="human_review"
```

## Default models

Default local judge:

```bash
ollama pull gemma4:e2b
```

Stronger/high-risk local judge:

```bash
ollama pull gemma4:e4b
```

Then test:

```bash
the-witness model test --backend ollama --model gemma4:e2b
```

## TUI usage

```bash
the-witness setup
the-witness doctor
the-witness start
```

The setup wizard guides first-run configuration before the dashboard opens. The TUI includes:

- setup wizard
- model manager/settings
- endpoint watchlist
- live request stream
- request and response inspectors
- Gemma verdict panel
- prompt repair panel
- human review queue
- logs and audit screen

## CLI reference

```bash
the-witness setup
the-witness doctor
the-witness start
the-witness model list
the-witness model test
the-witness model download
the-witness endpoint add
the-witness endpoint add-blackbox
the-witness endpoint list
the-witness replay <request-id>
the-witness export <request-id> --format markdown
```

## Fine-tuned Unsloth model

Confirmed Kaggle target slug:

```text
plasmafr/witness-gemma4-e2b-judge
```

Download after you have actually trained and uploaded the model:

```bash
the-witness model download --source kaggle --model witness-gemma4-e2b-judge
the-witness model test --backend unsloth --model witness-gemma4-e2b-judge
```

Honest status: Unsloth notebooks and pipeline exist, but the model is not trained or uploaded until you run the notebook and upload the artifact.

Fine-tuning assets:

- `training/notebooks/finetune_gemma4_e2b_unsloth.ipynb`
- `training/notebooks/finetune_gemma4_e4b_unsloth.ipynb`
- `training/dataset/witness_judge_train.jsonl`
- `training/dataset/witness_judge_val.jsonl`
- `witness_finetuning_pack.zip`

The bundled dataset is larger than 10 MB and validates with:

```bash
python3 training/scripts/validate_dataset.py
```

## Blackbox endpoint test

Set the key in your shell only:

```bash
export BLACKBOX_API_KEY="YOUR_KEY_HERE"
```

Direct upstream sanity test:

```bash
curl https://api.blackbox.ai/v1/chat/completions \
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

Create the watched endpoint:

```bash
the-witness endpoint add-blackbox
```

This creates:

- endpoint name: Blackbox Grok Code
- upstream URL: `https://api.blackbox.ai/v1`
- local proxy URL: `http://localhost:8787/v1`
- auth: bearer token from `BLACKBOX_API_KEY`
- model: `blackboxai/x-ai/grok-code-fast-1:free`
- profile: coding
- strictness: high
- retry limit: 4
- fallback: human_review

Test through The Witness:

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

Expected flow: local proxy receives the request, forwards to Blackbox, captures the response, judges it with Gemma 4, returns approved output or repairs/retries until approved or retry limit is reached, writes the retry chain to logs, and shows the flow in the TUI.

## Technology tracks

The Witness supports four hackathon technology tracks:

- Ollama: easiest local Gemma judge path.
- llama.cpp: resource-constrained local inference with OpenAI-compatible server mode.
- LiteRT: lightweight edge verification/prefilter path.
- Unsloth: optional fine-tuned Gemma 4 judge path.

See `docs/tracks.md` for details.

## Project docs

- `docs/architecture.md` — architecture and data flow.
- `docs/setup.md` — setup and health checks.
- `docs/kaggle_cli.md` — Kaggle auth/upload/download notes.
- `docs/tracks.md` — technology track mapping.
- `docs/demo_script.md` — demo flow.
- `docs/hackathon_writeup.md` — hackathon positioning.
- `docs/user_completion_guide.md` — user-side fine-tuning and completion guide.

## Endpoint watching

```bash
the-witness endpoint add --name "Codex" --upstream "https://api.openai.com/v1" --local "http://localhost:8787/v1" --profile coding --retry-limit 4 --strictness high
the-witness start --proxy-addr 127.0.0.1:8787
```

Secrets are redacted in TUI/logs.

## Approval loop

1. Capture request, prompts, model, and metadata.
2. Forward to upstream endpoint.
3. Capture candidate response.
4. Optional LiteRT prefilter.
5. Send original request plus candidate response to Gemma judge.
6. APPROVED: return response and log.
7. DISAPPROVED: block, repair prompt, retry.
8. NEEDS_HUMAN_REVIEW: pause/fallback and log.

## Safety note

The Witness improves reliability but does not guarantee perfect safety or correctness and does not replace qualified medical, legal, financial, or emergency professionals.
