#!/usr/bin/env bash
set -euo pipefail
MODEL_PATH="${1:-./models/gemma4-e2b.gguf}"
PORT="${PORT:-8080}"
BIN="${LLAMA_SERVER_BIN:-llama-server}"
if ! command -v "$BIN" >/dev/null 2>&1; then
  if [ -x "$HOME/.local/bin/llama-server" ]; then BIN="$HOME/.local/bin/llama-server"; else
    echo "llama-server not found. Build llama.cpp or set LLAMA_SERVER_BIN=/path/to/llama-server." >&2
    echo "Place a Gemma GGUF at $MODEL_PATH, then rerun." >&2
    exit 1
  fi
fi
if [ ! -f "$MODEL_PATH" ]; then
  echo "Model not found: $MODEL_PATH" >&2
  echo "Download/place a Gemma 4 GGUF model there, or pass a path: $0 /path/model.gguf" >&2
  exit 1
fi
echo "Starting llama.cpp OpenAI-compatible server at http://127.0.0.1:$PORT/v1"
exec "$BIN" -m "$MODEL_PATH" --host 127.0.0.1 --port "$PORT" -c "${CTX_SIZE:-4096}"
