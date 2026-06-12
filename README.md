# Theurgy

Theurgy is enterprise-level wizardry: the wizardry ethos for professional native desktop apps and enterprise-level websites.

It exists where pure shell-first wizardry starts to lose against integration pressure: many moving parts, fast state hydration, typed IPC, durable queues, platform lifecycle behavior, derived indexes, high-traffic routing, and operational reliability. Default to wizardry. Escalate to theurgy only when integration, responsiveness, scale, or platform quality justifies the compromise.

Theurgy is not a Rust project in spirit. It is an opinionated knowledge library and a minimalist practice set for building non-commercial open source software at professional quality. Right now, Rust is the simplest strong default for this layer: fast, memory-safe, widely tooled, free software, and less institutionally compromised than options such as Google's Go. To keep the system integrated, theurgy currently ships one implementation language: Rust. That can evolve as the state of the art evolves.

## What It Does

Use theurgy for work that needs:

- **Runtime:** a resident process instead of repeated shell process fan-out
- **Speed:** fast state, routing, indexing, IPC, events, or cache invalidation
- **Gloss:** professional native desktop behavior that shell scripts cannot provide cleanly
- **Web:** an enterprise web runtime while keeping source truth file-first where possible
- **Data:** optional transaction or query layers when plain files cannot carry the load
- **Quarantine:** signing, notarization, app review, publish keys, and similar platform machinery

A Rust-backed native desktop app can be solid in a way that an app backed by many shell scripts cannot quite be. For websites, theurgy should grow a Rust-based runtime that stays static-first and file-first where possible, while still handling fast routing, rendering, indexing, caching, and optional database-backed transaction layers.

## What Exists Now

- well-named spells backed by `theurgy-runtime`
- project scaffolding for native desktop apps and enterprise web apps
- first canonical schemas for Product IR, Desktop Surface IR, Mobile Surface IR, Action IR, State Snapshot IR, Runtime Status, Runtime Action Result, Operation Status, Operation History, Generated Runtime Metadata, and Runtime Manifest
- product runtime/compiler commands for validation, surface projection, app-manifest-driven native adapter emission, app inspection, and JSON action envelopes
- a first-phase enterprise web runtime harness contract for CGI/HTTP/FastCGI migration
- file-first project manifests
- release adapters for signing, notarization, TestFlight, App Store, and Play Store flows
- architecture docs for desktop, web, databases, and wizardry integration

Run:

```sh
spells/assay-theurgy
spells/check-theurgy-web-runtime
spells/capture-theurgy-cgi-context
spells/conjure-native-desktop sample-desktop
spells/conjure-enterprise-website sample-website
spells/inspect-theurgy-project sample-desktop
cargo run --bin theurgy-runtime -- validate-product-ir product.ir.json
cargo run --bin theurgy-runtime -- validate-action-ir action.ir.json
cargo run --bin theurgy-runtime -- validate-state-snapshot state.snapshot.json
cargo run --bin theurgy-runtime -- validate-runtime-status runtime-status.json
cargo run --bin theurgy-runtime -- validate-runtime-action-request action-request.json --manifest runtime.manifest.json
cargo run --bin theurgy-runtime -- validate-runtime-action-result action-result.json
cargo run --bin theurgy-runtime -- validate-operation-status operation-status.json
cargo run --bin theurgy-runtime -- validate-operation-history operation-history.json
cargo run --bin theurgy-runtime -- validate-runtime-manifest runtime.manifest.json
cargo run --bin theurgy-runtime -- validate-generated-runtime theurgy-runtime.json
cargo run --bin theurgy-runtime -- validate-surface-ir desktop.surface.ir.json
cargo run --bin theurgy-runtime -- project-surface product.ir.json --target macos
cargo run --bin theurgy-runtime -- compile-native product.ir.json --target linux --out /tmp/theurgy-linux
cargo run --bin theurgy-runtime -- compile-app /path/to/app --target macos --out /tmp/theurgy-macos
cargo run --bin theurgy-runtime -- run-state --manifest runtime.manifest.json
cargo run --bin theurgy-runtime -- run-status --manifest runtime.manifest.json
cargo run --bin theurgy-runtime -- subscribe-status --manifest runtime.manifest.json --once
cargo run --bin theurgy-runtime -- run-operation-status operation-id --manifest runtime.manifest.json
cargo run --bin theurgy-runtime -- run-history deployment-slug 40 --manifest runtime.manifest.json
cargo run --bin theurgy-runtime -- run-action refresh_state --json '{}' --manifest runtime.manifest.json
```

