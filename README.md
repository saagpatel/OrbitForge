# OrbitForge

[![Rust](https://img.shields.io/badge/Rust-%23dea584?style=flat-square&logo=rust)](#) [![TypeScript](https://img.shields.io/badge/TypeScript-3178c6?style=flat-square&logo=typescript)](#) [![Status](https://img.shields.io/badge/status-WIP-yellow?style=flat-square)](#)

> Real-time N-body gravity simulator — fling planets, crash stars, pilot spacecraft.

OrbitForge is a desktop gravity sandbox built with Rust physics and Three.js visuals. Place stars, planets, and spacecraft; watch gravity do its thing; fly through your own solar systems.

## Features

- **N-body simulation** — Velocity Verlet integration; collisions merge bodies with momentum/mass/volume conservation
- **Adaptive physics engine** — Brute-force O(n²) for <50 bodies, Barnes-Hut octree for 50–500, wgpu compute shader for 500+
- **13 visualization layers** — Orbital elements, Lagrange points, Kepler swept areas, gravity heatmaps, energy graphs, and more
- **Spacecraft flight** — WASD controls with Hohmann transfer and gravity-assist planning tools
- **9 presets + procedural generator** — From "Sun & Earth" to galaxy collisions

## Quick Start

```bash
git clone https://github.com/saagpatel/OrbitForge.git
cd OrbitForge
pnpm install
pnpm tauri dev
```

## Tech Stack

| Layer | Technology |
|-------|------------|
| Desktop shell | Tauri 2 |
| Physics engine | Rust + wgpu compute shaders |
| Rendering | React 19 + Three.js |
| Language | TypeScript 5 |

> **Status: Work in Progress** — Core simulator and all 9 scenarios are functional. UI polish and packaging ongoing.

## License

MIT