# OrbitForge Branch Governance

This document maps repository checks to branch protection and merge queue policy.

## Target Branches

- `main`
- `master` (if still used)

## Required Status Checks

- `git-hygiene / commitlint`
- `git-hygiene / pr-title`
- `git-hygiene / branch-name`
- `git-hygiene / secrets`
- `quality / lint-test-build`

When `PERF_PROFILE=production`, also require:

- `perf-enforced / perf-bundle`
- `perf-enforced / perf-build`
- `perf-enforced / perf-assets`
- `perf-enforced / perf-memory`

## Merge Queue Policy

- Enable merge queue for protected branches.
- Require all required checks to pass in queue context before merge.
- Do not allow bypass by stale green checks from older head SHA.

## Local Gate Equivalence

Use the canonical local gate command before pushing:

```bash
bash .codex/scripts/run_verify_commands.sh
```

This command list is defined in `.codex/verify.commands` and should remain aligned with required checks.