`compile-native` is the raw Product IR entrypoint and may project default surfaces and runtime command names from the product contract. `compile-app` is the real application compiler entrypoint: it reads `theurgy.project.toml`, validates the declared Product IR, Runtime Manifest, and target-appropriate Surface IR, rejects manifest path drift between those contracts, then emits adapter metadata from the declared app contracts. Runtime manifests may also declare `surfaces.legacyNativeDesktop` while a migrated app still carries a wizardry-apps native desktop IR; Theurgy validates and propagates that compatibility input without making shell-first wizardry-apps depend on Theurgy. Generated runtime metadata exposes the bridge as `stateCommand`, `statusCommand`, `subscribeStatusCommand`, `operationStatusCommand`, `actionCommand`, and `historyCommand`, with typed action contracts beside the commands. It also declares `adapterRuntimeTransport`: desktop adapters use `local-process-json`, while mobile adapters use `external-json-abi` so generated iOS/Android hosts do not pretend they can execute desktop shell commands directly. Long-running generated runtimes must expose `operationStatusCommand` so native adapters can track progress/final state through a typed operation status contract. Mobile generated runtimes also publish `runtimeActionRequestSchema`, `operationStatusRequestSchema`, and `operationHistoryRequestSchema`, so external mobile hosts can hand off action/status/history requests through a stable JSON ABI instead of informal command strings. Runtime manifests may declare `subscribeStatusCommand`; if they omit it, Theurgy falls back to `statusCommand`. That keeps professional apps explicit while preserving the opt-in boundary for shell-first wizardry-apps.

Install locally:

```sh
./install
./install --uninstall
```

The install path is `${THEURGY_HOME:-$HOME/theurgy}`. The installer writes user-local wrappers so wizardry spells can call theurgy spells by name, such as `assay-theurgy` or `conjure-native-desktop`.

## Design Commitments

- Wizardry remains the spell/menu/prototyping layer.
- Spells remain the normal user-facing interface.
- Rust replaces shell fan-out only for hot, stateful, or platform-integrated work.
- Plain files remain durable truth by default.
- Databases are optional.
- Free software and open formats are preferred over proprietary frameworks, commercial enclosures, and closed platform assumptions.
- Institutional cruft is quarantined and minimized.
- Generated blank projects are AGPL-3.0-or-later plus the Wizardry Addendum.
- theurgy itself is OWL 3.1, matching the wizardry project family.

## Institutional Cruft Quarantine

Theurgy is the quarantine layer for parts of professional app development that do not belong in pure wizardry:

- macOS bundle structure
- code signing and notarization workflows
- app verification, app-store review/policing workflows, and special publish keys
- Swift, SwiftUI, Xcode-shaped project output, and Apple lifecycle conventions
- GTK, platform windowing, desktop IPC, and app lifecycle adapters
- Rust build products, lockfiles, and toolchain metadata
- web server runtimes, caches, indexes, and schema migrations
- optional database layers
- generated native source trees

Theurgy should contain institutional complexity without normalizing it. Keep wizardry pure where possible. Keep wizardry-apps script-first and free of direct Rust, Cargo, Swift, signing, notarization, app verification/policing, special app-publish keys, and Apple-specific implementation machinery. Put unavoidable platform-specific or enterprise-specific machinery in theurgy, wrap it with spells, keep source files plain, prefer free/open alternatives, and document the compromise.

Theurgy is an opt-in escalation path. Existing wizardry-apps projects must remain able to build and run through their current shell-first flows without requiring theurgy. The product runtime/compiler track is for apps that explicitly choose a higher-integration native or enterprise runtime.

