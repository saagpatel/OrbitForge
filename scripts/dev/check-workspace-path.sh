#!/usr/bin/env bash
set -euo pipefail

# Warns when the workspace path contains ':' because several tools split path lists on ':'.
# Set STRICT_WORKSPACE_PATH=1 to enforce failure.
cwd="$(pwd)"
if [[ "$cwd" == *":"* ]]; then
  echo "Workspace path contains ':': $cwd"
  echo "Use a canonical clone path without ':' for reliable pnpm + Tauri behavior."
  if [[ "${STRICT_WORKSPACE_PATH:-0}" == "1" ]]; then
    exit 1
  fi
  exit 0
fi

echo "Workspace path is canonical: $cwd"
