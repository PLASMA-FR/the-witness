# The Witness: a local Gemma 4 firewall for AI reliability

Subtitle: The missing verification layer between AI generation and real-world action.

Suggested primary Impact Track: Safety & Trust.
Also relevant: Future of Education, Health & Sciences, Global Resilience, Digital Equity & Inclusivity.
Special Technology Tracks: Ollama, llama.cpp, LiteRT, Unsloth.

Project links for Kaggle attachments:

- Public code repository: https://github.com/PLASMA-FR/the-witness
- Quick install / live CLI demo: https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.sh
- Custom fine-tuned model download: https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge
- Fine-tuning notebook: https://colab.research.google.com/drive/17-CgEQLNg8bpnhhWzJwpapRxQyHIqybq?usp=sharing
- Ollama install page: https://ollama.com/download

## Summary

The Witness is a local-first terminal application that watches AI endpoints before their responses reach users. It runs as an OpenAI-compatible proxy. Existing apps point to a local proxy URL, The Witness forwards the request to the real upstream model, captures the candidate response, and asks Gemma 4 to judge it using a strict JSON verdict schema. If Gemma approves, the response is returned. If Gemma rejects it, The Witness blocks the answer, repairs the prompt, retries the same endpoint, and keeps the full audit trail. If the answer is risky or uncertain, it can pause the response for human review.

This matters because many AI tools now sit directly in education, coding, emergency response, health information, finance workflows, and multilingual communities. A weak answer is not just a bad chat message when it is copied into a lesson plan, a script, a safety notice, or a decision workflow. The Witness adds a second local model whose job is not to chat, but to verify.

## The problem

Most AI applications trust the first response they receive. That is fast, but brittle. Hallucinations, unsafe advice, prompt drift, incomplete code, and overconfident answers can pass straight into user workflows. Cloud-only moderation also creates problems: cost, latency, privacy, poor offline support, and weak access in communities with unstable internet.

The Witness tackles that gap. It gives builders a local verification layer that can run beside the tools they already use. No web dashboard is required. The interface is a modern Rust TUI, so it works over SSH, on low-resource machines, in labs, classrooms, field offices, and developer terminals.

## How it works

A user runs `the-witness start`. On first run, the setup wizard opens instead of the dashboard. The wizard explains the system, asks the user to choose a Gemma backend, checks hardware, lets the user pick or enter a model, helps install or pull the model, tests the judge, tests the proxy, and only then opens the main dashboard or demo mode.

The supported judge paths are:

- Ollama, the recommended default for the quickest local Gemma 4 setup.
- llama.cpp, for resource-constrained machines and GGUF-style local serving.
- LiteRT, for lightweight edge verification experiments.
- Unsloth, for a fine-tuned Gemma 4 judge.
- Manual OpenAI-compatible local endpoint, for advanced users.

For every watched endpoint, The Witness stores the endpoint name, upstream URL, local proxy URL, model name, validation profile, retry limit, timeout, strictness level, fallback behavior, and enabled state. Requests are captured with secrets redacted. The proxy currently focuses on non-streaming OpenAI-compatible chat completions for the MVP.

The Gemma judge must return only JSON:

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

The setup wizard runs two sanity tests before trusting the model. It expects Gemma to reject the false answer "2 + 2 equals 5 because numbers are flexible" and approve the correct answer "2 + 2 = 4 because adding two items to two more items gives a total of four items." It checks that the model responded, the output is valid JSON, the schema matches, the verdict is reasonable, latency is acceptable, and errors are shown clearly.

## Gemma 4 usage

Gemma 4 is the local judge. The base app supports general Gemma 4 models through Ollama, llama.cpp, LiteRT, and manual local endpoints. For the Unsloth technology track, I trained a custom Witness Gemma 4 E2B LoRA adapter for the judgment task. The adapter is published on Hugging Face:

https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge

It is intentionally an adapter-only LoRA artifact, not a full base model upload. The Witness loads the Gemma 4 E2B base model (`google/gemma-4-e2b`, or the configured equivalent) plus the adapter. That keeps distribution smaller and makes the training path reproducible. The training notebook is here:

https://colab.research.google.com/drive/17-CgEQLNg8bpnhhWzJwpapRxQyHIqybq?usp=sharing

The repository includes a model registry entry, Hugging Face download support, validation data, and scripts for checking the dataset. E4B was explored as a larger judge direction, but the published custom model is the E2B LoRA adapter because it fit the available runtime and better matches the local-first goal.

## Features in the MVP

The TUI includes a setup wizard, dashboard, endpoint watchlist, live request stream, request inspector, response inspector, Gemma verdict panel, prompt repair panel, human review queue, profiles screen, logs and audit screen, settings screen, and model manager.

Built-in profiles cover General Safety, Coding, Education, Medical, Finance, Legal, Scientific Research, Disaster Response, Arabic-English Multilingual, and Custom. Strictness levels are relaxed, medium, high, and critical. Each profile defines approval criteria, rejection criteria, human review triggers, and prompt repair style.

The CLI supports setup, doctor checks, model list/download/test, endpoint add/list/test/enable/disable, logs, replay, export, and start. Logs are written as JSONL, with export paths for Markdown and CSV-style summaries. Privacy mode can store metadata without full prompt and response bodies.

Prompt repair preserves the user’s original intent. It adds hidden corrective instructions with the rejection reason and required fix, asks the upstream model not to repeat the mistake, and becomes stricter after repeated failures. Internal judge details are not leaked to the final user.

## How to download and use

Quick install:

```bash
curl -fsSL https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.sh | bash
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

Ollama path:

```bash
ollama pull gemma4:e2b
the-witness model test --backend ollama --model gemma4:e2b
the-witness start
```

Fine-tuned adapter path:

```bash
the-witness model download --source huggingface --model witness-gemma4-e2b-judge
the-witness model test --backend unsloth --model ./models/witness-gemma4-e2b-judge
```

Add an endpoint:

```bash
the-witness endpoint add --name "Codex" --upstream "https://api.openai.com/v1" --local "http://localhost:8787/v1" --profile coding --retry-limit 4 --strictness high
```

Then configure the AI app to use `http://localhost:8787/v1` instead of the upstream URL.

## Impact

The Witness is strongest in places where trust, privacy, and connectivity all matter. A rural clinic can run a local verifier before health information leaves a tool. A teacher can use an education profile to catch misleading explanations before students see them. A disaster response team can run local checks when internet access is unreliable. A bilingual Arabic-English community can use multilingual validation without sending every prompt to a remote moderation service.

The project does not claim perfect safety. It makes reliability visible and inspectable. Every retry chain, rejected answer, repaired prompt, human override, endpoint error, and judge error is logged. That audit trail is the difference between hoping an AI answer was good and being able to show how it was checked.

## Technology track fit

For Ollama, The Witness gives the cleanest local Gemma 4 path: install, pull, test, start watching endpoints. For llama.cpp, it provides a route for small machines and offline deployments. For LiteRT, it sketches the edge verification path where the judge can run close to users. For Unsloth, it includes a custom fine-tuned Gemma 4 E2B LoRA judge trained for structured verdicts and published for download. The same app can demonstrate all four tracks because its backend abstraction treats Gemma as a swappable local judge.

The result is not another chatbot. It is infrastructure for safer AI use: a local witness that stands between generated text and real-world action.
