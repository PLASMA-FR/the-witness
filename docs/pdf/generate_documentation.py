#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path
import html

ROOT = Path('/home/admin/Gemma/witness')
OUT = ROOT / 'docs' / 'pdf'
OUT.mkdir(parents=True, exist_ok=True)

REPO_URL = 'https://github.com/PLASMA-FR/the-witness'
HF_URL = 'https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge'
COLAB_URL = 'https://colab.research.google.com/drive/17-CgEQLNg8bpnhhWzJwpapRxQyHIqybq?usp=sharing'

images = {
    'cover': '../../gallery/01_cover_the_witness.png',
    'arch': '../../gallery/02_architecture_diagram.png',
    'loop': '../../gallery/03_approval_loop.png',
    'dashboard': '../../demo_usage/screenshots/05_tui_dashboard.png',
    'models': '../../demo_usage/screenshots/06_model_manager.png',
    'endpoints': '../../demo_usage/screenshots/07_endpoint_watchlist.png',
    'stream': '../../demo_usage/screenshots/08_live_request_stream.png',
    'verdict': '../../demo_usage/screenshots/09_verdict_prompt_repair.png',
    'logs': '../../demo_usage/screenshots/10_logs_audit.png',
    'curl': '../../demo_usage/screenshots/11_curl_test.png',
    'hf': '../../demo_usage/screenshots/12_huggingface_model.png',
    'colab': '../../demo_usage/screenshots/13_colab_notebook.png',
    'tracks': '../../gallery/14_track_badge_wall.png',
    'impact': '../../gallery/07_impact_track_map.png',
}

features = [
    ('TUI dashboard', 'Live operational view of endpoint health, request stats, approvals, rejections, retry counts, and current judge configuration.', 'Judges and users can see the system working without opening a browser.'),
    ('Endpoint watchlist', 'Add, edit, enable, disable, test, and duplicate watched endpoint configurations.', 'Every endpoint can have its own proxy URL, profile, retry limit, strictness, and fallback behavior.'),
    ('OpenAI-compatible local proxy', 'Applications point at localhost while The Witness forwards to the real upstream endpoint.', 'Existing tools can adopt the verification layer with minimal configuration changes.'),
    ('Request/response monitoring', 'Captures model name, prompt content, metadata, latency, and candidate response before release.', 'Turns invisible model traffic into an auditable timeline.'),
    ('Gemma 4 judge', 'A local Gemma 4 model reviews candidate responses and returns structured safety and quality verdicts.', 'Keeps the trust layer local-first and inspectable.'),
    ('Structured JSON verdicts', 'The judge must output a strict schema with verdict, confidence, scores, risk, reason, and repair instruction.', 'Machine-readable decisions make automation and auditing possible.'),
    ('Approval states', 'Supports APPROVED, DISAPPROVED, and NEEDS_HUMAN_REVIEW.', 'Not every response is binary; risky cases can pause for a person.'),
    ('Prompt repair loop', 'Rejected answers produce hidden corrective instructions and a retry request.', 'The app improves outputs instead of simply failing silently.'),
    ('Automatic retry', 'Retries until approved or the endpoint retry limit is reached.', 'Gives upstream models a chance to correct mistakes before users see them.'),
    ('Human review queue', 'Risky or uncertain outputs can be approved, rejected, edited, regenerated, exported, or annotated.', 'Critical workflows get human judgment when needed.'),
    ('Audit logs', 'JSONL logs record decisions, retry chains, prompt repairs, and endpoint errors.', 'Post-incident review and hackathon demos have evidence.'),
    ('Secret redaction', 'Authorization headers and API keys are hidden in TUI and logs.', 'Demos and screenshots remain safe to share.'),
    ('Privacy mode', 'Can store metadata only instead of full prompts/responses.', 'Supports sensitive workflows and low-trust environments.'),
    ('Per-endpoint profiles', 'Profiles tune criteria for coding, education, medical, finance, legal, research, disaster response, and multilingual use.', 'Different domains need different review strictness.'),
    ('Model manager', 'Lists Gemma model choices and backends including Ollama, llama.cpp, LiteRT, Unsloth, and manual endpoints.', 'Users can choose speed, quality, or hardware fit.'),
    ('Ollama support', 'Recommended local backend with configurable Gemma model tags such as gemma4:e2b and gemma4:e4b.', 'The easiest path for local Gemma judging.'),
    ('llama.cpp support', 'Connects to local/resource-constrained GGUF inference servers.', 'Supports CPU/GPU constrained local deployments.'),
    ('LiteRT edge prefilter', 'Experimental lightweight edge classification path before full judging.', 'Enables future low-latency edge safety checks.'),
    ('Unsloth fine-tuning notebook', 'Public Colab notebook fine-tunes a Witness Gemma 4 E2B judge with LoRA/QLoRA.', 'Makes the judge specialization reproducible.'),
    ('Hugging Face model support', 'The fine-tuned Witness Gemma 4 E2B judge adapter is published on Hugging Face.', 'Users can download and inspect the model artifact path.'),
    ('Blackbox endpoint example', 'A ready endpoint template uses BLACKBOX_API_KEY through environment variables.', 'Shows real external endpoint use without committing secrets.'),
    ('Doctor command', 'Checks backend, model, schema, proxy, logs, endpoint, and optional integrations.', 'Pre-demo verification is fast and transparent.'),
    ('Curl installer', 'Install path can be scripted while still offering safer download-review-run steps.', 'Accessible for new users and judges.'),
    ('Demo mode', 'Deterministic local judge and mock/demo endpoint make the workflow visible without external keys.', 'Hackathon judges can evaluate the idea even without private API credentials.'),
]

