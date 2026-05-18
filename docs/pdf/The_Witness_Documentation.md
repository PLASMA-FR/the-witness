# The Witness

A Local-First Gemma 4 Reliability Firewall for AI Endpoints

> Do not just trust AI. Let The Witness see it first.

Repository: https://github.com/PLASMA-FR/the-witness  
Fine-tuned model: https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge  
Fine-tuning notebook: https://colab.research.google.com/drive/17-CgEQLNg8bpnhhWzJwpapRxQyHIqybq?usp=sharing

## Table of contents

1. Executive Summary
2. The Problem
3. The Solution
4. Core Workflow
5. Features
6. TUI Guide
7. Installation
8. Model Setup
9. Adding an Endpoint
10. Gemma 4 Judge Schema
11. Prompt Repair
12. Technology Tracks
13. Impact Tracks
14. Demo Walkthrough
15. Security and Privacy
16. Limitations
17. Reproducibility
18. Links
19. Final Pitch

## 1. Executive Summary

The Witness is a TUI-based local proxy that lets users add AI endpoints, watch every request and response, verify each response using Gemma 4, block bad answers, repair prompts, retry, and only return approved responses.

> The Witness is not another chatbot. It is the verification layer between AI generation and real-world action.

## 2. The Problem

AI tools are becoming agents. They write code, tutor students, summarize health information, support finance workflows, and automate work. Many tools still trust the first model response immediately. In sensitive domains, a bad response can cause incorrect code, unsafe commands, bad tutoring explanations, overconfident advice, hallucinated facts, and untraceable decisions.

## 3. The Solution

The Witness sits between an AI app and an upstream model endpoint. It watches requests, captures candidate responses, sends them to Gemma 4, receives verdicts, approves safe responses, rejects weak responses, repairs prompts, retries, escalates risky cases, and logs decisions.

```text
AI App / Agent
  ↓
The Witness Local Proxy
  ↓
Upstream AI Endpoint
  ↓
Candidate Response
  ↓
Gemma 4 Judge
  ↓
APPROVED / DISAPPROVED / NEEDS_HUMAN_REVIEW
  ↓
Return / Repair + Retry / Human Review
  ↓
Audit Log
```

## 4. Core Workflow

1. User adds endpoint.
2. The Witness creates a local proxy URL.
3. AI app sends request to local proxy.
4. The Witness forwards request to upstream endpoint.
5. Upstream returns candidate response.
6. Gemma 4 judges response.
7. If approved, response is returned.
8. If disapproved, response is blocked.
9. Prompt repair is generated.
10. Request is retried.
11. If risky, human review queue is used.
12. Logs are saved.

## 5. Features

| Feature | What it does | Why it matters |
|---|---|---|
| TUI dashboard | Live operational view of endpoint health, request stats, approvals, rejections, retry counts, and current judge configuration. | Judges and users can see the system working without opening a browser. |
| Endpoint watchlist | Add, edit, enable, disable, test, and duplicate watched endpoint configurations. | Every endpoint can have its own proxy URL, profile, retry limit, strictness, and fallback behavior. |
| OpenAI-compatible local proxy | Applications point at localhost while The Witness forwards to the real upstream endpoint. | Existing tools can adopt the verification layer with minimal configuration changes. |
| Request/response monitoring | Captures model name, prompt content, metadata, latency, and candidate response before release. | Turns invisible model traffic into an auditable timeline. |
| Gemma 4 judge | A local Gemma 4 model reviews candidate responses and returns structured safety and quality verdicts. | Keeps the trust layer local-first and inspectable. |
| Structured JSON verdicts | The judge must output a strict schema with verdict, confidence, scores, risk, reason, and repair instruction. | Machine-readable decisions make automation and auditing possible. |
| Approval states | Supports APPROVED, DISAPPROVED, and NEEDS_HUMAN_REVIEW. | Not every response is binary; risky cases can pause for a person. |
| Prompt repair loop | Rejected answers produce hidden corrective instructions and a retry request. | The app improves outputs instead of simply failing silently. |
| Automatic retry | Retries until approved or the endpoint retry limit is reached. | Gives upstream models a chance to correct mistakes before users see them. |
| Human review queue | Risky or uncertain outputs can be approved, rejected, edited, regenerated, exported, or annotated. | Critical workflows get human judgment when needed. |
| Audit logs | JSONL logs record decisions, retry chains, prompt repairs, and endpoint errors. | Post-incident review and hackathon demos have evidence. |
| Secret redaction | Authorization headers and API keys are hidden in TUI and logs. | Demos and screenshots remain safe to share. |
| Privacy mode | Can store metadata only instead of full prompts/responses. | Supports sensitive workflows and low-trust environments. |
| Per-endpoint profiles | Profiles tune criteria for coding, education, medical, finance, legal, research, disaster response, and multilingual use. | Different domains need different review strictness. |
| Model manager | Lists Gemma model choices and backends including Ollama, llama.cpp, LiteRT, Unsloth, and manual endpoints. | Users can choose speed, quality, or hardware fit. |
| Ollama support | Recommended local backend with configurable Gemma model tags such as gemma4:e2b and gemma4:e4b. | The easiest path for local Gemma judging. |
| llama.cpp support | Connects to local/resource-constrained GGUF inference servers. | Supports CPU/GPU constrained local deployments. |
| LiteRT edge prefilter | Experimental lightweight edge classification path before full judging. | Enables future low-latency edge safety checks. |
| Unsloth fine-tuning notebook | Public Colab notebook fine-tunes a Witness Gemma 4 E2B judge with LoRA/QLoRA. | Makes the judge specialization reproducible. |
| Hugging Face model support | The fine-tuned Witness Gemma 4 E2B judge adapter is published on Hugging Face. | Users can download and inspect the model artifact path. |
| Blackbox endpoint example | A ready endpoint template uses BLACKBOX_API_KEY through environment variables. | Shows real external endpoint use without committing secrets. |
| Doctor command | Checks backend, model, schema, proxy, logs, endpoint, and optional integrations. | Pre-demo verification is fast and transparent. |
| Curl installer | Install path can be scripted while still offering safer download-review-run steps. | Accessible for new users and judges. |
| Demo mode | Deterministic local judge and mock/demo endpoint make the workflow visible without external keys. | Hackathon judges can evaluate the idea even without private API credentials. |

