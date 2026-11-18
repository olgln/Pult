#!/bin/sh
# Rust line coverage for the whole workspace; fails below the threshold.
# src-tauri/src/main.rs is excluded: it only starts the window and cannot run in a test.
set -eu

PATH="$HOME/.cargo/bin:$PATH"
export PATH
cd "$(dirname "$0")/.."

MIN_LINES=100

log() {
    printf '%s %s\n' "$(date '+%Y-%m-%d %H:%M:%S')" "$*"
}

log "==> cargo llvm-cov (minimum ${MIN_LINES}% lines)"
cargo llvm-cov --workspace --summary-only \
    --ignore-filename-regex 'src-tauri/src/main\.rs$' \
    --fail-under-lines "$MIN_LINES"
log "Coverage passed."