schema = '''{
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
}'''

curl_cmd = '''curl http://localhost:8787/v1/chat/completions \\
  -H "Authorization: Bearer $BLACKBOX_API_KEY" \\
  -H "Content-Type: application/json" \\
  -d '{
    "model": "blackboxai/x-ai/grok-code-fast-1:free",
    "messages": [
      {
        "role": "user",
        "content": "Write a Python script that prints Hello World"
      }
    ]
  }' '''

# Markdown source: intentionally clean and portable. The richer HTML/CSS version below is used for the PDF.
md = f'''# The Witness

A Local-First Gemma 4 Reliability Firewall for AI Endpoints

> Do not just trust AI. Let The Witness see it first.

Repository: {REPO_URL}  
Fine-tuned model: {HF_URL}  
Fine-tuning notebook: {COLAB_URL}

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
''' + '\n'.join(f'| {a} | {b} | {c} |' for a,b,c in features) + f'''

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
git clone {REPO_URL}.git
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

Fine-tuned model: {HF_URL}

```bash
the-witness model download --source huggingface --model witness-gemma4-e2b-judge
```

Fine-tuning notebook: {COLAB_URL}

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
{curl_cmd}
```

Expected result: request appears in TUI, response is captured, Gemma judges it, rejected answers trigger repair/retry, approved answer is returned, logs are saved.

## 10. Gemma 4 Judge Schema

```json
{schema}
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

- GitHub: {REPO_URL}
- Fine-tuned model: {HF_URL}
- Fine-tuning notebook: {COLAB_URL}
- Ollama tags: `gemma4:e2b`, `gemma4:e4b`

## 19. Final Pitch

The Witness is a local-first reliability firewall for the age of AI agents.

As AI systems become more powerful, the question is no longer only: “Can the model answer?”

The question is: “Should this answer be allowed to act?”

The Witness turns Gemma 4 into the local trust layer that answers that question.

Do not just trust AI. Let The Witness see it first.
'''