## 6. TUI Guide

- Setup Wizard: choose backend, model, judge test, proxy test, readiness checklist.
- Dashboard: active endpoints, requests, approvals, rejections, retries, current model.
- Endpoint Watchlist: add/edit endpoints, copy local proxy URL, set profile, set retry limit.
- Live Request Stream: received, forwarded, judging, disapproved, retrying, approved.
- Model Manager: gemma4:e2b, gemma4:e4b, Hugging Face fine-tuned model, custom Ollama, llama.cpp, LiteRT, manual endpoint.
- Prompt Repair Panel: rejected response, rejection reason, suggested fix, repaired prompt.
- Human Review Queue: approve, reject, edit, regenerate.
- Logs / Audit: request timeline, retry chains, verdicts, exports.

## 7. Installation

Quick install:

```bash
curl -fsSL https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.sh | bash
```

Safer install:

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

Requirements: Rust, Cargo, optional Ollama, optional Gemma models, optional Blackbox API key, optional Hugging Face model.

## 8. Model Setup

Default judge: `gemma4:e2b`  
Strong/high-risk judge: `gemma4:e4b`

```bash
ollama pull gemma4:e2b
ollama pull gemma4:e4b
```

Fine-tuned model: https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge

```bash
the-witness model download --source huggingface --model witness-gemma4-e2b-judge
```

Fine-tuning notebook: https://colab.research.google.com/drive/17-CgEQLNg8bpnhhWzJwpapRxQyHIqybq?usp=sharing

## 9. Adding an Endpoint

Set key safely without putting a literal secret in shell history:

```bash
read -s BLACKBOX_API_KEY
export BLACKBOX_API_KEY
```

Add endpoint:

```bash
the-witness endpoint add-blackbox
```

Endpoint config: upstream `https://api.blackbox.ai/v1`, local proxy `http://localhost:8787/v1`, model `blackboxai/x-ai/grok-code-fast-1:free`, profile `coding`, strictness `high`, retry limit `4`, auth `bearer_env BLACKBOX_API_KEY`.

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

Expected result: request appears in TUI, response is captured, Gemma judges it, rejected answers trigger repair/retry, approved answer is returned, logs are saved.

## 10. Gemma 4 Judge Schema

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

## 11. Prompt Repair

When Gemma rejects a response, The Witness creates a repaired prompt that preserves intent, includes the rejection reason and suggested fix, asks the upstream model to avoid the mistake, becomes stricter after repeated failures, avoids leaking secrets, and prevents infinite loops.

Example: `print(Hello World)` is rejected because the Python string is not quoted. The repaired request asks for valid Python syntax. The approved response is `print("Hello World")`.

## 12. Technology Tracks

- Ollama: default local Gemma judge backend.
- llama.cpp: local/resource-constrained inference using Gemma models.
- LiteRT: experimental edge prefilter path.
- Unsloth: public Colab fine-tuning notebook and Hugging Face adapter.
- Cactus: architecture is Cactus-ready for future mobile companion work; mobile support is not claimed as completed.

## 13. Impact Tracks

Primary: Safety & Trust. The Witness provides explainable verification, structured verdicts, rejection reasons, prompt repair, human review, and audit logs.

Secondary: Digital Equity & Inclusivity, Future of Education, Health & Sciences, Global Resilience.

## 14. Demo Walkthrough

Install, pull gemma4:e2b, run setup, start TUI, add endpoint, send curl request, capture response, reject bad response, repair prompt, retry, approve, save audit report.

Relevant folders: `demo_usage/`, `gallery/`, and `demo_videos/` if present.

## 15. Security and Privacy

API keys are never stored in docs. Examples use env vars. Authorization headers are redacted. Privacy mode can store metadata only. Never paste real API keys into GitHub, screenshots, or videos.

## 16. Limitations

The Witness reduces risk but does not guarantee correctness. Gemma can be wrong. High-risk medical/legal/financial outputs still need professionals. Local performance depends on model and hardware. LiteRT is experimental. Cactus/mobile companion is future work. Streaming support may be limited. Fine-tuned model quality depends on training data.

## 17. Reproducibility

```bash
cargo fmt
cargo test
cargo build
bash scripts/verify.sh
the-witness doctor
```

Expected: build/test should pass. Doctor may warn if optional models or external keys are missing.

## 18. Links

- GitHub: https://github.com/PLASMA-FR/the-witness
- Fine-tuned model: https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge
- Fine-tuning notebook: https://colab.research.google.com/drive/17-CgEQLNg8bpnhhWzJwpapRxQyHIqybq?usp=sharing
- Ollama tags: `gemma4:e2b`, `gemma4:e4b`

## 19. Final Pitch

The Witness is a local-first reliability firewall for the age of AI agents.

As AI systems become more powerful, the question is no longer only: “Can the model answer?”

The question is: “Should this answer be allowed to act?”

The Witness turns Gemma 4 into the local trust layer that answers that question.

Do not just trust AI. Let The Witness see it first.
