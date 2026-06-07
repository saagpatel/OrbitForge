# OrbitForge Local RC Notes

## Current Local RC Status

- Canonical workspace path verified: `/Users/d/Projects/FunGamePrjs/OrbitForge`
- Local dependency install completed with `pnpm install --frozen-lockfile`
- Canonical verify command passed: `bash .codex/scripts/run_verify_commands.sh`
- Desktop launch succeeded with `pnpm exec tauri dev`
- macOS production build succeeded with `pnpm exec tauri build`
- macOS app bundling is now enabled and produces:
  - `src-tauri/target/release/bundle/macos/OrbitForge.app`
  - `src-tauri/target/release/bundle/dmg/OrbitForge_1.0.0_aarch64.dmg`
- The packaged `.app` stays running after launch, unlike the raw release binary path that was not a valid bundle-level smoke target

## Readiness Fixes Applied

- Tauri app metadata now matches the repo package version (`1.0.0`)
- The macOS bundle identifier no longer ends in `.app`
- Tauri bundling is explicitly enabled so `pnpm exec tauri build` emits a real `.app` and `.dmg`
- Share / Paste now use Tauri's native clipboard manager instead of `navigator.clipboard`, which failed in the packaged app smoke path
- Share/import encoding now has direct frontend regression coverage
- Scenario loading, imported-state normalization, and procedural generation now have direct Rust regression coverage
- Packaged-app scenario switching no longer crashes on multi-body scenes after stabilizing Hohmann and gravity-assist store selectors

## Known Manual Smoke Coverage

- Verified launch, control panel render, packaged build output, and packaged `.app` stability at launch
- Verified packaged-app scenario switching for:
  - `Inner Solar System` (`Bodies: 5`)
  - `Outer Solar System` (`Bodies: 5`)
  - `Asteroid Belt` (`Bodies: 204-206` across smoke runs)
- Verified packaged-app clipboard round-trip on the real app surface:
  - `Share` writes a compressed payload to the macOS clipboard
  - `Clear` drops the visible body count to `0`
  - `Paste` restores the shared state and returns the visible body count to `2` on the default scenario smoke pass
- Accessibility-based native smoke is now reliable enough for launch, scenario, and clipboard checks on the packaged app, but still becomes inconsistent during some deeper live flows
- Remaining manual deep-flow checks should continue to focus on:
  - scenario switching under live interaction
  - place/slingshot/spacecraft control
  - save/load dialog flow
  - mission progression and abort flow (panel visibility is confirmed, but start/progress/abort is not yet certified end-to-end)
