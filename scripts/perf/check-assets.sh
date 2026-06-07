#!/usr/bin/env bash
set -euo pipefail

# codex-os-managed
max_bytes="${ASSET_MAX_BYTES:-900000}"
results_dir=".perf-results"
mkdir -p "$results_dir"

asset_root=""
if [[ -d dist/assets ]]; then
  asset_root="dist/assets"
elif [[ -d public ]]; then
  asset_root="public"
fi

if [[ -z "$asset_root" ]]; then
  cat > "$results_dir/assets.json" <<EOF
{
  "status": "pass",
  "assetRoot": "none",
  "checkedFiles": 0,
  "maxBytes": $max_bytes,
  "capturedAt": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
}
EOF
  echo "No asset directory found; treating as pass."
  exit 0
fi

fail=0
checked=0
while IFS= read -r file; do
  checked=$((checked + 1))
  size=$(wc -c < "$file")
  if (( size > max_bytes )); then
    echo "Asset too large (>${max_bytes} bytes): $file"
    fail=1
  fi
done < <(find "$asset_root" -type f \( \
  -name "*.png" -o -name "*.jpg" -o -name "*.jpeg" -o -name "*.webp" -o -name "*.avif" -o \
  -name "*.svg" -o -name "*.gif" -o -name "*.ico" -o \
  -name "*.js" -o -name "*.mjs" -o -name "*.css" -o -name "*.wasm" \
\))

status="pass"
if (( fail != 0 )); then
  status="fail"
fi

cat > "$results_dir/assets.json" <<EOF
{
  "status": "$status",
  "assetRoot": "$asset_root",
  "checkedFiles": $checked,
  "maxBytes": $max_bytes,
  "capturedAt": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
}
EOF

exit $fail
