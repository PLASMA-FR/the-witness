#!/usr/bin/env python3
from __future__ import annotations

import math
import os
import subprocess
from pathlib import Path
from typing import Callable, Iterable, Sequence

from PIL import Image, ImageDraw, ImageFilter, ImageFont

ROOT = Path(__file__).resolve().parent
W, H = 1920, 1080
FPS = 18

FONT_REG = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"
FONT_BOLD = "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf"
FONT_MONO = "/usr/share/fonts/truetype/noto/NotoMono-Regular.ttf"
if not Path(FONT_MONO).exists():
    FONT_MONO = "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf"


def font(size: int, bold: bool = False, mono: bool = False):
    path = FONT_MONO if mono else (FONT_BOLD if bold else FONT_REG)
    return ImageFont.truetype(path, size)

F = {
    "title": font(76, bold=True),
    "subtitle": font(34),
    "h1": font(48, bold=True),
    "h2": font(34, bold=True),
    "body": font(28),
    "small": font(22),
    "tiny": font(18),
    "mono": font(25, mono=True),
    "mono_small": font(20, mono=True),
    "mono_tiny": font(16, mono=True),
}


def clamp(x, a=0.0, b=1.0):
    return max(a, min(b, x))


def ease(x):
    x = clamp(x)
    return 1 - (1 - x) ** 3


def ease_in_out(x):
    x = clamp(x)
    return x * x * (3 - 2 * x)


def lerp(a, b, t):
    return a + (b - a) * t


def mix(c1, c2, t):
    t = clamp(t)
    return tuple(int(lerp(a, b, t)) for a, b in zip(c1, c2))


def alpha(color, a):
    return tuple(color[:3]) + (int(a),)


_NOISE_CACHE = {}


def bg_gradient(c1, c2, accent=None, t=0.0):
    # Fast vertical gradient: build a 1px-wide strip and scale it up.
    strip = Image.new("RGB", (1, H), c1)
    sd = ImageDraw.Draw(strip)
    for y in range(H):
        sd.point((0, y), fill=mix(c1, c2, y / max(1, H - 1)))
    img = strip.resize((W, H)).convert("RGBA")
    if accent:
        glow = Image.new("RGBA", (W, H), (0, 0, 0, 0))
        gd = ImageDraw.Draw(glow)
        cx = int(W * (0.78 + 0.04 * math.sin(t * 0.4)))
        cy = int(H * (0.18 + 0.03 * math.cos(t * 0.3)))
        # Large translucent ellipses are much faster than per-frame Gaussian blur
        # and still read as a soft cinematic light source in motion.
        for r, a in [(820, 18), (560, 26), (320, 34)]:
            gd.ellipse((cx - r, cy - r, cx + r, cy + r), fill=alpha(accent, a))
        img.alpha_composite(glow)
    return img


