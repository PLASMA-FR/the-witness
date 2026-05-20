# The Witness Hackathon Writeup

The Witness is a local-first Gemma 4 reliability firewall for AI endpoints.

It is the missing local trust layer between AI generation and real-world action.

## 1. Problem

AI outputs are becoming actions. Coding agents edit files, tutors explain concepts, research assistants summarize evidence, and workflow tools make recommendations that people act on. Most of these systems still trust the first model response the moment it arrives.

That is the gap The Witness targets: not another chatbot, but a verification layer between model output and release.

## 2. Solution

The Witness runs as a local OpenAI-compatible proxy. A user adds the AI endpoints they depend on, points their app at the local proxy URL, and lets The Witness watch each request and response.

For every candidate response, Gemma 4 returns a structured JSON verdict. Approved responses are returned to the app. Blocked responses are not released. The Witness turns the rejection reason into a repaired retry prompt, sends the improved request to the same upstream endpoint, and saves the full audit trail. High-risk or uncertain responses can pause for human review.

## 3. How it works

```text
AI App
  -> The Witness Local Proxy
  -> Upstream Endpoint
  -> Candidate Response
  -> Gemma 4 Judge
  -> Approved / Blocked / Needs Human Decision
  -> Return / Repair + Retry / Human Review
  -> Audit Log
```

The verdict schema includes the decision, confidence, safety score, usefulness score, prompt-alignment score, correctness risk, rejection reason, suggested fix, repair instruction, and human-review flag.

## 4. What we built

The repository includes:

- Rust CLI for setup, doctor, model management, endpoint management, replay, export, dashboard, and service management.
- Ratatui TUI with first-run setup, endpoint watchlist, live requests, inspectors, verdicts, prompt repair, human review, profiles, logs, and settings.
- Local Web UI mission-control dashboard for a visual operator workflow.
- OpenAI-compatible proxy for non-streaming chat completions.
- Endpoint manager with per-endpoint profile, strictness, retry limit, timeout, auth, and fallback behavior.
- Model manager for Ollama, llama.cpp, LiteRT, Unsloth, and manual judge endpoints.
- Doctor command that detects missing local setup and prints fix commands.
- JSONL audit logs and exportable verification reports.
- Prompt repair loop that preserves the user intent while correcting issues found by the judge.
- Demo mode and mock upstream server so reviewers can test the approval loop without external API keys.
- Installers and service helpers for day-to-day local operation.
- Public fine-tuned Witness Gemma 4 E2B judge adapter on Hugging Face.
- Colab fine-tuning notebook built around Unsloth.

## 5. Gemma 4 usage

The default local judge path is Ollama with `gemma4:e2b`, chosen for fast approval classification. `gemma4:e4b` is the stronger/high-risk option when hardware allows.

The fine-tuned Witness judge is available here:

```text
https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge
```

Fine-tuning notebook:

```text
https://colab.research.google.com/drive/17-CgEQLNg8bpnhhWzJwpapRxQyHIqybq?usp=sharing
```

## 6. Track fit

Primary impact track: Safety & Trust.

The Witness is submitted primarily for Safety & Trust because it makes AI outputs more transparent, explainable, auditable, and controllable before they reach users or agents.

Technology paths demonstrated:

- Ollama for the default local Gemma judge.
- llama.cpp for resource-constrained local inference.
- LiteRT for edge prefiltering.
- Unsloth for the fine-tuned Witness judge workflow.

Cactus is not claimed in this submission.

## 7. Responsible use

The Witness reduces risk; it does not guarantee truth. The judge model can still be wrong, which is why high-risk cases can be escalated to human review.

Medical, legal, financial, emergency, and safety-critical workflows should involve qualified professionals. Local performance depends on hardware, selected Gemma model, and runtime setup.

## 8. Final pitch

The Witness turns Gemma 4 into a local trust layer for AI agents and endpoints. It watches the response before it reaches the user, explains what went wrong, repairs the prompt, retries safely, and leaves an audit trail teams can inspect.
