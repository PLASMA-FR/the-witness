# Setup Guide

Who this is for: anyone installing The Witness for the first time or preparing a demo machine.

What you will do: install the binary, choose a Gemma judge backend, pull or configure a model, run doctor, and protect your first endpoint.

The Witness does not assume your machine is ready. The setup wizard and doctor command check the pieces that matter before live endpoint watching.

## 1. Install

Linux/macOS:

```bash
curl -fsSL https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.sh | bash
```

Windows PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -Command "irm https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.ps1 | iex"
```

Manual build from a checkout:

```bash
git clone https://github.com/PLASMA-FR/the-witness.git
cd the-witness
cargo build --release
./target/release/the-witness setup
```

## 2. Choose a judge backend

Recommended first path: Ollama.

Supported judge paths:

| Backend | Use it when |
|---|---|
| Ollama | You want the easiest local Gemma setup. |
| llama.cpp | You have a GGUF/server workflow or resource-constrained machine. |
| LiteRT | You want an edge prefilter path with a compatible LiteRT model/runtime. |
| Unsloth | You want to use the fine-tuned Witness judge adapter. |
| Manual endpoint | You already run a local OpenAI-compatible judge endpoint. |
| Demo | You want to review the approval loop without external setup. |

## 3. Pull local models

```bash
ollama pull gemma4:e2b
ollama pull gemma4:e4b
```

`gemma4:e2b` is the fast/default local judge. `gemma4:e4b` is the stronger option for high-risk or stricter profiles. Model names are editable in config and the Web UI.

### Optional: register a custom Ollama model

This is an advanced addition, not the primary path. The Witness is designed and documented around Gemma 4 as the recommended judge. If you have another local Ollama tag that you want to experiment with, register it explicitly so it appears in the Web UI model manager:

```bash
the-witness model add-ollama \
  --model "my-local-judge:latest" \
  --display-name "My local experimental judge"
```

Set it as the current judge only when you intentionally want to use it:

```bash
the-witness model add-ollama \
  --model "my-local-judge:latest" \
  --display-name "My local experimental judge" \
  --set-default
```

Pull through Ollama at the same time if the model is available from your Ollama registry:

```bash
the-witness model add-ollama --model "my-local-judge:latest" --pull
```

The same flow is available in the Web UI under Settings -> Custom Ollama model. After selecting any custom judge, run:

```bash
the-witness model test --backend ollama --model "my-local-judge:latest"
the-witness doctor
```

Keep Gemma 4 (`gemma4:e2b` / `gemma4:e4b`) as the default recommendation for demos, docs, and hackathon judging unless you are intentionally testing a custom local model.

## 4. Run setup and doctor

```bash
the-witness setup
the-witness doctor
```

Doctor groups checks into core config, model readiness, endpoints, storage, and optional integrations. Each warning or failure includes a fix command.

Example:

```text
[FAIL] gemma4:e2b is missing
Why it matters: This is the default local judge model.
Fix: Run `ollama pull gemma4:e2b`.
```

## 5. Start the Web UI or TUI

Web UI:

```bash
the-witness dashboard
```

Open:

```text
http://127.0.0.1:8790
```

TUI:

```bash
the-witness start
```

## 6. Add your first endpoint

Blackbox example:

```bash
export BLACKBOX_API_KEY="YOUR_KEY_HERE"
the-witness endpoint add-blackbox
```

The key stays in your shell. The Witness stores only the environment variable name.

Manual endpoint:

```bash
the-witness endpoint add \
  --name "Local Tutor" \
  --upstream "http://localhost:8000/v1" \
  --local "http://localhost:8787/v1" \
  --profile education \
  --retry-limit 3 \
  --strictness medium
```

## 7. Verify the approval loop

Send a request through the local proxy URL shown by the endpoint list or Web UI:

```bash
curl http://localhost:8787/v1/chat/completions \
  -H 'content-type: application/json' \
  -d '{"model":"local-tutor","messages":[{"role":"user","content":"Explain why 2 + 2 = 4 in one sentence."}]}'
```

Then check:

```bash
the-witness logs
the-witness dashboard
```

## Troubleshooting

| Symptom | Why it matters | Fix |
|---|---|---|
| Ollama is not installed | The default local judge needs it. | Install Ollama, then run `ollama pull gemma4:e2b`. |
| `gemma4:e2b` is missing | The default judge model is not available. | `ollama pull gemma4:e2b` |
| Port 8787 is in use | The local proxy cannot bind its default port. | Stop the other process or configure another local proxy URL. |
| `BLACKBOX_API_KEY` is not set | The Blackbox example cannot authenticate. | `export BLACKBOX_API_KEY="YOUR_KEY_HERE"` |
| Dashboard backend is unreachable | The Web UI cannot load live app state. | Run `the-witness dashboard`. |
| Judge schema test has not passed | The proxy needs valid JSON verdicts before live use. | `the-witness model test --backend ollama --model gemma4:e2b` |

## Responsible setup note

The Witness reduces risk; it does not guarantee truth. High-risk medical, legal, financial, emergency, or destructive-action outputs should stay reviewable by qualified people.
