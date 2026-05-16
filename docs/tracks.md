# Hackathon track mapping

## Ollama Track
The Witness uses Gemma 4 locally through Ollama as the default local judge backend. The setup wizard checks the `ollama` command, uses `ollama pull <model>`, calls `/api/chat`, validates JSON verdicts, and runs sanity tests.

## llama.cpp Track
The Witness supports Gemma 4 through llama.cpp for offline and resource-constrained hardware. `scripts/run_llamacpp.sh` starts a localhost OpenAI-compatible server; the judge calls `/v1/chat/completions` and doctor checks runtime/server readiness.

## LiteRT Track
The Witness includes LiteRT as an edge verifier/prefilter. It validates the LiteRT runtime/model path and provides a fast classification adapter that can approve/disapprove/escalate to the full judge or human review. The MVP adapter is experimental and clearly routed through the backend interface.

## Unsloth Track
The Witness includes real fine-tuning notebooks and scripts for Gemma 4 E2B/E4B Witness judge models. The TUI Model Manager lists fine-tuned models, the notebooks run on Google Colab TPU with GPU/Unsloth fallback, optional Kaggle scripts upload/download artifacts, and model tests verify JSON schema behavior.

## Impact Tracks
- Safety & Trust: every response is judged before reaching users.
- Digital Equity & Inclusivity: local/offline backends and Arabic-English profile.
- Future of Education: education profile and tutor endpoint templates.
- Health & Sciences: high-risk human review safeguards.
- Global Resilience: disaster-response profile and audit trail.