css = r'''
@page {
  size: A4;
  margin: 16mm 14mm 18mm;
  @bottom-center {
    content: "The Witness • Local-first Gemma 4 reliability firewall • " counter(page);
    font-size: 9px;
    color: #6b7280;
  }
}
* { box-sizing: border-box; }
html { font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; color: #111827; }
body { margin: 0; background: #f5f7fa; line-height: 1.48; }
a { color: #047d70; text-decoration: none; }
h1, h2, h3 { letter-spacing: -0.03em; line-height: 1.05; margin: 0 0 12px; }
h1 { font-size: 58px; }
h2 { font-size: 30px; color: #0b0f14; border-bottom: 2px solid #e5e7eb; padding-bottom: 8px; margin-top: 12px; }
h3 { font-size: 18px; color: #101820; margin-top: 16px; }
p { margin: 0 0 10px; }
ul, ol { margin-top: 6px; padding-left: 22px; }
code, pre { font-family: "JetBrains Mono", "SFMono-Regular", Consolas, monospace; }
pre { background: #0b0f14; color: #e5fff9; padding: 13px 15px; border-radius: 13px; overflow-wrap: break-word; white-space: pre-wrap; border: 1px solid #1f3941; font-size: 10.5px; line-height: 1.42; }
section { page-break-inside: avoid; }
.page { page-break-after: always; min-height: 257mm; padding: 0; background: #ffffff; }
.page:last-child { page-break-after: auto; }
.cover { color: #f5f7fa; background: radial-gradient(circle at 20% 16%, rgba(47,255,208,0.22), transparent 28%), radial-gradient(circle at 80% 70%, rgba(81,216,138,0.17), transparent 30%), linear-gradient(135deg, #05080c 0%, #0b0f14 50%, #101820 100%); padding: 28mm 18mm 18mm; position: relative; overflow: hidden; }
.cover:before { content: ""; position: absolute; inset: 0; background-image: linear-gradient(rgba(47,255,208,.05) 1px, transparent 1px), linear-gradient(90deg, rgba(47,255,208,.05) 1px, transparent 1px); background-size: 28px 28px; opacity: .7; }
.cover > * { position: relative; }
.cover h1 { font-size: 78px; text-shadow: 0 0 34px rgba(47,255,208,.25); margin-top: 18mm; }
.cover .subtitle { font-size: 24px; color: #d6fff7; max-width: 760px; font-weight: 700; }
.cover .tagline { color: #9aa4b2; font-size: 18px; margin-top: 10px; }
.cover-card { margin-top: 24px; border: 1px solid rgba(47,255,208,.35); background: rgba(6,15,20,.72); border-radius: 24px; padding: 18px; box-shadow: 0 24px 90px rgba(0,0,0,.35); }
.cover-visual { width: 100%; border-radius: 18px; border: 1px solid rgba(47,255,208,.25); display: block; }
.badges { display: flex; flex-wrap: wrap; gap: 8px; margin: 18px 0; }
.badge { display: inline-block; border-radius: 999px; padding: 5px 10px; font-size: 10px; font-weight: 800; letter-spacing: .06em; text-transform: uppercase; background: #e8fff9; color: #06362f; border: 1px solid #b7fff0; }
.cover .badge { background: rgba(47,255,208,.12); color: #d9fff8; border-color: rgba(47,255,208,.4); }
.badge.red { background: #ffe7e7; color: #8d1616; border-color: #ffcaca; }
.badge.green { background: #e9fff1; color: #116b38; border-color: #bdf4d0; }
.badge.amber { background: #fff7df; color: #7b5200; border-color: #ffe3a0; }
.badge.dark { background: #101820; color: #f5f7fa; border-color: #2c3e50; }
.content { padding: 16mm 14mm; }
.toc-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 9px 18px; font-size: 12px; }
.toc-item { padding: 9px 10px; border-radius: 12px; background: #f4f8f8; border: 1px solid #e1eeee; }
.callout { border-radius: 16px; padding: 14px 16px; margin: 14px 0; border-left: 6px solid #2fffd0; background: #ecfffb; }
.callout.warning { border-left-color: #ff5c5c; background: #fff0f0; }
.callout.example { border-left-color: #f6c85f; background: #fff8e7; }
.callout.track { border-left-color: #51d88a; background: #edfff4; }
.hero-row, .two-col { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; align-items: start; }
.three-col { display: grid; grid-template-columns: repeat(3, 1fr); gap: 10px; }
.feature-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 10px; }
.card { border: 1px solid #e2e8f0; border-radius: 16px; padding: 12px; background: #ffffff; box-shadow: 0 8px 24px rgba(15,23,42,.05); page-break-inside: avoid; }
.card h3 { margin-bottom: 5px; }
.card .why { margin-top: 7px; color: #526173; font-size: 11px; }
.diagram { background: #0b0f14; color: #f5f7fa; border-radius: 22px; padding: 18px; border: 1px solid #1f3941; box-shadow: inset 0 0 0 1px rgba(47,255,208,.08); }
.flow { display: flex; flex-direction: column; gap: 7px; align-items: stretch; text-align: center; font-size: 13px; font-weight: 800; }
.flow .node { border: 1px solid rgba(47,255,208,.36); border-radius: 14px; padding: 9px; background: rgba(47,255,208,.08); }
.flow .arrow { color: #2fffd0; font-size: 18px; }
.loop { display: grid; grid-template-columns: 1fr 44px 1fr; gap: 10px; align-items: center; }
.loop .decision { background: #101820; color: #f5f7fa; border-radius: 18px; padding: 15px; text-align: center; }
.img-frame { margin: 12px 0; border-radius: 18px; border: 1px solid #dce7ea; padding: 7px; background: #f8fbfc; box-shadow: 0 14px 36px rgba(15,23,42,.08); page-break-inside: avoid; }
.img-frame img { width: 100%; display: block; border-radius: 12px; }
.caption { color: #5b6878; font-size: 10.5px; margin-top: 5px; }
.terminal { background: #0b0f14; color: #d7fff6; border-radius: 18px; padding: 14px; border: 1px solid #1d3842; font-family: "JetBrains Mono", Consolas, monospace; font-size: 11px; }
.terminal .bar { display: flex; gap: 6px; margin-bottom: 12px; }
.dot { width: 10px; height: 10px; border-radius: 50%; background: #ff5c5c; display: inline-block; }
.dot.amber { background: #f6c85f; } .dot.green { background: #51d88a; }
table { width: 100%; border-collapse: collapse; margin: 12px 0; font-size: 10.5px; }
th { background: #101820; color: #f5f7fa; text-align: left; }
th, td { padding: 8px; border: 1px solid #d9e2e7; vertical-align: top; }
tr:nth-child(even) td { background: #f7fafb; }
.section-kicker { text-transform: uppercase; letter-spacing: .12em; color: #047d70; font-weight: 900; font-size: 10px; margin-bottom: 6px; }
.big-quote { font-size: 26px; line-height: 1.1; color: #0b0f14; font-weight: 900; letter-spacing: -0.04em; }
.track-grid { display: grid; grid-template-columns: repeat(2,1fr); gap: 12px; }
.track-card { border-radius: 18px; padding: 14px; color: #f5f7fa; background: linear-gradient(135deg, #101820, #0b0f14); border: 1px solid rgba(47,255,208,.28); }
.track-card h3 { color: #2fffd0; }
.footer-links { font-size: 11px; word-break: break-word; }
.final { background: linear-gradient(135deg, #0b0f14, #101820); color: #f5f7fa; padding: 28mm 18mm; display: flex; flex-direction: column; justify-content: center; min-height: 257mm; }
.final h2 { color: #2fffd0; border: 0; font-size: 40px; }
.final .question { font-size: 30px; font-weight: 900; letter-spacing: -0.04em; color: #ffffff; margin: 16px 0; }
.small { font-size: 11px; color: #5d6978; }
.no-break { page-break-inside: avoid; }
'''

