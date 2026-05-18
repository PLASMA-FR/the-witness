# How to Use The Witness

The Witness is a local-first Gemma 4 reliability firewall for AI endpoints. It runs as a terminal-native TUI and OpenAI-compatible local proxy. Your AI app sends requests to The Witness, The Witness forwards them to the upstream endpoint, captures the response, asks Gemma 4 for a structured verdict, and only releases approved responses.

## 1. Install

```bash
curl -fsSL https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.sh | bash
```

## 2. Pull Gemma models

```bash
ollama pull gemma4:e2b
ollama pull gemma4:e4b
```

`gemma4:e2b` is the default local judge path. `gemma4:e4b` is the stronger optional path for higher-risk profiles when hardware allows.

## 3. Run setup

```bash
the-witness setup
```

The setup wizard selects the judge backend, model, health checks, proxy test, and demo/endpoint readiness before opening the main dashboard.

## 4. Run doctor

```bash
the-witness doctor
```

`doctor` checks backend configuration, model availability, judge schema readiness, proxy readiness, endpoint requirements, logs, and optional Hugging Face/Kaggle tooling. Some warnings are normal on a fresh machine until models, API keys, or optional fine-tuned assets are configured.

## 5. Start the TUI

```bash
the-witness start
```

The TUI includes the dashboard, model manager, endpoint watchlist, live request stream, verdict panel, prompt repair panel, human review queue, logs, and settings.

## 6. Add the Blackbox endpoint

Set the key in your shell only. Do not put real keys in config files, screenshots, logs, or commits.

```bash
export BLACKBOX_API_KEY="YOUR_KEY_HERE"
the-witness endpoint add-blackbox
```

This configures the upstream Blackbox OpenAI-compatible endpoint and exposes a local proxy route at `http://localhost:8787/v1`.

## 7. Test through The Witness

```bash
curl http://localhost:8787/v1/chat/completions   -H "Authorization: Bearer $BLACKBOX_API_KEY"   -H "Content-Type: application/json"   -d '{
    "model": "blackboxai/x-ai/grok-code-fast-1:free",
    "messages": [
      {
        "role": "user",
        "content": "Write a Python script that prints Hello World"
      }
    ]
  }'
```

If `BLACKBOX_API_KEY` is not configured, use the built-in demo/mock path for a local demonstration without external API calls.

## 8. Download the fine-tuned judge from Hugging Face

```bash
the-witness model download --source huggingface --model witness-gemma4-e2b-judge
```

Model link:

```text
https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge
```

The fine-tuned adapter is designed for structured JSON verdicts: `APPROVED`, `DISAPPROVED`, or `NEEDS_HUMAN_REVIEW`.

## 9. Fine-tuning notebook

```text
https://colab.research.google.com/drive/17-CgEQLNg8bpnhhWzJwpapRxQyHIqybq?usp=sharing
```

The notebook shows the reproducible Unsloth fine-tuning path for the Witness judge.

## 10. Expected result

When a request goes through The Witness:

1. The request is captured.
2. The upstream endpoint response is captured as a candidate response.
3. Gemma 4 judges the candidate response with a structured JSON schema.
4. Unsafe, incorrect, or misaligned responses are rejected.
5. The prompt is repaired using the rejection reason and suggested fix.
6. The request is retried.
7. Approved responses are returned to the original app.
8. The full verification chain is saved to logs/audit reports.

The Witness does not claim perfect safety. It adds a local, explainable verification layer between AI generation and real-world use.