def add_noise(img, opacity=14):
    key = opacity
    overlay = _NOISE_CACHE.get(key)
    if overlay is None:
        overlay = Image.new("RGBA", img.size, (0, 0, 0, 0))
        d = ImageDraw.Draw(overlay)
        step = 6
        for y in range(0, H, step):
            for x in range((y // step) % 2 * 3, W, step * 2):
                v = (x * 17 + y * 31) % 255
                a = int(opacity * (v / 255))
                d.point((x, y), fill=(255, 255, 255, a))
        _NOISE_CACHE[key] = overlay
    img.alpha_composite(overlay)


def text_size(draw, text, fnt):
    box = draw.textbbox((0, 0), text, font=fnt)
    return box[2] - box[0], box[3] - box[1]


def draw_center(draw, xy, text, fnt, fill, anchor="mm"):
    draw.text(xy, text, font=fnt, fill=fill, anchor=anchor)


def rounded(draw, box, radius, fill, outline=None, width=1):
    draw.rounded_rectangle(box, radius=radius, fill=fill, outline=outline, width=width)


def glow_rect(img, box, radius, fill, outline, glow, glow_radius=22, glow_alpha=90, width=2):
    # Fast glow approximation: three translucent outlines instead of a blurred layer.
    d = ImageDraw.Draw(img)
    x1, y1, x2, y2 = box
    for pad, a, w in [(10, glow_alpha * 0.18, 3), (5, glow_alpha * 0.28, 3), (0, glow_alpha * 0.45, 2)]:
        d.rounded_rectangle((x1 - pad, y1 - pad, x2 + pad, y2 + pad), radius=radius + pad, outline=alpha(glow, a), width=w)
    d.rounded_rectangle(box, radius=radius, fill=fill, outline=outline, width=width)


def type_text(text: str, start: float, dur: float, t: float):
    if t < start:
        return ""
    n = int(len(text) * clamp((t - start) / max(0.001, dur)))
    return text[:n]


def fade(draw, t, start, dur, max_alpha=255):
    return int(max_alpha * ease((t - start) / max(0.001, dur)))


def draw_terminal(img, x, y, w, h, title="the-witness", accent=(0, 229, 212), alpha_fill=210):
    d = ImageDraw.Draw(img)
    glow_rect(img, (x, y, x + w, y + h), 22, (6, 14, 21, alpha_fill), alpha(accent, 150), accent, 20, 70, 2)
    d.rounded_rectangle((x, y, x + w, y + 54), radius=22, fill=(13, 22, 32, 235))
    d.rectangle((x, y + 32, x + w, y + 54), fill=(13, 22, 32, 235))
    for i, c in enumerate([(255, 91, 86), (255, 189, 46), (39, 201, 63)]):
        d.ellipse((x + 24 + i * 28, y + 19, x + 38 + i * 28, y + 33), fill=c)
    d.text((x + 126, y + 16), title, font=F["mono_tiny"], fill=(180, 212, 220, 230))
    return (x + 30, y + 78, x + w - 30, y + h - 30)


def draw_lines(draw, x, y, lines, fnt, color=(220, 245, 245, 255), line_h=34, prefix=""):
    for i, line in enumerate(lines):
        draw.text((x, y + i * line_h), prefix + line, font=fnt, fill=color)


def draw_badge(draw, x, y, text, color, fill_alpha=50):
    tw, th = text_size(draw, text, F["small"])
    box = (x, y, x + tw + 30, y + 36)
    draw.rounded_rectangle(box, radius=18, fill=alpha(color, fill_alpha), outline=alpha(color, 190), width=1)
    draw.text((x + 15, y + 6), text, font=F["small"], fill=alpha(color, 255))
    return box[2]


def draw_progress(draw, x, y, w, label, p, color):
    draw.text((x, y - 32), label, font=F["small"], fill=(205, 225, 225, 245))
    draw.rounded_rectangle((x, y, x + w, y + 16), radius=8, fill=(20, 30, 38, 220))
    draw.rounded_rectangle((x, y, x + int(w * clamp(p)), y + 16), radius=8, fill=alpha(color, 230))


def draw_tui_box(img, x, y, w, h, title, accent, fill=(9, 18, 26, 220)):
    d = ImageDraw.Draw(img)
    glow_rect(img, (x, y, x + w, y + h), 18, fill, alpha(accent, 130), accent, 18, 60, 2)
    d.text((x + 24, y + 18), title, font=F["h2"], fill=(240, 252, 255, 255))
    d.line((x + 24, y + 66, x + w - 24, y + 66), fill=alpha(accent, 120), width=1)
    return (x + 24, y + 88)


def draw_json(draw, x, y, data, accent, line_h=30):
    draw.text((x, y), "{", font=F["mono_small"], fill=(210, 225, 230, 255))
    for i, line in enumerate(data):
        draw.text((x + 22, y + (i + 1) * line_h), line, font=F["mono_small"], fill=accent if i == 0 else (220, 238, 242, 255))
    draw.text((x, y + (len(data) + 1) * line_h), "}", font=F["mono_small"], fill=(210, 225, 230, 255))


def render_video(name: str, duration: float, frame_func: Callable[[float], Image.Image], thumb_t: float):
    out = ROOT / f"{name}.mp4"
    thumb = ROOT / f"{name.replace('.mp4','')}_thumbnail.png" if name.endswith('.mp4') else ROOT / f"{name}_thumbnail.png"
    if name.endswith('.mp4'):
        stem = name[:-4]
    else:
        stem = name
        out = ROOT / f"{name}.mp4"
        thumb = ROOT / f"{name}_thumbnail.png"
    frames = int(duration * FPS)
    cmd = [
        "ffmpeg", "-y", "-loglevel", "error", "-f", "rawvideo", "-pix_fmt", "rgba", "-s", f"{W}x{H}", "-r", str(FPS), "-i", "-",
        "-an", "-c:v", "libx264", "-preset", "veryfast", "-crf", "18", "-pix_fmt", "yuv420p", "-movflags", "+faststart", str(out)
    ]
    proc = subprocess.Popen(cmd, stdin=subprocess.PIPE)
    for i in range(frames):
        t = i / FPS
        frame = frame_func(t)
        proc.stdin.write(frame.tobytes())
    proc.stdin.close()
    rc = proc.wait()
    if rc != 0:
        raise RuntimeError(f"ffmpeg failed for {name}")
    frame_func(thumb_t).convert("RGB").save(thumb, quality=95)
    print(out)
    print(thumb)


def video1(t):
    img = bg_gradient((3, 9, 13), (4, 23, 30), (0, 229, 212), t)
    add_noise(img, 7)
    d = ImageDraw.Draw(img)
    if t < 4.0:
        a = fade(d, t, 0.3, 0.9)
        d.text((W//2, 380), "THE WITNESS", font=F["title"], fill=(240, 255, 255, a), anchor="mm")
        d.text((W//2, 455), "Local AI verification before AI action.", font=F["subtitle"], fill=(158, 238, 232, a), anchor="mm")
        d.text((W//2, 640), "Do not just trust AI. Let The Witness see it first.", font=F["body"], fill=(220, 245, 245, int(a*0.9)), anchor="mm")
        return img
    body = draw_terminal(img, 120, 100, 820, 790, "setup@the-witness", (0, 229, 212))
    cmd_lines = [
        ("$ ollama pull gemma4:e2b", 4.3, 1.2),
        ("pulling manifest... done", 5.5, 0.4),
        ("$ ollama pull gemma4:e4b", 6.2, 1.0),
        ("pulling manifest... done", 7.2, 0.4),
        ("$ the-witness setup", 8.0, 1.0),
        ("opening first-run setup wizard", 9.1, 0.5),
        ("$ the-witness doctor", 13.0, 1.0),
        ("$ the-witness start", 18.2, 1.0),
    ]
    y = body[1]
    for txt, st, dur in cmd_lines:
        shown = type_text(txt, st, dur, t) if txt.startswith("$") else (txt if t > st else "")
        if shown:
            col = (0, 240, 220, 255) if txt.startswith("$") else (160, 210, 210, 230)
            d.text((body[0], y), shown, font=F["mono"], fill=col)
            y += 42
    if t > 9.3:
        x, y0 = draw_tui_box(img, 1000, 108, 790, 350, "First-run setup", (0, 229, 212))
        items = ["Backend: Ollama", "Default model: gemma4:e2b", "Strong model: gemma4:e4b", "Fallback: human_review"]
        for i, it in enumerate(items):
            a = fade(d, t, 9.6 + i * 0.45, 0.35)
            draw_badge(d, x, y0 + i * 54, "SELECTED" if i == 0 else "CONFIG", (0, 229, 212))
            d.text((x + 145, y0 + i * 54 + 4), it, font=F["body"], fill=(235, 250, 250, a))
    if t > 14.0:
        x, y0 = draw_tui_box(img, 1000, 500, 790, 365, "Doctor readiness", (120, 255, 210))
        checks = ["Default backend: Ollama", "Default model: gemma4:e2b", "Strong model: gemma4:e4b", "Model registry found", "Logs writable", "Proxy port ready"]
        for i, ch in enumerate(checks):
            if t > 14.2 + i * 0.45:
                d.text((x, y0 + i * 42), "[PASS]", font=F["mono_small"], fill=(90, 255, 180, 255))
                d.text((x + 98, y0 + i * 42), ch, font=F["mono_small"], fill=(225, 245, 240, 255))
    if t > 19.0:
        box = (575, 730, 1345, 1010)
        glow_rect(img, box, 24, (5, 18, 24, 245), (0, 229, 212, 160), (0, 229, 212), 24, 90, 2)
        d.text((W//2, 765), "THE WITNESS DASHBOARD", font=F["h1"], fill=(245, 255, 255, 255), anchor="mm")
        stats = [("Watched endpoints", "0"), ("Active backend", "Ollama"), ("Judge model", "gemma4:e2b"), ("Fallback", "human_review"), ("Status", "Ready")]
        for i, (k, v) in enumerate(stats):
            xx = 640 + (i % 2) * 330
            yy = 820 + (i // 2) * 55
            d.text((xx, yy), k, font=F["small"], fill=(150, 210, 210, 255))
            d.text((xx + 210, yy), v, font=F["small"], fill=(0, 245, 218, 255))
    if t > 21.2:
        a = fade(d, t, 21.2, 0.8)
        d.rounded_rectangle((475, 42, 1445, 92), radius=25, fill=(0, 229, 212, int(a*0.15)))
        d.text((W//2, 55), "Before AI speaks, The Witness checks.  •  Powered by Gemma 4", font=F["body"], fill=(240, 255, 255, a), anchor="ma")
    return img


def video2(t):
    img = bg_gradient((7, 4, 18), (3, 8, 28), (180, 40, 255), t)
    add_noise(img, 10)
    d = ImageDraw.Draw(img)
    d.text((80, 55), "Watching the Blackbox Endpoint", font=F["h1"], fill=(245, 240, 255, 255))
    d.text((82, 112), "OpenAI-compatible traffic now flows through localhost:8787", font=F["body"], fill=(190, 205, 255, 220))
    body = draw_terminal(img, 70, 180, 820, 770, "endpoint@watch", (190, 54, 255))
    cmds = [
        ('export BLACKBOX_API_KEY="••••••••••••"', 1.0, 1.5),
        ('the-witness endpoint add-blackbox', 3.0, 1.4),
        ('curl http://localhost:8787/v1/chat/completions \\', 10.0, 1.1),
        ('  -H "Authorization: Bearer $BLACKBOX_API_KEY" \\', 11.2, 1.1),
        ('  -H "Content-Type: application/json" \\', 12.4, 0.9),
        ('  -d \'{"model":"blackboxai/x-ai/grok-code-fast-1:free"}\'', 13.4, 1.3),
    ]
    y = body[1]
    for cmd, st, dur in cmds:
        shown = type_text("$ " + cmd if not cmd.startswith("  ") else cmd, st, dur, t)
        if shown:
            d.text((body[0], y), shown, font=F["mono_small"], fill=(230, 225, 255, 255))
            y += 34
    if t > 4.5:
        x, y0 = draw_tui_box(img, 970, 165, 850, 360, "Endpoint card", (60, 160, 255), (10, 10, 32, 232))
        rows = [
            ("Endpoint", "Blackbox Grok Code"),
            ("Upstream", "https://api.blackbox.ai/v1"),
            ("Local proxy", "http://localhost:8787/v1"),
            ("Model", "blackboxai/x-ai/grok-code-fast-1:free"),
            ("Profile", "coding   Strictness: high   Retry: 4"),
            ("Auth", "bearer_env BLACKBOX_API_KEY"),
        ]
        for i, (k, v) in enumerate(rows):
            if t > 4.7 + i * 0.28:
                d.text((x, y0 + i * 36), k + ":", font=F["mono_small"], fill=(110, 190, 255, 255))
                d.text((x + 160, y0 + i * 36), v, font=F["mono_small"], fill=(238, 235, 255, 255))
    if t > 7.4:
        a = fade(d, t, 7.4, 0.5)
        d.rounded_rectangle((990, 550, 1800, 615), radius=18, fill=(35, 9, 50, int(a*0.88)), outline=(240, 80, 255, a), width=2)
        d.text((1020, 568), "Authorization: Bearer ********", font=F["mono"], fill=(255, 185, 255, a))
    # flow diagram
    if t > 14.0:
        nodes = [(1110, 700, "AI App"), (1350, 700, "Witness\nProxy :8787"), (1590, 700, "Blackbox\nUpstream"), (1350, 880, "Gemma 4\nJudge")]
        for idx, (cx, cy, label) in enumerate(nodes):
            a = fade(d, t, 14 + idx * 0.4, 0.4)
            d.rounded_rectangle((cx-95, cy-48, cx+95, cy+48), radius=20, fill=(18, 8, 38, int(a*0.9)), outline=(80, 180, 255, a), width=2)
            d.text((cx, cy-18 if "\n" in label else cy-3), label, font=F["small"], fill=(240, 245, 255, a), anchor="mm", align="center")
        if t > 15.8:
            for (x1,y1),(x2,y2) in [((1205,700),(1255,700)),((1445,700),(1495,700)),((1590,750),(1420,845)),((1280,845),(1350,750))]:
                d.line((x1,y1,x2,y2), fill=(255, 60, 220, 220), width=4)
                d.ellipse((x2-5,y2-5,x2+5,y2+5), fill=(255,60,220,255))
    if t > 17.3:
        x, y0 = draw_tui_box(img, 970, 820, 850, 160, "Live request stream", (255, 60, 220), (12, 5, 26, 230))
        rows = ["REQ-1024  Blackbox Grok Code  coding  forwarded", "REQ-1024  Blackbox Grok Code  coding  judging"]
        for i, row in enumerate(rows):
            if t > 17.5 + i * 1.1:
                d.text((x, y0 + i * 35), row, font=F["mono_small"], fill=(235, 235, 255, 255))
    if t > 21.8:
        d.text((W//2, 1005), "Every request is now visible. Add an endpoint. The Witness starts watching.", font=F["body"], fill=(255, 242, 255, 255), anchor="mm")
    return img


def video3(t):
    shift = ease_in_out((t-10)/8)
    img = bg_gradient(mix((28, 2, 8), (2, 23, 17), shift), mix((8, 6, 10), (2, 12, 18), shift), mix((255, 38, 70), (0, 255, 170), shift), t)
    add_noise(img, 8)
    d = ImageDraw.Draw(img)
    d.text((80, 58), "Rejected, Repaired, Approved", font=F["h1"], fill=(255, 248, 248, 255))
    d.text((82, 116), "The core firewall loop in motion", font=F["body"], fill=(240, 210, 210, 220))
    # request and response cards
    if t > 1:
        x, y = draw_tui_box(img, 90, 205, 760, 185, "User request", (255, 70, 90), (24, 10, 14, 235))
        d.text((x, y), "Write a Python script that prints Hello World", font=F["body"], fill=(255, 245, 245, 255))
    if t > 3:
        x, y = draw_tui_box(img, 90, 430, 760, 185, "Candidate response", (255, 70, 90), (24, 10, 14, 235))
        d.text((x, y), "print(Hello World)", font=F["mono"], fill=(255, 170, 180, 255))
    if t > 5:
        x, y = draw_tui_box(img, 950, 150, 830, 410, "Gemma 4 verdict", (255, 60, 80), (25, 8, 13, 240))
        data = [
            '"verdict": "DISAPPROVED",',
            '"confidence": 0.91,',
            '"correctness_risk": "medium",',
            '"rejection_reason": "Python string is not quoted.",',
            '"suggested_fix": "Use print(\\"Hello World\\")"',
        ]
        draw_json(d, x, y, data, (255, 90, 110, 255), 38)
    if 7 < t < 11.5:
        a = fade(d, t, 7, 0.4)
        d.rounded_rectangle((550, 665, 1370, 745), radius=30, fill=(255, 34, 70, int(a*0.2)), outline=(255, 55, 90, a), width=3)
        d.text((W//2, 690), "Blocked before reaching the app.", font=F["h1"], fill=(255, 235, 235, a), anchor="ma")
    if t > 10.8:
        x, y = draw_tui_box(img, 90, 675, 820, 300, "Prompt repair panel", (255, 185, 80), (22, 16, 8, 238))
        repair = ["Previous answer was rejected.", "Reason: Python string is not quoted.", "Required fix: Use valid Python syntax.", "Now generate a corrected answer."]
        for i, line in enumerate(repair):
            if t > 11.0 + i * 0.45:
                d.text((x, y + i * 42), line, font=F["mono_small"], fill=(255, 238, 210, 255))
    # retry conveyor
    if t > 14:
        cy = 760
        xs = [1060, 1260, 1460, 1660]
        labs = ["Attempt 1", "Rejected", "Attempt 2", "Approved"]
        cols = [(255,70,90),(255,70,90),(80,220,255),(0,255,170)]
        for i, x in enumerate(xs):
            p = fade(d, t, 14 + i * 0.7, 0.45)
            d.rounded_rectangle((x-80, cy-40, x+80, cy+40), radius=20, fill=alpha(cols[i], int(p*0.22)), outline=alpha(cols[i], p), width=2)
            d.text((x, cy-7), labs[i], font=F["small"], fill=(255,255,255,p), anchor="mm")
            if i < len(xs)-1 and t > 14.5 + i * 0.7:
                d.line((x+82, cy, xs[i+1]-82, cy), fill=(200,230,240,220), width=4)
    if t > 18.5:
        x, y = draw_tui_box(img, 950, 855, 830, 155, "Final response returned", (0, 255, 170), (3, 25, 18, 240))
        d.text((x, y + 10), 'print("Hello World")', font=F["mono"], fill=(180, 255, 225, 255))
    if t > 20.0:
        x, y = draw_tui_box(img, 950, 580, 830, 245, "Approved verdict", (0, 255, 170), (3, 24, 18, 240))
        data = ['"verdict": "APPROVED",', '"confidence": 0.98,', '"safety_score": 100,', '"prompt_alignment_score": 100']
        draw_json(d, x, y, data, (80,255,190,255), 32)
    if t > 25.5:
        d.text((W//2, 1015), "The Witness does not just detect failure. It repairs the path to success.", font=F["body"], fill=(230, 255, 245, 255), anchor="mm")
    return img


def video4(t):
    img = bg_gradient((5, 12, 25), (15, 22, 42), (190, 210, 255), t)
    add_noise(img, 6)
    d = ImageDraw.Draw(img)
    d.text((80, 60), "Model Manager: Four Tech Tracks", font=F["h1"], fill=(245, 248, 255, 255))
    d.text((82, 118), "One verifier, multiple Gemma deployment paths", font=F["body"], fill=(190, 205, 230, 235))
    body = draw_terminal(img, 80, 175, 760, 245, "models@the-witness", (180, 210, 255))
    d.text((body[0], body[1]), type_text("$ the-witness model list", 1.0, 1.2, t), font=F["mono"], fill=(232, 240, 255, 255))
    if t > 2.4:
        x, y = draw_tui_box(img, 80, 455, 1760, 250, "Model Manager", (180, 210, 255), (7, 12, 24, 235))
        rows = [
            ("Gemma 4 E2B via Ollama", "ollama", "local"),
            ("Gemma 4 E4B via Ollama", "ollama", "local"),
            ("Fine-tuned Witness Gemma 4 E2B Judge", "unsloth", "huggingface"),
            ("llama.cpp local judge", "llama.cpp", "local"),
            ("LiteRT edge prefilter", "litert", "edge"),
            ("Manual OpenAI-compatible judge", "manual", "endpoint"),
        ]
        d.text((x, y), "Model", font=F["mono_small"], fill=(130, 170, 230, 255))
        d.text((x+760, y), "Backend", font=F["mono_small"], fill=(130, 170, 230, 255))
        d.text((x+1040, y), "Source", font=F["mono_small"], fill=(130, 170, 230, 255))
        for i, row in enumerate(rows):
            if t > 2.8 + i * 0.25:
                yy = y + 36 + i*28
                d.text((x, yy), row[0], font=F["mono_tiny"], fill=(245, 248, 255, 255))
                d.text((x+760, yy), row[1], font=F["mono_tiny"], fill=(210, 225, 255, 255))
                d.text((x+1040, yy), row[2], font=F["mono_tiny"], fill=(210, 225, 255, 255))
    if t > 6.0:
        cards = [("Ollama", "Default local\nGemma 4 judge", (0,220,210)), ("llama.cpp", "Resource-constrained\nlocal inference", (160,210,255)), ("LiteRT", "Edge prefilter\nfast classification", (255,220,120)), ("Unsloth", "Fine-tuned\nWitness judge", (210,160,255))]
        for i,(title, sub, col) in enumerate(cards):
            x = 95 + i*455
            y = 750
            a = fade(d, t, 6.0+i*0.45, 0.5)
            d.rounded_rectangle((x,y,x+390,y+150), radius=28, fill=alpha(col, int(a*0.13)), outline=alpha(col,a), width=2)
            d.text((x+28,y+26), title, font=F["h2"], fill=(255,255,255,a))
            d.text((x+28,y+78), sub, font=F["small"], fill=(225,232,245,a))
    if t > 12.0:
        d.rounded_rectangle((230, 925, 1690, 1000), radius=24, fill=(10, 17, 34, 245), outline=(160,190,255,180), width=2)
        cmd = "$ the-witness model download --source huggingface --model witness-gemma4-e2b-judge"
        d.text((260, 950), type_text(cmd, 12.0, 2.5, t), font=F["mono_small"], fill=(235,240,255,255))
    if t > 15.5:
        d.text((W//2, 245), "https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge", font=F["body"], fill=(210, 220, 255, 255), anchor="mm")
    if t > 18.0:
        d.text((W//2, 305), "Colab fine-tuning notebook: 17-CgEQLNg8bpnhhWzJwpapRxQyHIqybq", font=F["small"], fill=(190, 205, 235, 245), anchor="mm")
    if t > 21.0:
        x0 = 620
        tracks = [("Ollama", True), ("llama.cpp", True), ("LiteRT", True), ("Unsloth", True), ("Cactus", False)]
        for i,(tr, ok) in enumerate(tracks):
            col = (90,255,190) if ok else (170,180,200)
            d.text((x0 + i*160, 380), tr, font=F["small"], fill=(240,245,255,255), anchor="mm")
            d.text((x0 + i*160, 420), "✓" if ok else "architecture-ready", font=F["small"], fill=col, anchor="mm")
    if t > 25.0:
        d.text((W//2, 1030), "One idea. Multiple Gemma deployment paths.", font=F["h2"], fill=(245,248,255,255), anchor="mm")
    return img


def video5(t):
    img = bg_gradient((23, 22, 18), (8, 15, 13), (220, 180, 90), t)
    add_noise(img, 16)
    d = ImageDraw.Draw(img)
    d.text((80, 58), "Audit Trail for Trust", font=F["h1"], fill=(255, 250, 235, 255))
    d.text((82, 116), "Verification should leave evidence, not just vibes.", font=F["body"], fill=(230, 220, 190, 230))
    body = draw_terminal(img, 80, 175, 690, 205, "audit@the-witness", (220, 180, 90), 225)
    d.text((body[0], body[1]), type_text("$ the-witness logs", 1.0, 1.0, t), font=F["mono"], fill=(255, 240, 210, 255))
    if t > 2.4:
        x, y = draw_tui_box(img, 80, 415, 820, 445, "Audit log timeline", (220, 180, 90), (24, 22, 17, 235))
        logs = ["10:24:31  Request received", "10:24:32  Candidate response captured", "10:24:33  Verdict: DISAPPROVED", "10:24:33  Reason: Missing uncertainty", "10:24:34  Human review required", "10:24:40  Reviewer action: Regenerate", "10:24:45  Verdict: APPROVED", "10:24:45  Exported audit report"]
        for i, line in enumerate(logs):
            if t > 2.7 + i * 0.45:
                col = (255, 120, 110, 255) if "DISAPPROVED" in line else ((110, 230, 170, 255) if "APPROVED" in line else (245, 235, 210, 255))
                d.text((x, y + i*38), line, font=F["mono_small"], fill=col)
    if t > 7.5:
        x, y = draw_tui_box(img, 980, 180, 760, 245, "Human Review Queue", (110, 230, 170), (13, 24, 19, 238))
        rows = ["High-risk response paused", "Profile: Health & Sciences", "Action: Approve / Reject / Edit / Regenerate"]
        for i, line in enumerate(rows):
            d.text((x, y+i*48), line, font=F["body" if i==0 else "small"], fill=(235,255,240,255))
    if t > 11.0:
        x, y = draw_tui_box(img, 980, 470, 760, 180, "Privacy mode", (220, 180, 90), (27, 24, 16, 238))
        d.text((x, y), "Privacy Mode: ON", font=F["h2"], fill=(255,235,190,255))
        d.text((x, y+55), "Prompt storage: metadata only", font=F["small"], fill=(245,235,210,255))
        d.text((x, y+95), "Secrets: redacted", font=F["small"], fill=(245,235,210,255))
    if t > 14.0:
        d.rounded_rectangle((980, 690, 1740, 755), radius=22, fill=(18, 17, 12, 240), outline=(220,180,90,180), width=2)
        d.text((1010, 710), type_text("$ the-witness export REQ-1024 --format markdown", 14.0, 1.8, t), font=F["mono_small"], fill=(255,240,210,255))
    if t > 16.3:
        x, y = draw_tui_box(img, 980, 790, 760, 210, "Audit Report", (110, 230, 170), (14, 24, 19, 238))
        rows = ["Endpoint: Blackbox Grok Code", "Profile: coding", "Attempts: 2", "Final verdict: APPROVED", "Rejected reason: unsafe/incomplete first output"]
        for i, row in enumerate(rows):
            d.text((x, y+i*30), row, font=F["mono_tiny"], fill=(235,255,238,255))
    if t > 20.0:
        cards = [("Safety & Trust", "Explainable verification"), ("Digital Equity", "Local-first, low-cost"), ("Future of Education", "Safer AI tutors"), ("Health & Sciences", "Uncertainty + review"), ("Global Resilience", "Offline-ready checks")]
        for i,(a,b) in enumerate(cards):
            xx = 90 + i*355
            yy = 900
            col = (220,180,90) if i%2 else (110,230,170)
            d.rounded_rectangle((xx, yy, xx+320, yy+120), radius=22, fill=alpha(col, 35), outline=alpha(col, 180), width=2)
            d.text((xx+18, yy+22), a, font=F["small"], fill=(255,250,235,255))
            d.text((xx+18, yy+62), b, font=F["tiny"], fill=(230,225,205,255))
    if t > 25.0:
        a = fade(d, t, 25.0, 0.8)
        d.rectangle((0,0,W,H), fill=(5, 8, 7, int(a*0.75)))
        d.text((W//2, 450), "In the places where AI mistakes matter most,", font=F["h1"], fill=(255,250,235,a), anchor="mm")
        d.text((W//2, 520), "trust must be local.", font=F["h1"], fill=(140,255,195,a), anchor="mm")
        d.text((W//2, 660), "THE WITNESS", font=F["title"], fill=(255,250,235,a), anchor="mm")
        d.text((W//2, 735), "Local AI verification before AI action.", font=F["subtitle"], fill=(230,220,190,a), anchor="mm")
    return img


def main():
    ROOT.mkdir(parents=True, exist_ok=True)
    render_video("video_01_endpoint_awakens", 24, video1, 21.5)
    render_video("video_02_endpoint_watch", 26, video2, 18.5)
    render_video("video_03_repair_loop", 29, video3, 21.0)
    render_video("video_04_model_manager_tracks", 28, video4, 22.0)
    render_video("video_05_audit_impact", 29, video5, 23.0)

    # The hackathon handoff expects short thumbnail names. Keep the descriptive
    # thumbnails too, but always refresh the exact deliverable filenames.
    aliases = [
        ("video_01_endpoint_awakens_thumbnail.png", "video_01_thumbnail.png"),
        ("video_02_endpoint_watch_thumbnail.png", "video_02_thumbnail.png"),
        ("video_03_repair_loop_thumbnail.png", "video_03_thumbnail.png"),
        ("video_04_model_manager_tracks_thumbnail.png", "video_04_thumbnail.png"),
        ("video_05_audit_impact_thumbnail.png", "video_05_thumbnail.png"),
    ]
    for src, dst in aliases:
        (ROOT / dst).write_bytes((ROOT / src).read_bytes())
        print(ROOT / dst)

if __name__ == "__main__":
    main()
