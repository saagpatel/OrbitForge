# OrbitForge Verification Snapshot

## Canonical Verification Entry Point

- `bash .codex/scripts/run_verify_commands.sh`

## Verification Contract

- Commands are defined in `.codex/verify.commands`.
- Required checks must be truthful and reproducible.
- `not-run` is treated as a blocking state for required gates.

## Recommended Required Checks (Branch Protection / Merge Queue)

- `git-hygiene / commitlint`
- `git-hygiene / pr-title`
- `git-hygiene / branch-name`
- `git-hygiene / secrets`
- `quality / lint-test-build`
- `perf-enforced / perf-bundle` (production profile)
- `perf-enforced / perf-build` (production profile)
- `perf-enforced / perf-assets` (production profile)
- `perf-enforced / perf-memory` (production profile)

See `docs/execution/BRANCH_GOVERNANCE.md` for the full branch-protection and merge-queue mapping.

## Evidence Policy

- Store machine-readable perf outputs in `.perf-results/`.
- Keep release readiness conclusions tied to command outcomes.

## Latest Local Result

- Canonical command `bash .codex/scripts/run_verify_commands.sh` passed.
- Validation was executed from canonical path clone: `/Users/d/Projects/FunGamePrjs/OrbitForge`.
- Required local gates now include Rust backend tests (`pnpm test:rust`).
