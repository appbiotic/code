#!/usr/bin/env bash

set -eou pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
REPO_DIR="$SCRIPT_DIR"

FEATURES="$(jq -r '."rust-analyzer.cargo.features" | join(",")' "$REPO_DIR/.vscode/settings.json")"

cargo fmt --all -- --check
cargo check --features "$FEATURES"
cargo clippy --features "$FEATURES" -- -D warnings
cargo test --all --features "$FEATURES"