def badge(text, cls=''):
    return f'<span class="badge {cls}">{html.escape(text)}</span>'

def code(text, lang=''):
    return f'<pre><code>{html.escape(text)}</code></pre>'

def img(key, caption):
    return f'<div class="img-frame"><img src="{images[key]}" alt="{html.escape(caption)}"><div class="caption">{html.escape(caption)}</div></div>'

def page(title, inner, kicker=None):
    k = f'<div class="section-kicker">{html.escape(kicker)}</div>' if kicker else ''
    return f'<section class="page"><div class="content">{k}<h2>{html.escape(title)}</h2>{inner}</div></section>'

feature_cards = ''.join(f'<div class="card"><h3>{html.escape(name)}</h3><p>{html.escape(desc)}</p><p class="why"><strong>Why it matters:</strong> {html.escape(why)}</p></div>' for name, desc, why in features)
quick_install_html = code('curl -fsSL https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.sh | bash')
safer_install_html = code('curl -fsSL https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.sh -o install.sh\nless install.sh\nbash install.sh')
manual_install_html = code(f'git clone {REPO_URL}.git\ncd the-witness\ncargo build --release\n./target/release/the-witness setup\n./target/release/the-witness doctor\n./target/release/the-witness start')
e2b_pull_html = code('ollama pull gemma4:e2b')
e4b_pull_html = code('ollama pull gemma4:e4b')
hf_download_html = code('the-witness model download --source huggingface --model witness-gemma4-e2b-judge')
key_html = code('read -s BLACKBOX_API_KEY\nexport BLACKBOX_API_KEY')
add_blackbox_html = code('the-witness endpoint add-blackbox')
curl_html = code(curl_cmd)
schema_html = code(schema)
repro_html = code('cargo fmt\ncargo test\ncargo build\nbash scripts/verify.sh\nthe-witness doctor')

