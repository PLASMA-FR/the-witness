#!/usr/bin/env bash
set -euo pipefail
cargo run -- setup
cargo run -- endpoint list
