# Theurgy AI Docs

## Read Order
- Read `README.md` for the public stance.
- Read `.github/THEURGY_ETHOS.md` for project policy.
- Read `.github/PUSH_READY_CHECKLIST.md` before adding paths or generated output.
- Read `docs/architecture.md` before changing runtime boundaries.
- Read `docs/web-runtime-harness.md` before changing enterprise web runtime adapters.
- Read `docs/platform-quarantine.md` before adding Apple, macOS, native desktop, signing, bundle, or platform-adapter machinery.
- macOS generated apps should be one professional app bundle, plus a daemon only when an independent lifecycle is required; do not reintroduce generic runtime wrappers, launch-pressure backoff gates, or multi-executable interactive hot paths.
- Keep app verification, notarization, store review, TestFlight, App Store, Google Play, and special app-publish keys in theurgy rather than pure wizardry or wizardry-apps.
- Keep host-native firewall, sandbox, and network-caging backends in theurgy rather than pure wizardry or wizardry-apps.
- Read `docs/databases.md` before introducing database-backed behavior.

## Canonical Position
- Theurgy extends wizardry; it does not replace wizardry.
- Use Rust for integrated runtime work that would otherwise fan out through many shell processes.
- The enterprise web runtime harness integrates existing free software components through explicit Wizardry contracts; do not introduce a full static-site generator or CMS as the Gazeta/Desk-class runtime core.
- The Product IR/native compiler track is opt-in; do not make existing shell-first wizardry-apps require theurgy to build or run.
- Keep plain files as durable truth unless a documented transaction or scale requirement justifies more.
- Keep Wizardry-facing names lowercase unless quoting a formal project name.
- Generated blank projects use AGPL-3.0-or-later plus Wizardry Addendum.
- Theurgy repository code is OWL 3.1 unless a file explicitly says otherwise.

## Migration Standards
- A theurgy migration should preserve the existing wizardry-native app contract when the app already uses a shared native schema or language. Do not replace a cross-platform generator with a single-platform rewrite just to introduce Rust.
- Move hot, stateful, graph-shaped, latency-sensitive, or platform-integrated behavior into a compiled Rust control plane.
- Keep generated native hosts as platform adapters when they are already the right place for AppKit, SwiftUI, GTK, permissions, menus, windows, or lifecycle glue.
- The normal desktop artifact should launch one solid compiled app executable and one primary compiled runtime. Separate daemons are appropriate only when they have an independent lifecycle; ordinary state, status, action, history, and operation-status calls must not be split across several helper binaries.
- On macOS, generated adapters should dispatch declared runtime contracts directly through one deliberately staged app-owned binary; avoid app -> generic wrapper -> helper subprocess chains and avoid splitting interactive commands across several executables, both of which multiply Gatekeeper assessment work.
- macOS staging must not auto-discover arbitrary local signing identities, stage theurgy's generic wrapper into app bundles, or copy quarantine/provenance metadata into generated artifacts.
- `stage-app-runtime` may stage an explicitly declared executable source runtime from an app's `source_root`, `app-blueprint/`, or app root for shell-first migrations; otherwise it builds the matching Cargo `--bin`. It also copies `source_root/libexec/` support files into package libexec locations before declared binaries are staged. This is an intermediate compatibility path, not a claim that shell fan-out is the final professional runtime.
- macOS test and staging code must reuse stable external XDG/workbench scratch roots for executable artifacts instead of minting randomized temporary executable paths on every run.
- Shell scripts may surround the app for install, repair, release, inspection, and compatibility, but they should not keep owning the core interactive runtime once a typed Rust core exists.
- The migration boundary must be explicit: typed commands, JSON/state schemas, documented XDG roots, release-bundled binaries, and tests that prove the app did not regress to shell-only runtime behavior.
- For existing apps, migrate by adding a compiled core beside the current backend first, then route new/hot capabilities through it before removing old compatibility paths.
- Release artifacts for every supported desktop target must include the compiled core at the time the migration lands; do not defer non-primary desktop targets without saying exactly what artifact remains incomplete.
- A cross-platform native desktop migration is incomplete if only the primary platform produces a native app artifact. Linux and macOS should both have explicit artifact builders when both are declared targets.
- Packaged native hosts must resolve staged runtime resources relative to the installed executable or bundle layout. Do not leave generated desktop hosts pointing at the source checkout as their only runtime path.
- During source-tree development, backend wrappers should avoid preferring stale release binaries over freshly built debug binaries. Installed app bundles should prefer the bundled compiled binary.
- Append-only logs are a good audit substrate, but migrated app features still need active-state projections. Memory, resident workers, artifacts, and desktop-control audits should not surface raw event logs as their primary UX or API.
- Compiled core reasoning context should be passed through typed sidecar state keyed by durable run identifiers, not by mutating user-visible prompts or transcript messages.
- Compiled core reasoning context should include exact mediated command affordances for governed tools when the host controller uses mediated shell commands as its tool protocol.
- Resident worker migrations should attach to the app's existing daemon or scheduled worker lifecycle before introducing another scheduler.
- Structured desktop-control adapters may call platform tools internally, but they must not expose freeform shell, AppleScript, or automation strings as the capability contract.
- For web migrations, work in the exact source checkout and branch that the deployment system watches. Do not prove a Theurgy port in a parallel checkout while production still deploys an older shell or Python backend.
- Keep CGI front doors as small compatibility wrappers during migration, but make them dispatch into allowlisted compiled runtimes instead of retaining large shell bodies.
- Split web runtimes by risk domain. Public reads, cache rebuilds, admin reads, admin mutations, payments, Nostr publishing, and secure chat controls should not be casually merged into one action surface.
- Add replay fixtures before switching endpoints. Captured environment, query, body, and file-state assertions are the basic proof that a CGI route survived the boundary change.
- Test and package against the deployment server's real Rust/Cargo constraints. Prefer small pinned free-software dependencies and vendor them when the server cannot reliably resolve the registry during deploy.
- Keep build outputs, dependency caches, and generated publish artifacts out of watched source commits. Deployment tooling should ignore `target/`, `node_modules/`, and other local build products even when staging ignored files.
- A web migration is not complete until the managed deployment path builds the Linux runtime normally, promotes it, live endpoints use the compiled binary, obsolete interpreted backends are absent from the active release, and browser-level behavior is verified.

## Validation
- Run `sh scripts/theurgy-cargo fmt --check`.
- Run `sh scripts/theurgy-cargo test`.
- Run `spells/assay-theurgy`.
- Run at least one conjure spell into a temporary directory and inspect it with `spells/inspect-theurgy-project`.
