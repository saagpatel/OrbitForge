# Contributing

## Repository Hygiene

Keep this repository lean and deterministic:

- Do not commit generated build output or local dependency installs.
- Do not commit editor metadata, OS artifacts, logs, or temporary files.
- Keep tracking limited to source, config, and required assets.

The current ignore policy in [.gitignore](.gitignore) covers common local clutter, including:

- dependency/build output (`node_modules/`, `dist/`, `src-tauri/target/`, `src-tauri/gen/`)
- OS/editor artifacts (`.DS_Store`, `.vscode/`, `.idea/`, swap files)
- local logs and temp files (`*.log`, `*.tmp`, debug logs)

## Empty Directory Strategy (`.gitkeep`)

Git does not track empty directories. If a directory must exist but is intentionally empty:

- add a `.gitkeep` file inside that directory
- keep only `.gitkeep` in that directory unless real tracked files are required
- remove `.gitkeep` once the directory contains meaningful tracked content

Use `.gitkeep` only for required structure (for example, runtime-created folders that must exist for scripts or tooling).

## Basic Verification Before Commit

Inspect `git status --short`, then follow [README verification](README.md#verification-commands)
for the pinned environment, focused and broader frontend/Rust checks, build,
and full guard/performance bundle. [package.json](package.json) and
[.codex/verify.commands](.codex/verify.commands) define the actual commands.
