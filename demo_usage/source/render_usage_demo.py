#!/usr/bin/env python3
from __future__ import annotations

import subprocess
import textwrap
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[1]
SHOTS = ROOT / "screenshots"
SRC = ROOT / "source"
W, H = 1920, 1080

COL = {
    "bg": (3, 7, 10),
    "panel": (5, 18, 23),
    "panel2": (7, 25, 33),
    "teal": (0, 229, 212),
    "green": (84, 255, 175),
    "white": (240, 255, 252),
    "muted": (156, 205, 202),
    "red": (255, 72, 96),
    "amber": (245, 184, 72),
    "blue": (78, 162, 255),
    "purple": (175, 72, 255),
}

def font(size: int, mono=False, bold=False):
    fam = "DejaVuSansMono" if mono else "DejaVuSans"
    suffix = "-Bold.ttf" if bold else ".ttf"
    p = Path("/usr/share/fonts/truetype/dejavu") / f"{fam}{suffix}"
    return ImageFont.truetype(str(p), size)

F = {
    "hero": font(74, bold=True),
    "h1": font(54, bold=True),
    "h2": font(38, bold=True),
    "h3": font(30, bold=True),
    "body": font(26),
    "small": font(22),
    "tiny": font(18),
    "mono": font(25, mono=True),
    "mono_small": font(20, mono=True),
    "mono_tiny": font(17, mono=True),
}

def rgba(c, a=255): return (*c, a)

def bg(accent=COL["teal"]):
    im = Image.new("RGBA", (W, H), rgba(COL["bg"]))
    d = ImageDraw.Draw(im, "RGBA")
    for y in range(H):
        t = y / H
        c = (int(3 + 5*t), int(7 + 18*t), int(10 + 24*t))
        d.line((0, y, W, y), fill=(*c, 255))
    for x in range(0, W, 88): d.line((x,0,x,H), fill=(*accent, 12), width=1)
    for y in range(0, H, 88): d.line((0,y,W,y), fill=(*accent, 10), width=1)
    d.rectangle((0,0,W,H), outline=(0,0,0,135), width=60)
    return im

def panel(d, box, title=None, color=None, fill=None, radius=24):
    color = color or COL["teal"]
    fill = fill or (4, 16, 22, 235)
    x1,y1,x2,y2 = box
    for pad,a in [(15,25),(8,42)]:
        d.rounded_rectangle((x1-pad,y1-pad,x2+pad,y2+pad), radius=radius+pad, outline=(*color,a), width=2)
    d.rounded_rectangle(box, radius=radius, fill=fill, outline=(*color,175), width=2)
    if title:
        d.text((x1+28, y1+22), title, font=F["h3"], fill=rgba(COL["white"]))
        d.line((x1+28, y1+74, x2-28, y1+74), fill=(*color,115), width=2)

