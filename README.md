# OrbitForge

A real-time N-body gravity simulator where you can fling planets, crash stars, and pilot spacecraft through your own solar systems.

Built with Rust physics, React/Three.js visuals, and Tauri 2 for the desktop shell.

![Tauri](https://img.shields.io/badge/Tauri_2-24C8D8?style=flat&logo=tauri&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![React](https://img.shields.io/badge/React_19-61DAFB?style=flat&logo=react&logoColor=black)
![Three.js](https://img.shields.io/badge/Three.js-000000?style=flat&logo=threedotjs&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?style=flat&logo=typescript&logoColor=white)

## What You Can Do

**Create** - Place stars, planets, and spacecraft. Drag to set velocity. Build anything from a simple orbit to a full solar system.

**Simulate** - Watch gravity evolve at up to 8x speed. Velocity Verlet integration keeps motion stable. Collisions merge bodies with momentum, mass, and volume conservation.

**Fly** - Select a spacecraft, use WASD, and thrust around. Shift doubles your power. Plan Hohmann transfers and gravity assists with the built-in tools.

**Explore** - Toggle orbital elements, Lagrange points, Kepler swept areas, gravity field heatmaps, orbital planes, energy graphs, and more.

## Scenarios

| Preset             | What it is                            |
| ------------------ | ------------------------------------- |
| Sun & Earth        | The basics                            |
| Inner Solar System | Mercury through Mars                  |
| Outer Solar System | Jupiter through Neptune               |
| Full Solar System  | All 8 planets                         |
| Binary Star        | Two stars in orbit                    |
| Figure-8           | Three bodies, one elegant loop        |
| Inclined Solar     | Tilted orbital planes                 |
| Asteroid Belt      | Hundreds of rocks                     |
| Galaxy Collision   | Two spiral galaxies smashing together |

Plus a procedural generator for creating custom systems.

## Performance

The physics engine scales automatically:

| Bodies   | Algorithm           | Complexity      |
| -------- | ------------------- | --------------- |
| <= 50    | Brute force         | O(n^2)          |
| 51 - 500 | Barnes-Hut octree   | O(n log n)      |
| > 500    | wgpu compute shader (Barnes-Hut fallback) | GPU accelerated when available |

Simulation targets 120Hz on a background thread. Rendering is decoupled via `requestAnimationFrame`.

## Controls

| Key             | Action            |
| --------------- | ----------------- |
| `Space`         | Pause / Play      |
| `R`             | Reset             |
| `C`             | Clear all bodies  |
| `Esc`           | Deselect          |
| `W` `A` `S` `D` | Spacecraft thrust |
| `Shift`         | Double thrust     |
| `F11`           | Screenshot mode   |
| `F12`           | Take screenshot   |

Mouse: click to select, scroll to zoom, drag to orbit camera. In **Place** mode, click to drop a body. Use **Slingshot** mode and drag to set launch velocity.

## Tech Stack

| Layer          | Tech                                          |
| -------------- | --------------------------------------------- |
| Physics engine | Rust (Velocity Verlet, Barnes-Hut, wgpu)      |
| Desktop shell  | Tauri 2                                       |
| UI framework   | React 19 + Zustand                            |
| 3D renderer    | Three.js (bloom, CSS2D labels, InstancedMesh) |
| Build tools    | Vite, TypeScript strict mode                  |

16 Tauri IPC commands bridge the Rust simulation thread and the React frontend.

## Getting Started

```bash
git clone https://github.com/saagpatel/OrbitForge.git
cd OrbitForge
pnpm install
pnpm tauri dev
```

Requires Rust, Node.js 22.19+ (22.x), pnpm 10.28.2, and the Tauri 2 prerequisites. CI uses Node.js 22.19.0, which satisfies the committed dependency engine floors.

## Normal Dev vs Lean Dev

Use normal dev when you want fastest incremental rebuilds and do not care about local artifact growth.

```bash
pnpm tauri dev
```

Use lean dev when you want lower disk usage over long sessions.

```bash
pnpm run dev:lean
```

What lean dev changes:

- Vite cache goes to a temporary directory (`$VITE_CACHE_DIR`) instead of `node_modules/.vite`.
- Rust build output goes to a temporary directory (`$CARGO_TARGET_DIR`) instead of `src-tauri/target`.
- On exit, temporary caches are removed and a targeted heavy-artifact cleanup runs.

Tradeoff:

- Lower disk usage after each session.
- Slower startup on the next run because build caches are intentionally discarded.

Workspace note:

- Use `/Users/d/Projects/FunGamePrjs/OrbitForge` as the canonical local checkout.
- Avoid the legacy colon-path checkout at `/Users/d/Projects/Fun:GamePrjs/OrbitForge` for active work.
- Run `bash scripts/dev/check-workspace-path.sh` to validate your current path.
- Run `bash scripts/dev/migrate-to-canonical-path.sh` to copy a local checkout to the canonical path.

## Cleanup Commands

Targeted cleanup (heavy build artifacts only, keeps dependencies):

```bash
pnpm run clean:heavy
```

Full local cleanup (the heavy artifacts above plus `node_modules`):

```bash
pnpm run clean:local
```

## Verification Commands

Run from the repository root with pnpm 10.28.2 (the packageManager pin), a Node
version supported by Vite 7 (20.19+ or 22.12+), Rust stable, and the native Tauri 2
prerequisites for your platform. The full bundle also requires gitleaks for its
local staged-secret guard; a missing scanner is not a successful scan.

```bash
# Locked dependency install without running prepare/Husky lifecycle hooks
pnpm install --frozen-lockfile --ignore-scripts

# Focused frontend fixture test, broader frontend suite, strict TypeScript
pnpm exec vitest run src/missions/MissionDefinition.test.ts
pnpm test
pnpm lint

# Rust library tests and frontend production build
pnpm test:rust
pnpm build

# Full local guard/quality/performance bundle
bash .codex/scripts/run_verify_commands.sh
```

`lint` is a TypeScript check, not ESLint. Canonical full-bundle definitions live in
[.codex/verify.commands](.codex/verify.commands), with the gate context in
[Verification](docs/execution/VERIFICATION.md). The bundle writes local build and
`.perf-results` artifacts and checks the staged Git context; it does not publish
an app. Run it on the intended review branch and preserve unrelated changes.
The `clean:*`/lean-dev commands remove local artifacts and are not verification
steps. Guard regression tests under `scripts/git/tests/` use temporary repositories.

For rendering, controls, or report/overlay changes, also inspect the affected
Tauri flow (or a browser for frontend-only behavior): preset loading, pause/play,
mission state, and overlays using disposable simulation data. Physics/backend
changes require the Rust lane as well as frontend checks; unit or performance
results do not establish device/GPU behavior or release signing readiness.

## Features at a Glance

- 9 preset scenarios + procedural generation
- Real-time collision detection with conservation laws
- Orbit prediction and trail rendering
- Hohmann transfer calculator
- Gravity assist planner
- Mission system with objectives
- Minimap overview
- Body info panel and separate orbital elements HUD
- Energy graph (kinetic + potential + total)
- Lagrange point visualization
- Kepler swept area display
- Gravity field heatmap
- Save / Load / Share (JSON + clipboard)
- Video recording (WebM export)
- Audio tied to collisions, with an ambient drone
- Audio volume slider in the control panel

## License

MIT
