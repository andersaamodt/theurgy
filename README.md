# theurgy

theurgy is enterprise-level wizardry.

It keeps wizardry's file-first, inspectable, hacker-friendly posture, but moves the parts that become slow or fragile at high complexity into integrated Rust runtimes. The goal is not to deprecate wizardry. The goal is to keep wizardry excellent at prototyping, proof-of-concept work, shell orchestration, and transparent tools while giving complex native desktop applications and high-performance websites a lower-level execution path.

## What Exists Now

This first repository version includes:

- a Rust `theurgy` CLI
- project scaffolding for native desktop apps and enterprise web apps
- file-first project manifests
- generated app licensing that follows the Wizardry Apps split
- architecture docs for desktop, web, databases, and Wizardry integration

Run:

```sh
cargo run -- doctor
cargo run -- new desktop sample-desktop /tmp/sample-desktop
cargo run -- new website sample-website /tmp/sample-website
cargo run -- inspect /tmp/sample-desktop
```

## Design Commitments

- Wizardry remains the human-facing spell/menu layer where it fits.
- Theurgy replaces script fan-out with Rust when startup, parallelism, IPC, rendering, or state hydration need one integrated runtime.
- Plain files remain the durable source of truth by default.
- Databases are optional acceleration and transaction tools, not the default authority.
- Generated blank projects are AGPL-3.0-or-later plus the Wizardry Addendum.
- Theurgy itself is OWL 3.1, matching the Wizardry project family.

## Tracks

**Native desktop apps** use Rust for the control plane, state model, command dispatch, and platform adapters. Native UI generation can target SwiftUI, GTK, or other platform-owned frontends without routing every action through a chain of shell scripts.

**Enterprise web apps** use Rust for routing, content indexing, static rendering, API handlers, and deployment artifacts. The web track is intentionally boring where boring is good: HTTP, files, caches, indexes, and free software components.

## Repository Map

- `src/` contains the Rust CLI and scaffold engine.
- `docs/` contains architecture decisions.
- `.github/` contains AI-facing standards.

## Storage

Theurgy project source belongs in the project checkout. Runtime state, caches, logs, and build products belong outside the checkout or in ignored paths. Generated project skeletons include local ignore rules for `target/`, `.theurgy-state/`, and logs.

