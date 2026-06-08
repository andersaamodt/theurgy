# theurgy

theurgy is enterprise-level wizardry.

It applies the wizardry ethos to professional-quality native desktop applications and enterprise-level websites. It does not replace wizardry. It protects wizardry by keeping wizardry pure, shell-first, minimal, file-first, and excellent for prototyping, proof-of-concept work, transparent UNIX workflows, and small-to-medium tools.

Theurgy exists for the places where that purity runs into hard performance and integration pressure: complex native desktop apps and scalable, reliable, fast websites. In those places, Theurgy makes the minimum necessary compromises against minimalism and toward institutional software practice, then standardizes those compromises so they stay inspectable, reusable, and as free/open as possible.

## What Theurgy Is For

Use Theurgy when the work needs at least one of these:

- a complex native desktop app with many moving parts
- a resident runtime instead of repeated shell process fan-out
- fast state hydration, indexing, routing, IPC, or event dispatch
- platform-native desktop behavior that cannot be cleanly faked in shell
- a professional app surface where startup, lifecycle, and responsiveness matter
- an enterprise-level website with high traffic, low latency, reliable routing, or derived indexes
- optional transaction/query layers that plain files cannot satisfy cleanly

A Rust-backed native desktop app can be solid in a way that an app backed by many shell scripts cannot quite be. Rust gives Theurgy one integrated runtime for state, actions, IPC, rendering preparation, and platform adapters, while spells remain the user-facing way into the system.

For websites, Theurgy should grow a Rust-based web runtime: file-first where possible, static-first where possible, but able to handle high-performance routing, rendering, indexing, caching, and optional database-backed transaction layers when enterprise-level reliability requires them.

## What Wizardry Is Still For

Use ordinary wizardry when the work is:

- a prototype
- a proof of concept
- a menu-driven UNIX workflow
- a small or medium app that does not suffer from process fan-out
- a shell orchestration task
- a file transformation pipeline
- an installer, repair tool, audit, probe, or admin spell
- a website that can remain simple, static, shell-built, and Headquarters-managed
- any workflow where POSIX sh keeps the system more understandable than a compiled runtime

Default to wizardry. Escalate to Theurgy only when integration, responsiveness, scale, or platform quality actually justifies the compromise.

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

Install locally:

```sh
./install
```

The install path is `${THEURGY_HOME:-$HOME/theurgy}`. The installer also writes user-local wrappers so wizardry spells can call Theurgy spells by name, such as `assay-theurgy` or `conjure-native-desktop`.

## Design Commitments

- Wizardry remains the human-facing spell/menu layer where it fits.
- Theurgy replaces shell fan-out with Rust only when startup, parallelism, IPC, rendering, routing, or state hydration need one integrated runtime.
- Spells remain the normal user-facing interface.
- Plain files remain the durable source of truth by default.
- Databases are optional acceleration, transaction, and replication tools, not the default authority.
- Free software and open formats are preferred over proprietary frameworks, commercial enclosures, and closed platform assumptions.
- Institutional cruft is quarantined, named, documented, and minimized.
- Generated blank projects are AGPL-3.0-or-later plus the Wizardry Addendum.
- Theurgy itself is OWL 3.1, matching the Wizardry project family.

## Institutional Cruft Quarantine

Theurgy should be the quarantine layer for the parts of professional app development that do not belong in pure wizardry.

That includes:

- macOS bundle structure
- code signing and notarization workflows
- app verification, app-store review/policing workflows, and special publish keys
- Swift, SwiftUI, Xcode-shaped project output, and Apple lifecycle conventions
- GTK, platform windowing, desktop IPC, and app lifecycle adapters
- Rust build products, lockfiles, and toolchain metadata
- web server runtimes, caches, indexes, and schema migrations
- optional database layers
- generated native source trees

This does not mean Theurgy should celebrate institutional complexity. It means Theurgy should contain it. The rule is:

- keep wizardry pure when possible
- keep wizardry-apps pure script-first and free of direct Rust, Cargo, Swift, signing, notarization, app verification/policing, special app-publish keys, and Apple-specific implementation machinery
- put unavoidable platform-specific or enterprise-specific machinery in Theurgy
- wrap that machinery with spells
- keep source files and manifests plain
- prefer free/open alternatives wherever they can meet the quality bar
- document every compromise and every workaround

