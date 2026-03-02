# OrbitForge Decision Log

## Active Decisions
1. Stable-first execution policy is mandatory.
2. Experimental features require explicit task-level opt-in.
3. Local verification uses `.codex/verify.commands` as canonical source.
4. Required gates block completion when status is `fail` or `not-run`.
5. Workspace paths containing `:` are treated as brittle and non-canonical.
6. Product docs are aligned to shipped behavior when implementation is intentionally deferred.
7. Rust backend tests are required in local/CI quality gate flow.

## Open Decisions
- Final release signing and notarization scope by target platform/channel.
