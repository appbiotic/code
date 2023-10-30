#!/usr/bin/env bash

set -eou pipefail

cargo fmt --all -- --check
cargo check
cargo clippy -- -D warnings
cargo test --all