Apple-specific code is the obvious example. Professional macOS apps may need SwiftUI, bundle metadata, signing, launch behavior, lifecycle hooks, verification, notarization, review gates, and publishing keys. The Apple adapter belongs in theurgy behind a narrow boundary; the app's durable project truth should remain file-first and portable where possible.

## Wizardry Compromises

Theurgy makes these compromises against pure shell-first wizardry. Keep this table current and keep pressure on every unsettled compromise.

| Compromise | Why it exists | Status |
| --- | --- | --- |
| Rust runtime instead of POSIX sh everywhere | Hot paths need integrated state, routing, indexing, IPC, and command dispatch without spawning many shell processes. | Settled for now |
| Cargo toolchain | Rust needs a build/package manager; Cargo builds `theurgy-runtime`, manages `Cargo.lock`, and creates `target/`. | Forced by Rust |
| Compiled binary opacity | Compiled code is less immediately inspectable than shell. | Undesired; mitigate with plain manifests, tests, and docs |
| Build artifacts and caches | Compiled and indexed systems create `target/`, caches, and runtime outputs. | Undesired; keep ignored or outside source trees |
| Rust toolchain requirement | Development needs Rust rather than only POSIX sh and small system tools. | Settled for now |
| Stronger schemas and typed contracts | Enterprise support needs stable manifests, structured snapshots, and typed actions. | Settled; keep formats plain and hand-editable |
| Product IR plus surface IRs | Shared product semantics should compile to platform-native desktop and mobile surfaces without pretending one widget tree fits every target. | Initial schemas landed |
| Generated native code | Desktop support may emit SwiftUI, GTK, or other platform-native sources. | Necessary |
| Platform-specific adapters | Native quality requires platform-owned behavior for windows, menus, IPC, and app lifecycle. | Necessary |
| Apple-language and closed-platform accommodation | Professional macOS support may require Swift, SwiftUI, bundle metadata, signing, and notarization. | Forced external; quarantine aggressively |
| App verification and publish-key accommodation | App distribution may require signing keys, review gates, verification metadata, and platform policing workflows. | Forced external |
| Longer edit-run loop | Compiled code can be slower to iterate on than a spell. | Undesired; keep prototyping in wizardry |
| Optional databases | Some workloads need transactions, query acceleration, replication, or frequent concurrent writes. | Conditional; files remain default truth |
| Derived indexes and caches | Enterprise sites need fast lookup and rendering paths. | Settled |
| HTTP/runtime servers | High-performance sites may need long-running processes instead of one-shot scripts. | Conditional; preserve static output where possible |
| Dependency risk | Professional Rust/web work can tempt framework and crate sprawl. | Undesired; prefer small free software dependencies |
| Less direct shell composability | Typed runtime actions are faster and safer but less pipe-friendly. | Undesired |

## Tracks

**Native desktop apps** use Rust for the control plane, state model, command dispatch, and platform adapters. Native UI generation can target SwiftUI, GTK, or other platform-owned frontends without routing every action through a chain of shell scripts.

**Enterprise web apps** use Rust for routing, content indexing, static rendering, API handlers, cache invalidation, and deployment artifacts. The web track should stay boring where boring is good: HTTP, files, caches, indexes, and free software components.

## Repository Map

- `src/` contains the Rust runtime and scaffold engine.
- `docs/` contains architecture decisions.
- `docs/platform-quarantine.md` defines where Apple and platform-specific machinery belongs.
- `docs/app-publish-secrets.md` documents protected app publishing credentials quarantined in theurgy.
- `tools/release/` contains quarantined app verification, signing, notarization, TestFlight, App Store, and Play Store release adapters.
- `.tests/release/` contains coverage for quarantined app-publishing release adapters.
- `.github/` contains AI-facing standards.
- `spells/` contains user-facing spells.
- `install` installs theurgy at `~/theurgy`, exposes spell wrappers, and removes them again with `--uninstall`.

## Storage

Theurgy project source belongs in the project checkout. Runtime state, caches, logs, and build products belong outside the checkout or in ignored paths. Generated project skeletons include local ignore rules for `target/`, `.theurgy-state/`, and logs.
