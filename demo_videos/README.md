# The Witness demo video clips

Five short clips for the Kaggle Gemma 4 Good Hackathon pitch video.

These are not plain screen recordings. They are rendered product-demo clips that make the terminal feel alive: typed commands, TUI panels, glowing status cards, request flows, verdict JSON, retry loops, model-track cards, and audit timelines.

## Files

| # | Clip | Duration | Video | Thumbnail |
|---:|---|---:|---|---|
| 1 | The Endpoint Awakens | 24s | `video_01_endpoint_awakens.mp4` | `video_01_thumbnail.png` |
| 2 | Watching the Blackbox Endpoint | 26s | `video_02_endpoint_watch.mp4` | `video_02_thumbnail.png` |
| 3 | Rejected, Repaired, Approved | 29s | `video_03_repair_loop.mp4` | `video_03_thumbnail.png` |
| 4 | Model Manager: Four Tech Tracks | 28s | `video_04_model_manager_tracks.mp4` | `video_04_thumbnail.png` |
| 5 | Audit Trail for Trust | 29s | `video_05_audit_impact.mp4` | `video_05_thumbnail.png` |

## Recommended order in the 3-minute video

1. Use Video 5 first for the human problem and trust mood.
2. Use Video 1 for setup and first-run readiness.
3. Use Video 2 for endpoint watching.
4. Use Video 3 for the main wow moment: reject, repair, retry, approve.
5. Use Video 4 for Gemma 4 backend and technology-track credibility.
6. Return to Video 5 for the ending line about local trust.

## Clip guide

### 1. The Endpoint Awakens

Feature shown: first-run setup wizard, Ollama backend, Gemma 4 E2B/E4B model choices, doctor checks, and dashboard launch.

Design style: cinematic dark terminal with electric teal glow. Mission-control energy, but calm enough to feel trustworthy.

Suggested narration: "The Witness starts by proving the local Gemma 4 judge is ready before it watches anything."

Best placement: after the opening problem statement, when the viewer needs to understand how the product starts.

### 2. Watching the Blackbox Endpoint

Feature shown: Blackbox endpoint setup, local proxy route, OpenAI-compatible request flow, secret redaction, and live request stream.

Design style: cyberpunk network monitor with deep black, neon purple, blue, and magenta.

Suggested narration: "Point your AI app at localhost, and every request becomes visible before the answer reaches the user."

Best placement: after setup, to show how The Witness sits between an app and an upstream AI endpoint.

### 3. Rejected, Repaired, Approved

Feature shown: Gemma 4 verdict, disapproval, rejection reason, prompt repair, retry loop, and approved final response.

Design style: dramatic red-to-green transformation with verdict cards and a retry conveyor.

Suggested narration: "The Witness does not just detect a bad answer. It repairs the path to a better one."

Best placement: center of the hackathon video. This is the wow clip.

### 4. Model Manager: Four Tech Tracks

Feature shown: model manager, Ollama, llama.cpp, LiteRT, Unsloth, Hugging Face model download, Colab notebook, and track map.

Design style: clean futuristic product UI with navy, silver, white, and subtle gradients.

Suggested narration: "The same verification idea runs across multiple Gemma deployment paths, from Ollama to a fine-tuned Unsloth adapter."

Best placement: after the wow clip, when judges need technical proof and track coverage.

### 5. Audit Trail for Trust

Feature shown: audit logs, human review queue, privacy mode, export report, and impact tracks.

Design style: documentary dashboard with warm white, muted green, soft black, and gold accents.

Suggested narration: "In the places where AI mistakes matter most, trust must be local and inspectable."

Best placement: use it as both the opening mood and closing impact shot.

## Regenerate the clips

From the repository root:

```bash
cd /home/admin/Gemma/witness
python3 demo_videos/render_videos.py
```

Requirements:

- Python 3
- Pillow
- ffmpeg
- DejaVu/Noto fonts installed under `/usr/share/fonts`

The render script writes MP4 files and PNG thumbnails into this directory.

## Notes for editors

- All clips are 1920x1080 MP4 files at 18 fps.
- The clips do not contain audio. Add subtle terminal typing, soft hits, low drones, or whooshes in the final editor if desired.
- API keys are never shown. The Blackbox key appears only as `$BLACKBOX_API_KEY` or bullets.
- The commands are intentionally realistic, but the clips are animated product demos rather than live screen recordings.
- Keep terminal text on screen long enough to read. If the final edit is fast, punch in on the terminal areas or add a voiceover.
