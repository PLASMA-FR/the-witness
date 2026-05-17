# The Witness gallery design notes

The gallery uses a consistent dark terminal-native identity:

- near-black backgrounds
- electric teal for The Witness and approved states
- purple/blue for endpoint/network flow
- muted green for local-first/safety
- warning red for rejected output
- amber/gold for human review and reproducibility
- high-contrast monospace terminal text
- rounded TUI panels with restrained glow

The visuals are deterministic PNG renders created with `gallery/source/render_gallery.py`, plus editable SVG source sketches for the core logo and diagrams.

No official Google, Kaggle, Hugging Face, or provider logos are used. External services are represented with text only.

Secrets are intentionally redacted. The only API-key text used in gallery assets is `$BLACKBOX_API_KEY`, `BLACKBOX_API_KEY`, bullets, or asterisks.
