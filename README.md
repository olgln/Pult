# Pult

A control panel for Claude Code sessions on macOS.

One window that lists every Claude Code session on the machine, grouped by project, with a
live status for each — working, needs you, done, error, not running — and a real terminal
for the selected one. Sessions live in tmux, so they keep running when the window is
closed, and the same session can be attached from any other terminal.

**Status:** early development, nothing to run yet.

## Principles

- Claude Code is used only as the unmodified `claude` binary inside a terminal. Pult never
  reads credentials or tokens, never calls the Anthropic API and does not use the Agent SDK.
- No network: no telemetry, no auto-updates, no remote fonts or scripts.
- Never a second process for a session that is already running.
- Sessions that Pult did not start can be viewed and attached, never managed.

## Stack

Tauri 2, Rust, Svelte 5, xterm.js, tmux.

## Licence

MIT — see `LICENSE`.
