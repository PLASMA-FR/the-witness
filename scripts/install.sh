#!/usr/bin/env bash
set -euo pipefail

WITNESS_REPO_URL="${WITNESS_REPO_URL:-https://github.com/PLASMA-FR/the-witness.git}"
WITNESS_INSTALL_DIR="${WITNESS_INSTALL_DIR:-$HOME/.local/bin}"
WITNESS_CONFIG_DIR="${WITNESS_CONFIG_DIR:-$HOME/.config/the-witness}"
WITNESS_DEFAULT_BACKEND="${WITNESS_DEFAULT_BACKEND:-ollama}"
WITNESS_DEFAULT_MODEL="${WITNESS_DEFAULT_MODEL:-gemma4:e2b}"
WITNESS_STRONG_MODEL="${WITNESS_STRONG_MODEL:-gemma4:e4b}"
WITNESS_FALLBACK="${WITNESS_FALLBACK:-human_review}"
WITNESS_SKIP_OLLAMA_PULL="${WITNESS_SKIP_OLLAMA_PULL:-0}"

usage() {
  cat <<'EOF'
The Witness installer

Usage:
  bash scripts/install.sh [--help]

Environment variables:
  WITNESS_INSTALL_DIR       Binary install directory (default: $HOME/.local/bin)
  WITNESS_CONFIG_DIR        Config directory (default: $HOME/.config/the-witness)
  WITNESS_REPO_URL          Git URL used when not run from a checkout
  WITNESS_DEFAULT_MODEL     Default Ollama model hint (default: gemma4:e2b)
  WITNESS_STRONG_MODEL      Optional stronger model hint (default: gemma4:e4b)
  WITNESS_SKIP_OLLAMA_PULL  Set to 1 to never prompt for ollama pull

The installer builds the local Rust binary and prints model pull commands.
It does not require API keys and will not download Ollama models unless run
interactively and the user explicitly confirms the prompt.
EOF
}

case "${1:-}" in
  -h|--help)
    usage
    exit 0
    ;;
  "") ;;
  *)
    printf '\033[1;31m[error]\033[0m Unknown argument: %s\n' "$1" >&2
    usage >&2
    exit 2
    ;;
esac

info() { printf '\033[1;34m[info]\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33m[warn]\033[0m %s\n' "$*"; }
err() { printf '\033[1;31m[error]\033[0m %s\n' "$*" >&2; }
have() { command -v "$1" >/dev/null 2>&1; }

require_tool() {
  local tool="$1"
  if ! have "$tool"; then
    err "Required tool missing: $tool"
    return 1
  fi
}

missing=0
for tool in git curl; do
  if ! require_tool "$tool"; then missing=1; fi
done

if ! have cargo; then
  err "Rust/cargo is missing. Install Rust with rustup, then rerun this installer:"
  printf '  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh\n' >&2
  printf '  source "$HOME/.cargo/env"\n' >&2
  missing=1
fi

if [ "$missing" -ne 0 ]; then
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" 2>/dev/null && pwd || pwd)"
CURRENT_DIR="$(pwd)"
WORK_DIR=""

if [ -f "$CURRENT_DIR/Cargo.toml" ] && grep -q 'name = "the-witness"' "$CURRENT_DIR/Cargo.toml"; then
  WORK_DIR="$CURRENT_DIR"
  info "Running inside existing The Witness checkout: $WORK_DIR"
elif [ -f "$SCRIPT_DIR/../Cargo.toml" ] && grep -q 'name = "the-witness"' "$SCRIPT_DIR/../Cargo.toml"; then
  WORK_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
  info "Running from repository script directory: $WORK_DIR"
else
  TMP_ROOT="${TMPDIR:-/tmp}/the-witness-install"
  rm -rf "$TMP_ROOT"
  info "Cloning The Witness from $WITNESS_REPO_URL"
  git clone --depth 1 "$WITNESS_REPO_URL" "$TMP_ROOT"
  WORK_DIR="$TMP_ROOT"
fi

cd "$WORK_DIR"

info "Building The Witness release binary"
cargo build --release

mkdir -p "$WITNESS_INSTALL_DIR" "$WITNESS_CONFIG_DIR"
install -m 0755 target/release/the-witness "$WITNESS_INSTALL_DIR/the-witness"
info "Installed binary: $WITNESS_INSTALL_DIR/the-witness"

if [ -f witness.toml ] && [ ! -f "$WITNESS_CONFIG_DIR/witness.toml" ]; then
  cp witness.toml "$WITNESS_CONFIG_DIR/witness.toml"
  info "Copied default config to $WITNESS_CONFIG_DIR/witness.toml"
elif [ -f "$WITNESS_CONFIG_DIR/witness.toml" ]; then
  info "Config already exists: $WITNESS_CONFIG_DIR/witness.toml"
else
  warn "Default witness.toml not found; run the-witness setup to create config."
fi

case ":$PATH:" in
  *":$WITNESS_INSTALL_DIR:"*) ;;
  *)
    warn "$WITNESS_INSTALL_DIR is not in PATH. Add this to your shell profile:"
    printf '  export PATH="%s:$PATH"\n' "$WITNESS_INSTALL_DIR"
    ;;
esac

THE_WITNESS="$WITNESS_INSTALL_DIR/the-witness"

info "Configured defaults"
printf '  backend: %s\n' "$WITNESS_DEFAULT_BACKEND"
printf '  default model: %s\n' "$WITNESS_DEFAULT_MODEL"
printf '  strong/high-risk model: %s\n' "$WITNESS_STRONG_MODEL"
printf '  fallback: %s\n' "$WITNESS_FALLBACK"

if have ollama; then
  info "Ollama detected"
  if ollama list 2>/dev/null | awk '{print $1}' | grep -qx "$WITNESS_DEFAULT_MODEL"; then
    info "$WITNESS_DEFAULT_MODEL is already available in Ollama"
  else
    warn "$WITNESS_DEFAULT_MODEL is not available in Ollama. Pull it with:"
    printf '  ollama pull %s\n' "$WITNESS_DEFAULT_MODEL"
    if [ "$WITNESS_SKIP_OLLAMA_PULL" != "1" ] && [ -t 0 ]; then
      printf 'Pull %s now? [y/N] ' "$WITNESS_DEFAULT_MODEL"
      read -r answer || answer=""
      case "$answer" in
        y|Y|yes|YES) ollama pull "$WITNESS_DEFAULT_MODEL" ;;
        *) warn "Skipping Ollama pull" ;;
      esac
    fi
  fi
  warn "Optional stronger/high-risk model: ollama pull $WITNESS_STRONG_MODEL"
else
  warn "Ollama not detected. Install Ollama if you want the default local judge, then run:"
  printf '  ollama pull %s\n' "$WITNESS_DEFAULT_MODEL"
  printf '  ollama pull %s  # optional stronger/high-risk model\n' "$WITNESS_STRONG_MODEL"
fi

info "Running doctor if possible"
if "$THE_WITNESS" doctor; then
  info "Doctor passed readiness checks"
else
  warn "Doctor reported incomplete readiness. This is expected until Ollama/models/Hugging Face adapter/endpoint setup are configured."
fi

cat <<EOF

The Witness install complete.

Next steps:
  the-witness setup
  the-witness doctor
  the-witness start

Optional Blackbox endpoint test:
  export BLACKBOX_API_KEY="YOUR_KEY_HERE"
  the-witness endpoint add-blackbox

Optional local models:
  ollama pull $WITNESS_DEFAULT_MODEL
  ollama pull $WITNESS_STRONG_MODEL

EOF
