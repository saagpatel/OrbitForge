# OrbitForge Automation Cadence

This project uses low-noise recurring summaries.

## Daily Gate Digest

- Purpose: summarize required gate health for active branch work.
- Output: pass/fail/not-run by gate and a short blocker list.

## Nightly Perf Drift

- Purpose: detect bundle/build/memory drift against approved baselines.
- Output: metric deltas, trend direction, and threshold breaches.

## Weekly Release Readiness

- Purpose: track readiness to cut a release candidate.
- Output: open blockers, risk posture, and go/no-go recommendation.

## Format Rules

- Keep user-facing updates beginner-friendly and big-picture.
- Include technical receipts only when requested or when escalation is required.
