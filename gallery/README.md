# The Witness gallery

High-resolution gallery assets for the Kaggle Gemma 4 Good Hackathon page, GitHub README, demo video, social posts, and launch materials.

These images use a consistent dark terminal-native visual identity: near-black backgrounds, electric teal, soft white, muted green, warning red, amber human-review accents, and blue/purple network flow highlights.

## Assets

| File | Shows | Best use | Recommended caption |
|---|---|---|---|
| `01_cover_the_witness.png` | Hero dashboard with AI app, Witness proxy, upstream endpoint, and Gemma verdict panel | Kaggle cover, GitHub hero, video opener | The Witness watches AI endpoints and verifies every response before it reaches the user. |
| `02_architecture_diagram.png` | End-to-end proxy, judge, verdict, repair, review, and audit flow | Kaggle explanation, README architecture section | The Witness sits between AI apps and model endpoints, verifying every response before it reaches the user. |
| `03_approval_loop.png` | Request → candidate → Gemma judge → repair → retry → approved loop | Core innovation slide | Bad answers are blocked, repaired, retried, and only released after approval. |
| `04_tui_dashboard.png` | Main TUI dashboard mockup with endpoints, stream, and verdict panel | README product screenshot | The TUI makes invisible AI traffic visible, explainable, and controllable. |
| `05_endpoint_watchlist.png` | Endpoint manager for Blackbox Grok Code | Feature gallery | Add an endpoint once. The Witness watches every request that flows through it. |
| `06_model_manager_tracks.png` | Model manager and technology tracks | Technical track slide | One trust layer, multiple Gemma deployment paths. |
| `07_impact_track_map.png` | Safety, equity, education, health/science, and resilience impact map | Impact section | The Witness is built for places where AI reliability matters most. |
| `08_huggingface_model.png` | Fine-tuned Witness Gemma 4 E2B judge model card | Unsloth/Hugging Face section | The fine-tuned judge is trained to classify AI responses as approved, rejected, or needing human review. |
| `09_colab_notebook.png` | Colab fine-tuning workflow | Reproducibility section | The public Colab notebook makes the fine-tuning path reproducible. |
| `10_blackbox_test.png` | Blackbox endpoint test with curl and TUI stream | Live demo proof | The Witness can sit in front of any OpenAI-compatible endpoint. |
| `11_human_review.png` | Human review queue for high-risk response | Safety section | High-risk responses can be paused before they reach users. |
| `12_audit_report.png` | Markdown-style audit report | Explainability section | Every decision can be exported as an audit trail. |
| `13_command_cheatsheet.png` | Install, setup, endpoint, and model commands | README quick-start image | From install to endpoint protection in a few commands. |
| `14_track_badge_wall.png` | Hackathon track fit and primary Safety & Trust positioning | Kaggle track section | The Witness is designed around trust, local deployment, and real-world AI reliability. |
| `social_01_launch.png` | Square launch card | Social post | Do not just trust AI. Let The Witness see it first. |
| `social_01_launch_4x5.png` | 4:5 launch card | Reels/LinkedIn/Twitter crop | Do not just trust AI. Let The Witness see it first. |
| `social_02_before_after.png` | Square before/after trust-layer explanation | Social post | Before: AI answers go straight to apps. After: Gemma verifies, repairs, and approves. |
| `social_02_before_after_4x5.png` | 4:5 before/after variant | Social post | Before: AI answers go straight to apps. After: Gemma verifies, repairs, and approves. |
| `social_03_tracks.png` | Square technology-track card | Social post | Ollama. llama.cpp. LiteRT. Unsloth. One local AI trust layer. |
| `social_03_tracks_4x5.png` | 4:5 technology-track card | Social post | Ollama. llama.cpp. LiteRT. Unsloth. One local AI trust layer. |
| `logo_witness_dark.png` | Dark logo lockup | README/project page | The Witness logo for dark backgrounds. |
| `logo_witness_light.png` | Light logo lockup | Docs or slides | The Witness logo for light backgrounds. |
| `logo_witness_icon.png` | Square icon | Avatar/favicon/social thumbnail | Terminal eye icon for The Witness. |
| `logo_witness_ascii.txt` | Terminal ASCII logo | CLI README or terminal splash | ASCII logo for terminal-native docs. |

## Source files

Editable source files live in `gallery/source/`:

- `render_gallery.py` — deterministic Python/Pillow renderer for all PNGs.
- `design_notes.md` — brand and asset notes.
- `architecture_diagram.svg` — editable architecture SVG.
- `approval_loop.svg` — editable approval-loop SVG.
- `logo.svg` — editable logo SVG.
- `social_templates.svg` — editable social template SVG.

## Regenerate

From the repository root:

```bash
python3 gallery/source/render_gallery.py
```

Requirements:

- Python 3
- Pillow
- DejaVu fonts under `/usr/share/fonts`

## Notes

- No real API keys are shown.
- No copyrighted logos are embedded.
- The images are product mockups and technical diagrams, not claims of perfect safety.
