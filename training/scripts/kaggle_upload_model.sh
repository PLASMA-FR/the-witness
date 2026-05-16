#!/usr/bin/env bash
set -euo pipefail
MODEL_DIR="${1:?usage: kaggle_upload_model.sh ./training/outputs/witness-gemma4-e2b-judge [witness-gemma4-e2b-judge]}"
MODEL_NAME="${2:-witness-gemma4-e2b-judge}"
SLUG="${KAGGLE_MODEL_SLUG:-your-kaggle-username/witness-gemma4-e2b-judge}"
if ! command -v kaggle >/dev/null 2>&1; then echo "kaggle CLI missing: python -m pip install kaggle" >&2; exit 1; fi
if [ ! -f "$HOME/.kaggle/kaggle.json" ] && { [ -z "${KAGGLE_USERNAME:-}" ] || [ -z "${KAGGLE_KEY:-}" ]; }; then echo "Kaggle credentials missing. Configure Kaggle CLI locally; never commit tokens." >&2; exit 1; fi
if [ ! -d "$MODEL_DIR" ]; then echo "Model dir missing: $MODEL_DIR" >&2; exit 1; fi
cat > "$MODEL_DIR/dataset-metadata.json" <<JSON
{"title":"$MODEL_NAME","id":"$SLUG","licenses":[{"name":"CC0-1.0"}]}
JSON
if kaggle models instances versions create "$MODEL_DIR" -m "Upload $MODEL_NAME"; then :; else
  echo "Kaggle model upload failed or unsupported; trying dataset create/version." >&2
  kaggle datasets create -p "$MODEL_DIR" || kaggle datasets version -p "$MODEL_DIR" -m "Update $MODEL_NAME"
fi
mkdir -p models
cat > models/remote_models.toml <<TOML
[remote_models.witness_gemma4_e2b_judge]
name = "Fine-tuned Witness Gemma 4 E2B Judge"
backend = "unsloth"
source = "kaggle"
slug = "$SLUG"
local_dir = "./models/witness-gemma4-e2b-judge"
TOML
echo "$SLUG"
