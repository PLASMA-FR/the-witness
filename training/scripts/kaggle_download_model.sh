#!/usr/bin/env bash
set -euo pipefail
KEY="${1:-witness_gemma4_e2b_judge}"
if ! command -v kaggle >/dev/null 2>&1; then echo "kaggle CLI missing: python -m pip install kaggle" >&2; exit 1; fi
if [ ! -f "$HOME/.kaggle/kaggle.json" ] && { [ -z "${KAGGLE_USERNAME:-}" ] || [ -z "${KAGGLE_KEY:-}" ]; }; then echo "Kaggle credentials missing." >&2; exit 1; fi
case "$KEY" in
  witness_gemma4_e2b_judge|witness-gemma4-e2b-judge) SLUG="${KAGGLE_MODEL_SLUG:-your-kaggle-username/witness-gemma4-e2b-judge}"; DEST="models/witness-gemma4-e2b-judge";;
  *) echo "unknown model key $KEY" >&2; exit 1;;
esac
mkdir -p "$DEST"
kaggle models instances versions download "$SLUG" --untar -p "$DEST" || kaggle datasets download -d "$SLUG" --unzip -p "$DEST"
if ! find "$DEST" -maxdepth 2 \( -name 'adapter_config.json' -o -name 'config.json' -o -name 'tokenizer.json' -o -name '*.safetensors' \) | grep -q .; then echo "Downloaded but no recognizable model files in $DEST" >&2; exit 1; fi
echo "Downloaded $SLUG to $DEST"
