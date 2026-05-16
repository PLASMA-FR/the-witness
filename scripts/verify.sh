#!/usr/bin/env bash
set -euo pipefail

pass() { printf '\033[1;32m[PASS]\033[0m %s\n' "$*"; }
fail() { printf '\033[1;31m[FAIL]\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33m[WARN]\033[0m %s\n' "$*"; }

run_required() {
  local label="$1"; shift
  printf '\n==> %s\n' "$label"
  if "$@"; then
    pass "$label"
  else
    fail "$label"
    return 1
  fi
}

run_optional_doctor() {
  printf '\n==> the-witness doctor\n'
  set +e
  ./target/debug/the-witness doctor
  local code=$?
  set -e
  if [ "$code" -eq 0 ]; then
    pass "doctor readiness passed"
  else
    warn "doctor ran but readiness is incomplete. This is expected on unconfigured machines without Ollama models, Kaggle credentials, or endpoint secrets."
  fi
}

cd "$(dirname "$0")/.."

run_required "cargo fmt --check" cargo fmt --check
run_required "cargo test" cargo test
run_required "cargo build" cargo build
run_required "dataset validation" python3 training/scripts/validate_dataset.py
run_required "model list" ./target/debug/the-witness model list
run_required "demo judge model test" ./target/debug/the-witness model test --backend demo --model demo-judge
run_optional_doctor

printf '\nVerification complete. Required checks passed; doctor readiness may require local setup.\n'
