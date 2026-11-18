#!/bin/sh
# Runs every check: Rust format, lint and tests; UI type check and tests with coverage.
set -eu

PATH="$HOME/.cargo/bin:$PATH"
export PATH
cd "$(dirname "$0")/.."

log() {
    printf '%s %s\n' "$(date '+%Y-%m-%d %H:%M:%S')" "$*"
}

step() {
    log "==> $*"
    "$@"
}

step cargo fmt --all --check
step cargo clippy --workspace --all-targets -- -D warnings
step cargo test --workspace
step npm run check
step npm test
log "All checks passed."