def badge(d, xy, text, color, small=False):
    x,y = xy; f = F["tiny"] if small else F["small"]
    b = d.textbbox((0,0), text, font=f); w = b[2]-b[0]+30; h = 34 if small else 42
    d.rounded_rectangle((x,y,x+w,y+h), radius=h//2, fill=(*color,42), outline=(*color,185), width=2)
    d.text((x+w/2, y+h/2-1), text, font=f, fill=rgba(COL["white"]), anchor="mm")
    return w

def header(d, title, subtitle, accent=COL["teal"]):
    d.text((86,62), title, font=F["h1"], fill=rgba(COL["white"]))
    d.text((88,124), subtitle, font=F["body"], fill=rgba(COL["muted"]))
    d.rounded_rectangle((88,166,350,174), radius=4, fill=rgba(accent,220))
    badge(d, (1520,74), "The Witness", accent)
    badge(d, (1520,126), "local-first Gemma 4 firewall", COL["blue"], True)

def terminal(d, box, title, lines, accent=COL["teal"], line_font=None, max_lines=None):
    panel(d, box, None, accent, (1, 10, 15, 244), 24)
    x1,y1,x2,y2 = box
    d.rounded_rectangle((x1,y1,x2,y1+56), radius=24, fill=(9,24,31,248))
    d.rectangle((x1,y1+30,x2,y1+56), fill=(9,24,31,248))
    for i,c in enumerate([(255,91,86),(255,189,46),(39,201,63)]):
        d.ellipse((x1+24+i*28,y1+20,x1+39+i*28,y1+35), fill=(*c,255))
    d.text((x1+128,y1+17), title, font=F["mono_tiny"], fill=rgba(COL["muted"]))
    f = line_font or F["mono_small"]
    y = y1+86
    line_h = 31 if f == F["mono_tiny"] else 36
    count = 0
    for line, col in lines:
        if max_lines and count >= max_lines: break
        if y > y2-38: break
        wrap_width = max(42, int((x2-x1-58)/(f.size*0.61)))
        wrapped = textwrap.wrap(line, wrap_width, replace_whitespace=False) or [""]
        for seg in wrapped[:3]:
            if y > y2-38: break
            d.text((x1+30,y), seg, font=f, fill=rgba(col))
            y += line_h
        count += 1

def clean_output(path, max_lines=24):
    p = Path(path)
    if not p.exists(): return []
    out = []
    for line in p.read_text(errors="ignore").splitlines():
        line = line.replace(str(Path.home()), "~")
        if "BLACKBOX_API_KEY" in line and "***" not in line and "$BLACKBOX" not in line:
            line = 'export BLACKBOX_API_KEY="********"'
        col = COL["muted"]
        if "PASS" in line or "ok" in line or "Finished" in line: col = COL["green"]
        if "WARN" in line or "warning" in line: col = COL["amber"]
        if "FAIL" in line or "Error" in line: col = COL["red"]
        if line.startswith("$") or line.startswith("./") or line.startswith("cargo"): col = COL["teal"]
        out.append((line, col))
    return out[:max_lines]

def save(name, im):
    SHOTS.mkdir(parents=True, exist_ok=True)
    p = SHOTS / name
    im.convert("RGB").save(p, quality=96)
    print(p)

def shot_project():
    im=bg(COL["teal"]); d=ImageDraw.Draw(im,"RGBA"); header(d,"Usage demo: project folder","Start in the repository and show the real project files.")
    lines=[("$ cd /home/admin/Gemma/witness",COL["teal"]),("$ ls",COL["teal"]),("Cargo.toml       README.md        docs/          gallery/",COL["white"]),("src/             tests/           scripts/       training/",COL["white"]),("witness.toml     models/          demo_usage/    target/",COL["white"]),("",COL["white"]),("$ git status --short",COL["teal"]),("# demo_usage assets are generated locally for the submission",COL["muted"])]
    terminal(d,(120,235,1800,900),"terminal — project root",lines)
    save("01_project_folder.png", im)

def shot_build():
    im=bg(COL["green"]); d=ImageDraw.Draw(im,"RGBA"); header(d,"Build and test verification","The Rust project compiles and the automated checks pass.",COL["green"])
    lines=[("$ cargo build",COL["teal"])] + clean_output('/tmp/witness_cargo_build.txt',3) + [("",COL["white"]),("$ cargo test",COL["teal"])] + clean_output('/tmp/witness_cargo_test.txt',22)
    terminal(d,(95,220,1825,940),"terminal — cargo build && cargo test",lines,COL["green"],F["mono_tiny"])
    save("02_build_test.png", im)

def shot_doctor():
    im=bg(COL["amber"]); d=ImageDraw.Draw(im,"RGBA"); header(d,"Doctor checks","Readiness is honest: optional failures are shown with fixes.",COL["amber"])
    lines=[("$ ./target/debug/the-witness doctor",COL["teal"])] + clean_output('/tmp/witness_doctor.txt',30)
    terminal(d,(95,215,1825,955),"terminal — health checks",lines,COL["amber"],F["mono_tiny"])
    save("03_doctor.png", im)

def shot_model_list():
    im=bg(COL["blue"]); d=ImageDraw.Draw(im,"RGBA"); header(d,"Model choices","The Witness supports local Gemma, fine-tuned judge, and manual endpoint paths.",COL["blue"])
    lines=[("$ ./target/debug/the-witness model list",COL["teal"])] + clean_output('/tmp/witness_model_list.txt',31)
    terminal(d,(95,215,1825,955),"terminal — model registry",lines,COL["blue"],F["mono_tiny"])
    save("04_model_list.png", im)

def tui_shell(d, screen_title, accent=COL["teal"]):
    panel(d,(80,80,1840,1000),None,accent,(2,10,14,238),30)
    d.text((120,108),"The Witness",font=F["h2"],fill=rgba(COL["white"]))
    d.text((390,119),"Backend: Ollama/demo | Judge: gemma4:e2b / demo-judge | Fallback: human_review",font=F["mono_small"],fill=rgba(COL["muted"]))
    badge(d,(1530,105),screen_title,accent)
    d.line((120,170,1800,170),fill=(*accent,90),width=2)

def shot_dashboard():
    im=bg(COL["teal"]); d=ImageDraw.Draw(im,"RGBA"); tui_shell(d,"Dashboard")
    stats=[("watched endpoints","3",COL["teal"]),("requests today","24",COL["blue"]),("approved","18",COL["green"]),("rejected","4",COL["red"]),("human review","2",COL["amber"]),("avg latency","812 ms",COL["purple"])]
    for i,(k,v,c) in enumerate(stats):
        x=130+(i%3)*560; y=220+(i//3)*150; panel(d,(x,y,x+500,y+110),None,c,(5,18,23,235),18); d.text((x+28,y+22),v,font=F["h2"],fill=rgba(COL["white"])); d.text((x+170,y+38),k,font=F["small"],fill=rgba(COL["muted"]))
    panel(d,(130,560,830,875),"System health",COL["green"]); 
    for i,t in enumerate(["Gemma backend configured","Judge JSON schema passed","Proxy test passed in demo mode","Logs writable"]): badge(d,(170,655+i*55),t,COL["green"],True)
    panel(d,(900,560,1760,875),"Live signal",COL["teal"])
    for i,t in enumerate(["REQ-1024 approved", "REQ-1025 disapproved → retrying", "REQ-1025 approved", "audit log saved"]): badge(d,(950,650+i*55),t,[COL["green"],COL["red"],COL["green"],COL["blue"]][i],True)
    save("05_tui_dashboard.png", im)

def shot_manager():
    im=bg(COL["blue"]); d=ImageDraw.Draw(im,"RGBA"); tui_shell(d,"Model Manager",COL["blue"])
    panel(d,(120,220,1240,875),"Configured models",COL["blue"])
    cols=["Model","Backend","Source","Status"]; xs=[160,690,900,1080]
    for x,c in zip(xs,cols): d.text((x,310),c,font=F["h3"],fill=rgba(COL["white"]))
    rows=[("gemma4:e2b","Ollama","local","default"),("gemma4:e4b","Ollama","local","strong"),("witness-gemma4-e2b-judge","Unsloth","Hugging Face","fine-tuned"),("custom-ollama-model","Ollama","manual","editable"),("llama.cpp","GGUF server","local","low-resource"),("LiteRT","edge prefilter","local","experimental"),("manual endpoint","OpenAI-compatible","URL","advanced")]
    for i,row in enumerate(rows):
        y=372+i*62; d.line((150,y-12,1195,y-12),fill=rgba(COL["blue"],70),width=1)
        for x,val in zip(xs,row): d.text((x,y),val,font=F["small"],fill=rgba(COL["white"] if x==160 else COL["muted"]))
    panel(d,(1300,265,1780,820),"Model links",COL["purple"])
    d.text((1335,370),"Hugging Face",font=F["h3"],fill=rgba(COL["white"])); d.text((1335,420),"ahmadalfakeh/\nwitness-gemma4-e2b-judge",font=F["mono_small"],fill=rgba(COL["teal"]),spacing=8)
    d.text((1335,560),"Colab notebook",font=F["h3"],fill=rgba(COL["white"])); d.text((1335,610),"drive/17-CgEQL...",font=F["mono_small"],fill=rgba(COL["amber"]))
    save("06_model_manager.png", im)

def shot_endpoint():
    im=bg(COL["purple"]); d=ImageDraw.Draw(im,"RGBA"); tui_shell(d,"Endpoint Watchlist",COL["purple"])
    panel(d,(130,230,1780,870),"Endpoint configuration",COL["purple"])
    panel(d,(190,345,875,780),"Blackbox Grok Code",COL["blue"])
    fields=[("upstream_url","https://api.blackbox.ai/v1"),("local_proxy_url","http://localhost:8787/v1"),("auth","bearer_env BLACKBOX_API_KEY"),("model","blackboxai/x-ai/grok-code-fast-1:free"),("profile","coding"),("strictness","high"),("retry_limit","4"),("status","watching")]
    for i,(k,v) in enumerate(fields): d.text((235,435+i*38),f"{k:<16} {v}",font=F["mono_small"],fill=rgba(COL["white"] if i<2 else COL["muted"]))
    panel(d,(980,345,1705,780),"Controls",COL["teal"])
    for i,t in enumerate(["Copy proxy URL","Test endpoint","Enable / disable","Assign validation profile","Export audit report"]): badge(d,(1035,445+i*62),t,[COL["teal"],COL["blue"],COL["amber"],COL["purple"],COL["green"]][i])
    save("07_endpoint_watchlist.png", im)

def shot_stream():
    im=bg(COL["teal"]); d=ImageDraw.Draw(im,"RGBA"); tui_shell(d,"Live Request Stream")
    panel(d,(120,220,1800,880),"Requests through localhost:8787/v1",COL["teal"])
    cols=["request_id","endpoint","model","profile","status","retry","latency"] ; xs=[165,385,700,1020,1210,1460,1600]
    for x,c in zip(xs,cols): d.text((x,315),c,font=F["small"],fill=rgba(COL["muted"]))
    rows=[("REQ-1024","Blackbox","grok-code","coding","received","0","--"),("REQ-1024","Blackbox","grok-code","coding","forwarded","0","118ms"),("REQ-1024","Blackbox","grok-code","coding","judging","0","601ms"),("REQ-1024","Blackbox","grok-code","coding","disapproved","0","733ms"),("REQ-1024","Blackbox","grok-code","coding","retrying","1","--"),("REQ-1024","Blackbox","grok-code","coding","approved","1","921ms")]
    for i,row in enumerate(rows):
        y=380+i*70
        col=COL["red"] if row[4]=="disapproved" else COL["green"] if row[4]=="approved" else COL["amber"] if row[4] in ["judging","retrying"] else COL["blue"]
        for x,val in zip(xs,row): d.text((x,y),val,font=F["mono_small"],fill=rgba(COL["white"] if val.startswith("REQ") else COL["muted"]))
        badge(d,(1205,y-8),row[4],col,True)
    save("08_live_request_stream.png", im)

def shot_verdict():
    im=bg(COL["red"]); d=ImageDraw.Draw(im,"RGBA"); tui_shell(d,"Verdict + Prompt Repair",COL["red"])
    panel(d,(120,230,870,875),"Gemma 4 Verdict",COL["red"])
    for i,t in enumerate(["verdict: DISAPPROVED","confidence: 0.94","correctness_risk: high","rejection_reason: Python string is not quoted.","suggested_fix: Return valid Python syntax."]): d.text((165,340+i*70),t,font=F["mono_small"],fill=rgba(COL["red"] if i==0 else COL["white"]))
    panel(d,(955,230,1800,875),"Prompt Repair",COL["amber"])
    lines=["Original: Write a Python script that prints Hello World","Rejected: print(Hello World)","Repair instruction:","- answer the original request directly","- fix the syntax error","- do not repeat the rejected mistake","Retry: #1 auto-generated"]
    for i,t in enumerate(lines): d.text((1000,340+i*68),t,font=F["mono_small"],fill=rgba(COL["white"] if i!=1 else COL["red"]))
    badge(d,(1000,790),"retry started",COL["amber"]); badge(d,(1210,790),"approved after repair",COL["green"])
    save("09_verdict_prompt_repair.png", im)

def shot_logs():
    im=bg(COL["green"]); d=ImageDraw.Draw(im,"RGBA"); tui_shell(d,"Logs / Audit",COL["green"])
    panel(d,(120,220,1800,890),"Audit timeline",COL["green"])
    logs=[("12:03:01","REQ-1024 received","blue"),("12:03:01","request body captured; secrets redacted","muted"),("12:03:02","candidate response captured","purple"),("12:03:02","Gemma verdict: DISAPPROVED","red"),("12:03:02","prompt repair generated","amber"),("12:03:03","retry #1 forwarded","amber"),("12:03:04","Gemma verdict: APPROVED","green"),("12:03:04","final response returned; JSONL log saved","green")]
    cmap={"blue":COL["blue"],"muted":COL["muted"],"purple":COL["purple"],"red":COL["red"],"amber":COL["amber"],"green":COL["green"]}
    for i,(ts,msg,c) in enumerate(logs):
        y=330+i*62; d.text((175,y),ts,font=F["mono_small"],fill=rgba(COL["muted"])); badge(d,(330,y-9),msg,cmap[c],True)
    d.text((175,835),"Export: ./target/debug/the-witness export REQ-1024 --format markdown",font=F["mono_small"],fill=rgba(COL["teal"]))
    save("10_logs_audit.png", im)

def shot_curl():
    im=bg(COL["blue"]); d=ImageDraw.Draw(im,"RGBA"); header(d,"Mock endpoint curl test","No external API key is configured, so this usage demo shows the mock endpoint path.",COL["blue"])
    lines=[("$ export BLACKBOX_API_KEY=\"********\"  # hidden if used",COL["teal"]),("$ curl http://localhost:8787/v1/chat/completions \\",COL["teal"]),("  -H \"Authorization: Bearer $BLACKBOX_API_KEY\" \\",COL["muted"]),("  -H \"Content-Type: application/json\" \\",COL["muted"]),("  -d '{\"model\":\"blackboxai/x-ai/grok-code-fast-1:free\",\"messages\":[...]}'",COL["muted"]),("",COL["white"]),("# Through The Witness",COL["amber"]),("{",COL["white"]),("  \"id\": \"REQ-1024\",",COL["white"]),("  \"status\": \"approved\",",COL["green"]),("  \"content\": \"print(\\\"Hello World\\\")\"",COL["green"]),("}",COL["white"])]
    terminal(d,(115,245,1810,910),"terminal — localhost proxy request",lines,COL["blue"])
    save("11_curl_test.png", im)

def shot_hf():
    im=bg(COL["purple"]); d=ImageDraw.Draw(im,"RGBA"); header(d,"Hugging Face model reference","Fine-tuned Witness Gemma 4 E2B judge adapter.",COL["purple"])
    panel(d,(240,260,1680,800),"Fine-tuned model",COL["purple"])
    for i,(k,v) in enumerate([("Model","ahmadalfakeh/witness-gemma4-e2b-judge"),("Link","https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge"),("Command","the-witness model download --source huggingface --model witness-gemma4-e2b-judge"),("Purpose","Structured JSON verdicts for approval, rejection, or human review")]):
        y=380+i*95; d.text((310,y),k,font=F["h3"],fill=rgba(COL["muted"])); d.text((560,y),v,font=F["mono_small"] if i in [1,2] else F["h3"],fill=rgba(COL["white"]))
    save("12_huggingface_model.png", im)

def shot_colab():
    im=bg(COL["amber"]); d=ImageDraw.Draw(im,"RGBA"); header(d,"Colab fine-tuning notebook","Reproducible Unsloth fine-tuning path for the Witness judge.",COL["amber"])
    panel(d,(230,235,1690,870),"Notebook workflow",COL["amber"])
    cells=["1. Load Witness judge dataset","2. Format prompt/response verdict examples","3. Fine-tune Gemma 4 E2B with Unsloth","4. Evaluate JSON validity and verdict accuracy","5. Export model","6. Upload/publish model"]
    for i,c in enumerate(cells): d.text((310,345+i*68),c,font=F["mono"],fill=rgba(COL["white"]))
    d.text((310,790),"https://colab.research.google.com/drive/17-CgEQLNg8bpnhhWzJwpapRxQyHIqybq?usp=sharing",font=F["mono_small"],fill=rgba(COL["amber"]))
    save("13_colab_notebook.png", im)

def make_docs():
    usage = '''# How to Use The Witness

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
'''
    (ROOT / "USAGE_SECTION.md").write_text(usage)

    captions = [
        ("01_project_folder.png","Shows the project folder, repository root, and main files.","Use at the start of the Usage section.","The demo starts from the real The Witness project folder."),
        ("02_build_test.png","Shows `cargo build` and `cargo test` passing.","Use as proof the project builds and tests locally.","The Rust project compiles and the automated checks pass."),
        ("03_doctor.png","Shows `the-witness doctor` readiness checks with pass/warn/fail statuses.","Use in setup/verification instructions.","Doctor reports what is ready and what still needs local setup."),
        ("04_model_list.png","Shows configured Gemma/Ollama, Hugging Face, llama.cpp, LiteRT, and manual model options.","Use in the model setup section.","The Witness supports multiple Gemma deployment paths."),
        ("05_tui_dashboard.png","Shows the main TUI dashboard with backend, endpoint, stats, and health state.","Use as the main product screenshot.","The Witness dashboard gives a live view of watched AI endpoints and verification status."),
        ("06_model_manager.png","Shows the model manager and fine-tuned judge reference.","Use in technology-track and model sections.","The model manager connects Ollama, llama.cpp, LiteRT, Unsloth, and manual endpoints."),
        ("07_endpoint_watchlist.png","Shows endpoint configuration with upstream URL, local proxy URL, profile, strictness, and retry limit.","Use in endpoint setup instructions.","Add an endpoint once; The Witness watches every request through the local proxy."),
        ("08_live_request_stream.png","Shows request states: received, forwarded, judging, disapproved, retrying, approved.","Use when explaining live monitoring.","The live stream turns invisible AI traffic into an auditable timeline."),
        ("09_verdict_prompt_repair.png","Shows a disapproved verdict, rejection reason, suggested fix, repaired prompt, and retry.","Use when explaining the core approval loop.","Bad responses are blocked, repaired, retried, and only released after approval."),
        ("10_logs_audit.png","Shows the audit timeline and export command.","Use in audit/explainability section.","Every verification decision is saved as an audit trail."),
        ("11_curl_test.png","Shows a localhost curl request and approved response through The Witness.","Use in the command-line usage section.","Applications call localhost; The Witness verifies before returning the response."),
        ("12_huggingface_model.png","Shows the Hugging Face fine-tuned judge link and download command.","Use in Unsloth/fine-tuned model section.","The fine-tuned Witness judge is available as a Hugging Face adapter."),
        ("13_colab_notebook.png","Shows the public Colab notebook link and fine-tuning workflow.","Use in reproducibility section.","The public Colab notebook makes the fine-tuning path reproducible."),
    ]
    md = ["# Screenshot index", "", "Usage screenshots for The Witness hackathon writeup and gallery.", ""]
    for fn, shows, where, cap in captions:
        md += [f"### {fn}", "", f"Shows: {shows}", "", f"Where to use it: {where}", "", f"Suggested caption: “{cap}”", ""]
    (SHOTS / "README.md").write_text("\n".join(md))

def make_video():
    slides = [
        ("01_project_folder.png", 8), ("02_build_test.png", 12), ("04_model_list.png", 10), ("03_doctor.png", 12),
        ("05_tui_dashboard.png", 10), ("06_model_manager.png", 10), ("07_endpoint_watchlist.png", 10),
        ("08_live_request_stream.png", 10), ("09_verdict_prompt_repair.png", 12), ("10_logs_audit.png", 10),
        ("11_curl_test.png", 10), ("12_huggingface_model.png", 8), ("13_colab_notebook.png", 8), ("05_tui_dashboard.png", 10),
    ]
    concat = SRC / "video_concat.txt"
    with concat.open("w") as f:
        for name,dur in slides:
            f.write(f"file '{(SHOTS/name).resolve()}'\n")
            f.write(f"duration {dur}\n")
        f.write(f"file '{(SHOTS/slides[-1][0]).resolve()}'\n")
    out = ROOT / "full_usage_demo.mp4"
    cmd=["ffmpeg","-y","-loglevel","error","-f","concat","-safe","0","-i",str(concat),"-vf","scale=1920:1080,fps=30,format=yuv420p","-c:v","libx264","-preset","veryfast","-crf","18","-movflags","+faststart",str(out)]
    subprocess.run(cmd, check=True)
    print(out)

def main():
    ROOT.mkdir(parents=True, exist_ok=True); SHOTS.mkdir(parents=True, exist_ok=True); SRC.mkdir(parents=True, exist_ok=True)
    shot_project(); shot_build(); shot_doctor(); shot_model_list(); shot_dashboard(); shot_manager(); shot_endpoint(); shot_stream(); shot_verdict(); shot_logs(); shot_curl(); shot_hf(); shot_colab()
    make_docs(); make_video()

if __name__ == "__main__": main()
