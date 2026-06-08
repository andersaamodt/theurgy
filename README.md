# Theurgy

Theurgy is enterprise-level wizardry: the wizardry ethos applied to professional native desktop apps and enterprise-level websites.

It exists where pure shell-first wizardry starts to lose against integration pressure: many moving parts, fast state hydration, typed IPC, durable queues, platform lifecycle behavior, derived indexes, high-traffic routing, and operational reliability. Default to wizardry. Escalate to theurgy only when integration, responsiveness, scale, or platform quality justifies the compromise.

Theurgy is not a Rust project in spirit. It is an opinionated knowledge library and minimalist practice set for building non-commercial open source software at professional quality. Right now, Rust is the simplest strong default for this layer: lower-level than most web/app frameworks, free software, fast, memory-safe, widely tooled, and less institutionally compromised than options such as Google's Go. To keep the system integrated, theurgy currently ships one implementation language: Rust. That can evolve as the state of the art evolves.

## What It Does

Use theurgy for work that needs:

- a resident runtime instead of repeated shell process fan-out
- fast state, routing, indexing, IPC, event dispatch, or cache invalidation
- professional native desktop behavior that shell scripts cannot provide cleanly
- enterprise web runtime behavior while keeping source truth file-first where possible
- optional transaction/query layers when plain files cannot carry the load
- quarantined platform machinery such as signing, notarization, app review, and publish keys

A Rust-backed native desktop app can be solid in a way that an app backed by many shell scripts cannot quite be. For websites, theurgy should grow a Rust-based web runtime: static-first where possible, file-first where possible, but capable of high-performance routing, rendering, indexing, caching, and optional database-backed transaction layers.

## What Exists Now

- well-named spells backed by `theurgy-runtime`
- native desktop and enterprise web project scaffolding
- file-first project manifests
- generated app licensing that follows the Wizardry Apps split
- quarantined app publishing adapters for signing, notarization, TestFlight, App Store, and Play Store flows
- architecture docs for desktop, web, databases, and wizardry integration

Run:

```sh
spells/assay-theurgy
spells/conjure-native-desktop sample-desktop
spells/conjure-enterprise-website sample-website
spells/inspect-theurgy-project sample-desktop
```

Install locally:

```sh
./install
```

The install path is `${THEURGY_HOME:-$HOME/theurgy}`. The installer writes user-local wrappers so wizardry spells can call theurgy spells by name, such as `assay-theurgy` or `conjure-native-desktop`.

## Design Commitments

- Wizardry remains the spell/menu/prototyping layer.
- Spells remain the normal user-facing interface.
- Rust replaces shell fan-out only for hot, stateful, graph-shaped, or platform-integrated work.
- Plain files remain durable truth by default.
- Databases are optional transaction, query, acceleration, or replication tools.
- Free software and open formats are preferred over proprietary frameworks, commercial enclosures, and closed platform assumptions.
- Institutional cruft is quarantined, named, documented, and minimized.
- Generated blank projects are AGPL-3.0-or-later plus the Wizardry Addendum.
- theurgy itself is OWL 3.1, matching the wizardry project family.

## Institutional Cruft Quarantine

Theurgy is the quarantine layer for the parts of professional app development that do not belong in pure wizardry:

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

Apple-specific code is the obvious example. Professional macOS apps may need SwiftUI, bundle metadata, signing, launch behavior, lifecycle hooks, verification, notarization, review gates, and publishing keys. The Apple adapter belongs in theurgy behind a narrow boundary; the app's durable project truth should remain file-first and portable where possible.

## Wizardry Compromises

Theurgy makes these compromises against pure shell-first wizardry. Keep this table current and keep pressure on every unsettled compromise.

