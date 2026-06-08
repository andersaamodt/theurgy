# theurgy

theurgy is enterprise-level wizardry.

It keeps wizardry's file-first, inspectable, hacker-friendly posture, but moves the parts that become slow or fragile at high complexity into integrated Rust runtimes. The goal is not to deprecate wizardry. The goal is to keep wizardry excellent at prototyping, proof-of-concept work, shell orchestration, and transparent tools while giving complex native desktop applications and high-performance websites a lower-level execution path.

## What Exists Now

This first repository version includes:

- well-named spells backed by a Rust runtime
- project scaffolding for native desktop apps and enterprise web apps
- file-first project manifests
- generated app licensing that follows the Wizardry Apps split
- architecture docs for desktop, web, databases, and Wizardry integration

Run:

```sh
spells/assay-theurgy
spells/conjure-native-desktop sample-desktop
spells/conjure-enterprise-website sample-website
spells/inspect-theurgy-project sample-desktop
```

## Design Commitments

- Wizardry remains the human-facing spell/menu layer where it fits.
- Theurgy replaces script fan-out with Rust when startup, parallelism, IPC, rendering, or state hydration need one integrated runtime.
- Plain files remain the durable source of truth by default.
- Databases are optional acceleration and transaction tools, not the default authority.
- Generated blank projects are AGPL-3.0-or-later plus the Wizardry Addendum.
- Theurgy itself is OWL 3.1, matching the Wizardry project family.

## Wizardry Compromises

Theurgy exists because complex native desktop apps and enterprise-level websites need some capabilities that pure shell-first wizardry does not provide efficiently. These are intentional compromises against the ordinary wizardry way. Keep this list current, and minimize every item.

- **Rust runtime instead of POSIX sh everywhere**: hot paths move into compiled Rust so state hydration, routing, indexing, IPC, and command dispatch do not spawn many shell processes. Minimize this by keeping spells as the user-facing entrypoints and keeping Rust contracts readable.
- **Cargo toolchain**: Cargo is Rust's build and package manager. It builds `theurgy-runtime`, manages `Cargo.lock`, and creates `target/` artifacts. Minimize this by hiding Cargo from normal workflows behind spells and keeping dependency use sparse.
- **Compiled binary opacity**: a compiled runtime is less immediately inspectable than a shell script. Minimize this with small Rust modules, plain text manifests, tests, and documented behavior.
- **Build artifacts and caches**: compiled projects create `target/` and other local outputs that wizardry scripts usually avoid. Minimize this by ignoring artifacts and keeping runtime state outside source trees.
- **Toolchain requirement**: Theurgy needs a Rust toolchain for development, unlike pure wizardry spells that need only POSIX sh plus small system tools. Minimize this by making the required toolchain explicit and avoiding unnecessary external build systems.
- **Stronger schemas and typed contracts**: enterprise support needs manifests, structured snapshots, and typed actions rather than loose shell text flows. Minimize this by keeping formats plain text, stable, and hand-editable.
- **Generated native code**: desktop support may emit SwiftUI, GTK, or other platform-native sources. Minimize this by making generated output reproducible and keeping the source manifest authoritative.
- **Platform-specific adapters**: native desktop quality requires platform-owned behavior for windows, menus, IPC, and app lifecycle. Minimize this by keeping adapters thin over one shared runtime model.
- **Longer edit-run loop**: compiled code can be slower to iterate on than editing a spell. Minimize this by keeping prototyping in wizardry and using Theurgy only when integration/performance pressure justifies it.
- **Optional databases**: some enterprise cases need transactions, query acceleration, replication, or high-frequency user/session writes. Minimize this by making files the default source of truth and treating databases as optional indexes or transaction layers.
- **Derived indexes and caches**: enterprise websites need fast lookup and rendering paths. Minimize this by deriving them deterministically from source files and making rebuild behavior explicit.
- **HTTP/runtime servers**: high-performance websites may require long-running local or deployed processes rather than one-shot scripts. Minimize this by keeping configuration file-first and preserving static output where possible.
- **Dependency risk**: enterprise-quality Rust and web work can tempt framework and crate sprawl. Minimize this by preferring small free software dependencies, documenting every dependency class, and avoiding proprietary or commercial-enclosure assumptions.
- **Less direct shell composability**: typed runtime actions are safer and faster but less free-form than piping spells together. Minimize this by exposing meaningful spell entrypoints and preserving CLI parity for GUI behavior.

## Tracks

**Native desktop apps** use Rust for the control plane, state model, command dispatch, and platform adapters. Native UI generation can target SwiftUI, GTK, or other platform-owned frontends without routing every action through a chain of shell scripts.

**Enterprise web apps** use Rust for routing, content indexing, static rendering, API handlers, and deployment artifacts. The web track is intentionally boring where boring is good: HTTP, files, caches, indexes, and free software components.

## Repository Map

- `src/` contains the Rust runtime and scaffold engine.
- `docs/` contains architecture decisions.
- `.github/` contains AI-facing standards.
- `spells/` contains user-facing spells.

## Storage

Theurgy project source belongs in the project checkout. Runtime state, caches, logs, and build products belong outside the checkout or in ignored paths. Generated project skeletons include local ignore rules for `target/`, `.theurgy-state/`, and logs.