html_doc = f"""<!doctype html>
<html>
<head>
  <meta charset="utf-8">
  <title>The Witness Documentation</title>
  <link rel="stylesheet" href="style.css">
</head>
<body>
<section class="page cover">
  <div class="badges">{badge('Gemma 4')}{badge('Ollama')}{badge('llama.cpp')}{badge('LiteRT')}{badge('Unsloth')}{badge('Hugging Face')}{badge('Safety & Trust')}</div>
  <h1>The Witness</h1>
  <p class="subtitle">A Local-First Gemma 4 Reliability Firewall for AI Endpoints</p>
  <p class="tagline">Do not just trust AI. Let The Witness see it first.</p>
  <div class="cover-card">
    <img class="cover-visual" src="{images['cover']}" alt="The Witness cover visual">
  </div>
  <p class="footer-links"><strong>GitHub:</strong> {REPO_URL}<br><strong>Fine-tuned model:</strong> {HF_URL}<br><strong>Colab notebook:</strong> {COLAB_URL}</p>
</section>

<section class="page"><div class="content">
  <div class="section-kicker">Navigation</div>
  <h2>Table of Contents</h2>
  <div class="toc-grid">
    {''.join(f'<div class="toc-item"><strong>{i}.</strong> {t}</div>' for i,t in enumerate(['Executive Summary','The Problem','The Solution','Core Workflow','Features','TUI Guide','Installation','Model Setup','Adding an Endpoint','Gemma 4 Judge Schema','Prompt Repair','Technology Tracks','Impact Tracks','Demo Walkthrough','Security and Privacy','Limitations','Reproducibility','Links','Final Pitch'],1))}
  </div>
  <div class="callout"><strong>Reader promise:</strong> this booklet shows the project workflow, the local proxy architecture, the TUI, the Gemma judge schema, setup commands, demo path, tracks, and honest limitations.</div>
</div></section>

{page('1. Executive Summary', f'''
  <div class="hero-row">
    <div>
      <p><strong>The Witness</strong> is a TUI-based local proxy that lets users add AI endpoints, watch every request and response, verify each response using Gemma 4, block bad answers, repair prompts, retry, and only return approved responses.</p>
      <div class="callout"><strong>“The Witness is not another chatbot.”</strong><br>It is the verification layer between AI generation and real-world action.</div>
      <p>Instead of trusting the first response from an upstream model, The Witness asks a local Gemma 4 judge whether the response is safe, useful, aligned with the prompt, and low-risk enough to pass through.</p>
    </div>
    <div class="terminal"><div class="bar"><span class="dot"></span><span class="dot amber"></span><span class="dot green"></span></div><strong>$ the-witness start</strong><br><br>Dashboard → Endpoint Watchlist → Live Stream → Gemma Verdict → Prompt Repair → Audit Log</div>
  </div>
''', 'Overview')}

{page('2. The Problem', '''
  <p>AI tools are becoming agents. They write code, tutor students, summarize health information, support finance workflows, and automate work. But many tools trust the first model response immediately.</p>
  <div class="three-col">
    <div class="card"><h3>Incorrect code</h3><p>A weak answer can create broken scripts, unsafe shell commands, or insecure defaults.</p></div>
    <div class="card"><h3>Bad tutoring</h3><p>Students may see plausible but wrong explanations before anyone reviews them.</p></div>
    <div class="card"><h3>Overconfident advice</h3><p>Health, finance, and legal topics require caution, uncertainty, and human review.</p></div>
    <div class="card"><h3>Hallucinated facts</h3><p>Agents may invent APIs, citations, or operational details.</p></div>
    <div class="card"><h3>Untraceable decisions</h3><p>Without logs, teams cannot reconstruct what was approved, rejected, or retried.</p></div>
    <div class="card"><h3>Hidden risk</h3><p>The failure happens between the model and the user. The Witness puts a gate there.</p></div>
  </div>
  <div class="callout warning"><strong>Risk statement:</strong> The point is not to claim perfect safety. The point is to add a local, explainable verification gate before model output reaches real users.</div>
''', 'Why it matters')}

{page('3. The Solution', f'''
  <div class="two-col">
    <div>
      <p>The Witness sits between the AI app and the upstream model endpoint. The original application calls a localhost OpenAI-compatible proxy. The Witness forwards the request, captures the candidate response, and asks Gemma 4 for a structured verdict before releasing anything.</p>
      <ul>
        <li>watches the endpoint</li><li>captures request/response pairs</li><li>sends candidate responses to Gemma 4</li><li>approves, rejects, repairs, retries, or pauses for human review</li><li>writes an audit trail</li>
      </ul>
    </div>
    <div class="diagram"><div class="flow"><div class="node">AI App / Agent</div><div class="arrow">↓</div><div class="node">The Witness Local Proxy</div><div class="arrow">↓</div><div class="node">Upstream AI Endpoint</div><div class="arrow">↓</div><div class="node">Candidate Response</div><div class="arrow">↓</div><div class="node">Gemma 4 Judge</div><div class="arrow">↓</div><div class="node">Return / Repair + Retry / Human Review</div><div class="arrow">↓</div><div class="node">Audit Log</div></div></div>
  </div>
''', 'Architecture')}

{page('4. Core Workflow', f'''
  <div class="loop no-break"><div class="decision">{badge('DISAPPROVED','red')}<br><br>Block candidate response<br>Generate prompt repair<br>Retry upstream endpoint</div><div style="text-align:center;font-size:32px;color:#2fffd0">⇄</div><div class="decision">{badge('APPROVED','green')}<br><br>Return final response<br>Mark request approved<br>Save audit log</div></div>
  <ol>
    <li>User adds endpoint.</li><li>The Witness creates a local proxy URL.</li><li>AI app sends request to local proxy.</li><li>The Witness forwards request to upstream endpoint.</li><li>Upstream returns candidate response.</li><li>Gemma 4 judges response.</li><li>If approved, response is returned.</li><li>If disapproved, response is blocked.</li><li>Prompt repair is generated.</li><li>Request is retried.</li><li>If risky, human review queue is used.</li><li>Logs are saved.</li>
  </ol>
''', 'Usage flow')}

<section class="page"><div class="content"><div class="section-kicker">Capabilities</div><h2>5. Features</h2><div class="feature-grid">{feature_cards}</div></div></section>

{page('6. TUI Guide', f'''
  <p>The Witness is intentionally a modern TUI, not a web GUI. It is designed for developers, local-first users, and hackathon judges who want to see actual endpoint traffic and verification decisions.</p>
  <div class="two-col">{img('dashboard','Dashboard: backend/model/fallback status, watched endpoints, request statistics, and system health.')}{img('models','Model Manager: Ollama, Hugging Face fine-tuned model, llama.cpp, LiteRT, and manual endpoints.')}</div>
  <div class="two-col">{img('endpoints','Endpoint Watchlist: local proxy URL, upstream URL, profile, strictness, retry limit, and auth redaction.')}{img('stream','Live Request Stream: received, forwarded, judging, disapproved, retrying, and approved statuses.')}</div>
''', 'Screens')}

{page('7. TUI Deep Dive', f'''
  <div class="two-col">{img('verdict','Gemma Verdict and Prompt Repair panel: rejection reason, suggested fix, repaired prompt, and retry status.')}{img('logs','Logs / Audit screen: full event timeline, rejected/approved events, retry chains, and export path.')}</div>
  <div class="callout example"><strong>Setup Wizard:</strong> choose backend, choose model, test judge, test proxy, and pass the readiness checklist before the dashboard opens.</div>
  <div class="callout"><strong>Human Review Queue:</strong> approve, reject, edit, regenerate, export a report, mark unsafe, or add a note when the judge is uncertain or the domain is high-risk.</div>
''', 'Screens continued')}

{page('8. Installation', f'''
  <h3>Quick install</h3>{quick_install_html}
  <h3>Safer install</h3>{safer_install_html}
  <h3>Manual install</h3>{manual_install_html}
  <div class="callout"><strong>Requirements:</strong> Rust, Cargo, optional Ollama, optional Gemma models, optional Blackbox API key, optional Hugging Face model. Doctor warnings are normal when optional integrations are not configured.</div>
''', 'Getting started')}

{page('9. Model Setup', f'''
  <div class="two-col"><div>
  <h3>Default local judge</h3>{e2b_pull_html}
  <p><strong>E2B</strong> is the fast/default judge for quick approval classification.</p>
  <h3>Strong/high-risk judge</h3>{e4b_pull_html}
  <p><strong>E4B</strong> is stronger for coding and high-risk profiles when hardware allows.</p>
  </div><div>
  <h3>Fine-tuned Witness judge</h3><p>{HF_URL}</p>{hf_download_html}
  <h3>Fine-tuning notebook</h3><p>{COLAB_URL}</p>
  <p>The notebook fine-tunes a Witness Gemma 4 E2B Judge using Unsloth to output structured JSON verdicts.</p>
  </div></div>
  {img('hf','Hugging Face reference for the fine-tuned Witness Gemma 4 E2B judge adapter.')}
  {img('colab','Colab notebook reference for the Unsloth fine-tuning workflow.')}
''', 'Gemma and backends')}

{page('10. Adding an Endpoint', f'''
  <p>The Blackbox endpoint example demonstrates how a real upstream provider can be watched without storing secrets in the repository.</p>
  <h3>Set key safely</h3>{key_html}
  <h3>Add endpoint</h3>{add_blackbox_html}
  <table><tr><th>Field</th><th>Value</th></tr><tr><td>upstream</td><td>https://api.blackbox.ai/v1</td></tr><tr><td>local proxy</td><td>http://localhost:8787/v1</td></tr><tr><td>model</td><td>blackboxai/x-ai/grok-code-fast-1:free</td></tr><tr><td>profile</td><td>coding</td></tr><tr><td>strictness</td><td>high</td></tr><tr><td>retry limit</td><td>4</td></tr><tr><td>auth</td><td>bearer_env BLACKBOX_API_KEY</td></tr></table>
  <h3>Test through The Witness</h3>{curl_html}
  {img('curl','Curl test: applications call localhost while The Witness forwards, judges, repairs, and returns approved responses.')}
''', 'Endpoint watching')}

{page('11. Gemma 4 Judge Schema', f'''
  {schema_html}
  <table><tr><th>Field</th><th>Purpose</th></tr><tr><td>verdict</td><td>APPROVED, DISAPPROVED, or NEEDS_HUMAN_REVIEW.</td></tr><tr><td>confidence</td><td>How confident the judge is in the decision.</td></tr><tr><td>safety_score</td><td>Safety assessment for the candidate response.</td></tr><tr><td>usefulness_score</td><td>Whether the answer is practically helpful.</td></tr><tr><td>prompt_alignment_score</td><td>How well the response follows the original user request.</td></tr><tr><td>correctness_risk</td><td>Low, medium, or high risk of factual or operational incorrectness.</td></tr><tr><td>rejection_reason</td><td>Human-readable reason for blocking the answer.</td></tr><tr><td>suggested_fix</td><td>What must change before retrying.</td></tr><tr><td>improved_prompt_instruction</td><td>Hidden corrective instruction used for prompt repair.</td></tr><tr><td>requires_human_review</td><td>Whether The Witness should pause for a person.</td></tr></table>
''', 'Structured verdicts')}

{page('12. Prompt Repair', '''
  <p>When Gemma rejects a response, The Witness creates a repaired request that preserves original user intent while adding hidden corrective instructions. It includes the rejection reason, the required fix, and stricter guidance after repeated failures.</p>
  <div class="two-col"><div class="card"><h3>Rejected attempt</h3><p><strong>Original request:</strong> Write a Python script that prints Hello World.</p><pre><code>print(Hello World)</code></pre><p><span class="badge red">DISAPPROVED</span> Python string is not quoted.</p></div><div class="card"><h3>Repaired attempt</h3><p>The previous answer was rejected because the Python string was not quoted. Generate a corrected answer using valid Python syntax.</p><pre><code>print("Hello World")</code></pre><p><span class="badge green">APPROVED</span> Returned to the app.</p></div></div>
  <div class="callout warning"><strong>Internal details stay internal:</strong> prompt repair should not leak secrets or unnecessary judge internals to the final user.</div>
''', 'Repair loop')}

{page('13. Technology Tracks', f'''
  <div class="track-grid"><div class="track-card"><h3>Ollama</h3><p>The default local judge backend runs Gemma 4 through Ollama. The TUI supports gemma4:e2b, gemma4:e4b, and custom Ollama models.</p></div><div class="track-card"><h3>llama.cpp</h3><p>The project includes a llama.cpp backend for local/resource-constrained inference using Gemma models.</p></div><div class="track-card"><h3>LiteRT</h3><p>The project includes a LiteRT edge prefilter path for lightweight classification before escalating to a full judge. This path should be treated as experimental unless fully configured.</p></div><div class="track-card"><h3>Unsloth</h3><p>The public Colab notebook uses Unsloth and the fine-tuned Witness Gemma 4 E2B judge is published as a Hugging Face adapter.</p></div><div class="track-card"><h3>Cactus-ready</h3><p>The architecture is Cactus-ready for a future mobile companion that routes lightweight checks on-device and heavier checks to the desktop Witness instance. Completed mobile support is not claimed.</p></div></div>
  {img('tracks','Track badge wall: The Witness spans Ollama, llama.cpp, LiteRT, Unsloth, Hugging Face, and safety/trust positioning.')}
''', 'Hackathon fit')}

{page('14. Impact Tracks', f'''
  <div class="callout track"><strong>Primary impact track: Safety & Trust.</strong> The Witness gives explainable verification, structured verdicts, rejection reasons, prompt repair, human review, and audit logs.</div>
  <div class="three-col"><div class="card"><h3>Digital Equity & Inclusivity</h3><p>Local-first, low-cost, terminal-native, privacy-friendly, low-bandwidth, and multilingual-ready.</p></div><div class="card"><h3>Future of Education</h3><p>AI tutors can be verified before students see answers.</p></div><div class="card"><h3>Health & Sciences</h3><p>Risky medical/scientific responses can be paused, uncertainty required, and human review triggered.</p></div><div class="card"><h3>Global Resilience</h3><p>Local verification can help field teams and low-connectivity environments where cloud safety tools are unavailable.</p></div></div>
  {img('impact','Impact map: Safety & Trust is the primary fit, with education, health/sciences, digital equity, and resilience as secondary tracks.')}
''', 'Impact')}

{page('15. Demo Walkthrough', '''
  <ol><li>Install The Witness.</li><li>Pull gemma4:e2b.</li><li>Run setup.</li><li>Start TUI.</li><li>Add endpoint.</li><li>Send curl request.</li><li>The Witness captures the response.</li><li>Gemma rejects a bad response.</li><li>Prompt repair happens.</li><li>Retry succeeds.</li><li>Audit report is saved.</li></ol>
  <div class="callout example"><strong>Demo assets:</strong> the repository includes <code>demo_usage/</code> with screenshots, a usage video, and a usage section. The <code>gallery/</code> folder contains polished images for Kaggle and GitHub.</div>
  <table><tr><th>Asset</th><th>Purpose</th></tr><tr><td>demo_usage/full_usage_demo.mp4</td><td>Full usage walkthrough video.</td></tr><tr><td>demo_usage/screenshots/</td><td>Usage screenshots for the writeup/gallery.</td></tr><tr><td>gallery/</td><td>Polished submission visuals and diagrams.</td></tr></table>
''', 'Demo')}

{page('16. Security and Privacy', '''
  <ul><li>API keys are never stored in docs.</li><li>Examples use environment variables.</li><li>Authorization headers are redacted.</li><li>Privacy mode can avoid storing full prompts/responses.</li><li>Logs can store metadata only.</li><li>The project avoids committing tokens or model weights.</li></ul>
  <div class="callout warning"><strong>Never paste real API keys into GitHub, screenshots, or videos.</strong> Use environment variables such as <code>$BLACKBOX_API_KEY</code> and keep local shell history clean when possible.</div>
  <div class="card"><h3>Secret redaction example</h3><pre><code>Authorization: Bearer ********
auth: bearer_env BLACKBOX_API_KEY</code></pre></div>
''', 'Security')}

{page('17. Limitations', '''
  <ul><li>The Witness reduces risk but does not guarantee correctness.</li><li>Gemma judge decisions can be wrong.</li><li>High-risk medical/legal/financial outputs still need professionals.</li><li>Local performance depends on model and hardware.</li><li>LiteRT may be experimental if not fully tested.</li><li>Cactus/mobile companion is future work unless implemented.</li><li>Streaming support may be limited.</li><li>Fine-tuned model quality depends on training data.</li></ul>
  <div class="callout"><strong>Honest claim:</strong> The value of The Witness is a practical reliability layer, not a promise of perfect safety.</div>
''', 'Known boundaries')}

{page('18. Reproducibility', f'''
  {repro_html}
  <p>Expected result: build/test should pass. Doctor may warn if Ollama models, Hugging Face tools, external API keys, or optional integrations are missing on the machine.</p>
  <div class="callout example"><strong>Before a live demo:</strong> run <code>the-witness doctor</code>, make sure the selected backend is reachable, confirm the local proxy port is free, and use demo mode if no external key is configured.</div>
''', 'Verification')}

{page('19. Links', f'''
  <table><tr><th>Resource</th><th>Link</th></tr><tr><td>GitHub repository</td><td>{REPO_URL}</td></tr><tr><td>Fine-tuned model</td><td>{HF_URL}</td></tr><tr><td>Fine-tuning notebook</td><td>{COLAB_URL}</td></tr><tr><td>Default Ollama tag</td><td>gemma4:e2b</td></tr><tr><td>Strong Ollama tag</td><td>gemma4:e4b</td></tr></table>
  <div class="callout"><strong>Model names are configurable.</strong> The Witness provides practical defaults but lets users enter any custom Ollama model, path, or local OpenAI-compatible judge endpoint.</div>
''', 'References')}

<section class="page final">
  <h2>20. Final Pitch</h2>
  <p class="big-quote" style="color:#f5f7fa">The Witness is a local-first reliability firewall for the age of AI agents.</p>
  <p>As AI systems become more powerful, the question is no longer only:</p>
  <div class="question">“Can the model answer?”</div>
  <p>The question is:</p>
  <div class="question">“Should this answer be allowed to act?”</div>
  <p>The Witness turns Gemma 4 into the local trust layer that answers that question.</p>
  <h2 style="margin-top:24px">Do not just trust AI.<br>Let The Witness see it first.</h2>
</section>
</body></html>
"""

