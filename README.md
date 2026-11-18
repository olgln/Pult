# Pult

A control panel for Claude Code sessions on macOS.

One window that lists every Claude Code session on the machine, grouped by project, with a
live status for each — working, needs you, done, error, not running — and a real terminal
for the selected one. Sessions live in tmux, so they keep running when the window is
closed, and the same session can be attached from any other terminal.

**Status:** early development: an empty window, no session features yet.

## Principles

- Claude Code is used only as the unmodified `claude` binary inside a terminal. Pult never
  reads credentials or tokens, never calls the Anthropic API and does not use the Agent SDK.
- No network: no telemetry, no auto-updates, no remote fonts or scripts.
- Never a second process for a session that is already running.
- Sessions that Pult did not start can be viewed and attached, never managed.

## Stack

Tauri 2, Rust, Svelte 5, xterm.js, tmux.

## Architecture

```
crates/core/        pult-core: all session logic (models, and later index, registry, tmux)
crates/cli/         the `pult` command, a thin layer over pult-core
src-tauri/          the desktop app: window, plugins, invoke commands that call pult-core
src-tauri/icons/    app icons, generated from icon-source.svg with `tauri icon` (macOS sizes kept)
ui/src/lib/         UI logic: calls into Rust (api.ts) and pure helpers
ui/src/components/  UI views (Svelte components)
scripts/            check.sh (all checks) and coverage.sh (Rust coverage)
```

Rule: session logic (what sessions exist, their status, whether one may be started) lives
only in `crates/core`. The app and the CLI call it and never re-implement it. Tests sit
next to the code they cover (`#[cfg(test)]` modules, `*.test.ts` files).

## Development

Prerequisites: macOS, Xcode Command Line Tools, Rust via rustup (the toolchain is pinned in
`rust-toolchain.toml`), Node.js with npm, and for coverage
`cargo install --locked cargo-llvm-cov@0.9.1`.

```sh
npm ci                     # install UI dependencies (install scripts are disabled in .npmrc)
npm run tauri dev          # run the app with the Vite dev server on port 1420
npm run build              # build the UI into dist/
cargo build -p pult-app    # build the app binary
scripts/check.sh           # format, clippy, Rust tests, svelte-check, UI tests with coverage
scripts/coverage.sh        # Rust line coverage with a minimum threshold
```

## Licence

MIT — see `LICENSE`.
