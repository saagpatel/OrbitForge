#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TARGET="${1:-/Users/d/Projects/FunGamePrjs/OrbitForge}"

if [[ "$ROOT" == "$TARGET" ]]; then
  echo "Source and target are the same path. Nothing to migrate."
  exit 0
fi

mkdir -p "$TARGET"

echo "Copying repository to canonical path..."
rsync -a \
  --exclude ".DS_Store" \
  --exclude "node_modules" \
  --exclude "dist" \
  --exclude ".perf-results" \
  --exclude "coverage" \
  --exclude "src-tauri/target" \
  "$ROOT/" "$TARGET/"

echo "Done."
echo "Next steps:"
echo "  cd '$TARGET'"
echo "  pnpm install"
echo "  bash scripts/dev/check-workspace-path.sh"
echo "  bash .codex/scripts/run_verify_commands.sh"