readme = f'''# The Witness PDF documentation

This directory contains the generated PDF documentation booklet for The Witness.

## Final PDF

`docs/pdf/The_Witness_Documentation.pdf`

## Source files

- `The_Witness_Documentation.md` — portable Markdown source text.
- `source.html` — print-ready HTML layout used for the final PDF.
- `style.css` — custom visual design and print CSS.
- `generate_documentation.py` — helper script that regenerates the Markdown, HTML, CSS, and README source files.

## Generation tool

The PDF was generated with headless Chromium using CSS print layout:

```bash
chromium --headless --no-sandbox --disable-gpu \\
  --print-to-pdf=docs/pdf/The_Witness_Documentation.pdf \\
  file:///home/admin/Gemma/witness/docs/pdf/source.html
```

Chromium was used because it supports modern HTML/CSS layouts, local images, page breaks, and high-quality PDF printing.

## Regenerate

From the project root:

```bash
cd /home/admin/Gemma/witness
python3 docs/pdf/generate_documentation.py
chromium --headless --no-sandbox --disable-gpu \\
  --print-to-pdf=docs/pdf/The_Witness_Documentation.pdf \\
  file:///home/admin/Gemma/witness/docs/pdf/source.html
```

## Verify

```bash
ls -lh docs/pdf/The_Witness_Documentation.pdf
pdfinfo docs/pdf/The_Witness_Documentation.pdf | grep Pages || true
python3 - <<'PY'
from pathlib import Path
for p in Path('docs/pdf').rglob('*'):
    if p.is_file() and p.suffix not in {'.png', '.jpg', '.mp4'}:
        text = p.read_text(errors='ignore')
        assert 'REAL_SECRET_PLACEHOLDER_DO_NOT_USE' not in text
print('docs/pdf secret placeholder scan ok')
PY
cargo fmt
cargo test
cargo build
```

The documentation intentionally uses environment variables such as `$BLACKBOX_API_KEY` and never includes real credentials.
'''

(OUT / 'The_Witness_Documentation.md').write_text(md)
(OUT / 'source.html').write_text(html_doc)
(OUT / 'style.css').write_text(css)
(OUT / 'README.md').write_text(readme)
print('wrote docs/pdf source files')
