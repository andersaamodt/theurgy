# Native Desktop Track

The native desktop track is for complex apps where wizardry script fan-out becomes a bottleneck.

## Goals

- one integrated Rust control plane
- typed actions instead of free-form shell fragments
- state snapshots that native UI layers can hydrate quickly
- platform-owned controls where they matter
- documented XDG state and config roots
- CLI parity for core workflows
- Product IR as the shared app contract
- Desktop Surface IR as the target-specific window/menu/layout projection
- deterministic native adapter output that can be rebuilt from source

## Initial Scaffold

`spells/conjure-native-desktop NAME [PATH]` creates:

- `theurgy.project.toml`
- `src/app.theurgy`
- `src/main.rs`
- `.github/AI_DOCS.md`
- `LICENSE`
- `WIZARDRY_ADDENDUM.md`

If `PATH` is omitted, the spell uses `NAME`. The generated desktop project is intentionally small, but it starts with the right boundaries: Rust runtime, file-first metadata, ignored local state, and sellable copyleft licensing.

## Runtime Compiler Track

`theurgy-runtime validate-product-ir PATH` validates the shared product contract.
`theurgy-runtime validate-runtime-manifest PATH` validates the runtime bridge contract.
`theurgy-runtime validate-surface-ir PATH` validates a desktop or mobile surface projection.
`theurgy-runtime validate-operation-status-request PATH` validates the typed request envelope for operation progress lookup.
`theurgy-runtime validate-operation-history-request PATH` validates the typed request envelope for operation history lookup.
`theurgy-runtime validate-operation-status PATH` validates the typed status record for a runtime operation.
`theurgy-runtime project-surface PATH --target macos|linux|ios|android` emits a target surface projection.
`theurgy-runtime compile-native PATH --target macos|linux|ios|android --out OUT_DIR` emits a deterministic native adapter root.
`theurgy-runtime compile-app APP_DIR --target macos|linux|ios|android --out OUT_DIR` reads `theurgy.project.toml` and compiles from the app's declared Product IR.
`theurgy-runtime inspect-app APP_DIR` validates and summarizes the declared Product IR, target surfaces, runtime bridge commands, compatibility posture, and long-running action coverage.

This compiler track is opt-in. Shell-first wizardry-apps projects do not depend on it unless they choose the Theurgy native runtime path.
