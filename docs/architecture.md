# Architecture

Who this is for: developers and reviewers who want to understand how The Witness is put together.

What you will do: follow the request path from an AI app through the proxy, judge, repair loop, human review queue, and audit log.

## System overview

The Witness is a local-first reliability firewall for OpenAI-compatible AI endpoints.

```text
AI App
  -> Local Proxy Route
  -> Upstream AI Endpoint
  -> Candidate Response
  -> Gemma 4 Judge
  -> Approved / Blocked / Needs Human Decision
  -> Return / Repair + Retry / Queue
  -> JSONL Audit Log
```

## Core pieces

| Area | Role |
|---|---|
| CLI | Starts setup, doctor, dashboard, TUI, proxy, model, endpoint, replay, export, and service flows. |
| TUI | Terminal-native operator interface for setup and live monitoring. |
| Web UI | Local mission-control dashboard for endpoints, live requests, repairs, reviews, models, logs, doctor, and settings. |
| Proxy | Receives app traffic, forwards requests, captures candidate responses, calls the judge, and returns only approved or configured fallback responses. |
| Judge | Uses Gemma 4 through Ollama, llama.cpp, LiteRT, Unsloth, or a manual OpenAI-compatible endpoint. |
| Repair | Converts rejection reasons into hidden retry instructions without changing the user’s actual request. |
| Storage | Writes JSONL audit events and supports replay/export. |

## Verdict schema

Gemma 4 must return valid JSON with:

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

## Safety boundaries

The Witness reduces risk; it does not guarantee truth. The architecture is built to make uncertainty visible: disapproved responses are blocked, risky cases can be queued for human review, and every decision can be audited.
