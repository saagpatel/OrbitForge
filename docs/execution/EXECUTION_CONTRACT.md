# OrbitForge Execution Contract

This contract defines how autonomous work runs in this repository.

## Scope
- Use `codex-execution-os` as the main orchestrator.
- Prefer stable workflows and tools. Use experimental behavior only when explicitly documented in a task.

## Batch Rules
- Default batch size: 1 to 5 tightly related tasks.
- Retry budget: up to 2 retries per failing task.
- Stop criteria:
  - Any destructive command would be required.
  - A required verification gate fails twice after remediation.
  - A release credential or signing prerequisite is missing.

## Escalation Conditions
Escalate to PM/user when:
- A decision changes product behavior or user-facing claims.
- A gate cannot be made truthful without policy input.
- A platform release requirement is unknown (for example notarization target).

## Deterministic Execution Rules
- Use canonical commands from `README.md`, `package.json`, `.codex/verify.commands`, and CI workflows.
- Keep verification deterministic with `.codex/scripts/run_verify_commands.sh`.
- Keep snapshots current:
  - `docs/execution/PLAN.md`
  - `docs/execution/CHECKPOINTS.md`
  - `docs/execution/DECISIONS.md`
  - `docs/execution/VERIFICATION.md`

## Workspace Safety
- Canonical workspace path must not contain `:`.
- Run `bash scripts/dev/check-workspace-path.sh` before long sessions.
- If current path is non-canonical, migrate to a safe clone path before release gates.

## Reporting Cadence
- Daily: gate digest
- Nightly: perf drift
- Weekly: release readiness summary

These are documented in `docs/execution/AUTOMATION_CADENCE.md`.