| Compromise | Why it exists | Status |
| --- | --- | --- |
| Rust runtime instead of POSIX sh everywhere | Hot paths need integrated state hydration, routing, indexing, IPC, and command dispatch without spawning many shell processes. | Settled for now |
| Cargo toolchain | Rust needs a build/package manager; Cargo builds `theurgy-runtime`, manages `Cargo.lock`, and creates `target/`. | Forced by Rust |
| Compiled binary opacity | Compiled code is less immediately inspectable than shell. | Undesired; mitigate with small modules, plain manifests, tests, and docs |
| Build artifacts and caches | Compiled and indexed systems create `target/`, caches, and runtime outputs. | Undesired; keep ignored or outside source trees |
| Rust toolchain requirement | Development needs Rust rather than only POSIX sh and small system tools. | Settled for now; revisit if better free/open low-level options emerge |
| Stronger schemas and typed contracts | Enterprise support needs stable manifests, structured snapshots, and typed actions. | Settled, but keep formats plain and hand-editable |
| Generated native code | Desktop support may emit SwiftUI, GTK, or other platform-native sources. | Necessary, but generated output must stay reproducible |
| Platform-specific adapters | Native quality requires platform-owned behavior for windows, menus, IPC, and app lifecycle. | Necessary |
| Apple-language and closed-platform accommodation | Professional macOS support may require Swift, SwiftUI, bundle metadata, signing, and notarization. | Forced external; quarantine aggressively |
| App verification and publish-key accommodation | App distribution may require signing keys, review gates, verification metadata, and platform policing workflows. | Forced external; keep credentials out of repos and flows explicit |
| Longer edit-run loop | Compiled code can be slower to iterate on than a spell. | Undesired; keep prototyping in wizardry |
| Optional databases | Some workloads need transactions, query acceleration, replication, or frequent concurrent writes. | Conditional; files remain default truth |
| Derived indexes and caches | Enterprise sites need fast lookup and rendering paths. | Settled, but derive deterministically from source files |
| HTTP/runtime servers | High-performance sites may need long-running processes instead of one-shot scripts. | Conditional; preserve static output where possible |
| Dependency risk | Professional Rust/web work can tempt framework and crate sprawl. | Undesired; prefer small free software dependencies |
| Less direct shell composability | Typed runtime actions are faster and safer but less pipe-friendly. | Undesired; preserve meaningful spell entrypoints and CLI parity |

## Forge And Project Types

Theurgy should not become a generic new App Forge app type. Forge project types answer what is being made:

- cross-platform app
- native desktop app
- native mobile app
- game project
- website project

Theurgy answers what runtime tier backs the project. In Forge, it should appear only where justified:

- native desktop app with runtime `theurgy`
- website project with runtime `theurgy`

Website support in Forge should mean repo and project management, not deployment. Headquarters should remain the deployment controller for Headquarters-managed sites. Forge can create, import, inspect, and manage the source repo; Headquarters can publish, monitor, and operate the live site.

## Tracks

**Native desktop apps** use Rust for the control plane, state model, command dispatch, and platform adapters. Native UI generation can target SwiftUI, GTK, or other platform-owned frontends without routing every action through a chain of shell scripts.

**Enterprise web apps** use Rust for routing, content indexing, static rendering, API handlers, cache invalidation, and deployment artifacts. The web track should stay boring where boring is good: HTTP, files, caches, indexes, and free software components.

## Repository Map

- `src/` contains the Rust runtime and scaffold engine.
- `docs/` contains architecture decisions.
- `docs/platform-quarantine.md` defines where Apple and platform-specific machinery belongs.
- `docs/app-publish-secrets.md` documents protected app publishing credentials quarantined in theurgy.
- `tools/release/` contains quarantined app verification, signing, notarization, TestFlight, App Store, and Play Store release adapters.
- `.tests/release/` preserves adversarial coverage for quarantined app-publishing release adapters.
- `.github/` contains AI-facing standards.
- `spells/` contains user-facing spells.
- `install` installs theurgy at `~/theurgy` and exposes spell wrappers.
- `uninstall` removes the installed wrappers and `~/theurgy` install.

## Storage

Theurgy project source belongs in the project checkout. Runtime state, caches, logs, and build products belong outside the checkout or in ignored paths. Generated project skeletons include local ignore rules for `target/`, `.theurgy-state/`, and logs.
