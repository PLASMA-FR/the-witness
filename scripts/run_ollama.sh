#!/usr/bin/env bash
set -euo pipefail
MODEL="${1:-gemma4:e2b}"
if ! command -v ollama >/dev/null 2>&1; then
  echo "Ollama is not installed. Install Ollama first, then run: ollama pull $MODEL" >&2
  exit 1
fi
if ! curl -fsS http://localhost:11434/api/tags >/dev/null 2>&1; then
  echo "Ollama does not appear to be running. Start Ollama, then run: ollama pull $MODEL" >&2
  exit 1
fi
ollama pull "$MODEL"
