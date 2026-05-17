#!/usr/bin/env python3
from __future__ import annotations

import math
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "source"
W, H = 1920, 1080
SQ = 1600
P45 = (1440, 1800)

COL = {
    "bg": (3, 7, 10),
    "panel": (6, 18, 23),
    "panel2": (8, 24, 32),
    "teal": (0, 229, 212),
    "green": (80, 255, 170),
    "white": (240, 255, 252),
    "muted": (155, 205, 202),
    "red": (255, 72, 96),
    "amber": (245, 184, 72),
    "purple": (175, 72, 255),
    "blue": (78, 162, 255),
}


def font(size: int, mono=False, bold=False):
    fam = "dejavu/DejaVuSansMono" if mono else "dejavu/DejaVuSans"
    suffix = "-Bold.ttf" if bold else ".ttf"
    path = Path("/usr/share/fonts/truetype") / f"{fam}{suffix}"
    if not path.exists():
        path = Path("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf")
    return ImageFont.truetype(str(path), size)

F = {
    "hero": font(96, bold=True),
    "h1": font(58, bold=True),
    "h2": font(40, bold=True),
    "h3": font(30, bold=True),
    "body": font(28),
    "small": font(22),
    "tiny": font(18),
    "mono": font(26, mono=True),
    "mono_small": font(21, mono=True),
    "mono_tiny": font(17, mono=True),
}


def rgba(c, a=255): return (*c, a)

def blend(a, b, t): return tuple(int(a[i]*(1-t)+b[i]*t) for i in range(3))

