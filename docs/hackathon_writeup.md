# The Witness Hackathon Writeup

The Witness is a local-first Gemma 4 reliability firewall for AI endpoints.

Most AI apps trust a model response the moment it arrives. The Witness adds a verification layer between generation and real-world use. Apps send OpenAI-compatible requests to a local proxy. The proxy forwards the request to the chosen upstream model, captures the candidate response, asks Gemma 4 to judge it against a strict JSON schema, and only releases approved responses. If Gemma disapproves, The Witness blocks the answer, repairs the prompt with the rejection reason, retries the same upstream endpoint, and saves the full audit trail.

## Primary impact track: Safety & Trust

The project is built for teams that need more than a chatbot transcript. Education tools, coding assistants, health-information helpers, finance workflows, disaster-response systems, and multilingual apps all need a place where risky or low-quality responses can be stopped before users rely on them.

The Witness focuses on practical trust:

- every request receives a traceable request ID;
- secrets are redacted from the TUI, Web UI, API, logs, and exports;
- Gemma verdicts use a fixed JSON schema instead of free-form prose;
- disapproved responses are not silently returned;
- retry chains show the original prompt, rejected response, verdict, repair, and final result;
- uncertain or high-risk answers can be paused for human review;
- privacy mode can keep only metadata instead of full prompts and responses.

## Core loop

1. A user adds an endpoint to watch.
2. Their AI app points at The Witness local proxy.
3. The Witness forwards the request to the real upstream endpoint.
4. The upstream returns a candidate response.
5. Gemma 4 judges the candidate using this schema: verdict, confidence, safety score, usefulness score, prompt-alignment score, correctness risk, rejection reason, suggested fix, repair instruction, and human-review flag.
6. Approved responses are returned to the original app.
7. Disapproved responses are blocked, repaired, retried, and logged.
8. Human-review verdicts are queued for manual decision.

## First-run setup

The TUI includes a setup wizard so the app does not assume the machine is ready. It guides users through backend selection, model selection, hardware checks, model install or configuration, judge schema tests, verdict sanity tests, proxy tests, endpoint tests, and a final readiness checklist. Users can choose demo mode when they want to explore the workflow before connecting a real API.

## Technology tracks

The Witness is designed around four Gemma deployment paths:

- Ollama: default local backend, with `gemma4:e2b` as the fast judge and `gemma4:e4b` as the stronger high-risk option.
- llama.cpp: local GGUF/server path for resource-constrained machines.
- LiteRT: lightweight edge-verification path for small local classification checks.
- Unsloth: fine-tuned Gemma judge workflow using a public Colab notebook and a Hugging Face-hosted adapter.

Fine-tuned Witness Gemma 4 E2B Judge:
https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge

Fine-tuning notebook:
https://colab.research.google.com/drive/17-CgEQLNg8bpnhhWzJwpapRxQyHIqybq?usp=sharing

## Demo endpoint

The Blackbox example shows how a real coding endpoint can be routed safely:

- upstream: `https://api.blackbox.ai/v1`
- model: `blackboxai/x-ai/grok-code-fast-1:free`
- local proxy: `http://localhost:8787/v1`
- auth env var: `BLACKBOX_API_KEY`
- profile: `coding`
- strictness: `high`
- retry limit: `4`

The config stores the environment variable name, not the secret value.

## Product surface

The release includes:

- Rust CLI commands for setup, doctor, model management, endpoint management, replay, export, dashboard, and service management;
- a Ratatui TUI for setup, endpoint watching, request stream, verdicts, prompt repair, human review, profiles, logs, and settings;
- a local Web UI mission-control dashboard for operators who want a richer visual view;
- JSONL audit logging and exportable reports;
- installers and service helpers for day-to-day use.

## Honest limitations

The Witness reduces risk; it does not guarantee correctness. Gemma can be wrong. High-risk medical, legal, and financial answers still require qualified human judgment. The MVP starts with non-streaming OpenAI-compatible chat completions. External backends such as Ollama, llama.cpp, LiteRT, Unsloth, Hugging Face, and Blackbox require the user to configure the relevant local runtime, model files, or API credentials before live use.

The goal is not to replace experts. The goal is to make AI systems less reckless by adding a local verification gate, a retry-and-repair loop, and an audit trail before responses reach users.
