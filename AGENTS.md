<!-- portfolio-context:start -->

# Portfolio Context

## What This Project Is

OrbitForge is a desktop N-body gravity sandbox built with Rust physics and React/Three.js visuals. Users place stars, planets, and spacecraft, run procedural scenarios, inspect orbital/energy layers, and pilot spacecraft through simulated systems.

## Current State

The repo is work-in-progress game/simulation product work. Core simulator behavior and nine scenarios are functional; UI polish and packaging are still ongoing. Existing untracked `.perf-results` are local artifacts and should not be swept into source commits.

## Stack

| Layer          | Technology                  |
| -------------- | --------------------------- |
| Desktop shell  | Tauri 2                     |
| Physics engine | Rust + wgpu compute shaders |
| Rendering      | React 19 + Three.js         |
| Language       | TypeScript 5                |

> **Status: Work in Progress** - Core simulator and all 9 scenarios are functional. UI polish and packaging ongoing.

## How To Run

```bash
git clone https://github.com/saagpatel/OrbitForge.git
cd OrbitForge
pnpm install
pnpm tauri dev
```

## Known Risks

- Physics correctness depends on deterministic integration and collision handling.
- The engine switches between brute force, Barnes-Hut, and wgpu compute paths by body count; verify all paths after physics changes.
- Keep generated performance artifacts out of source commits.
- Treat UI polish and packaging as still in progress.

## Next Recommended Move

Continue with UI polish, packaging, and verification of the physics/rendering boundary from the canonical checkout.

<!-- portfolio-context:end -->

<!-- codex-execution-contract:start -->

# Codex Execution Contract

## Communication Contract

- Follow `/Users/d/.codex/policies/communication/BigPictureReportingV1.md` for all user-facing formal delivery, blocker, waiting, risk, decision, or explicit status/report updates.
- Keep ordinary in-flight updates conversational, warm, PM-readable, operator-grade, and low-noise.
- Keep technical details in internal artifacts unless explicitly requested by the user or required by failure, risk, or verification.
- Honor toggles literally: `simple mode`, `show receipts`, `tech mode`, `debug mode`.

## Definition of Done (Git + Performance)

- Work on non-default branch only.
- Branch must match `codex/<type>/<slug>`.
- Commit messages must follow Conventional Commits.
- Commits must be atomic by concern.
- PR must include sections: What, Why, How, Testing, Performance impact, Risk / Notes.
- If lockfile changed, include lockfile rationale in PR body.
- Required checks before done-state:
  - git hygiene
  - bundle delta
  - build delta
  - performance budgets
  - assets/memory checks
- Required gates block completion when `fail` or `not-run`.

## Verification Contract

- Canonical commands are in `.codex/verify.commands`.
- Use `.codex/scripts/run_verify_commands.sh` for deterministic execution.

<!-- codex-execution-contract:end -->