def bg(w=W, h=H, accent=COL["teal"], dark=(3,7,10)):
    im = Image.new("RGB", (w,h), dark)
    d = ImageDraw.Draw(im, "RGBA")
    for y in range(h):
        t = y/(h-1)
        c = blend(dark, (4,18,24), t)
        d.line((0,y,w,y), fill=(*c,255))
    step = 80 if w > 1200 else 64
    for x in range(0,w,step): d.line((x,0,x,h), fill=(*accent,15), width=1)
    for y in range(0,h,step): d.line((0,y,w,y), fill=(*accent,12), width=1)
    # soft orbital rings
    for r in range(260, 980, 120):
        d.rounded_rectangle((w//2-r, h//2-r//2, w//2+r, h//2+r//2), radius=80, outline=(*accent, 18), width=2)
    # vignette
    d.rectangle((0,0,w,h), outline=(0,0,0,120), width=58)
    return im.convert("RGBA")


def panel(d, box, title=None, color=None, fill=None, width=2, radius=28):
    color = color or COL["teal"]
    fill = fill or (5,18,23,228)
    x1,y1,x2,y2 = box
    # glow approximation
    for i,a in enumerate([50,32,18]):
        pad = 7+i*7
        d.rounded_rectangle((x1-pad,y1-pad,x2+pad,y2+pad), radius=radius+pad, outline=(*color,a), width=2)
    d.rounded_rectangle(box, radius=radius, fill=fill, outline=(*color,165), width=width)
    if title:
        d.text((x1+30,y1+24), title, font=F["h3"], fill=rgba(COL["white"]))
        d.line((x1+30,y1+78,x2-30,y1+78), fill=(*color,125), width=2)


def badge(d, xy, text, color, small=False):
    x,y = xy; f = F["tiny"] if small else F["small"]
    b = d.textbbox((0,0), text, font=f)
    w = b[2]-b[0] + 32; h = 36 if small else 44
    d.rounded_rectangle((x,y,x+w,y+h), radius=h//2, fill=(*color,45), outline=(*color,180), width=2)
    d.text((x+w/2,y+h/2-2), text, font=f, fill=rgba(COL["white"]), anchor="mm")
    return w


def arrow(d, start, end, color=COL["teal"], width=5):
    x1,y1=start; x2,y2=end
    d.line((x1,y1,x2,y2), fill=(*color,220), width=width)
    ang = math.atan2(y2-y1, x2-x1)
    pts=[]
    for da in [math.pi*0.82, -math.pi*0.82]:
        pts.append((x2+22*math.cos(ang+da), y2+22*math.sin(ang+da)))
    d.polygon([(x2,y2), *pts], fill=(*color,230))


def terminal(d, box, title, lines, accent=COL["teal"]):
    panel(d, box, None, accent, (3,12,17,238), radius=24)
    x1,y1,x2,y2=box
    d.rounded_rectangle((x1,y1,x2,y1+54), radius=24, fill=(9,23,30,245))
    d.rectangle((x1,y1+30,x2,y1+54), fill=(9,23,30,245))
    for i,c in enumerate([(255,91,86),(255,189,46),(39,201,63)]):
        d.ellipse((x1+24+i*28,y1+19,x1+38+i*28,y1+33), fill=(*c,255))
    d.text((x1+125,y1+17), title, font=F["mono_tiny"], fill=rgba(COL["muted"],245))
    y=y1+86
    for line,col in lines:
        if len(line)>72: line=line[:69]+"…"
        d.text((x1+28,y), line, font=F["mono_small"], fill=rgba(col)); y+=38


def title(d, main, sub=None, x=96, y=70, accent=COL["teal"]):
    d.text((x,y), main, font=F["h1"], fill=rgba(COL["white"]))
    if sub:
        d.text((x,y+70), sub, font=F["body"], fill=rgba(COL["muted"],240))
    d.rounded_rectangle((x,y+126,x+260,y+134), radius=4, fill=rgba(accent,210))


def save(im, name):
    path = ROOT / name
    path.parent.mkdir(parents=True, exist_ok=True)
    im.convert("RGB").save(path, quality=96)
    print(path)


def cover():
    im=bg(accent=COL["teal"]); d=ImageDraw.Draw(im,"RGBA")
    title(d,"THE WITNESS","Local AI verification before AI action.",86,64)
    d.text((86,230),"Powered by Gemma 4 • Local-first • Explainable • Auditable",font=F["body"],fill=rgba(COL["white"],235))
    # flow nodes
    nodes=[(160,415,360,560,"AI App"),(650,390,940,585,"The Witness\nProxy"),(1240,415,1585,560,"Upstream\nEndpoint")]
    for x1,y1,x2,y2,label in nodes:
        panel(d,(x1,y1,x2,y2),None,COL["teal"] if "Witness" in label else COL["blue"],(6,20,27,232))
        d.text(((x1+x2)/2,(y1+y2)/2-12),label,font=F["h2"],fill=rgba(COL["white"]),anchor="mm",align="center")
    arrow(d,(360,488),(650,488),COL["teal"]); arrow(d,(940,488),(1240,488),COL["teal"])
    panel(d,(455,675,1465,940),"Gemma 4 Judge Verdict",COL["teal"],(4,20,23,235))
    badge(d,(510,775),"APPROVED",COL["green"]); badge(d,(745,775),"DISAPPROVED",COL["red"]); badge(d,(1040,775),"HUMAN REVIEW",COL["amber"])
    d.text((510,855),'verdict: structured JSON • confidence: 0.98 • audit: written',font=F["mono_small"],fill=rgba(COL["muted"],245))
    # witness icon
    d.ellipse((1590,165,1780,355),outline=rgba(COL["teal"],220),width=5)
    d.ellipse((1652,228,1718,294),fill=rgba(COL["teal"],70),outline=rgba(COL["teal"],230),width=4)
    return im


def architecture():
    im=bg(accent=COL["blue"]); d=ImageDraw.Draw(im,"RGBA")
    title(d,"Architecture: local verification firewall","Every response is judged before it reaches the user.",86,56,COL["blue"])
    steps=[("AI App / Agent","sends OpenAI-compatible request",COL["blue"]),("The Witness Local Proxy","captures request + response",COL["teal"]),("Upstream AI Endpoint","returns candidate response",COL["purple"]),("Gemma 4 Judge","local JSON verdict",COL["green"]),("Verdict Router","approve / repair / review",COL["amber"]),("Audit Log","JSONL / SQLite timeline",COL["teal"])]
    xs=[130,440,750,1060,1370,1540]; ys=[330,330,330,330,640,640]
    boxes=[]
    for (name,sub,col),x,y in zip(steps,xs,ys):
        w=250 if x<1500 else 240; box=(x,y,x+w,y+145); boxes.append(box)
        panel(d,box,None,col,(6,17,25,235),radius=22)
        d.text((x+22,y+26),name,font=F["h3" if w>245 else "small"],fill=rgba(COL["white"]))
        d.text((x+22,y+82),sub,font=F["small"],fill=rgba(COL["muted"],240))
    for a,b in zip(boxes[:4],boxes[1:4]+[boxes[4]]): arrow(d,(a[2],(a[1]+a[3])//2),(b[0],(b[1]+b[3])//2),COL["teal"])
    arrow(d,(boxes[4][2],710),(boxes[5][0],710),COL["teal"])
    side=[("OpenAI-compatible proxy",122,540,COL["blue"]),("Local Gemma verification",1040,540,COL["green"]),("Prompt repair loop",1320,850,COL["red"]),("Human override",1040,850,COL["amber"])]
    for txt,x,y,col in side: badge(d,(x,y),txt,col,True)
    d.text((112,980),"APPROVED returns response • DISAPPROVED repairs and retries • NEEDS_HUMAN_REVIEW pauses for a person",font=F["body"],fill=rgba(COL["white"],235))
    return im


def approval_loop():
    im=bg(accent=COL["green"]); d=ImageDraw.Draw(im,"RGBA")
    title(d,"Approval Loop","Bad answers are blocked, repaired, retried, then released only after approval.",86,56,COL["green"])
    center=(960,565); r=300
    labels=[("Request",-90,COL["blue"]),("Candidate\nResponse",-25,COL["purple"]),("Gemma\nJudge",42,COL["teal"]),("DISAPPROVED",110,COL["red"]),("Prompt\nRepair",180,COL["amber"]),("Retry",245,COL["blue"]),("APPROVED",315,COL["green"])]
    points=[]
    for lab,ang,col in labels:
        rad=math.radians(ang); x=center[0]+r*math.cos(rad); y=center[1]+r*math.sin(rad); points.append((x,y,col))
        panel(d,(x-130,y-62,x+130,y+62),None,col,(5,18,24,235),radius=22)
        d.text((x,y-8),lab,font=F["h3"] if "\n" not in lab else F["small"],fill=rgba(COL["white"]),anchor="mm",align="center")
    for (x1,y1,c1),(x2,y2,c2) in zip(points,points[1:]+points[:1]): arrow(d,(x1,y1),(x2,y2),c2,4)
    panel(d,(120,680,560,915),"Bad response",COL["red"]); d.text((160,790),"print(Hello World)",font=F["mono"],fill=rgba(COL["red"]))
    panel(d,(680,760,1240,930),"Rejection",COL["amber"]); d.text((725,855),'"Python string is not quoted."',font=F["body"],fill=rgba(COL["white"]))
    panel(d,(1360,680,1800,915),"Repaired response",COL["green"]); d.text((1410,790),'print("Hello World")',font=F["mono"],fill=rgba(COL["green"]))
    return im


def tui_dashboard():
    im=bg(accent=COL["teal"]); d=ImageDraw.Draw(im,"RGBA")
    shell=(80,80,1840,1000); panel(d,shell,None,COL["teal"],(2,10,14,235),radius=30)
    d.text((120,106),"The Witness",font=F["h2"],fill=rgba(COL["white"])); d.text((410,118),"Backend: Ollama  |  Judge: gemma4:e2b  |  Fallback: human_review",font=F["mono_small"],fill=rgba(COL["muted"]))
    panel(d,(120,190,545,850),"Watched endpoints",COL["blue"])
    for i,e in enumerate(["Blackbox Grok Code","Local Tutor","Medical Assistant","Finance Assistant"]): badge(d,(155,290+i*105),e,COL["teal"] if i==0 else COL["blue"])
    panel(d,(595,190,1245,850),"Live Request Stream",COL["teal"])
    rows=[("REQ-1024","forwarded",COL["blue"]),("REQ-1024","judging",COL["amber"]),("REQ-1024","disapproved",COL["red"]),("REQ-1024","retrying",COL["amber"]),("REQ-1024","approved",COL["green"])]
    for i,(rid,st,col) in enumerate(rows):
        y=295+i*90; d.text((635,y),rid,font=F["mono"],fill=rgba(COL["white"])); badge(d,(850,y-10),st,col)
    panel(d,(1295,190,1800,850),"Gemma Verdict",COL["green"])
    for i,line in enumerate(['verdict: APPROVED','confidence: 0.98','safety_score: 100','prompt_alignment_score: 100']): d.text((1340,310+i*75),line,font=F["mono_small"],fill=rgba(COL["white"]))
    d.text((120,925),"q quit  |  n new endpoint  |  p profile  |  l logs  |  m models",font=F["mono"],fill=rgba(COL["teal"]))
    return im


def endpoint_watchlist():
    im=bg(accent=COL["purple"]); d=ImageDraw.Draw(im,"RGBA")
    title(d,"Endpoint Watchlist","Add an endpoint once. The Witness watches every request that flows through it.",86,58,COL["purple"])
    panel(d,(120,220,1800,900),"Endpoint Manager",COL["purple"])
    panel(d,(180,335,820,790),"Blackbox Grok Code",COL["blue"])
    fields=[("Upstream","https://api.blackbox.ai/v1"),("Local proxy","http://localhost:8787/v1"),("Auth","bearer_env BLACKBOX_API_KEY"),("Profile","coding"),("Strictness","high"),("Retry limit","4"),("Status","watching")]
    for i,(k,v) in enumerate(fields):
        y=435+i*46; d.text((220,y),k,font=F["small"],fill=rgba(COL["muted"])); d.text((420,y),v,font=F["mono_small"],fill=rgba(COL["white"]))
    badge(d,(510,720),"Copy proxy URL",COL["teal"])
    panel(d,(930,335,1740,790),"Traffic now visible",COL["teal"])
    for i,txt in enumerate(["request received","forwarded upstream","candidate captured","Gemma judging","approved response returned"]):
        badge(d,(990,435+i*60),txt,[COL["blue"],COL["purple"],COL["amber"],COL["teal"],COL["green"]][i])
    return im


def model_manager():
    im=bg(accent=COL["blue"]); d=ImageDraw.Draw(im,"RGBA")
    title(d,"Model Manager: Gemma Track Coverage","One trust layer, multiple Gemma deployment paths.",86,58,COL["blue"])
    panel(d,(100,220,1245,850),"Configured Judge Models",COL["blue"])
    cols=["Model","Backend","Source","Status"]; xs=[145,640,840,1050]
    for x,c in zip(xs,cols): d.text((x,310),c,font=F["h3"],fill=rgba(COL["white"]))
    rows=[("Gemma 4 E2B","Ollama","Local","Default"),("Gemma 4 E4B","Ollama","Local","Strong"),("Witness Gemma 4 E2B Judge","Unsloth","Hugging Face","Fine-tuned"),("Gemma GGUF","llama.cpp","Local","Low-resource"),("LiteRT Prefilter","LiteRT","Edge","Experimental"),("Custom Judge","Manual","Endpoint","Advanced")]
    for i,row in enumerate(rows):
        y=380+i*65; d.line((130,y-14,1190,y-14),fill=rgba(COL["blue"],70),width=1)
        for x,val in zip(xs,row): d.text((x,y),val,font=F["small"],fill=rgba(COL["muted"] if x!=1050 else COL["green"]))
    tracks=[("Ollama","local judge",COL["teal"]),("llama.cpp","low-resource",COL["blue"]),("LiteRT","edge prefilter",COL["amber"]),("Unsloth","fine-tuned",COL["purple"]),("Cactus","architecture-ready",COL["green"])]
    for i,(a,b,c) in enumerate(tracks):
        x=1310; y=245+i*118; panel(d,(x,y,1780,y+82),None,c,(7,16,24,230),radius=18); d.text((x+24,y+18),a,font=F["h3"],fill=rgba(COL["white"])); d.text((x+210,y+25),b,font=F["small"],fill=rgba(COL["muted"]))
    return im


def impact_map():
    im=bg(accent=COL["green"]); d=ImageDraw.Draw(im,"RGBA")
    title(d,"Impact Track Map","The Witness is built for places where AI reliability matters most.",86,58,COL["green"])
    cx,cy=960,560; panel(d,(760,430,1160,690),None,COL["teal"],(4,20,23,235)); d.text((cx,520),"THE\nWITNESS",font=F["h1"],fill=rgba(COL["white"]),anchor="mm",align="center")
    items=[("Safety & Trust","Explainable verdicts\naudit logs\nhuman review",260,300,COL["teal"]),("Digital Equity","Local-first\nlow bandwidth\nprivacy-friendly",1180,300,COL["green"]),("Future of Education","Safer AI tutors\nclearer explanations",210,740,COL["blue"]),("Health & Sciences","Uncertainty checks\nrisky answers paused",760,790,COL["amber"]),("Global Resilience","Offline-ready\nfield verification",1290,740,COL["purple"])]
    for name,body,x,y,col in items:
        panel(d,(x,y,x+390,y+210),None,col,(6,18,25,235),radius=22)
        d.text((x+28,y+28),name,font=F["h3"],fill=rgba(COL["white"])); d.text((x+28,y+88),body,font=F["small"],fill=rgba(COL["muted"]),spacing=6)
        arrow(d,(cx,cy),(x+195,y+105),col,3)
    return im


def hf_model():
    im=bg(accent=COL["purple"]); d=ImageDraw.Draw(im,"RGBA")
    title(d,"Fine-tuned Witness Gemma 4 E2B Judge","Structured verdicts for AI response approval.",86,58,COL["purple"])
    panel(d,(220,240,1700,835),"Hugging Face Model Card",COL["purple"])
    rows=[("Model","ahmadalfakeh/witness-gemma4-e2b-judge"),("Hosted on","Hugging Face"),("Trained with","Unsloth"),("Purpose","APPROVED / DISAPPROVED / HUMAN REVIEW JSON verdicts")]
    for i,(k,v) in enumerate(rows):
        y=365+i*95; d.text((300,y),k,font=F["h3"],fill=rgba(COL["muted"])); d.text((620,y),v,font=F["h3" if i<3 else "small"],fill=rgba(COL["white"]))
    d.rounded_rectangle((300,720,1620,775),radius=22,fill=rgba(COL["purple"],40),outline=rgba(COL["purple"],170),width=2)
    d.text((330,732),"https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge",font=F["mono_small"],fill=rgba(COL["white"]))
    return im


def colab():
    im=bg(accent=COL["amber"]); d=ImageDraw.Draw(im,"RGBA")
    title(d,"Reproducible Fine-tuning Notebook","The public Colab path makes the judge model reproducible.",86,58,COL["amber"])
    panel(d,(180,220,1740,890),"Colab notebook workflow",COL["amber"])
    cells=["1. Load Witness judge dataset","2. Format prompt/response verdict examples","3. Fine-tune Gemma 4 E2B with Unsloth","4. Evaluate JSON validity and verdict accuracy","5. Export model","6. Upload/publish model"]
    for i,c in enumerate(cells):
        y=330+i*75; d.rounded_rectangle((250,y,1665,y+54),radius=14,fill=(18,15,10,220),outline=rgba(COL["amber"],95),width=1); d.text((285,y+14),c,font=F["mono_small"],fill=rgba(COL["white"]))
    d.text((250,825),"https://colab.research.google.com/drive/17-CgEQLNg8bpnhhWzJwpapRxQyHIqybq?usp=sharing",font=F["mono_small"],fill=rgba(COL["amber"]))
    return im


def blackbox_test():
    im=bg(accent=COL["purple"]); d=ImageDraw.Draw(im,"RGBA")
    title(d,"Blackbox Endpoint Test","The Witness can sit in front of any OpenAI-compatible endpoint.",86,58,COL["purple"])
    lines=[('$ export BLACKBOX_API_KEY="••••••••"',COL["teal"]),('$ the-witness endpoint add-blackbox',COL["teal"]),('$ curl http://localhost:8787/v1/chat/completions \\',COL["white"]),('  -H "Authorization: Bearer $BLACKBOX_API_KEY" \\',COL["muted"]),('  -H "Content-Type: application/json"',COL["muted"])]
    terminal(d,(120,250,900,820),"blackbox@test",lines,COL["purple"])
    panel(d,(1010,250,1780,820),"TUI stream",COL["teal"])
    for i,(txt,col) in enumerate([("REQ-1024 received",COL["blue"]),("REQ-1024 forwarded",COL["purple"]),("REQ-1024 judging",COL["amber"]),("REQ-1024 approved",COL["green"])]): badge(d,(1080,380+i*90),txt,col)
    return im


def human_review():
    im=bg(accent=COL["amber"]); d=ImageDraw.Draw(im,"RGBA")
    title(d,"Human Review Queue","High-risk responses can be paused before they reach users.",86,58,COL["amber"])
    panel(d,(340,240,1580,865),"Human Review Queue",COL["amber"])
    fields=[("Request","REQ-2011"),("Profile","Health & Sciences"),("Risk","high"),("Reason","medical-style answer requires uncertainty"),("Action required","Approve | Reject | Edit | Regenerate")]
    for i,(k,v) in enumerate(fields):
        y=365+i*90; d.text((430,y),k,font=F["h3"],fill=rgba(COL["muted"])); d.text((800,y),v,font=F["h3" if i<3 else "body"],fill=rgba(COL["white"] if i!=2 else COL["red"]))
    for i,(txt,col) in enumerate([("Approve",COL["green"]),("Reject",COL["red"]),("Edit",COL["blue"]),("Regenerate",COL["amber"])]): badge(d,(460+i*260,760),txt,col)
    return im


def audit_report():
    im=bg(accent=COL["green"]); d=ImageDraw.Draw(im,"RGBA")
    title(d,"Audit Report","Every decision can be exported as an audit trail.",86,58,COL["green"])
    panel(d,(310,220,1610,910),"Markdown export",COL["teal"])
    lines=["# Audit Report","Request ID: REQ-1024","Endpoint: Blackbox Grok Code","Profile: coding","Attempts: 2","","Attempt 1: DISAPPROVED","Reason: syntax error in Python code","Repair: require valid quoted string","","Attempt 2: APPROVED","Final response returned"]
    y=330
    for line in lines:
        col=COL["red"] if "DISAPPROVED" in line else COL["green"] if "APPROVED" in line else COL["white"]
        d.text((390,y),line,font=F["mono"],fill=rgba(col)); y+=45
    return im


def cheatsheet():
    im=bg(accent=COL["teal"]); d=ImageDraw.Draw(im,"RGBA")
    title(d,"Command Cheatsheet","From install to endpoint protection in a few commands.",86,58,COL["teal"])
    lines=[("# Install",COL["amber"]),("curl -fsSL https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.sh | bash",COL["white"]),("",COL["white"]),("# Setup",COL["amber"]),("ollama pull gemma4:e2b",COL["teal"]),("the-witness setup",COL["teal"]),("the-witness doctor",COL["teal"]),("the-witness start",COL["teal"]),("",COL["white"]),("# Add endpoint",COL["amber"]),("export BLACKBOX_API_KEY=\"...\"",COL["muted"]),("the-witness endpoint add-blackbox",COL["teal"]),("",COL["white"]),("# Download fine-tuned model",COL["amber"]),("the-witness model download --source huggingface --model witness-gemma4-e2b-judge",COL["teal"])]
    terminal(d,(170,220,1750,900),"commands@the-witness",lines,COL["teal"])
    return im


def badge_wall():
    im=bg(accent=COL["purple"]); d=ImageDraw.Draw(im,"RGBA")
    title(d,"Gemma 4 Good Hackathon Track Fit","The Witness is designed around trust, local deployment, and real-world AI reliability.",86,58,COL["purple"])
    panel(d,(120,230,1800,880),None,COL["purple"])
    d.text((190,300),"Main Track",font=F["h2"],fill=rgba(COL["white"])); badge(d,(190,380),"Exceptional vision + real technical execution",COL["teal"])
    d.text((190,505),"Impact",font=F["h2"],fill=rgba(COL["white"]));
    for i,txt in enumerate(["Safety & Trust","Digital Equity & Inclusivity","Future of Education","Health & Sciences","Global Resilience"]): badge(d,(190+(i%3)*430,585+(i//3)*80),txt,[COL["teal"],COL["green"],COL["blue"],COL["amber"],COL["purple"]][i])
    d.text((1080,300),"Technology",font=F["h2"],fill=rgba(COL["white"]));
    for i,txt in enumerate(["Ollama","llama.cpp","LiteRT","Unsloth","Cactus-ready architecture"]): badge(d,(1080,380+i*72),txt,[COL["teal"],COL["blue"],COL["amber"],COL["purple"],COL["green"]][i])
    d.text((190,800),"Primary selected track: Safety & Trust",font=F["h3"],fill=rgba(COL["green"]))
    return im


def social_launch(size=(SQ,SQ)):
    im=bg(size[0],size[1],COL["teal"]); d=ImageDraw.Draw(im,"RGBA"); cx=size[0]//2
    d.ellipse((cx-190,240,cx+190,620),outline=rgba(COL["teal"],230),width=9); d.ellipse((cx-70,360,cx+70,500),fill=rgba(COL["teal"],65),outline=rgba(COL["teal"],235),width=5)
    d.text((cx,760),"The Witness",font=F["hero"],fill=rgba(COL["white"]),anchor="mm")
    d.text((cx,900),"Do not just trust AI.\nLet The Witness see it first.",font=F["h2"],fill=rgba(COL["teal"]),anchor="mm",align="center",spacing=10)
    return im


def social_before_after(size=(SQ,SQ)):
    im=bg(size[0],size[1],COL["blue"]); d=ImageDraw.Draw(im,"RGBA"); w,h=size
    d.text((w//2,125),"Before / After The Witness",font=F["h2"],fill=rgba(COL["white"]),anchor="mm")
    panel(d,(110,260,w//2-35,1260),"Before",COL["red"]); d.text((165,500),"AI response\ngoes straight\nto the app.",font=F["h2"],fill=rgba(COL["white"]),spacing=16)
    panel(d,(w//2+35,260,w-110,1260),"After",COL["green"]); d.text((w//2+90,470),"Gemma 4 verifies,\nrejects, repairs,\nand approves.",font=F["h3"],fill=rgba(COL["white"]),spacing=14)
    return im


def social_tracks(size=(SQ,SQ)):
    im=bg(size[0],size[1],COL["purple"]); d=ImageDraw.Draw(im,"RGBA"); w,h=size; cx=w//2
    d.text((cx,150),"Ollama. llama.cpp.\nLiteRT. Unsloth.",font=F["h1"],fill=rgba(COL["white"]),anchor="mm",align="center",spacing=10)
    d.text((cx,310),"One local AI trust layer.",font=F["h2"],fill=rgba(COL["teal"]),anchor="mm")
    cards=[("Ollama",280,500,COL["teal"]),("llama.cpp",920,500,COL["blue"]),("LiteRT",280,880,COL["amber"]),("Unsloth",920,880,COL["purple"])]
    for name,x,y,col in cards:
        panel(d,(x,y,x+400,y+250),None,col,(6,18,25,235)); d.text((x+200,y+105),name,font=F["h2"],fill=rgba(COL["white"]),anchor="mm")
    return im


def logo_imgs():
    for name,dark in [("logo_witness_dark.png",True),("logo_witness_light.png",False)]:
        im=Image.new("RGBA",(1400,500),(3,7,10,255) if dark else (246,250,248,255)); d=ImageDraw.Draw(im,"RGBA")
        fg=COL["teal"] if dark else (0,120,115); text=COL["white"] if dark else (3,20,22)
        d.rounded_rectangle((80,100,430,400),radius=40,outline=rgba(fg,230),width=8)
        d.ellipse((150,165,360,335),outline=rgba(fg,235),width=8); d.ellipse((225,220,285,280),fill=rgba(fg,120),outline=rgba(fg,255),width=5)
        arrow(d,(430,250),(520,250),fg,6)
        d.text((580,180),"The Witness",font=F["hero"],fill=rgba(text)); d.text((585,300),"Local AI verification before AI action",font=F["h3"],fill=rgba(fg))
        save(im,name)
    im=Image.new("RGBA",(512,512),(3,7,10,255)); d=ImageDraw.Draw(im,"RGBA")
    d.rounded_rectangle((64,88,448,424),radius=58,outline=rgba(COL["teal"],235),width=10); d.ellipse((130,170,382,342),outline=rgba(COL["teal"],235),width=10); d.ellipse((220,230,292,302),fill=rgba(COL["teal"],105),outline=rgba(COL["teal"],255),width=6)
    save(im,"logo_witness_icon.png")


def main():
    ROOT.mkdir(parents=True, exist_ok=True); SRC.mkdir(parents=True, exist_ok=True)
    assets=[
        ("01_cover_the_witness.png",cover()),("02_architecture_diagram.png",architecture()),("03_approval_loop.png",approval_loop()),("04_tui_dashboard.png",tui_dashboard()),("05_endpoint_watchlist.png",endpoint_watchlist()),("06_model_manager_tracks.png",model_manager()),("07_impact_track_map.png",impact_map()),("08_huggingface_model.png",hf_model()),("09_colab_notebook.png",colab()),("10_blackbox_test.png",blackbox_test()),("11_human_review.png",human_review()),("12_audit_report.png",audit_report()),("13_command_cheatsheet.png",cheatsheet()),("14_track_badge_wall.png",badge_wall()),("social_01_launch.png",social_launch()),("social_01_launch_4x5.png",social_launch(P45)),("social_02_before_after.png",social_before_after()),("social_02_before_after_4x5.png",social_before_after(P45)),("social_03_tracks.png",social_tracks()),("social_03_tracks_4x5.png",social_tracks(P45)),
    ]
    for name,im in assets: save(im,name)
    logo_imgs()

if __name__ == "__main__": main()
