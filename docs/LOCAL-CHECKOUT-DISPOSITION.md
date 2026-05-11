# OrbitForge — Local Checkout Disposition

There are two on-disk copies of this repo on the operator's machine.
**Only `/Users/d/Projects/Fun:GamePrjs/OrbitForge` is canonical.** The
other copy at `/Users/d/Projects/FunGamePrjs/OrbitForge` (note the
missing colon) is a typo-induced sibling clone and should be removed.

> **Audience:** anyone deciding which path to `cd` into, or wondering
> why filesystem search returns two results.

---

## The two checkouts

| Path | Branch tip | Status |
|---|---|---|
| `/Users/d/Projects/Fun:GamePrjs/OrbitForge` | `codex/chore/bootstrap-codex-os` at `07ef263` (feat: harden runtime safety and release gates) | **Canonical** — work here. |
| `/Users/d/Projects/FunGamePrjs/OrbitForge` | `codex/chore/bootstrap-codex-os` at `a383f37` (chore: finalize codex os bootstrap baseline) | Stale — one commit behind canonical and ~61 dirty files of abandoned WIP. |

Both copies point `origin` at the same GitHub repository
(`saagpatel/OrbitForge.git`). They are not divergent forks. The
duplication is a **typo in the parent directory name** — `Fun:GamePrjs`
(with colon) vs `FunGamePrjs` (no colon).

---

## Why the duplicate exists

The operator's portfolio buckets use a `:`-separated naming
convention for categorized directories — `Misc:NoGoPRJs/`,
`Hardware:GadgetPRJs/`, and so on. `Fun:GamePrjs/` is the canonical
games bucket.

At some point a clone went into `FunGamePrjs/` (no colon) — probably
a typo or a tab-completion miss in a shell that doesn't quote colons
well. The newer clone went into the correctly-named `Fun:GamePrjs/`
and accumulated more work, leaving the typo-bucket clone stale.

Same upstream remote, no divergence in shared history, just different
on-disk paths.

---

## Disposition

| Path | Recommended action |
|---|---|
| `/Users/d/Projects/Fun:GamePrjs/OrbitForge` | Keep. Make all future commits here. |
| `/Users/d/Projects/FunGamePrjs/OrbitForge` | Delete locally after confirming no uncommitted work. |

### Safe deletion procedure

Run only after confirming the dup has no uncommitted work worth
preserving:

```bash
# 1. Confirm the dup is not divergent from origin
cd /Users/d/Projects/FunGamePrjs/OrbitForge
git fetch origin
git status                                # should be clean OR show only codex-os scaffolding edits
git log --oneline origin/master..HEAD | head -10
# ↑ shows commits ONLY on the local stale branch that are not on origin/master

# 2. (Optional) stash anything you want to preserve — probably nothing.
git stash push -u -m "pre-deletion stash of dup checkout"

# 3. Remove the dup directory
cd /Users/d/Projects/FunGamePrjs
rm -rf OrbitForge

# 4. If FunGamePrjs is now empty, remove the typo'd parent too
cd /Users/d/Projects
rmdir FunGamePrjs    # only succeeds if it's empty
```

After deletion, `/Users/d/Projects/Fun:GamePrjs/OrbitForge` is the
only checkout that matters.

---

## Are there other typo'd siblings?

Worth a one-time sweep. From `~/Projects`:

```bash
find /Users/d/Projects -maxdepth 2 -type d -name "FunGamePrjs"
# ↑ should return at most the one parent. If empty after the dup
#   deletion, the typo dir is gone entirely.
```

If `find` returns additional repos under `FunGamePrjs/`, repeat the
deletion procedure for each. The convention-correct parent
`Fun:GamePrjs/` is the only one that should keep accumulating clones.

---

## Why a doc instead of just deleting the dup

The dup deletion is operator-side filesystem work. This file is in
the canonical repo so:

1. **Portfolio scans don't see filesystem state** — they look at the
   GitHub repo. A note here means the dup question is answered the
   next time anyone wonders.
2. **The `rm -rf` is operator-only** — Claude Code should not
   autonomously delete filesystem directories. Documenting the
   recommendation is the appropriate scope.
3. **The typo pattern could recur** — if it does (another tab-miss),
   this file is the place to extend the rule.

---

## What OrbitForge actually is

(Context for anyone landing here without prior exposure.)

OrbitForge is a real-time N-body gravity simulator desktop app:
place stars / planets / spacecraft, drag to set velocity, watch
Velocity-Verlet-integrated gravity unfold, fly a spacecraft with WASD
thrust. Built with Rust (physics core) + React/Three.js (visuals) +
Tauri 2 (desktop shell). 13 visualization layers including orbital
elements, Lagrange points, gravity field heatmaps.

For full feature detail see `README.md`.

---

## Portfolio operating system instructions

| Aspect | Posture |
|---|---|
| Canonical local path | `/Users/d/Projects/Fun:GamePrjs/OrbitForge` |
| Dup local path | `/Users/d/Projects/FunGamePrjs/OrbitForge` — safe to delete |
| Portfolio status | Treat as `Active` or `Cold Storage` based on the operator's product intent; this disposition only addresses the local-filesystem dup |
| Resurface conditions | Only if a third clone appears (would mean the typo cleanup didn't take or recurred) |

---

## Last known reference

| Field | Value |
|---|---|
| Canonical remote | `https://github.com/saagpatel/OrbitForge.git` |
| Last meaningful commit (canonical clone) | `07ef263` feat(reliability): harden runtime safety and release gates |
| Last commit on stale dup | `a383f37` chore(repo): finalize codex os bootstrap baseline |
| Default branch | `master` |
