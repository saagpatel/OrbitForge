<!-- portfolio-context:start -->
# Portfolio Context

## What This Project Is

OrbitForge is a desktop N-body gravity sandbox built with Rust physics and React/Three.js visuals. Users place stars, planets, and spacecraft, run procedural scenarios, inspect orbital/energy layers, and pilot spacecraft through simulated systems.

## Current State

The repo is work-in-progress game/simulation product work. Core simulator behavior and nine scenarios are functional; UI polish and packaging are still ongoing. Existing untracked `.perf-results` are local artifacts and should not be swept into source commits.

## Stack

| Layer | Technology |
|-------|------------|
| Desktop shell | Tauri 2 |
| Physics engine | Rust + wgpu compute shaders |
| Rendering | React 19 + Three.js |
| Language | TypeScript 5 |

> **Status: Work in Progress** — Core simulator and all 9 scenarios are functional. UI polish and packaging ongoing.

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

Add only the context file for this recovery pass, then continue with UI polish, packaging, and verification of the physics/rendering boundary.

<!-- portfolio-context:end -->