Apple-specific code is a good example. A professional macOS app may need SwiftUI, bundle metadata, signing, launch behavior, menu behavior, lifecycle hooks, verification, notarization, review gates, and publishing keys. Those details should not pollute wizardry or wizardry-apps. Theurgy can own the Apple adapter and keep it behind a narrow, documented boundary, while the app's durable project truth remains file-first and portable where possible.

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
- **Apple-language and closed-platform accommodation**: professional macOS support may require Swift, SwiftUI, app bundle metadata, signing, notarization, and other Apple-specific conventions. Minimize this by quarantining Apple-specific code under Theurgy adapters and never moving that machinery into pure wizardry.
- **App verification and publish-key accommodation**: professional app distribution may require signing keys, notarization credentials, store review gates, app verification metadata, and platform policing workflows. Minimize this by keeping credentials out of repos, keeping publishing flows explicit, and quarantining those platform-specific release surfaces in Theurgy.
- **Longer edit-run loop**: compiled code can be slower to iterate on than editing a spell. Minimize this by keeping prototyping in wizardry and using Theurgy only when integration/performance pressure justifies it.
- **Optional databases**: some enterprise cases need transactions, query acceleration, replication, or high-frequency user/session writes. Minimize this by making files the default source of truth and treating databases as optional indexes or transaction layers.
- **Derived indexes and caches**: enterprise websites need fast lookup and rendering paths. Minimize this by deriving them deterministically from source files and making rebuild behavior explicit.
- **HTTP/runtime servers**: high-performance websites may require long-running local or deployed processes rather than one-shot scripts. Minimize this by keeping configuration file-first and preserving static output where possible.
- **Dependency risk**: enterprise-quality Rust and web work can tempt framework and crate sprawl. Minimize this by preferring small free software dependencies, documenting every dependency class, and avoiding proprietary or commercial-enclosure assumptions.
- **Less direct shell composability**: typed runtime actions are safer and faster but less free-form than piping spells together. Minimize this by exposing meaningful spell entrypoints and preserving CLI parity for GUI behavior.

## Forge And Project Types

Theurgy should not become a generic new App Forge app type.

Forge project types answer what is being made:

- cross-platform app
- native desktop app
- native mobile app
- game project
- website project

Theurgy answers what runtime tier backs the project. It is an orthogonal dimension. In Forge, Theurgy should appear only where it is justified:

- native desktop app with runtime `theurgy`
- website project with runtime `theurgy`

Website support in Forge should mean repo and project management, not deployment. Headquarters should remain the deployment controller for Headquarters-managed sites. Forge can create, import, inspect, and manage the source repo; Headquarters can publish, monitor, and operate the live site.

## Tracks

**Native desktop apps** use Rust for the control plane, state model, command dispatch, and platform adapters. Native UI generation can target SwiftUI, GTK, or other platform-owned frontends without routing every action through a chain of shell scripts.

**Enterprise web apps** use Rust for routing, content indexing, static rendering, API handlers, cache invalidation, and deployment artifacts. The web track should remain boring where boring is good: HTTP, files, caches, indexes, and free software components.

## Repository Map

- `src/` contains the Rust runtime and scaffold engine.
- `docs/` contains architecture decisions.
- `docs/platform-quarantine.md` defines where Apple and platform-specific machinery belongs.
- `docs/app-publish-secrets.md` documents protected app publishing credentials now quarantined in Theurgy.
- `tools/release/` contains quarantined app verification, signing, notarization, TestFlight, App Store, and Play Store release adapters.
- `.github/` contains AI-facing standards.
- `spells/` contains user-facing spells.
- `install` installs Theurgy at `~/theurgy` and exposes spell wrappers.
- `uninstall` removes the installed wrappers and `~/theurgy` install.

## Storage

Theurgy project source belongs in the project checkout. Runtime state, caches, logs, and build products belong outside the checkout or in ignored paths. Generated project skeletons include local ignore rules for `target/`, `.theurgy-state/`, and logs.
